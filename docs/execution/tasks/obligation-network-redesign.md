# Obligation Network Redesign

## Purpose

Replace tool-name-driven next action selection with runtime facts,
obligations, resolver plans, write contracts, and progress keys.

## Status

open

## Depends On

- [deep-redesign-gates.md](deep-redesign-gates.md)

## Files To Read

1. [../../current-state.md](../../current-state.md)
2. [../current-blockers.md](../current-blockers.md)
3. [../../architecture/runtime/obligation-network/README.md](../../architecture/runtime/obligation-network/README.md)
4. [../../architecture/artifacts/root-repair.md](../../architecture/artifacts/root-repair.md)
5. [../../architecture/tools/doc-tools.md](../../architecture/tools/doc-tools.md)

## Files To Touch

- `docs/architecture/runtime/obligation-network/`
- `docs/architecture/artifacts/root-identity.md`
- `docs/architecture/artifacts/root-repair.md`
- `crates/lkjagent-runtime/src/kernel/`
- `crates/lkjagent-tools/src/artifact_next_example.rs`
- focused runtime and tools tests

## Focused Gate

```sh
cargo fmt --check
cargo test -p lkjagent-tools artifact_next_missing_root_returns_write_contract
cargo test -p lkjagent-tools root_identity
cargo test -p lkjagent-runtime missing_root
cargo test -p lkjagent-runtime obligation
```

## Acceptance

- Missing root audit facts force `fs.batch_write` with a root identity contract.
- Missing root recovery never forces another same-root `doc.audit`.
- Root identity paths are flat and avoid single-child subdirectories.
- Root identity content can pass `doc.audit`.
- `artifact.next` missing-root output yields the same contract shape.
- Repeat guards use progress keys that include target, action class, and fact digest.
- Completion remains blocked until audit and verification facts are present.

## Must Not

- Do not route `missing_root` to another `doc.audit`.
- Do not seed a story artifact with only one README.
- Do not use graph policy as fallback dispatch authority.
- Do not let direct graph evidence satisfy audit-owned requirements.
