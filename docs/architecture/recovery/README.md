# Recovery

## Purpose

Define deterministic recovery after parse faults, parameter faults, admission
contradictions, repeat refusals, audit failures, compaction gaps, and false
completion attempts.

## Table of Contents

- [fault-classes.md](fault-classes.md): fault signatures and routing.
- [retry-budget.md](retry-budget.md): retry counts and escalation rules.
- [non-repetition.md](non-repetition.md): repeated invalid action prevention.
- [tool-escape-hatches.md](tool-escape-hatches.md): read, audit, and repair tools that stay admitted.
- [partial-handoff.md](partial-handoff.md): structured handoff when repair cannot continue.

## Contract

Recovery is a runtime-owned finite-state machine. The model can fill admitted
content, but it does not choose the class, budget, or allowed escape tools.

## Status

design-only as a separate architecture directory; pieces exist under runtime
authority and action reliability.
