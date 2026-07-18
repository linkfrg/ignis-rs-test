#[derive(Debug, Copy, Clone, PartialEq, Eq, glib::ErrorDomain)]
#[error_domain(name = "ignis-notifications-glib-error")]
pub enum GNotificationServiceError {
    DBusError,
    NoConnection,
    IOError,
    JSONError,
    NotificationNotFound,
    ConnectionInitializedTwice,
}

impl From<&notifications::Error> for GNotificationServiceError {
    fn from(e: &notifications::Error) -> Self {
        match e {
            notifications::Error::NoConnection => GNotificationServiceError::NoConnection,
            notifications::Error::DBusError(_) => GNotificationServiceError::DBusError,
            notifications::Error::IOError(_) => GNotificationServiceError::IOError,
            notifications::Error::JSONError(_) => GNotificationServiceError::JSONError,
            notifications::Error::NotificationNotFound(_) => {
                GNotificationServiceError::NotificationNotFound
            }
            notifications::Error::ConnectionInitializedTwice => {
                GNotificationServiceError::ConnectionInitializedTwice
            }
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

    pub const IGNIS_NOTIFICATIONS_GLIB_IO_ERROR: IgnisNotificationsGLibError =
        super::GNotificationServiceError::IOError as i32;

    pub const IGNIS_NOTIFICATIONS_GLIB_JSON_ERROR: IgnisNotificationsGLibError =
        super::GNotificationServiceError::JSONError as i32;

    pub const IGNIS_NOTIFICATIONS_GLIB_NOTIFICATION_NOT_FOUND: IgnisNotificationsGLibError =
        super::GNotificationServiceError::NotificationNotFound as i32;

    pub const IGNIS_NOTIFICATIONS_GLIB_NOTIFICATION_CONNECTION_INITIALIZED_TWICE:
        IgnisNotificationsGLibError =
        super::GNotificationServiceError::ConnectionInitializedTwice as i32;

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ignis_notifications_glib_error_quark() -> glib::ffi::GQuark {
        <super::GNotificationServiceError as ErrorDomain>::domain().into_glib()
    }
}
