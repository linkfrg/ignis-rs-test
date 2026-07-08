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
        self.data.write().unwrap().clear()
    }
}

// Run with `dbus-run-session cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    use fake::Fake;
    use fake::faker::lorem::en::Sentence;
    use notify_rust::{Notification, NotificationHandle, Urgency};
    use rand::seq::IndexedRandom;
    use tempfile::TempDir;

    async fn send_random_notification() -> NotificationHandle {
        let mut rng = rand::rng();

        let summary: String = Sentence(3..6).fake();
        let body: String = Sentence(6..12).fake();
        let app_name: String = Sentence(1..3).fake();
        let icon: String = String::from("cat-sleeping-symbolic");

        // Urgency levels
        // 0 - Low
        // 1 - Normal
        // 2 - Critical
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
            .show()
            .unwrap()
    }

    async fn run_service() -> NotificationService {
        let service = NotificationService::new(None, None).unwrap();
        service.run().await.unwrap();

        service
    }

    #[tokio::test]
    async fn test_notification() {
        let service = run_service().await;
        let test_notification = send_random_notification().await;

        let notification = service
            .get_notification_by_id(test_notification.id())
            .unwrap();

        assert_eq!(test_notification.appname, notification.app_name);
        // TODO: check icon in the separate test
        assert_eq!(test_notification.icon, notification.icon.unwrap());
        assert_eq!(test_notification.summary, notification.summary);
        assert_eq!(test_notification.body, notification.body);
        // assert_eq!(test_notification.urgency, notification.urgency);
        assert_eq!(i32::from(test_notification.timeout), notification.timeout);
    }

    #[tokio::test]
    async fn test_get_notification() {
        let service = run_service().await;
        let handle = send_random_notification().await;

        assert!(service.get_notifications().is_sorted_by_key(|x| x.id));
        assert!(service.get_notification_by_id(handle.id()).is_some());
        assert_eq!(service.get_notifications().last().unwrap().id, handle.id());
    }

    #[tokio::test]
    async fn test_close_notification() {}

    #[tokio::test]
    async fn test_invoke_action() {}

    #[tokio::test]
    async fn test_clear_notifications() {}

    #[tokio::test]
    async fn test_image_data() {}

    #[tokio::test]
    async fn test_timeout() {}
}
