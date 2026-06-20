# Turn Authority

## Purpose

Define the runtime decision object that owns an endpoint turn and the matching
dispatch.

## Object

`TurnAuthority` contains:

- active mode.
- selection input snapshot.
- effective dispatch policy.
- completion policy.
- endpoint decision.
- prompt authority card.
- dispatch refusal card.

It is a pure value. It is safe to store with the pending model action so
dispatch uses the same authority that was rendered to the endpoint.

## Endpoint Decisions

Closed decisions are:

- `CallModel`: refresh context and call the endpoint.
- `RuntimeCompact`: run hard compaction without a model action.
- `DeliverOwner`: intake queued owner work before the next model turn.
- `DeferMaintenance`: stop maintenance because owner work exists.
- `ClosedIdle`: record idle state and do not call the endpoint.
- `WaitForRetry`: wait until the endpoint retry deadline.

## Turn Flow

The daemon builds the snapshot, decides authority, applies runtime-only
actions, renders exactly one authority card, calls the endpoint only when
selected, stores the authority with the model turn, and dispatches the parsed
action against that same authority.
