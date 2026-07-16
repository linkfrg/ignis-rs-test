use crate::NotificationService;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Action {
    pub(crate) notification_id: u32,
    pub(crate) label: String,
    pub(crate) action_key: String,
}

#[derive(Default, Clone)]
pub struct ActionHandle {
    pub(crate) inner: Arc<Action>,
    pub(crate) service: NotificationService,
}

impl ActionHandle {
    pub fn notification_id(&self) -> u32 {
        self.inner.notification_id
    }

    pub fn label(&self) -> String {
        self.inner.label.clone()
    }

    pub fn action_key(&self) -> String {
        self.inner.action_key.clone()
    }

    pub async fn invoke(&self) -> Result<()> {
        self.service
            .invoke_action(self.inner.notification_id, &self.inner.action_key)
            .await?;

        Ok(())
    }
}
