use crate::{network_simulator::NetworkSimulator, process::Process};

pub trait EntryPoint: Send {
    fn on_start(&mut self, process: &mut Process<NetworkSimulator>);
}