use crate::NotificationServiceSignal;
use crate::constants::IMAGE_DIR;
use crate::data::ServiceData;
use crate::notification::Notification;
use gdk_pixbuf::{Colorspace, Pixbuf};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use zbus::fdo;
use zbus::object_server::SignalEmitter;
use zbus::{interface, zvariant::OwnedValue};
use zvariant::{Array, Structure};

pub struct DBusService {
    data: Arc<Mutex<ServiceData>>,
    tx: mpsc::Sender<NotificationServiceSignal>,
}

#[interface(name = "org.freedesktop.Notifications")]
impl DBusService {
    async fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: Vec<String>,
        hints: HashMap<String, OwnedValue>,
        timeout: i32,
    ) -> u32 {
        let id: u32;
        let mut data = self.data.lock().await;

        let replace = replaces_id != 0;

        if replace {
            id = replaces_id;
        } else {
            data.counter += 1;
            id = data.counter;
        }

        let new_notification = Notification {
            id: id,
            app_name: app_name.to_string(),
            icon: self.get_icon(app_icon, &hints, id),
            summary: summary.to_string(),
            body: body.to_string(),
            actions: actions,
            urgency: hints
                .get("urgency")
                .and_then(|v| v.downcast_ref::<u8>().ok())
                .unwrap_or(0),
            timeout: timeout,
        };

        data.add_notification(id, new_notification, replace);

        let _ = self.tx
            .send(NotificationServiceSignal::Notified { id, replace })
            .await;

        return id;
    }

    async fn get_server_information(&self) -> (&str, &str, &str, &str) {
        ("Ignis Notification Service", "linkfrg", "1.0", "1.2")
    }

    async fn get_capabilities(&self) -> &[&str] {
        &["actions", "body", "icon-static", "persistence"]
    }

    async fn close_notification(
        &self,
        id: u32,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) -> fdo::Result<()> {
        emitter.notification_closed(id, 3).await?;
        let mut data = self.data.lock().await;
        data.remove_notification(id);

        let _ = self.tx.send(NotificationServiceSignal::Closed { id }).await;

        Ok(())
    }

    // Reason:
    // 1 - The notification expired.
    // 2 - The notification was dismissed by the user.
    // 3 - The notification was closed by a call to CloseNotification.
    // 4 - Undefined/reserved reasons.
    #[zbus(signal)]
    async fn notification_closed(
        emitter: &SignalEmitter<'_>,
        id: u32,
        reason: u32,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn action_invoked(
        emitter: &SignalEmitter<'_>,
        id: u32,
        action_key: &str,
    ) -> zbus::Result<()>;
}

impl DBusService {
    pub fn new(data: Arc<Mutex<ServiceData>>, tx: mpsc::Sender<NotificationServiceSignal>) -> Self {
        Self { data: data, tx: tx }
    }
    fn save_pixbuf(&self, value: &OwnedValue, notification_id: u32) -> Option<String> {
        let s = value.downcast_ref::<Structure>().ok()?;
        let fields = s.fields();

        let width = fields.get(0)?.downcast_ref::<i32>().ok()?;
        let height = fields.get(1)?.downcast_ref::<i32>().ok()?;
        let rowstride = fields.get(2)?.downcast_ref::<i32>().ok()?;
        let has_alpha = fields.get(3)?.downcast_ref::<bool>().ok()?;
        let bits_per_sample = fields.get(4)?.downcast_ref::<i32>().ok()?;
        // let channels = fields.get(5)?.downcast_ref::<i32>().ok()?;
        let data: Vec<u8> = fields
            .get(6)?
            .downcast_ref::<Array>()
            .ok()?
            .iter()
            .map(|v| u8::try_from(v).unwrap())
            .collect();

        let path = (&*IMAGE_DIR).join(notification_id.to_string());

        Pixbuf::from_bytes(
            &glib::Bytes::from(&data),
            Colorspace::Rgb,
            has_alpha,
            bits_per_sample,
            width,
            height,
            rowstride,
        )
        .savev(&path, "png", &[])
        .ok()?;

        Some(path.to_string_lossy().to_string())
    }

    fn get_icon(
        &self,
        app_icon: &str,
        hints: &HashMap<String, OwnedValue>,
        notification_id: u32,
    ) -> Option<String> {
        if let Some(value) = hints.get("image-data") {
            return self.save_pixbuf(value, notification_id);
        }

        if let Some(value) = hints.get("image-path") {
            return value.downcast_ref::<String>().ok();
        }

        if let Some(value) = hints.get("icon_data") {
            return self.save_pixbuf(value, notification_id);
        }

        return Some(app_icon.to_string());
    }
}
