use crate::DesktopNotification;
use crate::data::ServiceData;
use crate::dbus::{DBusService, DBusServiceSignals};
use crate::error::{NotificationServiceError, Result};
use crate::signals::NotificationServiceSignal;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use zbus::Connection;
use zbus::connection::Builder;
use zbus::object_server::InterfaceRef;

pub struct NotificationService {
    data: Arc<RwLock<ServiceData>>,
    connection: Mutex<Option<Connection>>,
    outer_tx: Option<mpsc::Sender<NotificationServiceSignal>>,
    cache_dir: Option<PathBuf>,
}

impl NotificationService {
    pub fn new(
        outer_tx: Option<mpsc::Sender<NotificationServiceSignal>>,
        cache_dir: Option<PathBuf>,
    ) -> Result<Self> {
        Ok(Self {
            data: Arc::new(RwLock::new(ServiceData::new(cache_dir.clone())?)),
            connection: Mutex::new(None),
            outer_tx,
            cache_dir,
        })
    }

    pub fn new_in_memory(outer_tx: Option<mpsc::Sender<NotificationServiceSignal>>) -> Self {
        Self {
            data: Arc::new(RwLock::new(ServiceData::new_in_memory())),
            connection: Mutex::new(None),
            outer_tx,
            cache_dir: None,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let service = DBusService::new(
            Arc::clone(&self.data),
            self.outer_tx.clone(),
            self.cache_dir.clone(),
        )?;

        let connection = Builder::session()?
            .name("org.freedesktop.Notifications")?
            .serve_at("/org/freedesktop/Notifications", service)?
            .build()
            .await?;

        *self.connection.lock().await = Some(connection);

        Ok(())
    }

    async fn get_dbus_interface(&self) -> Result<InterfaceRef<DBusService>> {
        Ok(self
            .connection
            .lock()
            .await
            .as_ref()
            .ok_or(NotificationServiceError::NoConnection)?
            .object_server()
            .interface("/org/freedesktop/Notifications")
            .await?)
    }

    pub async fn close_notification(&self, id: u32) -> Result<()> {
        self.get_dbus_interface()
            .await?
            .notification_closed(id, 2)
            .await?;

        self.data.write().unwrap().remove_notification(id)?;

        Ok(())
    }

    pub async fn invoke_action(&self, notification_id: u32, action_key: &str) -> Result<()> {
        self.get_dbus_interface()
            .await?
            .action_invoked(notification_id, action_key)
            .await?;

        Ok(())
    }

    pub fn get_notifications(&self) -> Vec<DesktopNotification> {
        self.data
            .read()
            .unwrap()
            .notifications
            .values()
            .cloned()
            .collect()
    }

    pub fn get_notification_by_id(&self, id: u32) -> Option<DesktopNotification> {
        self.data.read().unwrap().notifications.get(&id).cloned()
    }

    pub fn clear_notifications(&self) -> Result<()> {
        // FIXME: it should emit NotificationClosed D-Bus signal for each notification
        self.data.write().unwrap().clear()
    }
}

// Run with `dbus-run-session cargo test -- --test-threads=1`
// WARNING: must be run serially to avoid D-Bus name conflicts
#[cfg(test)]
mod tests {
    use std::{collections::HashMap, time::Duration};

    use super::*;

    use fake::Fake;
    use fake::faker::lorem::en::Sentence;
    use notify_rust::{
        CloseReason, Notification, NotificationHandle, NotificationResponse, Urgency,
    };
    use rand::seq::IndexedRandom;
    use tempfile::TempDir;
    use tokio::sync::oneshot;

    struct TestContext {
        _temp_dir: TempDir,
        service: NotificationService,
    }

    fn no_tmp_cleanup() -> bool {
        std::env::var_os("NO_TMP_CLEANUP")
            .map(|v| v == "1")
            .unwrap_or(false)
    }

    async fn send_random_notification() -> NotificationHandle {
        let mut rng = rand::rng();

        let summary: String = Sentence(3..6).fake();
        let body: String = Sentence(6..12).fake();
        let app_name: String = Sentence(1..3).fake();
        let icon: String = String::from("cat-sleeping-symbolic");

        let urgency_levels = [Urgency::Low, Urgency::Normal, Urgency::Critical];
        let urgency: Urgency = urgency_levels.choose(&mut rng).unwrap().to_owned();

        // If -1 - expiration time is dependent on the server's settings
        // If 0 - never expire
        // >0 - timeout time in milliseconds

        let timeouts = [-1, 0, 500, 1000];
        let timeout: i32 = timeouts.choose(&mut rng).unwrap().to_owned();

        Notification::new()
            .appname(&app_name)
            .summary(&summary)
            .body(&body)
            .icon(&icon)
            .timeout(timeout)
            .urgency(urgency)
            .show_async()
            .await
            .unwrap()
    }

    async fn send_multiple_random_notifications(quantity: u32) -> HashMap<u32, NotificationHandle> {
        let mut map: HashMap<u32, NotificationHandle> = HashMap::new();

        for _ in 0..quantity {
            let handle = send_random_notification().await;
            let id = handle.id();
            map.insert(id, handle);
        }

        map
    }

    async fn setup() -> TestContext {
        let mut temp_dir = TempDir::new().unwrap();

        if no_tmp_cleanup() {
            temp_dir.disable_cleanup(true);
        }

        let service = NotificationService::new(None, Some(temp_dir.path().to_path_buf())).unwrap();
        service.run().await.unwrap();

        TestContext {
            _temp_dir: temp_dir,
            service,
        }
    }

    #[tokio::test]
    async fn test_single_notification() {
        let ctx = setup().await;
        let test_notification = send_random_notification().await;

        let notification = ctx
            .service
            .get_notification_by_id(test_notification.id())
            .unwrap();

        assert_eq!(test_notification.appname, notification.app_name);
        assert_eq!(test_notification.icon, notification.icon.unwrap());
        assert_eq!(test_notification.summary, notification.summary);
        assert_eq!(test_notification.body, notification.body);
        // assert_eq!(test_notification.urgency, notification.urgency);
        assert_eq!(i32::from(test_notification.timeout), notification.timeout);
    }

    #[tokio::test]
    async fn test_multiple_notifications() {
        let ctx = setup().await;
        send_multiple_random_notifications(50).await;

        assert!(ctx.service.get_notifications().is_sorted_by_key(|x| x.id));
        assert_eq!(ctx.service.get_notifications().len(), 50);
    }

    #[tokio::test]
    async fn test_close_notification() {
        let ctx = setup().await;
        let handle = send_random_notification().await;
        let id = handle.id();

        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            handle
                .wait_for_action_async(|response| {
                    match response {
                        NotificationResponse::Closed(reason) => tx.send(reason.to_owned()).unwrap(),
                        _ => unimplemented!(),
                    };
                })
                .await;
        });
        // FIXME: Hacky workaround to prevent the test from hanging
        // For some reason calling NotificationService.close_notification() immediately
        // makes the handle "miss" the signal and therefore never call the closure
        tokio::time::sleep(Duration::from_millis(1)).await;

        // TODO: maybe rename it to "dismiss_notification()"..?
        ctx.service.close_notification(id).await.unwrap();

        let close_reason = rx.await.unwrap();
        assert_eq!(close_reason, CloseReason::Dismissed);
    }

    #[tokio::test]
    async fn test_invoke_action() {
        let ctx = setup().await;
        let handle = Notification::new()
            .summary("i am waiting")
            .action("default", "default")
            .action("asked", "no one asked")
            .show_async()
            .await
            .unwrap();
        let id = handle.id();

        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            handle
                .wait_for_action_async(|response| {
                    match response {
                        NotificationResponse::Action(action) => {
                            tx.send(action.clone()).unwrap();
                        }
                        _ => unimplemented!(),
                    };
                })
                .await;
        });

        // FIXME: the same here
        tokio::time::sleep(Duration::from_millis(1)).await;

        // TODO: implement NotificationAction object
        ctx.service.invoke_action(id, "asked").await.unwrap();
        let action_key = rx.await.unwrap();
        assert_eq!(action_key, "asked")
    }

    #[tokio::test]
    async fn test_clear_notifications() {
        let ctx = setup().await;

        send_multiple_random_notifications(10).await;

        assert_eq!(ctx.service.get_notifications().len(), 10);
        ctx.service.clear_notifications().unwrap();

        assert_eq!(ctx.service.get_notifications().len(), 0);
    }

    #[tokio::test]
    async fn test_image_data() {}

    #[tokio::test]
    async fn test_timeout() {}
}
