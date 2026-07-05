# Personal As Records

## Purpose

Record the decision to model personal work as generic workspace records.

## Context

Journal, calendar, TODO, finance, contact, and routine work needs durable owner
files, CLI helpers, state cells, edges, and proof evidence without creating a
second runtime authority.

## Decision

Personal work uses generic Markdown records under `workspace/records/` with
stable record ids, frontmatter refs, fingerprints, and state keys. Friendly CLI
commands may create or read those records, but their effects are ledger-backed
record writes, events, or enqueue operations.

## Consequences

Personal records use the same runtime events, state cells, state edges, checks,
artifacts, prompt admission, and proof bundles as other work. Unknown record
kinds remain valid so owner workflows can grow without central enum rewrites.

## Rejected Alternatives

Dedicated personal tables, private command state, or prompt-only personal modes
would create separate gates and failure modes outside the state-ledger control
plane.
