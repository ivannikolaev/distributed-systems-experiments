use std::sync::Arc;
use crate::entry_point::EntryPoint;
use crate::message::Message;
use crate::network_simulator::NetworkSimulator;
use crate::process::Process;
use crate::process_behavior::ProcessBehavior;
use crate::experiment::{Experiment, ProcessConfig};

// --- Behaviors ---

struct P1Behavior;
impl ProcessBehavior for P1Behavior {
    fn on_message(&mut self, process: &mut Process<NetworkSimulator>, message: Message) {
        process.local_event();
        process.send(2, "Hello from P1".to_string());
    }
}

struct P1EntryPoint;
impl EntryPoint for P1EntryPoint {
    fn on_start(&mut self, process: &mut Process<NetworkSimulator>) {
        process.send(2, "Hello from P1".to_string());
    }
}

struct P2Behavior;
impl ProcessBehavior for P2Behavior {
    fn on_message(&mut self, process: &mut Process<NetworkSimulator>, message: Message) {
        process.local_event();
        process.send(3, "Hello from P2".to_string());
    }
}

struct P3Behavior;
impl ProcessBehavior for P3Behavior {
    fn on_message(&mut self, process: &mut Process<NetworkSimulator>, message: Message) {
        process.local_event();
    }
}

// --- Experiment ---

pub struct LamportClocksExperiment;

impl Experiment for LamportClocksExperiment {
    fn setup(&self) -> Vec<ProcessConfig> {
        vec![
            ProcessConfig { id: 1, behavior: Box::new(P1Behavior), entry_point: Some(Box::new(P1EntryPoint)) },
            ProcessConfig { id: 2, behavior: Box::new(P2Behavior), entry_point: None },
            ProcessConfig { id: 3, behavior: Box::new(P3Behavior), entry_point: None },
        ]
    }
}