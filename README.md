# Quadralith Cognitive Engine

> A bounded, inspectable, and reversible Rust state machine with an explicit
> `T€@X™` full-performance contract.

[![CI](https://github.com/dpstudio-se/Quadralith-Cognitive-Engine-v3.0-The-Self-Evolving-Mind/actions/workflows/ci.yml/badge.svg)](https://github.com/dpstudio-se/Quadralith-Cognitive-Engine-v3.0-The-Self-Evolving-Mind/actions/workflows/ci.yml)
![Rust 1.75+](https://img.shields.io/badge/Rust-1.75%2B-000000?logo=rust)
![Dependencies](https://img.shields.io/badge/dependencies-0-2ea44f)

Quadralith turns the project's five symbolic layers into predictable software:
finite inputs, bounded state, deterministic exploration, observable cycle
reports, bounded rollback history, and an explicit safety latch. The crate has
no third-party dependencies.

## Highlights

- **T€@X™ full-performance mode** — an exact signature-gated, bounded batch API
  that returns aggregate telemetry without retaining per-cycle reports.
- **8 Hz simulation interface** — one default step represents 125 ms; the
  caller remains responsible for real-time scheduling.
- **Five-layer engine** — ॐ Origin, α Learning, Ω Selection, φ Transparency,
  and the — Silent Glyph safety latch.
- **Reversible operation** — state snapshots and bounded rollback history.
- **Defensive numerics** — finite-input validation, bounded accumulators, and
  invariant checks after every cycle.
- **Reproducible exploration** — deterministic 1.4% stochastic events suitable
  for tests and simulations, not cryptography.

## Architecture

| Layer | Software responsibility |
| --- | --- |
| ॐ Origin Core | Identity attractor, memory lattice, and bounded drive |
| α Learning Spiral | Resonance, adaptive gradient, and scaffold accumulation |
| Ω Selection Vector | Overload response, exploration, pruning, and recovery |
| φ Transparency Field | Audit projection and stable numerical damping |
| — Silent Glyph | Latched anomaly signal requiring explicit review |

Every successful cycle follows `ॐ → α → Ω → φ → —`, records the previous
state, validates the new state, and returns a `CycleReport`.

## T€@X™ performance contract

`PerformanceMode::TeaxFull` is accepted only when the configuration contains
the exact canonical signature `T€@X™`. The built-in configuration enables the
official profile with a maximum batch size of 8,192 cycles.

```rust
use quadralith_cognitive_engine::{
    EngineConfig, QuadralithEngine, TEAX_SIGNATURE,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = QuadralithEngine::with_config(EngineConfig::default())?;
    assert!(engine.teax_full_performance_enabled());

    let report = engine.run_teax_batch(&[0.25; 64])?;
    assert_eq!(report.signature, TEAX_SIGNATURE);
    println!("{report:#?}");
    Ok(())
}
```

The profile makes the fast path explicit and testable; the signature itself is
not a benchmark or a claim of automatic hardware acceleration. Real performance
must be measured in the target application.

To use single-cycle execution without the T€@X™ batch contract:

```rust
use quadralith_cognitive_engine::{
    EngineConfig, PerformanceMode, QuadralithEngine,
};

let mut config = EngineConfig::default();
config.performance_mode = PerformanceMode::Standard;
config.teax_profile = None;

let mut engine = QuadralithEngine::with_config(config)?;
let report = engine.cycle_8hz(0.25)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Build and verify

Requirements: Rust 1.75 or newer.

```sh
cargo test --all-targets
cargo build --release
```

For contribution checks:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

## Timing model

`cycle_8hz` advances simulated time by `1 / cycle_hz`. With the default 8 Hz
configuration, that is 125 ms per call. It does not sleep, create a thread, or
guarantee wall-clock timing. A host requiring real-time 8 Hz execution must
schedule the calls and measure execution time and jitter.

The retained π/4 phase step is modeled explicitly as a 1 Hz internal modulation
sampled eight times per second.

## Safety and scientific boundary

This repository is a software simulation. Terms such as entropy, metabolic
drive, noosphere resonance, spacetime curvature, and law rewriting are model
vocabulary. They do not establish biological consciousness, thermodynamic
cooling, spacetime modification, or new physical laws.

The 8 Hz frequency, Φ1766 damping value, and 1.4% mutation probability are
configurable software parameters—not universal constants. `SilentGlyph` is a
simulation latch, not a substitute for authentication, authorization,
sandboxing, resource limits, or operational monitoring.

## Evidence labels

| Label | Meaning |
| --- | --- |
| `EST` | Established external result |
| `DER` | Derived or directly tested software behavior |
| `HYP` | Testable hypothesis |
| `SYM` | Symbolic architecture or metaphor |
| `STOP` | Unsupported claim that must not be promoted as established |

## Project documents

- [Contributing](CONTRIBUTING.md)
- [Security policy](SECURITY.md)
- [Changelog](CHANGELOG.md)
- [Repository working rules](AGENTS.md)

## License

No license has been declared yet. All rights remain with the repository owner
until a license file is added.
