# Store Tests

## Purpose

Map integration coverage for the native direct store.

## Table of Contents

- [budget-epoch.rs](budget_epoch.rs): owner-resume budget epoch accounting.
- [durable-boundaries.rs](durable_boundaries.rs): schema and transaction boundaries.
- [journal-effects.rs](journal_effects.rs): mkdir targets and exact settled retry identity.
- [native-constraints.rs](native_constraints.rs): native relational constraints.
- [native-direct-loop.rs](native_direct_loop.rs): direct projection and settlement continuity.
- [native-reopen.rs](native_reopen.rs): reopen and incompatible schema rejection.
- [native-settlement.rs](native_settlement.rs): observation and close settlement.
- [native-transactions.rs](native_transactions.rs): intake, compilation, provider, and effect atomicity.
