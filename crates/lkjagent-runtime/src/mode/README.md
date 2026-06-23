# lkjagent-runtime Active Mode

## Purpose

This directory owns deterministic runtime mission and active-mode selection.
One decision chooses the policy layer visible for a turn: owner work, recovery,
maintenance, compaction, completion, or closed idle.

## Table of Contents

- [mod.rs](mod.rs): module exports and path-based module map.
- [authority/](authority/README.md): turn inputs, mission selection, snapshots, and reducer.
- [ledger/](ledger/README.md): decision record, event, fingerprint, and ledger projections.
- [policy/](policy/README.md): tool admission, endpoint decisions, and active-mode policy.
- [recovery/](recovery/README.md): recovery plans and route metadata.
- [completion/](completion/README.md): completion policies and central close gate.
- [render/](render/README.md): compact authority and policy rendering.
