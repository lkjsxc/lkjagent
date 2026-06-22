# Hard State

## Purpose

Hard state owns legal execution. Each durable case has one current node, one
phase, allowed tools, blocked tools, and completion gates.

## Nodes

The runtime uses these hard nodes for documentation and recovery work:

```text
queued
admitted
intake
objective-contract
topic-contract
seed-structure
seed-audit
expansion-plan
local-expansion
structure-audit
relation-pass
semantic-audit
repairing
recovering
context-compacting
maintenance-discovery
maintenance-refactor
verifying
complete
blocked
abandoned
```

## Authority

- A transition is legal only when the graph names it for the current node.
- A tool is admitted only when the hard state allows it and no guard track
  blocks it.
- Completion is legal only from `verifying` or an equivalent complete-ready
  node with every gate satisfied.
- Audit-owned evidence is recorded by the audit reducer, not by model text.

## Links

- Weighted pressure: [weighted-state.md](weighted-state.md).
- Completion gates: [transition-handbook.md](transition-handbook.md).
- Tool authority: [../architecture/runtime/authority/tool-policy.md](../architecture/runtime/authority/tool-policy.md).

## Status

implemented
