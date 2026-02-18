use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotificationServiceError {
    #[error("D-Bus error")]
    DBusError(#[from] zbus::Error),

    #[error("Connection not exist error")]
    NoConnection,
}

pub type Result<T> = std::result::Result<T, NotificationServiceError>;
