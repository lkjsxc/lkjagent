# Engine

## Purpose

Define the plan-family helpers that remain inside the broader state-ledger
runtime.

## Table of Contents

- [task-model.md](task-model.md): current task rows and their state-ledger
  replacement shape.
- [plan-and-steps.md](plan-and-steps.md): ordered artifact work as plan state
  cells.
- [step-kinds.md](step-kinds.md): known plan operation helpers and envelopes.
- [turn-cycle.md](turn-cycle.md): mapping from old turn cycle to runtime
  decisions.
- [retry-and-escalation.md](retry-and-escalation.md): finite recovery ladder as
  state transitions.
- [completion.md](completion.md): checks-gated closure for steps and cases.
- [templates/](templates/): template contracts for initial events and state
  cells.

## Failure This Prevents

Plan work remains useful without reasserting fixed steps as the only runtime
control plane.
