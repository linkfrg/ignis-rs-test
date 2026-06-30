use std::cell::RefCell;
use std::sync::OnceLock;

use glib::prelude::*;
use glib::subclass::prelude::*;
use notifications::Notification;

#[derive(Default)]
pub struct GNotificationImp {
    pub notification: RefCell<Notification>,
}

#[glib::object_subclass]
impl ObjectSubclass for GNotificationImp {
    const NAME: &'static str = "IgnisNotificationsGLibNotification";
    type Type = super::GNotificationWrapped;
    type ParentType = glib::Object;
}

impl ObjectImpl for GNotificationImp {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
        PROPERTIES.get_or_init(|| {
            vec![
                glib::ParamSpecUInt::builder("id").read_only().build(),
                glib::ParamSpecString::builder("app-name")
                    .read_only()
                    .build(),
                glib::ParamSpecString::builder("icon").read_only().build(),
                glib::ParamSpecString::builder("summary")
                    .read_only()
                    .build(),
                glib::ParamSpecString::builder("body").read_only().build(),
                glib::ParamSpecBoxed::builder::<glib::StrV>("actions")
                    .read_only()
                    .build(),
                glib::ParamSpecUInt::builder("urgency").read_only().build(),
                glib::ParamSpecInt::builder("timeout").read_only().build(),
            ]
        })
    }

    fn property(&self, _id: usize, _pspec: &glib::ParamSpec) -> glib::Value {
        match _pspec.name() {
            "id" => self.get_id().to_value(),
            "app-name" => self.get_app_name().to_value(),
            "icon" => self.get_icon().to_value(),
            "summary" => self.get_summary().to_value(),
            "body" => self.get_body().to_value(),
            "actions" => self.get_actions().to_value(),
            "urgency" => (self.get_urgency() as u32).to_value(),
            "timeout" => self.get_timeout().to_value(),
            _ => unimplemented!(),
        }
    }
}
impl GNotificationImp {
    pub fn get_id(&self) -> u32 {
        self.notification.borrow().id
    }

    pub fn get_app_name(&self) -> String {
        self.notification.borrow().app_name.clone()
    }

    pub fn get_icon(&self) -> Option<String> {
        self.notification.borrow().icon.clone()
    }

    pub fn get_summary(&self) -> String {
        self.notification.borrow().summary.clone()
    }

    pub fn get_body(&self) -> String {
        self.notification.borrow().body.clone()
    }

    pub fn get_actions(&self) -> Vec<String> {
        self.notification.borrow().actions.clone()
    }

    pub fn get_urgency(&self) -> u8 {
        self.notification.borrow().urgency
    }

    pub fn get_timeout(&self) -> i32 {
        self.notification.borrow().timeout
    }
}

pub(crate) mod ffi {
    use super::*;
    use glib::translate::*;

    pub type IgnisNotificationsGLibNotification =
        <super::GNotificationImp as super::ObjectSubclass>::Instance;

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ignis_notifications_glib_notification_new()
    -> *mut IgnisNotificationsGLibNotification {
        glib::Object::builder::<super::super::GNotificationWrapped>()
            .build()
            .to_glib_full()
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ignis_notifications_glib_notification_get_type() -> glib::ffi::GType {
        <super::super::GNotificationWrapped as StaticType>::static_type().into_glib()
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ignis_notifications_glib_notification_get_id(
        this: *mut IgnisNotificationsGLibNotification,
    ) -> u32 {
        let imp = unsafe { (*this).imp() };
        imp.get_id()
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ignis_notifications_glib_notification_get_app_name(
        this: *mut IgnisNotificationsGLibNotification,
    ) -> *mut glib::ffi::gchar {
        let imp = unsafe { (*this).imp() };
        imp.get_app_name().to_glib_full()
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ignis_notifications_glib_notification_get_icon(
        this: *mut IgnisNotificationsGLibNotification,
    ) -> *mut glib::ffi::gchar {
        let imp = unsafe { (*this).imp() };
        imp.get_icon().to_glib_full()
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ignis_notifications_glib_notification_get_summary(
        this: *mut IgnisNotificationsGLibNotification,
    ) -> *mut glib::ffi::gchar {
        let imp = unsafe { (*this).imp() };
        imp.get_summary().to_glib_full()
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ignis_notifications_glib_notification_get_body(
        this: *mut IgnisNotificationsGLibNotification,
    ) -> *mut glib::ffi::gchar {
        let imp = unsafe { (*this).imp() };
        imp.get_body().to_glib_full()
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ignis_notifications_glib_notification_get_actions(
        this: *mut IgnisNotificationsGLibNotification,
    ) -> glib::ffi::GStrv {
        let imp = unsafe { (*this).imp() };
        imp.get_actions().to_glib_full()
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ignis_notifications_glib_notification_get_urgency(
        this: *mut IgnisNotificationsGLibNotification,
    ) -> u8 {
        let imp = unsafe { (*this).imp() };
        imp.get_urgency()
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn ignis_notifications_glib_notification_get_timeout(
        this: *mut IgnisNotificationsGLibNotification,
    ) -> i32 {
        let imp = unsafe { (*this).imp() };
        imp.get_timeout()
    }
}
