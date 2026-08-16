use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

use crate::message::Message;
use crate::message_sender::MessageSender;

#[derive(Debug)]
pub struct Process<S: MessageSender> {
    pub id: usize,
    clock: u64,
    inbox: mpsc::Receiver<Message>,
    sender: Arc<S>,
    is_running: Arc<AtomicBool>,
}

impl<S: MessageSender> Process<S> {
    pub fn clock(&self) -> u64 {
        self.clock
    }

    pub fn new(id: usize, inbox: mpsc::Receiver<Message>, sender: Arc<S>, is_running: Arc<AtomicBool>) -> Self {
        Process { id, clock: 0, inbox, sender, is_running }
    }

    pub fn local_event(&mut self) {
        self.clock += 1;
        println!("Process {}: Local event (clock: {} -> {})", self.id, self.clock - 1, self.clock);
    }

    pub fn send(&mut self, to: usize, payload: String) {
        self.clock += 1;
        println!("Process {}: Sending '{}' (clock: {} -> {})", self.id, payload, self.clock - 1, self.clock);
        let message = Message::new(self.id, self.clock, payload);
        self.sender.send_message(to, message);
    }

    fn receive(&mut self, sender_id: usize, timestamp: u64, payload: &str) {
        let current_clock = self.clock;
        self.clock = std::cmp::max(self.clock, timestamp) + 1;
        println!(
            "Process {}: Received '{}' from P{} (clock: {} → {})",
            self.id, payload, sender_id, current_clock, self.clock
        );
    }

    pub fn run<F>(&mut self, mut on_message: F)
    where
        F: FnMut(&mut Process<S>, Message),
    {
        while self.is_running.load(Ordering::Relaxed) {
            match self.inbox.try_recv() {
                Ok(message) => {
                    self.receive(message.sender_id, message.timestamp, &message.payload);
                    on_message(self, message);
                }
                Err(mpsc::TryRecvError::Empty) => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    break;
                }
            }
        }
        println!("Shutting down P{}", self.id);
    }

}