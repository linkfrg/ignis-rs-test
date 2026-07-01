use notifications::NotificationServiceError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, glib::ErrorDomain)]
#[error_domain(name = "ignis-notifications-glib-error")]
pub enum GNotificationServiceError {
    DBusError,
    NoConnection,
}

impl From<&NotificationServiceError> for GNotificationServiceError {
    fn from(e: &NotificationServiceError) -> Self {
        match e {
            NotificationServiceError::NoConnection => GNotificationServiceError::NoConnection,
            NotificationServiceError::DBusError(_) => GNotificationServiceError::DBusError,
        }
    }
}

#[allow(dead_code)]
pub(crate) mod ffi {
    use glib::{prelude::*, translate::*};

    pub type IgnisNotificationsGLibError = i32;

    pub const IGNIS_NOTIFICATIONS_GLIB_DBUS_ERROR: IgnisNotificationsGLibError =
        super::GNotificationServiceError::DBusError as i32;
    pub const IGNIS_NOTIFICATIONS_GLIB_NO_CONNECTION: IgnisNotificationsGLibError =
        super::GNotificationServiceError::NoConnection as i32;

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ignis_notifications_glib_error_quark() -> glib::ffi::GQuark {
        <super::GNotificationServiceError as ErrorDomain>::domain().into_glib()
    }
}
