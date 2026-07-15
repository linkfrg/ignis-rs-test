use crate::NotificationServiceError;
use crate::dbus::DBusServiceSignals;
use crate::{Result, dbus::get_interface_ref};
use serde::{Deserialize, Serialize};
use zbus::connection::Connection;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Action {
    #[serde(skip)]
    pub(crate) connection: Option<Connection>,

    pub notification_id: u32,
    pub label: String,
    pub action_key: String,
}

impl Action {
    pub async fn invoke(&self) -> Result<()> {
        let interface = get_interface_ref(
            self.connection
                .as_ref()
                .ok_or(NotificationServiceError::NoConnection)?,
        )
        .await?;

        interface
            .action_invoked(self.notification_id, &self.action_key)
            .await?;

        Ok(())
    }
}
