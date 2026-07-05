# Protocol

## Purpose

Define model output envelopes, tool-call grammar, and fault taxonomy as
projections of persisted runtime decisions.

## Table of Contents

- [envelopes.md](envelopes.md): decision-selected block forms and action
  envelopes.
- [plan-line-grammar.md](plan-line-grammar.md): exact plan helper grammar and
  examples.
- [faults.md](faults.md): parse faults, contamination, and retry hints.

## Failure This Prevents

The model receives the grammar for the active decision instead of a broad tool
language with repair heuristics.
