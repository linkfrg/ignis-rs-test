use crate::private_prelude::*;

// NOTE: These signals are intended to alert consumers from outside (e.g, from D-Bus)
// If the action is requested by user directly (by calling a corresponding function)
// consumer must not rely on the signals to modify the data
// The data must be updated in-place by the called function

/// Represents an event happened in the service.
#[derive(Clone)]
pub enum NotificationServiceSignal {
    /// A notification was closed.
    CloseNotification {
        /// The ID of the closed notification.
        id: u32,

        /// The reason why the notification is closed.
        reason: CloseReason,
    },
    /// A new notification arrived.
    Notified {
        /// The ID of the new notification.
        id: u32,

        /// A handle to the notification.
        notification: NotificationHandle,

        /// Whether this notification replaces an old one with this ID.
        replace: bool,
    },
}
