#[derive(Clone)]
pub enum NotificationServiceSignal {
    Closed { id: u32 },
    Notified { id: u32, replace: bool },
}
