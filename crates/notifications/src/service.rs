use crate::CloseReason;
use crate::data::ServiceData;
use crate::dbus::{DBusService, DBusServiceSignals};
use crate::error::{NotificationServiceError, Result};
use crate::notification::NotificationHandle;
use crate::signals::NotificationServiceSignal;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::sync::mpsc;
use zbus::Connection;
use zbus::connection::Builder;
use zbus::object_server::InterfaceRef;

#[derive(Default)]
pub(crate) struct NotificationServiceInner {
    pub(crate) data: ServiceData,
    pub(crate) connection: OnceLock<Option<Connection>>,
    pub(crate) outer_tx: Option<mpsc::Sender<NotificationServiceSignal>>,
    pub(crate) cache_dir: Option<PathBuf>,
}

#[derive(Default, Clone)]
pub struct NotificationService {
    pub(crate) inner: Arc<NotificationServiceInner>,
}

impl NotificationService {
    pub fn new(
        outer_tx: Option<mpsc::Sender<NotificationServiceSignal>>,
        cache_dir: Option<PathBuf>,
    ) -> Result<Self> {
        Ok(Self {
            inner: Arc::new(NotificationServiceInner {
                data: ServiceData::new(cache_dir.clone())?,
                connection: OnceLock::new(),
                outer_tx,
                cache_dir,
            }),
        })
    }

    pub fn new_in_memory(outer_tx: Option<mpsc::Sender<NotificationServiceSignal>>) -> Self {
        Self {
            inner: Arc::new(NotificationServiceInner {
                data: ServiceData::new_in_memory(),
                connection: OnceLock::new(),
                outer_tx,
                cache_dir: None,
            }),
        }
    }

    pub async fn run(&self) -> Result<()> {
        let service = DBusService::new(self.clone())?;

        let connection = Builder::session()?
            .name("org.freedesktop.Notifications")?
            .serve_at("/org/freedesktop/Notifications", service)?
            .build()
            .await?;

        self.inner
            .connection
            .set(Some(connection))
            .map_err(|_| NotificationServiceError::ConnectionInitializedTwice)?;

        Ok(())
    }

    pub(crate) fn get_connection(&self) -> Result<Connection> {
        Ok(self
            .inner
            .connection
            .get()
            .ok_or(NotificationServiceError::NoConnection)?
            .to_owned()
            .ok_or(NotificationServiceError::NoConnection)?)
    }

    async fn get_dbus_interface(&self) -> Result<InterfaceRef<DBusService>> {
        Ok(self
            .get_connection()?
            .object_server()
            .interface("/org/freedesktop/Notifications")
            .await?)
    }

    pub async fn dismiss_notification(&self, id: u32) -> Result<()> {
        self.get_dbus_interface()
            .await?
            .notification_closed(id, CloseReason::Dismissed.into())
            .await?;

        self.inner.data.remove_notification(id)?;

        Ok(())
    }

    pub async fn invoke_action(&self, notification_id: u32, action_key: &str) -> Result<()> {
        self.get_dbus_interface()
            .await?
            .action_invoked(notification_id, action_key)
            .await?;

        Ok(())
    }

    pub fn get_notifications(&self) -> Vec<NotificationHandle> {
        self.inner
            .data
            .get_notifications()
            .values()
            .map(|n| NotificationHandle {
                inner: Arc::clone(n),
                service: self.clone(),
            })
            .collect()
    }

    pub fn get_notification_by_id(&self, id: u32) -> Option<NotificationHandle> {
        self.inner
            .data
            .get_notifications()
            .get(&id)
            .map(|n| NotificationHandle {
                inner: n.clone(),
                service: self.clone(),
            })
    }

    pub async fn clear_notifications(&self) -> Result<()> {
        for id in self.inner.data.get_notifications().keys() {
            self.get_dbus_interface()
                .await?
                .notification_closed(id.to_owned(), CloseReason::Dismissed.into())
                .await?;
        }

        self.inner.data.clear()
    }
}

// Run with `dbus-run-session cargo test -- --test-threads=1`
// WARNING: must be run serially to avoid D-Bus name conflicts
#[cfg(test)]
mod tests {

    use super::*;
    use std::{collections::HashMap, time::Duration};

    use crate::Urgency;

    use fake::Fake;
    use fake::faker::lorem::en::Sentence;
    use notify_rust::Urgency as ClientUrgency;
    use notify_rust::{CloseReason, Notification, NotificationHandle, NotificationResponse};
    use rand::seq::IndexedRandom;
    use tempfile::TempDir;
    use tokio::sync::oneshot;

    impl From<Urgency> for ClientUrgency {
        fn from(value: Urgency) -> Self {
            match value {
                Urgency::Low => ClientUrgency::Low,
                Urgency::Normal => ClientUrgency::Normal,
                Urgency::Critical => ClientUrgency::Critical,
            }
        }
    }

    struct TestContext {
        _temp_dir: TempDir,
        service: NotificationService,
    }

    fn no_tmp_cleanup() -> bool {
        std::env::var_os("NO_TMP_CLEANUP")
            .map(|v| v == "1")
            .unwrap_or(false)
    }

    fn create_random_notification() -> Notification {
        let summary: String = Sentence(3..6).fake();
        let body: String = Sentence(6..12).fake();
        let app_name: String = Sentence(1..3).fake();
        let icon: String = String::from("cat-sleeping-symbolic");

        let mut notification = Notification::new();

        notification
            .appname(&app_name)
            .summary(&summary)
            .body(&body)
            .icon(&icon);

        notification
    }

    async fn send_multiple_random_notifications(quantity: u32) -> HashMap<u32, NotificationHandle> {
        let mut map: HashMap<u32, NotificationHandle> = HashMap::new();

        for _ in 0..quantity {
            let handle = create_random_notification().show_async().await.unwrap();
            let id = handle.id();
            map.insert(id, handle);
        }

        map
    }

    async fn setup_with_details(temp_dir: Option<TempDir>) -> TestContext {
        let mut temp_dir = temp_dir.unwrap_or_else(|| TempDir::new().unwrap());

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

    async fn setup() -> TestContext {
        setup_with_details(None).await
    }

    #[tokio::test]
    async fn test_single_notification() {
        let ctx = setup().await;

        let client_urgency_levels = [
            ClientUrgency::Low,
            ClientUrgency::Normal,
            ClientUrgency::Critical,
        ];

        let client_urgency: ClientUrgency = client_urgency_levels
            .choose(&mut rand::rng())
            .unwrap()
            .to_owned();

        let handle = create_random_notification()
            .urgency(client_urgency)
            .show_async()
            .await
            .unwrap();

        let notification = ctx.service.get_notification_by_id(handle.id()).unwrap();

        assert_eq!(handle.appname, notification.app_name());
        assert_eq!(handle.icon, notification.icon().unwrap());
        assert_eq!(handle.summary, notification.summary());
        assert_eq!(handle.body, notification.body());
        assert_eq!(client_urgency, notification.urgency().into());
        assert_eq!(i32::from(handle.timeout), notification.timeout());
    }

    #[tokio::test]
    async fn test_multiple_notifications() {
        let ctx = setup().await;
        send_multiple_random_notifications(50).await;

        assert!(ctx.service.get_notifications().is_sorted_by_key(|x| x.id()));
        assert_eq!(ctx.service.get_notifications().len(), 50);
    }

    #[tokio::test]
    async fn test_dismiss_notification() {
        let ctx = setup().await;
        let handle = create_random_notification().show_async().await.unwrap();
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

        ctx.service.dismiss_notification(id).await.unwrap();

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

        let n = ctx.service.get_notification_by_id(id).unwrap();
        assert_eq!(n.actions().len(), 2);

        for action in n.actions() {
            if action.action_key() == "asked" {
                action.invoke().await.unwrap();
            }
        }

        let action_key = rx.await.unwrap();
        assert_eq!(action_key, "asked")
    }

    #[tokio::test]
    async fn test_clear_notifications() {
        let ctx = setup().await;

        send_multiple_random_notifications(10).await;

        assert_eq!(ctx.service.get_notifications().len(), 10);
        ctx.service.clear_notifications().await.unwrap();

        assert_eq!(ctx.service.get_notifications().len(), 0);
    }

    #[tokio::test]
    async fn test_image_data() {}

    #[tokio::test]
    async fn test_timeout() {}
}
