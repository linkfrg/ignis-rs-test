mod gerror;
mod imp;
use gerror::GIgnisNotificationsError;
use glib::Object;
use glib::subclass::prelude::*;
use notification_service::{Notification, NotificationService, NotificationServiceSignal};
use tokio::sync::MutexGuard;

glib::wrapper! {
    pub struct DesktopNotification(ObjectSubclass<imp::DesktopNotification>);
}

glib::wrapper! {
    pub struct IgnisNotifications(ObjectSubclass<imp::IgnisNotifications>);
}

fn handler(signal: NotificationServiceSignal) {
    match signal {
        NotificationServiceSignal::Closed { id } => println!("closed {id}"),
        NotificationServiceSignal::Notified { id, replace } => {
            println!("new notified {} {}", id, replace)
        }
    }
}

impl DesktopNotification {
    fn from_rust(notification: Notification) -> Self {
        let obj: Self = Object::builder().build();

        let imp = imp::DesktopNotification::from_obj(&obj);
        *imp.notification.borrow_mut() = Some(notification);

        obj
    }
}

impl IgnisNotifications {
    pub fn new() -> Self {
        Object::builder().build()
    }

    async fn get_service(&self) -> MutexGuard<'_, NotificationService> {
        let self_ = imp::IgnisNotifications::from_obj(self);
        self_.service.lock().await
    }

    pub async fn run(&self) -> Result<(), glib::Error> {
        self.get_service()
            .await
            .run_with_handler(handler)
            .await
            .map_err(|e| {
                let msg = &e.to_string();
                glib::Error::new(GIgnisNotificationsError::from(e), msg)
            })?;

        Ok(())
    }

    pub async fn get_notifications(&self) -> Vec<DesktopNotification> {
        self.get_service()
            .await
            .get_notifications()
            .await
            .into_iter()
            .map(|i| DesktopNotification::from_rust(i))
            .collect()
    }

    pub async fn close_notification(&self, id: &u32) -> Result<(), glib::Error> {
        self.get_service()
            .await
            .close_notification(id)
            .await
            .map_err(|e| {
                let msg = &e.to_string();
                glib::Error::new(GIgnisNotificationsError::from(e), msg)
            })
    }
}
