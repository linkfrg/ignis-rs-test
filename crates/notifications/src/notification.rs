use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::action::Action;
use crate::{NotificationService, action::ActionHandle};

#[derive(Copy, Clone, Serialize, Deserialize, Default)]
pub enum Urgency {
    #[default]
    Low,
    Normal,
    Critical,
}

impl From<u8> for Urgency {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Low,
            1 => Self::Normal,
            2 => Self::Critical,
            _ => Self::Low, // fallback
        }
    }
}

impl From<Urgency> for u8 {
    fn from(value: Urgency) -> Self {
        match value {
            Urgency::Low => 0,
            Urgency::Normal => 1,
            Urgency::Critical => 2,
        }
    }
}

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
}
