pub mod imp;

use glib::subclass::prelude::*;

glib::wrapper! {
    pub struct GDesktopNotification(ObjectSubclass<imp::GDesktopNotificationImp>);
}

impl GDesktopNotification {
    pub(crate) fn new_from_rust(notification: notifications::DesktopNotification) -> Self {
        let obj: GDesktopNotification = glib::Object::builder().build();

        obj.imp().notification.replace(notification);

        obj
    }
}
