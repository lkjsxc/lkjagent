# Store Schema

## Purpose

Define the fresh native SQLite schema and the fields that carry authority.

## Matters And Operations

`matters` stores opaque ID, run and scenario identity, objective, lifecycle,
priority, and created and updated causal sequences.

`obligations` stores opaque ID, matter ID, predicate kind and payload, required
flag, state, current passed same-matter check reference, and invalidating event.

`operations` stores opaque ID, matter ID, semantic kind, typed inputs and
outputs, budget, admission requirement, state, current flag, and unique
idempotency key. `operation_edges` stores unique typed acyclic edges.

## Runtime

`runs` binds source commit, executable and configuration fingerprints, clocks,
model identity, and scenario identity.

`runtime_events` stores matter, unique run causal sequence, kind, monotonic and
wall time, typed payload, and source identity.

`state_cells`, `state_cell_history`, and `state_edges` store concurrent facts,
old and new fingerprints, source events, guards, evidence, and dependencies.

`runtime_decisions` stores matter, event, optional operation, selected cells and
edges, operation and idempotency identity, prompt state, selected time, context,
tool, grammar, budget, recovery, and exit fingerprints, required observations
and checks, wake time, status, and settlement event. Selection time is stored as
`selected_monotonic_ms`. Each decision also stores `tool_count`, `prompt_tokens`,
`prompt_token_cap`, `semantic_duplicate_count`, `harness_json_count`, and
`unresolved_material_conflict_count`. Durable `useful` and `progressed` booleans
are derived from outcome evidence rather than model claims.

`context_frames`, `prompt_cards`, and `provider_exchanges` reference one
decision and source fingerprints. Exchanges store redacted request and response
refs, usage, timing, and outcome.

`failure_lineages` binds operation and decisions to prompt, tool-view, budget,
fault signature, strategy, external-condition fingerprint, next eligibility,
and remaining budget. Its causal tuple is unique.

## Effects And Checks

`tool_admissions` stores decision, action ordinal and fingerprint, model or
harness origin, effectful flag, accepted or rejected status, and typed reason.

`effect_journal` stores decision, command ordinal, unique non-null admission,
unique idempotency key, state, and intended, prior, and outcome fingerprints.
Observations reference exactly one journal row; rejected admissions have none.

`observations` stores a unique effect reference, status, attempt outcome,
bounded content reference, fingerprint, and contamination.

`checks` stores matter, operation, decision, stable kind and parameters, current
and passed flags, measured result, evidence fingerprint, and artifact or
document references.

## Conversation And Intake

`conversation_messages` stores unique logical ID and monotonic sequence, owner
or agent role, immutable body and fingerprint, lifecycle, causal event, and
optional replacement.

`owner_turns`, `commands`, `diagnostics`, `outbox_messages`, `config`, and
`daemon_leases` keep intake, operator surfaces, delivery, settings, and process
ownership separate from conversation.

## Workspace

`workspace_documents` stores unique document ID and normalized current path,
state, closed-registry kind, managed flag, effective date, project, and current
revision.

`workspace_revisions` stores document, optional parent, SHA-256, immutable
content-blob reference, provider tokenizer and count, conservative count,
admission count, and creating operation. Document and fingerprint are unique.

`content_blobs` preserves exact bytes by SHA-256. `workspace_aliases`,
`workspace_tombstones`, `workspace_relations`, `workspace_search_rows`, and
`workspace_index_debt` reference document IDs and revision fingerprints.

## Removal

A fresh store creates no task, step, template, plan-family, bridge, finish, or
synthetic-idle tables. Production selection, prompt compilation, effects,
recovery, and completion read none of `TaskSnapshot`, task, step, template, or
bridge projections.
