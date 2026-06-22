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

The runtime computes one authority before an endpoint call and one authority
before dispatch. Exactly one active mode owns the effective tool policy,
endpoint decision, recovery route, compaction decision, maintenance boundary,
and completion policy. Prompt text explains the authority; it does not create
it.

## Status

partially implemented. Pure selection, authority cards, endpoint decisions,
effective dispatch policy, completion policy, and many `agent.done` refusals
exist. Durable authority snapshots, stale-action pre-dispatch refusal, richer
compaction snapshots, and artifact-aware completion proof remain open.
