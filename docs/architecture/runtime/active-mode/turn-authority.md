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
- one registry-derived valid example or a runtime-only action note.

The value is pure. It is safe to store with a pending model action so dispatch
can compare the policy rendered to the endpoint with current runtime facts.

## Endpoint Decisions

Closed decisions are:

- `CallModel`: refresh context and call the endpoint.
- `RuntimeCompact`: run hard compaction without a model action.
- `DeliverOwner`: intake queued owner work before the next model turn.
- `DeferMaintenance`: stop maintenance because owner work exists.
- `ClosedIdle`: record idle state and do not call the endpoint.
- `WaitForRetry`: wait until the endpoint retry deadline.

## Dispatch Refresh

The daemon computes authority before every endpoint call and before every tool
dispatch. Dispatch uses the authority rendered to the endpoint only when the
current facts still admit that authority. If a stronger runtime-owned fact
appears before dispatch, the runtime refuses the stale model action and runs
the stronger decision first.

Stronger facts are hard compaction, queued owner work while maintenance owns
the stale action, recoverable owner work while maintenance owns the stale
action, and closed idle with no pending model action.

## Turn Flow

The daemon builds the snapshot, decides authority, applies runtime-only
decisions, renders exactly one authority card, calls the endpoint only when
selected, stores the authority with the model turn, refreshes authority before
dispatch, and dispatches only against the effective policy that remains
admitted.

## Status

partially implemented. Endpoint authority and cached dispatch authority exist.
Full stale-action refusal before dispatch remains open.
