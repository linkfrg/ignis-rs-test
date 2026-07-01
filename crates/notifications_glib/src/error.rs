use notifications::NotificationServiceError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, glib::ErrorDomain)]
#[error_domain(name = "ignis-notifications-glib-error")]
pub enum IgnisNotificationsGLibErrorImp {
    DBusError,
    NoConnection,
}
pub trait IntoGLibError<T> {
    fn into_glib_error(self) -> Result<T, glib::Error>;
}

impl From<&NotificationServiceError> for IgnisNotificationsGLibErrorImp {
    fn from(e: &NotificationServiceError) -> Self {
        match e {
            NotificationServiceError::NoConnection => IgnisNotificationsGLibErrorImp::NoConnection,
            NotificationServiceError::DBusError(_) => IgnisNotificationsGLibErrorImp::DBusError,
        }
    }
}

impl<T> IntoGLibError<T> for Result<T, NotificationServiceError> {
    fn into_glib_error(self) -> Result<T, glib::Error> {
        self.map_err(|e| {
            let domain = IgnisNotificationsGLibErrorImp::from(&e);
            glib::Error::new(domain, &e.to_string())
        })
    }
}

#[allow(dead_code)]
pub(crate) mod ffi {
    use glib::{prelude::*, translate::*};

    pub type IgnisNotificationsGLibError = i32;

    pub const IGNIS_NOTIFICATIONS_GLIB_DBUS_ERROR: IgnisNotificationsGLibError =
        super::IgnisNotificationsGLibErrorImp::DBusError as i32;
    pub const IGNIS_NOTIFICATIONS_GLIB_NO_CONNECTION: IgnisNotificationsGLibError =
        super::IgnisNotificationsGLibErrorImp::NoConnection as i32;

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn ignis_notifications_glib_error_quark() -> glib::ffi::GQuark {
        <super::IgnisNotificationsGLibErrorImp as ErrorDomain>::domain().into_glib()
    }
}
