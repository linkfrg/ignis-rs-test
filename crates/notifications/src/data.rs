use crate::file_utils::get_history_file_path;
use crate::notification::DesktopNotification;
use crate::{NotificationServiceError, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct ServiceData {
    pub counter: u32,
    pub notifications: BTreeMap<u32, Arc<DesktopNotification>>,

    #[serde(skip)]
    file_path: Option<PathBuf>, // If None - no file operations
}

impl ServiceData {
    pub fn new(cache_dir: Option<PathBuf>) -> Result<Self> {
        let file_path = get_history_file_path(cache_dir)?;

        let json_str = match fs::read_to_string(&file_path) {
            Ok(s) => s,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::new_empty(Some(file_path)));
            }
            Err(e) => return Err(e.into()),
        };

        let mut obj: Self = serde_json::from_str(&json_str)?;
        obj.file_path = Some(file_path);

        Ok(obj)
    }

    pub fn new_in_memory() -> Self {
        Self::new_empty(None)
    }

    fn new_empty(file_path: Option<PathBuf>) -> Self {
        Self {
            counter: 0,
            notifications: BTreeMap::new(),
            file_path,
        }
    }

    pub fn add_notification(
        &mut self,
        id: u32,
        new_notification: Arc<DesktopNotification>,
        replace: bool,
    ) -> Result<()> {
        if !replace {
            self.notifications.insert(id, new_notification);
        } else {
            if let Some(old_notification) = self.notifications.get_mut(&id) {
                *old_notification = new_notification;
            }
        }
        self.save_to_file()?;

        Ok(())
    }

    pub fn remove_notification(&mut self, id: u32) -> Result<()> {
        self.notifications
            .remove(&id)
            .ok_or_else(|| NotificationServiceError::NotificationNotFound(id))?;

        self.save_to_file()?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        self.notifications.clear();
        self.counter = 0;
        self.save_to_file()?;
        Ok(())
    }

    fn save_to_file(&self) -> Result<()> {
        if let Some(file_path) = self.file_path.clone() {
            let json_str = serde_json::to_string_pretty(self)?;
            fs::write(&file_path, json_str)?;
        };

        Ok(())
    }
}
