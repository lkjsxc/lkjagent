# No Async Runtime

## Purpose

Record the decision to keep the daemon sequential and blocking.

## Context

The product runs one owner queue and at most one endpoint call per turn.
Concurrency would add failure surfaces without increasing throughput for the
core mission.

## Decision

The daemon uses a sequential blocking loop. The LLM client may use blocking
HTTP. No async runtime is required.

## Consequences

State transitions are easier to test and resume. Timeouts bound endpoint, shell,
and tool effects.

## Rejected Alternatives

Adding an async runtime would complicate locking, transactions, signal handling,
and tests for work that is intentionally sequential.
