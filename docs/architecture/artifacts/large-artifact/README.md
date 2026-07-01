# Large Artifact Engine

## Purpose

Define the daemon-owned engine for long structured artifacts that exceed one
model response. The engine converts an owner objective into durable atoms,
admits one bounded write contract, audits real files, assembles final targets,
and closes only from store-backed readiness.

## Table of Contents

- [objective-frame.md](objective-frame.md): deterministic extraction from the owner request.
- [profile-library.md](profile-library.md): shared artifact profiles and atom templates.
- [atom-graph.md](atom-graph.md): durable atom, edge, contract, event, assembly, and readiness rows.
- [write-contract-loop.md](write-contract-loop.md): bounded write selection and mutation admission.
- [audit-assembly-readiness.md](audit-assembly-readiness.md): real-file audit, assembly, and completion projection.
- [runtime-projection.md](runtime-projection.md): facts exposed to resolver, prompts, status, and task inspection.

## Contract

The large-artifact flow is:

```text
OwnerObjective
-> ObjectiveFrame
-> ArtifactPlan
-> ArtifactAtomGraph
-> NextAtomSelection
-> WriteContract
-> ModelAuthoredContent
-> ContractValidation
-> AtomAudit
-> DeterministicAssembly
-> ReadinessProjection
-> CompletionGate
```

The daemon owns the objective frame, profile, root, atom graph, exact paths,
byte budgets, count floors, active contract, audit rules, assembly, readiness,
and completion gate. The model writes only bounded content for the active atom.

## Status

implemented.
