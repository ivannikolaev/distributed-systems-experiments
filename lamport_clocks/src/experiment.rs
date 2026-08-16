use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc::{self, Receiver, Sender}};
use std::thread;
use std::time::Duration;
use crate::entry_point::EntryPoint;
use crate::message::Message;
use crate::network_simulator::NetworkSimulator;
use crate::process::Process;
use crate::process_behavior::ProcessBehavior;

pub struct ProcessConfig {
    pub id: usize,
    pub behavior: Box<dyn ProcessBehavior>,
    pub entry_point: Option<Box<dyn EntryPoint>>,
}

pub trait Experiment {
    fn setup(&self) -> Vec<ProcessConfig>;

    fn duration(&self) -> Duration {
        Duration::from_secs(1)
    }

    fn run(self) where Self: Sized {
        let configs = self.setup();

        let mut senders: HashMap<usize, Sender<Message>> = HashMap::new();
        let mut receivers: HashMap<usize, Receiver<Message>> = HashMap::new();
        for config in &configs {
            let (tx, rx) = mpsc::channel::<Message>();
            senders.insert(config.id, tx);
            receivers.insert(config.id, rx);
        }

        let network = Arc::new(NetworkSimulator::new(senders, 100, 0.0));

        let is_running = Arc::new(AtomicBool::new(true));
        let mut handles = Vec::new();
        for config in configs {
            let rx = receivers.remove(&config.id).unwrap();
            let mut behavior = config.behavior;
            let mut entry_point = config.entry_point;
            let mut process = Process::new(config.id, rx, Arc::clone(&network), Arc::clone(&is_running));
            let handle = thread::spawn(move || {
                if let Some(ref mut ep) = entry_point {
                    ep.on_start(&mut process);
                }
                process.run(|p, m| {
                    behavior.on_message(p, m);
                });
            });
            handles.push(handle);
        }

        thread::sleep(self.duration());
        is_running.store(false, Ordering::Relaxed);

        for handle in handles {
            handle.join().unwrap();
        }
    }
}