use tokio::sync::mpsc;

#[derive(Clone)]
pub enum NotificationServiceSignal {
    Closed { id: u32 },
    Notified { id: u32, replace: bool },
}
pub struct SignalHelper {}

impl SignalHelper {
    pub async fn send_notified(
        tx: &mpsc::Sender<NotificationServiceSignal>,
        id: u32,
        replace: bool,
    ) {
        tx.send(NotificationServiceSignal::Notified {
            id: id,
            replace: replace,
        })
        .await
        .unwrap_or_else(|e| panic!("Channel send error: {e}"));
    }

    pub async fn send_closed(tx: &mpsc::Sender<NotificationServiceSignal>, id: &u32) {
        tx.send(NotificationServiceSignal::Closed { id: *id })
            .await
            .unwrap_or_else(|e| panic!("Channel send error: {e}"));
    }
}
