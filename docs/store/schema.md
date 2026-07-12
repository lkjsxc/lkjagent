# Store Schema

## Purpose

Define the fresh active SQLite tables and authority constraints.

## Identity And Intake

- `matters`: objective and reducer-derived lifecycle projection.
- `owner_turns`: queue order, raw text, delivery, and matter link.
- `conversation_messages`: logical ID, sequence, role, immutable body, exact final
  receipt, lifecycle, cause, and replacement.
- `obligations`: required predicate and reducer-derived state.

## Runtime

- `runtime_events`: immutable causal inputs and typed payloads.
- `state_cells`: current reduced state and source event.
- `runtime_decisions`: immutable selection/spec, compiler attachments, status.
- `provider_exchanges`: request intent, response outcome, usage, timing.
- `context_items`: source identity, revision, semantic key, trust, body reference.
- `daemon_leases`: process ownership and heartbeat.
- `config_fingerprints`: effective nonsecret configuration identity.

## Effects And Checks

- `tool_admissions`: parsed call, exact tool spec, decision, status, reason.
- `effect_journal`: idempotency, stage/exchange phase, fingerprints, settlement.
- `effect_targets`: normalized path, exact bytes, mode, stage identity.
- `observations`: one bounded result per attempted effect.
- `checks`: obligation parameters, measured result, source revision, freshness.
- `workspace_documents`: stable document ID, current path, current revision.
- `workspace_revisions`: immutable SHA-256, bytes reference, parent, effect.

Add state edges or search tables only with their first real consumer.

## Constraints

Foreign keys are enabled. Causal sequences, logical messages, decision/effect
identities, idempotency keys, and document revisions are unique. Conversation
sequence is positive and globally unique. Owner rows have no completion receipt;
agent rows require receipt bytes and a fingerprint computed from the ordered
current passed checks at close. Accepted effect admissions have
one journal; rejected admissions have none. An attempted effect has one
observation. Close requires current checks and no unsettled journal.

## Fresh Store

Active setup creates no task, step, fixed template, plan, bridge snapshot, or
synthetic idle table. Product source contains no reader for those retired rows.
An old store is rejected without mutation.

Internal payloads and endpoint transport may use JSON. Raw payload objects are
never rendered into model context.
