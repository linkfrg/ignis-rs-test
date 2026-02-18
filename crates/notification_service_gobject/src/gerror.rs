use notification_service::NotificationServiceError;

#[derive(Debug, Copy, Clone, glib::ErrorDomain)]
#[error_domain(name = "gignis-notifications--error")]
pub enum GIgnisNotificationsError {
    DBusError,
    NoConnection,
}

impl From<NotificationServiceError> for GIgnisNotificationsError {
    fn from(e: NotificationServiceError) -> Self {
        match e {
            NotificationServiceError::NoConnection => GIgnisNotificationsError::NoConnection,
            NotificationServiceError::DBusError(_) => GIgnisNotificationsError::DBusError,
        }
    }
}
