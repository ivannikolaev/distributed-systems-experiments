
use crate::message::Message;
use crate::network_simulator::NetworkSimulator;
use crate::process::Process;

pub trait ProcessBehavior: Send {
    fn on_message(&mut self, process: &mut Process<NetworkSimulator>, message: Message);
}