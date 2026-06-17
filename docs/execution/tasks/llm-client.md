# LLM Client

## Purpose

Implement lkjagent-llm: the chat-completions client sending the narrow
request subset, reading the narrow response subset, with capped exponential
backoff and cache metric capture.

## Status

done

## Depends On

[bootstrap-workspace.md](bootstrap-workspace.md); message list type from
[context-engine.md](context-engine.md).

## Files To Read

- [../../architecture/llm/endpoint.md](../../architecture/llm/endpoint.md)
- [../../architecture/llm/sampling.md](../../architecture/llm/sampling.md)
- [../../architecture/protocol/recovery.md](../../architecture/protocol/recovery.md)
- [../../architecture/context/caching.md](../../architecture/context/caching.md)

## Files To Touch

- crates/lkjagent-llm/src/: wire.rs (serde types for exactly the
  documented fields), client.rs (one request function), backoff.rs (pure
  schedule), error.rs.
- crates/lkjagent-llm/tests/: wire serialization tables; a local stub
  server test for the request-response path and backoff behavior, labeled
  as the test double it is.

## Focused Gate

```sh
cargo test -p lkjagent-llm
cargo clippy -p lkjagent-llm -- -D warnings
```

## Acceptance

- Requests contain exactly the documented fields, with stop set to the
  closing act tag and stream false; asserted byte-level in tests.
- finish_reason length maps to the oversize handling path; connection
  failures map to the backoff schedule; the schedule is pure and
  table-tested including its cap.
- usage token counts and available cache metrics are returned to the
  caller for the ledger and transcript.
- This crate is the only one depending on HTTP and serde; asserted by the
  check-style dependency allowlist.
- Blocker row 6 done; llm area status moves in the ledger.

## Must Not

- Do not implement streaming, provider abstraction, or model fallback
  chains.
- Do not retry inside the client beyond the documented schedule; the loop
  owns escalation.
- Do not log or store the API key anywhere.
