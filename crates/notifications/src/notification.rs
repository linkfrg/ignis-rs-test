use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct DesktopNotification {
    pub id: u32,
    pub app_name: String,
    pub icon: Option<String>,
    pub summary: String,
    pub body: String,
    pub actions: Vec<String>,
    pub urgency: u8,
    pub timeout: i32,
}
