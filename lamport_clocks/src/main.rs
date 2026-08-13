mod message;
mod process;

use std::{sync::mpsc, thread};

use process::Process;

use crate::message::Message;

#[derive(Debug, Clone)]
struct Event {
    timestamp: u64,
    process_id: usize,
    description: String,
}

impl Event {
    fn total_order_key(&self) -> (u64, usize) {
        (self.timestamp, self.process_id)
    }
}

fn main() {
    let (tx1, rx1) = mpsc::channel::<Message>();
    let (tx2, rx2) = mpsc::channel::<Message>();
    let (tx3, rx3) = mpsc::channel::<Message>();

    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let mut p1 = Process::new(1, rx1);
    let mut p2 = Process::new(2, rx2);
    let mut p3 = Process::new(3, rx3);

    let event_tx1 = event_tx.clone();
    let h1 = thread::spawn(move || {
        p1.run(|process, msg| {
            let evt = Event {
                timestamp: msg.timestamp,
                process_id: msg.sender_id,
                description: format!("recv '{}' at P{}", msg.payload, process.id),
            };
            event_tx1.send(evt).unwrap();

            process.local_event();
            event_tx1.send(Event {
                timestamp: process.clock(),
                process_id: process.id,
                description: "local event".to_string(),
            }).unwrap();

            process.send("Hello from P1 to P2".to_string(), tx2.clone());
            event_tx1.send(Event {
                timestamp: process.clock(),
                process_id: process.id,
                description: "send 'Hello from P1 to P2'".to_string(),
            }).unwrap();
        });
        println!("Process 1 finished, clock = {}", p1.clock());
    });

    let event_tx2 = event_tx.clone();
    let h2 = thread::spawn(move || {
        p2.run(|process, msg| {
            let evt = Event {
                timestamp: msg.timestamp,
                process_id: msg.sender_id,
                description: format!("recv '{}' at P{}", msg.payload, process.id),
            };
            event_tx2.send(evt).unwrap();

            process.local_event();
            event_tx2.send(Event {
                timestamp: process.clock(),
                process_id: process.id,
                description: "local event".to_string(),
            }).unwrap();

            process.send("Hello from P2 to P3".to_string(), tx3.clone());
            event_tx2.send(Event {
                timestamp: process.clock(),
                process_id: process.id,
                description: "send 'Hello from P2 to P3'".to_string(),
            }).unwrap();
        });
        println!("Process 2 finished, clock = {}", p2.clock());
    });

    let event_tx3 = event_tx.clone();
    let h3 = thread::spawn(move || {
        p3.run(|process, msg| {
            let evt = Event {
                timestamp: msg.timestamp,
                process_id: msg.sender_id,
                description: format!("recv '{}' at P{}", msg.payload, process.id),
            };
            event_tx3.send(evt).unwrap();

            process.local_event();
            event_tx3.send(Event {
                timestamp: process.clock(),
                process_id: process.id,
                description: "local event".to_string(),
            }).unwrap();
        });
        println!("Process 3 finished, clock = {}", p3.clock());
    });

    drop(event_tx);

    let message = Message::new(0, 0, "Initial".to_string());
    tx1.send(message).unwrap();
    drop(tx1);

    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();

    println!("\n=== All processes finished ===\n");

    let mut events: Vec<Event> = event_rx.iter().collect();
    events.sort_by(|a, b| {
        let key_a = a.total_order_key();
        let key_b = b.total_order_key();
        key_a.cmp(&key_b)
    });

    println!("Global Total Order of Events (by Lamport timestamp, then process ID):");
    println!("{}", "─".repeat(70));
    for (i, event) in events.iter().enumerate() {
        println!("{:>2}. [t={}, P{}] {}", i + 1, event.timestamp, event.process_id, event.description);
    }
    println!("{}", "─".repeat(70));
}
