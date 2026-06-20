# Active Mode

## Purpose

Define the runtime authority that selects exactly one mode for each turn and
makes that mode the source for prompt policy, dispatch policy, compaction,
maintenance, and completion.

## Table of Contents

- [selection.md](selection.md): input facts and mode priority.
- [turn-authority.md](turn-authority.md): pure decision object and turn flow.
- [prompt-rendering.md](prompt-rendering.md): model-visible authority card.
- [dispatch-policy.md](dispatch-policy.md): effective policy used by tools.
- [completion-policy.md](completion-policy.md): close gates by mode.
- [preemption.md](preemption.md): owner work over maintenance.
- [compaction.md](compaction.md): runtime-owned context pressure handling.
- [maintenance.md](maintenance.md): idle work boundary and allowed tools.

## Contract

The runtime computes one turn authority before an endpoint call and before any
dispatch. That authority selects one active mode, one effective tool policy,
one endpoint decision, and one completion policy. Prompt text explains the
authority; it does not create it.

## Status

partially implemented. Pure turn authority selection, endpoint decision,
completion policy, and policy rendering exist. Full endpoint-turn authority
and shared dispatch authority remain open.
