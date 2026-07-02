# Engine

## Purpose

This directory defines the plan ledger, step engine, turn cycle, retry ladder,
completion rules, and templates.

## Table of Contents

- [task-model.md](task-model.md): durable task state and lifecycle.
- [plan-and-steps.md](plan-and-steps.md): ordered steps, attempts, and plan
  mutation.
- [step-kinds.md](step-kinds.md): the bounded work kinds the model may answer.
- [turn-cycle.md](turn-cycle.md): one daemon cycle from intake to persistence.
- [retry-and-escalation.md](retry-and-escalation.md): finite retry ladder.
- [completion.md](completion.md): checks-gated step and task closure.
- [templates/](templates/): template contracts for task families.

## Failure This Prevents

A single table-of-contents owner prevents competing engine contracts from
emerging in separate directories.
