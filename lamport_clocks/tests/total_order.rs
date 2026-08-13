/// Integration tests for Lamport clock total ordering property
///
/// The total order is defined as:
/// - Events are ordered by (timestamp, process_id)
/// - If timestamp(a) < timestamp(b), then a comes before b
/// - If timestamps are equal, lower process_id comes first

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct EventKey {
    timestamp: u64,
    process_id: usize,
}

impl EventKey {
    fn new(timestamp: u64, process_id: usize) -> Self {
        EventKey {
            timestamp,
            process_id,
        }
    }
}

#[test]
fn test_total_order_different_timestamps() {
    // Events with different timestamps should be ordered by timestamp
    let e1 = EventKey::new(3, 2);
    let e2 = EventKey::new(5, 1);
    assert!(e1 < e2, "Event with t=3 should come before t=5");
}

#[test]
fn test_total_order_same_timestamp_different_processes() {
    // Events with same timestamp ordered by process ID
    let e1 = EventKey::new(5, 1);
    let e2 = EventKey::new(5, 2);
    let e3 = EventKey::new(5, 3);
    
    assert!(e1 < e2, "P1 should come before P2 at same timestamp");
    assert!(e2 < e3, "P2 should come before P3 at same timestamp");
    assert!(e1 < e3, "P1 should come before P3 at same timestamp");
}

#[test]
fn test_total_order_transitivity() {
    // Verify transitivity: if a < b and b < c, then a < c
    let events = vec![
        EventKey::new(1, 1),
        EventKey::new(3, 2),
        EventKey::new(5, 1),
        EventKey::new(5, 3),
        EventKey::new(10, 2),
    ];

    for i in 0..events.len() {
        for j in (i + 1)..events.len() {
            assert!(events[i] < events[j], "Transitivity violated: {:?} should be < {:?}", events[i], events[j]);
        }
    }
}
