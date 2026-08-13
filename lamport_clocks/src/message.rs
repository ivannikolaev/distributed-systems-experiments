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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = Message::new(1, 5, "test".to_string());
        assert_eq!(msg.sender_id, 1);
        assert_eq!(msg.timestamp, 5);
        assert_eq!(msg.payload, "test");
    }

    #[test]
    fn test_message_clone() {
        let msg1 = Message::new(2, 10, "hello".to_string());
        let msg2 = msg1.clone();
        assert_eq!(msg1, msg2);
    }
}