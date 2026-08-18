# Contributing

Thank you for improving the Quadralith Cognitive Engine.

## Development flow

1. Create a focused branch.
2. Keep symbolic vocabulary and executable behavior clearly separated.
3. Add or update tests for every state transition.
4. Run the complete local check set:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo build --release
```

5. Open a pull request describing behavior, boundaries, and validation.

## Design requirements

- All external numeric inputs must be finite and validated.
- Accumulating state must be bounded or have a documented capacity policy.
- `T€@X™` full performance must remain signature-gated and measurable.
- Never weaken rollback, authorization, isolation, or auditability silently.
- A software test demonstrates software behavior, not a new physical law.

