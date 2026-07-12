# Functional Style

## Purpose

Define pure product logic and narrow effect boundaries.

## Core

Reduction, selection, prompt assembly, parsing, admission, check evaluation, and
completion are pure functions over plain typed values. Inputs contain explicit
current time, config, state, and source facts. Outputs contain commands or events,
not hidden effects.

Use immutable values and total matches. Preserve unknown extensible state. Stable
ordering and fingerprints do not depend on hash-map or database insertion order.

## Edges

SQLite, files, clocks, endpoint calls, shells, terminal IO, and process lifecycle
belong in edge crates or app orchestration. External intent is durable before an
effect. Results return as typed values and become events.

## Errors

Product crates return typed errors. They do not panic, unwrap, expect, use unsafe
code, or silently substitute success. Bounded diagnostics retain causal identity
without leaking secrets or failed bodies into model context.

## Source Shape

Each source file is at most 200 lines and owns one cohesive behavior. Splitting
creates real module boundaries rather than forwarding wrappers. Delete unused
paths rather than hiding them behind flags.

## Tests

Prefer compact table tests at contract boundaries and real black-box task proof.
A test double may prove deterministic mechanics but cannot serve as semantic
acceptance or product behavior.
