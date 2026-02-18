use crate::Notification;
use crate::data::ServiceData;
use crate::dbus::{DBusService, DBusServiceSignals};
use crate::error::{NotificationServiceError, Result};
use crate::signals::{NotificationServiceSignal, SignalHelper};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use zbus::Connection;
use zbus::connection::Builder;
use zbus::object_server::InterfaceRef;

pub struct NotificationService {
    data: Arc<Mutex<ServiceData>>,
    connection: Option<Connection>,
    rx: Arc<Mutex<mpsc::Receiver<NotificationServiceSignal>>>,
    tx: mpsc::Sender<NotificationServiceSignal>,
}

impl NotificationService {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(32);

        Self {
            data: Arc::new(Mutex::new(ServiceData::new())),
            connection: None,
            rx: Arc::new(Mutex::new(rx)),
            tx: tx.clone(),
        }
    }

    pub async fn run_with_handler<F>(&mut self, message_handler: F) -> Result<()>
    where
        F: FnMut(NotificationServiceSignal) + Send + 'static,
    {
        let service = DBusService::new(self.data.clone(), self.tx.clone());

        let connection = Builder::session()?
            .name("org.freedesktop.Notifications")?
            .serve_at("/org/freedesktop/Notifications", service)?
            .build()
            .await?;

        self.connection = Some(connection);

        self.listen_signals(message_handler).await;

        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        self.run_with_handler(|_| {}).await
    }

    async fn listen_signals<F>(&mut self, mut message_handler: F)
    where
        F: FnMut(NotificationServiceSignal) + Send + 'static,
    {
        let rx = Arc::clone(&self.rx);

        tokio::spawn(async move {
            let mut rx = rx.lock().await;

            while let Some(signal) = rx.recv().await {
                message_handler(signal)
            }
        })
        .await
        .unwrap();
    }

    async fn get_dbus_interface(&self) -> Result<InterfaceRef<DBusService>> {
        Ok(self
            .connection
            .as_ref()
            .ok_or(NotificationServiceError::NoConnection)?
            .object_server()
            .interface("/org/freedesktop/Notifications")
            .await?)
    }

    pub async fn close_notification(&mut self, id: &u32) -> Result<()> {
        self.get_dbus_interface()
            .await?
            .notification_closed(id, &2) //
            .await?;

        self.data.lock().await.remove_notification(id);

        SignalHelper::send_closed(&self.tx, id).await;

        Ok(())
    }

    pub async fn invoke_action(&self, notification_id: &u32, action_key: &String) -> Result<()> {
        self.get_dbus_interface()
            .await?
            .action_invoked(notification_id, action_key)
            .await?;

        Ok(())
    }

    pub async fn get_notifications(&self) -> Vec<Notification> {
        self.data
            .lock()
            .await
            .notifications
            .values()
            .cloned()
            .collect()
    }
}

impl Default for NotificationService {
    fn default() -> Self {
        Self::new()
    }
}
