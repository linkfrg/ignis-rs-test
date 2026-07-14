use crate::CloseReason;

/// NOTE: These signals are intended to alert consumers from outside (e.g, from D-Bus)
/// If the action is requested by user directly (by calling a corresponding function)
/// consumer must not rely on the signals to modify the data
/// The data must be updated in-place by the called function
#[derive(Clone)]
pub enum NotificationServiceSignal {
    CloseNotification {
        id: u32,
        reason: CloseReason,
    },
    Notified {
        id: u32,
        notification: crate::DesktopNotification,
        replace: bool,
    },
}
