pub mod imp;

use glib::subclass::prelude::*;

glib::wrapper! {
    pub struct GNotificationWrapped(ObjectSubclass<imp::GNotificationImp>);
}

impl GNotificationWrapped {
    pub(crate) fn new_from_rust(notification: notifications::Notification) -> Self {
        let obj: GNotificationWrapped = glib::Object::builder().build();

        obj.imp().notification.replace(notification);

        obj
    }
}
