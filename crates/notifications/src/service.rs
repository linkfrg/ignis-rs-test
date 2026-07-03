use crate::DesktopNotification;
use crate::data::ServiceData;
use crate::dbus::{DBusService, DBusServiceSignals};
use crate::error::{NotificationServiceError, Result};
use crate::signals::NotificationServiceSignal;
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
    tx: mpsc::Sender<NotificationServiceSignal>,
}

impl NotificationService {
    pub fn new(outer_tx: Option<mpsc::Sender<NotificationServiceSignal>>) -> Self {
        let (tx, mut rx) = mpsc::channel(32);

        if let Some(outer_tx) = outer_tx {
            tokio::spawn(async move {
                while let Some(signal) = rx.recv().await {
                    let _ = outer_tx.send(signal).await;
                }
            });
        }

        Self {
            data: Arc::new(RwLock::new(ServiceData::new())),
            connection: Mutex::new(None),
            tx,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let service = DBusService::new(Arc::clone(&self.data), self.tx.clone());

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

        self.data.write().unwrap().remove_notification(id);

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

    pub fn clear_notifications(&self) {
        self.data.write().unwrap().clear();
    }
}

impl Default for NotificationService {
    fn default() -> Self {
        Self::new(None)
    }
}

// Run with `dbus-run-session cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    use tokio::process::Command;

    #[tokio::test]
    async fn test_notification() {
        let summary = "test summary 1";
        let body = "test body 1";

        let service = NotificationService::default();
        service.run().await.unwrap();
        println!("After service run");

        let output = Command::new("notify-send")
            .arg(summary)
            .arg(body)
            .output()
            .await
            .expect("failed to execute notify-send");

        assert!(
            output.status.success(),
            "notify-send failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
