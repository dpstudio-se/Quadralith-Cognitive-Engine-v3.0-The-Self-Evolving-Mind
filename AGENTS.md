# Repository working rules

## Scope

This repository contains a software state machine. Preserve the symbolic names,
but do not present simulation variables as verified biology, cognition,
thermodynamics, cosmology, or spacetime physics.

## Required boundaries

- Mark claims as `EST`, `DER`, `HYP`, `SYM`, or `STOP` where ambiguity matters.
- Treat 8 Hz, Φ1766, and 1.4% mutation as configurable parameters.
- A passing software test verifies software behavior only.
- Never claim that `SilentGlyph` rewrites physical laws.
- Do not disable authentication, authorization, isolation, or audit controls in
  the name of transparency.
- Keep public API behavior documented and add tests for every state transition.
- Reject non-finite inputs and keep accumulated state bounded.
- Preserve rollback and safety-latch behavior unless a reviewed replacement
  provides equivalent or stronger guarantees.

## Validation

```sh
cargo test
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
```

