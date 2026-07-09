# Store Schema

## Core Tables

matters:

- id primary key;
- scenario_key nullable stable live-scenario identity;
- objective, lifecycle, priority;
- created_sequence and updated_sequence.

obligations:

- id primary key and matter_id foreign key;
- predicate_kind and predicate_payload;
- required boolean and state;
- satisfied_by_check_id nullable foreign key;
- invalidated_by_event_id nullable foreign key.

operations:

- id primary key and matter_id foreign key;
- semantic_kind, inputs, outputs, budget;
- requires_admission boolean;
- state, current boolean, idempotency_key unique;
- parent and dependency edges in operation_edges.

## Runtime Tables

runs:

- id primary key, source commit, binary and configuration fingerprints;
- monotonic and wall boundaries, model identity, scenario identity;
- evidence snapshots contain exactly the declared run, never seeded history.

runtime_events:

- id primary key, matter_id, causal sequence, kind;
- monotonic_ms and wall time;
- typed payload stored internally;
- source_ref naming decision, owner turn, or effect.

runtime_decisions:

- id primary key, matter_id, operation_id;
- selected state keys and fingerprints;
- selected_monotonic_ms;
- context, tool-view, grammar, budget, recovery, and exit fingerprints;
- tool_count, prompt_tokens, prompt_token_cap, semantic_duplicate_count,
  harness_json_count, and unresolved_material_conflict_count;
- status and settlement event;
- useful and progressed booleans derived from durable outcome evidence.

context_frames and prompt_cards link to one decision and source fingerprints.
provider_exchanges link to one decision and store redacted request, response,
usage, timing, and outcome refs.

failure_lineages link operation and decisions to prompt, tool-view, budget,
fault signature, strategy, external-condition fingerprint, next eligibility,
and remaining budget. The full causal tuple is unique.

## Effect Tables

tool_admissions:

- id primary key, decision_id, action fingerprint;
- origin is model or harness; every external or workspace effect has one
  admission regardless of origin;
- effectful boolean distinguishes read-only admissions from effect intents;
- accepted or rejected status and typed reason;
- unique decision and action ordinal.

effect_journal:

- id primary key, decision_id;
- admission_id non-null unique foreign key;
- idempotency key unique;
- prepared, applying, committed, recovered, compensated, or failed state;
- intended, prior, and outcome fingerprints.

observations:

- id primary key and effect_id unique foreign key;
- status, attempt_outcome, bounded content ref, fingerprint, contamination.

checks:

- id primary key, matter_id, operation_id, decision_id;
- stable check kind and parameters;
- current boolean, passed boolean, measured result;
- evidence fingerprint and artifact or document refs.

## Conversation And Workspace

conversation_messages use logical_id unique, sequence unique, role, lifecycle,
causal refs, body fingerprint, and replacement relation.

workspace_documents use document_id primary key, normalized path unique, kind,
managed boolean, state, effective date, project, and current_revision_id.
workspace_revisions use revision ID, document ID, parent revision, SHA-256,
immutable bytes ref, tokenizer ID, provider token count, deterministic
conservative token count, and admission token count.

workspace_aliases, tombstones, index_debt, search_rows, and relation rows use
document IDs rather than path identity.

## Internal Encoding

Typed internal payloads may use JSON inside SQLite. Raw payload JSON is never
rendered to the model. Prompt compilation projects typed fields to
attribute-free cards.

## Removal

Do not create task, step, template, plan-family, bridge, or synthetic-idle
tables in the fresh runtime store.
