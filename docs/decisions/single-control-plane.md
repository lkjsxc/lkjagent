# Single Control Plane

## Purpose

Record that events, reduced state, and persisted decisions are runtime authority.

## Context

The harness must direct bounded model work without competing policy, graph,
prompt, or dispatcher authorities.

## Decision

Runtime events reduce into durable state cells and typed edges. The pure
selector derives one `RuntimeDecision` from current state, policy, and time.
That decision persists before prompt compilation, endpoint calls, admission,
effects, recovery, compaction, checks, or completion.

Matters, obligations, operations, events, cells, decisions, admissions,
observations, and checks are one causal ledger. Workspace source bytes remain
owner content and are referenced by document identity and revision.

## Consequences

Prompt rendering, parser contracts, tool admission, effect dispatch, status,
resume, record commands, diagnostics, TUI views, and proof all reference the
same decision and causal events. There is no prompt-only policy,
dispatcher-only policy, private command state, or second graph authority.

## Rejected Alternatives

A separate execution graph, hidden tool policy, or model-selected completion
could disagree with the persisted decision and make recovery unauditable.
