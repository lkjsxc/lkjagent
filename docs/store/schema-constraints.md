# Schema Constraints

## Purpose

Define integrity rules that make native rows authoritative and auditable.

## Identity And Scope

All IDs are nonempty opaque text generated before dependent work. Campaign rows
carry `run_id`. Matters use unique run and scenario identity when a scenario is
declared. Evidence capture rejects copied history and scopes every query to its
one declared run.

## Matter And Operation

- obligations reference a matter and use a current passed same-matter check;
- operations reference a matter and have a unique idempotency key;
- operation edges reject self edges, duplicate triples, and reducer cycles;
- a current effect operation requires exactly one accepted admission.

## Runtime Lineage

- events have unique run and causal sequence;
- decisions reference a matter, source event, and optional operation;
- prompt, context, exchange, admission, effect, observation, check, and message
  rows reference their decision or causal event;
- request fingerprints, attempt ordinals, and provider exchange IDs are unique;
- rendered decision fingerprints are non-null before execution.

## Conversation

Messages have unique logical ID and monotonic sequence, owner or agent role,
immutable body and fingerprint, lifecycle, causal event, and optional
replacement. Replacement preserves sequence and one current rendering. Commands
and diagnostics use separate tables.

## Workspace

Documents have unique ID and normalized current path. Current revisions belong
to their document. Revisions preserve exact bytes by SHA-256 and reject duplicate
document fingerprints. Aliases, tombstones, relations, search rows, and index
debt use document IDs rather than path identity.

## Database Policy

Every connection enables foreign keys, WAL, busy timeout, and full synchronous
durability for effect settlement. One daemon owns writes. Fresh-schema changes
run in an exclusive transaction and pass integrity, foreign-key, index, and
state-machine probes before activation. Evidence snapshots use SQLite Online
Backup at a quiesced boundary.
