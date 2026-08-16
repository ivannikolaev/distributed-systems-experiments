#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub sender_id: usize,
    pub timestamp: u64,
    pub payload: String,
}

impl Message {
    pub fn new(sender_id: usize, timestamp: u64, payload: String) -> Self {
        Message {
            sender_id,
            timestamp,
            payload,
        }
    }
}