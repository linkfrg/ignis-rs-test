use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::Result;
use crate::action::Action;
use crate::urgency::Urgency;
use crate::{NotificationService, action::ActionHandle};

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct DesktopNotification {
    pub(crate) id: u32,
    pub(crate) app_name: String,
    pub(crate) icon: Option<String>,
    pub(crate) summary: String,
    pub(crate) body: String,
    pub(crate) actions: Vec<Arc<Action>>,
    pub(crate) urgency: Urgency,
    pub(crate) timeout: i32,
}

#[derive(Clone, Default)]
pub struct NotificationHandle {
    pub(crate) inner: Arc<DesktopNotification>,
    pub(crate) service: NotificationService,
}

impl NotificationHandle {
    pub fn id(&self) -> u32 {
        self.inner.id
    }

    pub fn app_name(&self) -> String {
        self.inner.app_name.clone()
    }

    pub fn icon(&self) -> Option<String> {
        self.inner.icon.clone()
    }

    pub fn summary(&self) -> String {
        self.inner.summary.clone()
    }

    pub fn body(&self) -> String {
        self.inner.body.clone()
    }

    pub fn actions(&self) -> Vec<ActionHandle> {
        self.inner
            .actions
            .iter()
            .map(|a| ActionHandle {
                inner: a.clone(),
                service: self.service.clone(),
            })
            .collect()
    }

    pub fn urgency(&self) -> Urgency {
        self.inner.urgency
    }

    pub fn timeout(&self) -> i32 {
        self.inner.timeout
    }

    pub async fn dismiss(&self) -> Result<()> {
        self.service.dismiss_notification(self.id()).await?;
        Ok(())
    }
}
