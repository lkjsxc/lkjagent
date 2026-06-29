# Obligation Network

## Purpose

Define the runtime fact, obligation, resolver, and contract network that selects
one safe next action without falling back to tool-name ladders.

## Contract

The runtime owns this pure algebra:

```text
RuntimeSnapshot + RuntimeEvent -> RuntimeFacts
RuntimeFacts -> Vec<Obligation>
Vec<Obligation> + RuntimeFacts -> ResolverPlan
ResolverPlan -> RuntimeDecision
RuntimeDecision + ModelAction -> ToolAdmission
Observation -> RuntimeEvent
```

The graph may rank and suggest. It never admits a tool, repeats a fallback, or
closes a case. The model may author semantic content only when the persisted
runtime decision contains a write contract with exact paths and limits.

## Root Repair Rule

A document audit that reports `missing_root` or an artifact next observation
that reports `root_missing` creates a root identity obligation. The resolver
must choose a semantic write contract and force `fs.batch_write`. It must not
choose another same-root `doc.audit` until a write progresses, a new fact digest
appears, or a blocked handoff is recorded.

## Table of Contents

- [facts.md](facts.md): runtime facts derived from snapshots, events, and observations.
- [obligations.md](obligations.md): obligation set and priority rules.
- [resolvers.md](resolvers.md): resolver plans and decision mapping.
- [progress.md](progress.md): progress keys and repeated-route guards.

## Related Files

- [../authority/README.md](../authority/README.md)
- [../../artifacts/root-identity.md](../../artifacts/root-identity.md)
- [../../artifacts/root-repair.md](../../artifacts/root-repair.md)
