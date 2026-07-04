# Source

## Purpose

Map lkjagent-store source modules.

## Table of Contents

- [lib.rs](lib.rs): module exports.
- [context-rows.rs](context_rows.rs): context item row helpers.
- [decision-rows.rs](decision_rows.rs): runtime decision row helpers.
- [memory.rs](memory.rs): duplicate-suppressed memory writes and search.
- [plan-schema.rs](plan_schema.rs): plan and state-ledger schema setup.
- [plan-names.rs](plan_names.rs): database enum name parsing.
- [plan-rows.rs](plan_rows.rs): row structs for the plan store.
- [plan-access.rs](plan_access.rs): queue, task, and step accessors.
- [plan-hydrate.rs](plan_hydrate.rs): snapshot hydration from normalized rows.
- [plan-commit.rs](plan_commit.rs): atomic turn state commits.
- [plan-turn.rs](plan_turn.rs): command transaction helpers.
- [plan-inspect.rs](plan_inspect.rs): schema and log inspection helpers.
- [row-json.rs](row_json.rs): shared JSON and fingerprint error mapping.
- [state-rows.rs](state_rows.rs): state cell row helpers and hydration.
- [state-schema.rs](state_schema.rs): state-ledger table and index setup.
