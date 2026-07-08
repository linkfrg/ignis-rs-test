use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotificationServiceError {
    #[error("D-Bus error: {0}")]
    DBusError(#[from] zbus::Error),

    #[error("Connection not exist error")]
    NoConnection,

    #[error("I/O Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error(
        "JSON error (you may need to manually fix or remove history file at ~/.cache/ignis_notifications/notifications.json): {0}"
    )]
    JSONError(#[from] serde_json::Error),

    #[error("No notification found with id: {0}")]
    NotificationNotFound(u32),
}

pub type Result<T> = std::result::Result<T, NotificationServiceError>;
