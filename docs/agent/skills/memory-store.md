# Skill: Memory Store

## Purpose

Change the store schema, transcripts, retrieval, or distillation while
keeping the store the single honest record: append-only events, editable
memory, one transaction per write.

## Trigger

The store schema, transcripts, retrieval, or distillation is changing.

## Context

- [../../architecture/memory/store.md](../../architecture/memory/store.md): tables, transaction rules, the lock row.
- [../../architecture/memory/transcripts.md](../../architecture/memory/transcripts.md): event kinds and ordering guarantees.
- [../../architecture/memory/retrieval.md](../../architecture/memory/retrieval.md): ranking and the digest builder.
- [../../architecture/memory/distillation.md](../../architecture/memory/distillation.md): when rows are written and their quality bar.
- [../../decisions/sqlite-store.md](../../decisions/sqlite-store.md): the settled ground.

## Procedure

1. Write the schema or behavior change into the owning memory doc first;
   a new event kind also needs the hygiene or recovery row that produces
   it, and a new memory kind needs a distillation rule and a ranking
   weight.
2. Since nothing is preserved across schema changes by policy, change the
   schema in place: the setup code creates the new shape, and existing
   stores are reset rather than migrated. Say so in the handoff.
3. Implement in lkjagent-store: schema setup, typed row structs, and
   narrow query functions; no SQL strings outside this crate.
4. Keep the two-write invariants: queue delivery plus owner event in one
   transaction; every event append is one transaction; events have no
   update or delete path in the API surface at all.
5. Test against real SQLite in memory: ordering under interleaved writers,
   delivery exactly-once, FTS ranking with kind weights, digest selection
   under the 2,048-token budget.
6. Update [../../product/observability.md](../../product/observability.md)
   if what the CLI can show changed.

## Checks

- `cargo test -p lkjagent-store` passes, including the exactly-once
  delivery test and the ranking table test.
- The API surface exposes no update or delete for events (compile-time:
  no such function exists).
- `cargo run -p lkjagent-xtask -- quiet verify` prints ok verify.

## Must Not

- Do not write migration shims; reset semantics are the policy and the
  handoff states them.
- Do not let any other crate open the database file; the store crate is
  the only SQL boundary.
- Do not add an embedding or vector dependency; retrieval is lexical by
  decision.
- Do not store skill bodies, secrets, or config values in the store.
