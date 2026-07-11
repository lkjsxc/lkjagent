# Schema Constraints

## Purpose

Define integrity rules that make native rows authoritative and auditable.

## Identity And Scope

All IDs are nonempty opaque text generated before dependent work. Campaign rows
carry `run_id`. Matters use unique run and scenario identity when a scenario is
declared. An evidence store starts fresh and rejects any second run. Production
stores may retain many runs, but every diagnostic query is explicitly scoped.

## Matter And Operation

- obligations reference a matter and use a current passed same-matter check;
- operations reference a matter, have a unique idempotency key, and use a state
  check constraint;
- operation edges reject self edges, duplicate triples, and reducer cycles;
- a current effect operation requires exactly one accepted admission.

## Runtime Lineage

- events have unique run and causal sequence;
- decisions reference a matter, source event, and optional operation, with a
  unique selection sequence;
- prompt, context, exchange, admission, effect, observation, check, and message
  rows reference their decision or causal event;
- request fingerprints, attempt ordinals, and provider exchange IDs are unique;
- rendered decision fingerprints are non-null before execution.

## Effect Journal

Only accepted admissions may own effects. Effect admission and idempotency keys
are unique and non-null. Exactly one immutable observation settles each attempted
effect, and a partial unique index permits only one current terminal outcome.
Target ordinals and paths are unique within an effect. A generated bundle may
settle successfully only when every exact target and closed part membership
matches intended presence or absence. Root refs come from durable intents, and
child intents require a present same-case parent. Artifacts, refs, observation,
and journal state settle in the same transaction. Rejected admissions have no
effect row.

## Conversation

Messages have unique logical ID and monotonic sequence, owner or agent role,
immutable body and fingerprint, lifecycle, causal event, and optional
replacement. Replacement preserves sequence and one current rendering. Commands
and diagnostics use separate tables.

## Workspace

Documents have unique ID and normalized current path. State is `active`,
`invalid`, `archived`, or `tombstoned`; only non-tombstoned rows require current
files. `managed` controls header and token admission, never ownership of external
bytes. Current revisions belong to their document. Revisions preserve exact
bytes by SHA-256 and reject duplicate document fingerprints. Aliases,
tombstones, relations, search rows, and index debt use document IDs rather than
path identity. Exactly one active index-debt row exists per document and
projection.

## Database Policy

Every connection enables foreign keys, WAL, busy timeout, and full synchronous
durability for effect settlement. One daemon owns writes. Fresh-schema changes
run in an exclusive transaction and pass integrity, foreign-key, index, and
state-machine probes before activation. Evidence snapshots use SQLite Online
Backup at a quiesced boundary.
