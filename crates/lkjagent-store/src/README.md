# Source

## Purpose

Map lkjagent-store source modules.

## Table of Contents

- [lib.rs](lib.rs): module exports.
- [plan-schema.rs](plan_schema.rs): ten-table schema setup.
- [plan-names.rs](plan_names.rs): database enum name parsing.
- [plan-rows.rs](plan_rows.rs): row structs for the plan store.
- [plan-access.rs](plan_access.rs): queue, task, and step accessors.
- [plan-hydrate.rs](plan_hydrate.rs): snapshot hydration from normalized rows.
- [plan-commit.rs](plan_commit.rs): atomic turn state commits.
- [plan-turn.rs](plan_turn.rs): command transaction helpers.
- [plan-inspect.rs](plan_inspect.rs): schema and log inspection helpers.
