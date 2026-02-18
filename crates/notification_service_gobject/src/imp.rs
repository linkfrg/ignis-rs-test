use glib::subclass::prelude::*;
use notification_service::{Notification, NotificationService};
use std::cell::RefCell;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct DesktopNotification {
    pub notification: RefCell<Option<Notification>>,
}

#[glib::object_subclass]
impl ObjectSubclass for DesktopNotification {
    const NAME: &'static str = "IgnisDesktopNotification";
    type Type = super::DesktopNotification;
    type ParentType = glib::Object;
}

impl ObjectImpl for DesktopNotification {}

#[derive(Default)]
pub struct IgnisNotifications {
    pub service: Mutex<NotificationService>,
}

#[glib::object_subclass]
impl ObjectSubclass for IgnisNotifications {
    const NAME: &'static str = "IgnisNotifications";
    type Type = super::IgnisNotifications;
    type ParentType = glib::Object;
}

impl ObjectImpl for IgnisNotifications {}
