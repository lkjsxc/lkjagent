# Tools

## Purpose

Define the tool catalog, policy layers, decision-specific tool views,
admissions, observations, and guards.

## Table of Contents

- [registry.md](registry.md): canonical descriptor fields for the tool catalog.
- [policy.md](policy.md): layered policy that derives per-decision access.
- [toolset-view-and-admission.md](toolset-view-and-admission.md): prompt-visible
  tool projection and tool-call admission.
- [observations.md](observations.md): returned observation envelope and example.
- [guards.md](guards.md): path, budget, repeat, and recovery guards.

## Failure This Prevents

Tools shown to the model are exactly the tools the harness can admit for the
same persisted runtime decision.
