# Distributed Systems Experiments

Implementing classic distributed systems algorithms from seminal papers.

This repository is a learning journey through foundational concepts of distributed
systems — starting from theory and working towards working code.

## Papers & Implementations

### Lamport, 1978 — Time, Clocks, and the Ordering of Events in a Distributed System
- [ ] Lamport Clocks
- [ ] Distributed Mutex Algorithm

### Fidge/Mattern, 1988 — Vector Clocks
- [ ] Vector Clocks

## Structure

This is a Cargo workspace:

```
distributed-systems-experiments/
  lamport_clocks/
  vector_clocks/
```

## Running

```bash
# run a specific experiment
cargo run -p lamport_clocks
```

## References

- [Time, Clocks, and the Ordering of Events in a Distributed System — Leslie Lamport (1978)](https://lamport.azurewebsites.net/pubs/time-clocks.pdf)