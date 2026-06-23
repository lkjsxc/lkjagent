# lkjagent-runtime Active Mode

## Purpose

This directory owns deterministic active-mode selection. One mode decides which
policy layer is visible for a turn: owner task, recovery, maintenance,
compaction, or closed idle.

## Table of Contents

- [mod.rs](mod.rs): module exports.
- [admission.rs](admission.rs): pure tool admission and next-tool examples.
- [authority.rs](authority.rs): pure turn authority assembly.
- [completion.rs](completion.rs): mode-specific completion policies.
- [decision.rs](decision.rs): endpoint decisions for runtime actions.
- [input.rs](input.rs): turn authority snapshot input.
- [ledger.rs](ledger.rs): persisted decision-record assembly.
- [ledger_data.rs](ledger_data.rs): decision record, kind, and fingerprint data.
- [ledger_event.rs](ledger_event.rs): event labels and fingerprint helper.
- [ledger_fields.rs](ledger_fields.rs): decision-to-ledger field projection.
- [mission.rs](mission.rs): runtime mission enum and mode mapping.
- [model.rs](model.rs): active-mode input, mode, and policy records.
- [policy.rs](policy.rs): allowed tools and policy-layer flags per mode.
- [reducer.rs](reducer.rs): pure runtime snapshot and event reducer.
- [render.rs](render.rs): compact active-mode notice rendering.
- [select.rs](select.rs): pure mode selector.
