use std::sync::Arc;

use crate::message::Message;

pub trait MessageSender {
    fn send_message(&self, to: usize, message: Message);
}

// для любого T который реализует MessageSender
// Arc<T> тоже реализует MessageSender
impl<T: MessageSender> MessageSender for Arc<T> {
    fn send_message(&self, to: usize, message: Message) {
        self.as_ref().send_message(to, message);
    }
}