use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use crate::message::Message;
use crate::message_sender::MessageSender;

pub struct NetworkSimulator {
    senders: HashMap<usize, Sender<Message>>,
    delay_ms: u64,
    loss_rate: f64,
}

impl NetworkSimulator {
    pub fn new(senders: HashMap<usize, Sender<Message>>, delay_ms: u64, loss_rate: f64) -> Self {
        NetworkSimulator {
            senders,
            delay_ms,
            loss_rate,
        }
    }
}

impl MessageSender for NetworkSimulator {
    fn send_message(&self, to: usize, message: Message) {
        let from = message.sender_id;
        println!("Network: Sending message from P{} to P{}", from, to);
        thread::sleep(Duration::from_millis(self.delay_ms));
        if rand::random::<f64>() < self.loss_rate {
            println!("Network: Message from P{} to P{} lost", from, to);
            return;
        }
        match self.senders.get(&to) {
            Some(tx) => {
                tx.send(message).unwrap();
                println!("Network: Message from P{} to P{} delivered", from, to);
            },
            None => println!("Network: Process P{} not found", to),
        }
    }
}