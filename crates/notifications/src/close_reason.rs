#[derive(Copy, Clone)]
pub enum CloseReason {
    Expired,
    Dismissed,
    DBusCall,
    Other,
}

impl From<u32> for CloseReason {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Expired,
            2 => Self::Dismissed,
            3 => Self::DBusCall,
            _ => Self::Other,
        }
    }
}

impl From<CloseReason> for u32 {
    fn from(value: CloseReason) -> Self {
        match value {
            CloseReason::Expired => 1,
            CloseReason::Dismissed => 2,
            CloseReason::DBusCall => 3,
            CloseReason::Other => 4,
        }
    }
}
