# Source

## Purpose

Map lkjagent-store source modules.

## Table of Contents

- [lib.rs](lib.rs): module exports.
- [admission-rows.rs](admission_rows.rs): tool admission row helpers.
- [artifact-rows.rs](artifact_rows.rs): artifact ref and fingerprint rows.
- [context-rows.rs](context_rows.rs): context item row helpers.
- [decision-rows.rs](decision_rows.rs): runtime decision row helpers.
- [event-rows.rs](event_rows.rs): runtime event append and reducer apply helpers.
- [effect-recovery.rs](effect_recovery.rs): unresolved effect reconciliation.
- [exchange-rows.rs](exchange_rows.rs): provider exchange row helpers.
- [memory.rs](memory.rs): duplicate-suppressed memory writes and search.
- [observation-rows.rs](observation_rows.rs): bounded observation rows.
- [plan-schema.rs](plan_schema.rs): plan and state-ledger schema setup.
- [plan-names.rs](plan_names.rs): database enum name parsing.
- [plan-access.rs](plan_access.rs): queue, task, step, row, and schema inspection accessors.
- [plan-hydrate.rs](plan_hydrate.rs): snapshot hydration from normalized rows.
- [plan-commit.rs](plan_commit.rs): atomic turn state commits.
- [plan-turn.rs](plan_turn.rs): command transaction helpers.
- [prompt-rows.rs](prompt_rows.rs): prompt frame row helpers.
- [record-schema.rs](record_schema.rs): workspace record and operation revision setup.
- [record-rows.rs](record_rows.rs): workspace record metadata and history rows.
- [row-json.rs](row_json.rs): shared JSON and fingerprint error mapping.
- [state-rows.rs](state_rows.rs): state cell row helpers and hydration.
- [state-schema.rs](state_schema.rs): state-ledger table and index setup.
- [workspace-rows.rs](workspace_rows.rs): manifests, aliases, audits, and operation access.
- [workspace-search.rs](workspace_search.rs): SQLite search projection schema,
  deterministic replacement, and canonical rows.
