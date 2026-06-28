# Deep Redesign Provider Handoff

## Purpose

Classify reasoning-only provider output before parsing and stop repeated same-prompt loops.

## Status

open

## Depends On

[deep-redesign-runtime-authority.md](deep-redesign-runtime-authority.md)

## Files To Read

- [../../model-interface/provider-anomalies.md](../../model-interface/provider-anomalies.md)
- [../../architecture/llm/endpoint.md](../../architecture/llm/endpoint.md)

## Files To Touch

- `crates/lkjagent-llm/src/wire/response.rs`
- `crates/lkjagent-runtime/src/daemon/loop/endpoint.rs`
- `crates/lkjagent-runtime/src/kernel/fault.rs`
- provider anomaly tests

## Focused Gate

```sh
cargo test -p lkjagent-llm
cargo test -p lkjagent-runtime --test provider_anomaly
```

## Acceptance

- Reasoning-only responses do not increment parse fault counters.
- Exhausted anomaly budget pauses or records blocked handoff with case and prompt ids.

## Must Not

- Do not concatenate provider reasoning into action text.
