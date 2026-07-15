use serde::{Deserialize, Serialize};

use crate::Action;

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

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct DesktopNotification {
    pub id: u32,
    pub app_name: String,
    pub icon: Option<String>,
    pub summary: String,
    pub body: String,
    pub actions: Vec<Action>,
    pub urgency: Urgency,
    pub timeout: i32,
}
