# Protocol

## Purpose

Define model output envelopes, attribute-less XML-like tool-call grammar, and
fault taxonomy as projections of persisted runtime decisions.

## Table of Contents

- [envelopes.md](envelopes.md): decision-selected block forms and active
  XML-like action envelope.
- [faults.md](faults.md): parse faults, contamination, and retry hints.

## Core Rule

The model never emits JSON for actions. The selected `RuntimeDecision` renders
one allowed envelope and one copyable XML-like skeleton. Internal exchange files
and flat data configuration may use JSON because they are not model context.

## Failure This Prevents

The model receives the grammar for the active decision instead of a broad tool
language with repair heuristics.
