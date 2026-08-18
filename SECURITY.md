# Security policy

## Reporting

Please report security issues privately to the repository owner rather than in a
public issue. Include the affected version, reproduction steps, expected impact,
and any suggested mitigation.

## Project boundary

`SilentGlyph` is a simulation safety latch. It is not a security boundary by
itself. Applications embedding this crate remain responsible for authentication,
authorization, sandboxing, secret handling, resource limits, logging, and recovery.

The deterministic exploratory generator is not cryptographically secure and must
never be used for credentials, tokens, signing keys, or security decisions.

