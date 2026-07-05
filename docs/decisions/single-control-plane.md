# Single Control Plane

## Purpose

Record the decision that durable state rows and persisted runtime decisions are
the only runtime authority.

## Context

The harness must direct a weak model without asking it to navigate competing
policy surfaces. The earlier plan ledger proved the value of durable authority,
but fixed task and step enums cannot express arbitrary simultaneous state.

## Decision

The state ledger is the single control plane. Runtime state is stored as durable
case, event, state-cell, state-edge, decision, context, admission, observation,
check, record-evidence, and artifact rows. A `RuntimeDecision` is persisted for
each turn before prompts, endpoint calls, tool admission, effects, recovery,
compaction, or completion.

The plan remains a state family for ordered artifact work, not the only control
plane.

## Consequences

Prompt rendering, parser contracts, action admission, effect dispatch, status,
resume, record commands, state-edge diagnostics, and proof bundles all project
the same decision row. There is no prompt-only policy, dispatcher-only policy,
private command state, or second graph authority.

## Rejected Alternatives

A separate graph, mode layer, or hidden tool policy could disagree with the
selected decision and make legal output impossible to identify from the prompt.
