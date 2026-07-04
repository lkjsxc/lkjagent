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
- [exchange-rows.rs](exchange_rows.rs): provider exchange row helpers.
- [memory.rs](memory.rs): duplicate-suppressed memory writes and search.
- [observation-rows.rs](observation_rows.rs): bounded observation rows.
- [plan-schema.rs](plan_schema.rs): plan and state-ledger schema setup.
- [plan-names.rs](plan_names.rs): database enum name parsing.
- [plan-rows.rs](plan_rows.rs): row structs for the plan store.
- [plan-access.rs](plan_access.rs): queue, task, and step accessors.
- [plan-hydrate.rs](plan_hydrate.rs): snapshot hydration from normalized rows.
- [plan-commit.rs](plan_commit.rs): atomic turn state commits.
- [plan-turn.rs](plan_turn.rs): command transaction helpers.
- [prompt-rows.rs](prompt_rows.rs): prompt frame row helpers.
- [plan-inspect.rs](plan_inspect.rs): schema and log inspection helpers.
- [row-json.rs](row_json.rs): shared JSON and fingerprint error mapping.
- [state-rows.rs](state_rows.rs): state cell row helpers and hydration.
- [state-schema.rs](state_schema.rs): state-ledger table and index setup.
