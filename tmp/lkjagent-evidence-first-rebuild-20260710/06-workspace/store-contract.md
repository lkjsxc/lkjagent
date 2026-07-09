# Store Contract

## Connection Policy

Enable foreign keys, WAL, full synchronous durability for effect settlement,
busy timeout, and one daemon writer. Readers use short read transactions.
Checkpoint by size and during quiescent maintenance, never during an active
effect commit.

## Native Tables

- matters: lifecycle, objective, created and updated sequence;
- obligations: required predicate and current state;
- operations: semantic kind, dependencies, state, budget, idempotency key;
- runtime_events and runtime_decisions;
- runs binding a fresh evaluation store to source, binary, configuration,
  model, and scenario;
- tool_admissions, effect_journal, observations, checks;
- conversation_messages with unique sequence and logical ID;
- workspace_documents with unique document ID and normalized current path;
- workspace_revisions with immutable content hash, bytes ref, and parent;
- workspace_aliases and tombstones;
- index_debt and search_rows;
- context_items, context_edges, provider_exchanges, and token_usage.

## Constraints

Use foreign keys for every matter, operation, decision, admission, effect,
observation, check, message, and revision relation. Enforce unique idempotency
keys, conversation sequence, logical current message, document path, document
ID, revision hash, and one terminal effect outcome.

## Revisions

Store immutable prior and intended bytes in a content-addressed area until the
operation and retention policy permit cleanup. Fingerprint-only history is not
enough for rollback or same-day journal preservation.

Use SHA-256 with the sha256: prefix for workspace revision and evidence byte
fingerprints. Use separate clearly named hashes for non-content ranking or
cache keys.

## Search

Search rows are derived from current document revisions and carry document ID,
revision hash, excerpt offsets, kind, date, project, state, and tokenizer ID.
They never own source content.

## Schema Gate

Run foreign-key check, integrity check, uniqueness probes, operation-journal
state validation, and rebuild equivalence in deterministic and live evidence.
