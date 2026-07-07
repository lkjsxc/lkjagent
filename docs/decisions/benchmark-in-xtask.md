# Benchmark In Xtask

## Purpose

Record the decision to host evaluation gates in xtask.

## Context

Benchmarks, replay, repository checks, and proof collection are developer
operations, not daemon runtime behavior.

## Decision

The benchmark corpus, judges, replay smoke, and proof collector live in
`lkjagent-xtask`.

## Consequences

The daemon stays focused on owner turns and matters. Evaluation code can depend on core,
store, and effects without importing the app binary.

## Rejected Alternatives

A separate benchmark crate would add workspace surface area without owning a
runtime responsibility.
