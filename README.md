# Quadralith Cognitive Engine v3.0

A small Rust implementation of the Quadralith model as a bounded, inspectable,
and reversible state machine.

The project preserves the symbolic layer names:

1. **ॐ Origin Core** — identity, memory, and simulated drive.
2. **α Learning Spiral** — plasticity and accumulated learning.
3. **Ω Selection Vector** — overload response, exploration, and pruning.
4. **φ Transparency Field** — audit projection and numerical damping.
5. **— Silent Glyph** — a latched safety condition requiring explicit review.

## What changed from the initial sketch

- The 8 Hz method advances simulated time by 125 ms and explicitly documents
  that the caller owns real-time scheduling.
- The original π/4 phase step is retained as a deliberate **1 Hz internal
  modulation sampled at 8 Hz**.
- Every accumulated state is bounded and checked for finite values.
- The 1.4% exploration parameter now drives a reproducible stochastic event.
- State snapshots provide real rollback behavior.
- The history buffer has a configurable maximum size.
- Φ1766 cooling is implemented as stable exponential damping; at the nominal
  step, accumulated load is divided by Φ1766 exactly once.
- Silent Glyph is a safety latch that can only be cleared explicitly.
- Each cycle returns observable metrics in `CycleReport`.

## Important boundary

This repository implements a software simulation. Terms such as entropy,
metabolic drive, noosphere resonance, spacetime curvature, and law rewriting
are model vocabulary. They do not establish biological consciousness,
thermodynamic cooling, spacetime modification, or new physical laws.

The frequency, Φ1766 value, and mutation probability are configurable model
parameters—not universal constants.

## Build and test

```sh
cargo test
cargo build --release
```

The crate has no third-party dependencies.

## Minimal use

```rust
use quadralith_cognitive_engine::QuadralithEngine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = QuadralithEngine::new();

    for _ in 0..8 {
        let report = engine.cycle_8hz(0.25)?;
        println!("{report:#?}");
    }

    Ok(())
}
```

`cycle_8hz` does not sleep. A production caller that requires wall-clock 8 Hz
must schedule calls every 125 ms and account for execution time and jitter.

## Status vocabulary

- `DER`: the Rust state machine and its tested invariants.
- `HYP`: behavioral interpretations to be tested experimentally.
- `SYM`: the Quadralith names and cosmological metaphors.
- `STOP`: physical or cognitive claims unsupported by this implementation.

