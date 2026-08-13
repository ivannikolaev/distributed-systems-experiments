use std::sync::mpsc;

use crate::message::Message;

#[derive(Debug)]
pub struct Process {
    pub id: usize,
    clock: u64,
    inbox: mpsc::Receiver<Message>,
}

impl Process {
    pub fn clock(&self) -> u64 {
        self.clock
    }

    pub fn new(id: usize, inbox: mpsc::Receiver<Message>) -> Self {
        Process { id, clock: 0, inbox }
    }

    /// Creates a process with a specific initial clock value (for testing)
    #[cfg(test)]
    fn with_clock(id: usize, clock: u64) -> Self {
        let (_, rx) = mpsc::channel();
        Process { id, clock, inbox: rx }
    }

    pub fn local_event(&mut self) {
        self.clock += 1;
        println!("Process {}: Local event, clock = {}", self.id, self.clock);
    }

    pub fn send(&mut self, payload: String, tx: mpsc::Sender<Message>) {
        self.clock += 1;
        println!("Process {}: Sending '{}' (clock={})", self.id, payload, self.clock);
        let message = Message::new(self.id, self.clock, payload);
        tx.send(message).unwrap();
    }

    /// Receive a message and update clock accordingly (for testing)
    #[cfg(test)]
    pub fn receive_message(&mut self, msg: &Message) {
        self.clock = std::cmp::max(self.clock, msg.timestamp) + 1;
        println!(
            "Process {}: Received '{}' from P{} (clock: {} → {})",
            self.id, msg.payload, msg.sender_id, msg.timestamp, self.clock
        );
    }

    fn receive(&mut self, sender_id: usize, timestamp: u64, payload: &str) {
        self.clock = std::cmp::max(self.clock, timestamp) + 1;
        println!(
            "Process {}: Received '{}' from P{} (clock: {} → {})",
            self.id, payload, sender_id, timestamp, self.clock
        );
    }

    pub fn run<F>(&mut self, on_message: F)
    where
        F: Fn(&mut Process, Message),
    {
        while let Ok(message) = self.inbox.recv() {
            self.receive(message.sender_id, message.timestamp, &message.payload);
            on_message(self, message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_initial_clock() {
        let (_, rx) = mpsc::channel();
        let p = Process::new(1, rx);
        assert_eq!(p.clock(), 0);
    }

    #[test]
    fn test_local_event_increments_clock() {
        let mut p = Process::with_clock(1, 0);
        p.local_event();
        assert_eq!(p.clock(), 1);

        p.local_event();
        assert_eq!(p.clock(), 2);
    }

    #[test]
    fn test_send_increments_clock() {
        let mut p = Process::with_clock(1, 5);
        let (tx, _rx) = mpsc::channel();
        p.send("test".to_string(), tx);
        assert_eq!(p.clock(), 6);
    }

    #[test]
    fn test_receive_low_timestamp() {
        // Receive timestamp lower than local clock
        let mut p = Process::with_clock(1, 10);
        let msg = Message::new(2, 3, "old".to_string());
        p.receive_message(&msg);
        // max(10, 3) + 1 = 11
        assert_eq!(p.clock(), 11);
    }

    #[test]
    fn test_receive_high_timestamp() {
        // Receive timestamp higher than local clock
        let mut p = Process::with_clock(1, 3);
        let msg = Message::new(2, 10, "future".to_string());
        p.receive_message(&msg);
        // max(3, 10) + 1 = 11
        assert_eq!(p.clock(), 11);
    }

    #[test]
    fn test_receive_equal_timestamp() {
        // Receive timestamp equal to local clock
        let mut p = Process::with_clock(1, 7);
        let msg = Message::new(2, 7, "same".to_string());
        p.receive_message(&msg);
        // max(7, 7) + 1 = 8
        assert_eq!(p.clock(), 8);
    }

    #[test]
    fn test_clock_never_decreases() {
        let mut p = Process::with_clock(1, 0);

        // Sequence of events
        p.local_event(); // 1
        p.local_event(); // 2
        p.local_event(); // 3

        // Receive old message
        let old_msg = Message::new(2, 1, "old".to_string());
        p.receive_message(&old_msg); // max(3, 1) + 1 = 4
        assert!(p.clock() >= 3);

        // Receive newer message
        let new_msg = Message::new(2, 10, "new".to_string());
        p.receive_message(&new_msg); // max(4, 10) + 1 = 11
        assert!(p.clock() >= 4);
    }

    #[test]
    fn test_lamport_clock_property() {
        // Verify: if A -> B (A happens before B), then clock(A) < clock(B)
        let mut p = Process::with_clock(1, 0);

        let clock_before = p.clock();
        p.local_event();
        let clock_after = p.clock();

        assert!(clock_after > clock_before, "Local event must increase clock");
    }
}