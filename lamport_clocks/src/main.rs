use crate::lamport_clocks_experiment::LamportClocksExperiment;
use crate::experiment::Experiment;

mod message;
mod process;
mod message_sender;
mod network_simulator;
mod experiment;
mod process_behavior;
mod entry_point;
mod lamport_clocks_experiment;

fn main() {
    LamportClocksExperiment.run();    
    println!("Experiment completed!")
}
