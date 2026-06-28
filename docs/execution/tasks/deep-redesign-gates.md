# Deep Redesign Gates

## Purpose

Prove the redesign through focused tests, corpus checks, quiet verify, and Docker Compose.

## Status

open

## Depends On

All earlier deep-redesign tasks.

## Files To Read

- [../../operations/verification.md](../../operations/verification.md)
- [../../evaluation/uploaded-run-fixtures.md](../../evaluation/uploaded-run-fixtures.md)

## Files To Touch

- benchmark fixtures and judges as needed
- checked-in smoke fixtures as appropriate

## Focused Gate

```sh
cargo fmt --check
cargo test -p lkjagent-protocol
cargo test -p lkjagent-tools
cargo test -p lkjagent-runtime
cargo test -p lkjagent-store
cargo run -p lkjagent-xtask -- check-docs
cargo run -p lkjagent-xtask -- check-lines
cargo run -p lkjagent-xtask -- check-style
cargo run -p lkjagent-xtask -- benchmark check-corpus
cargo run -p lkjagent-xtask -- quiet verify
docker compose run --rm verify
```

## Acceptance

- Local quiet verify passes.
- Docker Compose verify passes from the image build.
- Any endpoint smoke result is recorded only if endpoint configuration exists.

## Must Not

- Do not claim unrun gates.
