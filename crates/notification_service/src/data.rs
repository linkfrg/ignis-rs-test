use crate::constants::FILE_PATH;
use crate::notification::Notification;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct ServiceData {
    pub counter: u32,
    pub notifications: HashMap<u32, Notification>,
}

impl ServiceData {
    pub fn new() -> Self {
        fs::read_to_string(&*FILE_PATH)
            .and_then(|json_str| {
                serde_json::from_str(&json_str)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
            })
            .map_err(|e| {
                println!("Error reading/deserializing file: {e}, falling back to empty data")
            })
            .ok()
            .unwrap_or_else(Self::new_empty)
    }

    fn new_empty() -> Self {
        Self {
            counter: 0,
            notifications: HashMap::new(),
        }
    }
    pub fn add_notification(&mut self, id: u32, new_notification: Notification, replace: bool) {
        if !replace {
            self.notifications.insert(id, new_notification.clone());
        } else {
            if let Some(old_notification) = self.notifications.get_mut(&id) {
                *old_notification = new_notification.clone();
            }
        }
        self.save_to_file();
    }

    pub fn remove_notification(&mut self, id: &u32) {
        self.notifications.remove(id);
        self.save_to_file();
    }

    fn save_to_file(&self) {
        let res = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
            .and_then(|json_str| fs::write(&*FILE_PATH, json_str));

        if let Err(e) = res {
            println!("Error saving data to file: {e}");
        }
    }
}
