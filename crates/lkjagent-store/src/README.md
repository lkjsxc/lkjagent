# Source

## Purpose

Map lkjagent-store source modules. Existing runtime modules remain until the
cutover task removes them.

## Table of Contents

- [lib.rs](lib.rs): module exports.
- [plan-schema.rs](plan_schema.rs): ten-table schema setup.
- [plan-rows.rs](plan_rows.rs): row structs for the plan store.
- [plan-access.rs](plan_access.rs): queue, task, and step accessors.
- [plan-turn.rs](plan_turn.rs): atomic command transaction helpers.
- [plan-inspect.rs](plan_inspect.rs): schema and log inspection helpers.
