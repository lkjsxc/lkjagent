# Benchmarks

## Purpose

Define the deterministic benchmark corpus.

## Corpus Record

Each benchmark case is an objective, optional fixture workspace, expected plan
shape, and checks from [../checks/catalog.md](../checks/catalog.md). Judges are
check evaluators, not separate task-family code.

## Gate

`cargo run -p lkjagent-xtask -- bench check-corpus` validates the corpus. A
passing run prints `ok bench-corpus`.

## Coverage

The corpus includes manuscript planning, docs-tree planning, file work,
questions, blocked reports, parse faults, check failures, and replayed live
failure signatures.

## Fixture Rule

Fixtures are recorded from real runs or constructed directly from the written
contract. Each fixture states which source it uses and what failure or behavior
it guards.
