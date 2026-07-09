# Schema Constraints

## Identity And Scope

All IDs are nonempty opaque text primary keys generated before dependent work.
Every campaign-owned row carries `run_id` referencing `runs(id)`. Matters carry
unique `(run_id, scenario_key)` when a scenario key exists. Evidence capture
uses a fresh store and rejects any second run. Production stores may retain many
runs, but every diagnostic query is explicitly scoped.

## Matter And Operation

- obligations require a matter; `satisfied_by_check_id` must point to a passed,
  current check for the same matter before state becomes satisfied;
- operations require a matter, have unique idempotency key, and use a check
  constraint over state;
- operation edges have unique `(from_id, to_id, kind)`, reject self edges, and
  are acyclic under the reducer;
- a current effect operation has `requires_admission=1` and exactly one
  admission; superseded historical operations remain immutable.

## Runtime Lineage

- events have unique `(run_id, causal_sequence)` and source identity;
- decisions reference matter, event, and optional operation, with unique
  selection sequence;
- each prompt card, context frame, exchange, admission, effect, observation,
  check, and conversation message references its decision or causal event;
- provider exchange IDs, attempt ordinals, and request fingerprints are unique;
- every rendered fingerprint column is non-null after decision persistence.

## Effect Journal

Admissions have origin `model` or `harness`, status `accepted` or `rejected`,
and unique `(decision_id, action_ordinal)`. Only accepted admissions may own an
effect. `effect_journal.admission_id` and idempotency key are unique and non-null.
Effect states are prepared, applying, committed, recovered, compensated, or
failed. Exactly one immutable observation settles every attempted effect; a
partial unique index permits only one current terminal outcome. Rejected
admissions have no effect row.

## Conversation

Conversation messages have globally unique logical ID and monotonic sequence,
role owner or agent, immutable body, body fingerprint, lifecycle, causal event,
and optional replacement relation. Replacement cannot change sequence or create
a second current rendering. Slash commands and diagnostics use separate tables.

## Workspace

Documents have unique document ID and normalized current path. State is active,
invalid, archived, or tombstoned; only non-tombstoned rows require current files.
Kinds come from the documented closed registry. `managed` controls header and
token admission, never ownership of external source bytes.

Revisions require document, parent revision when present, SHA-256, immutable
content-blob ref, provider tokenizer/count, conservative count, and creation
operation. Unique `(document_id, fingerprint)` prevents duplicate revisions.
The current revision must belong to the document. Content blobs are keyed by
SHA-256 and retain exact bytes through effect settlement and retention.

Aliases, tombstones, relations, search rows, and index debt reference document
IDs and revision fingerprints. Search rows are unique by document, revision,
field, and byte range. One active debt row exists per document and projection.

## Database Policy

At every connection enable foreign keys, WAL, busy timeout, and full synchronous
durability for effect settlement. One daemon owns writes. Migrations run inside
an exclusive transaction, are forward-only for the fresh schema, and pass
integrity, foreign-key, index, and state-machine probes before activation.
SQLite Online Backup creates evidence snapshots at a quiesced boundary.
