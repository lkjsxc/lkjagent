# State Model

## Purpose

This directory defines owner input, objective envelopes, ranked neutral state
tracks, state intensity, and transition quality.

## Table of Contents

- [multi-state.md](multi-state.md): candidate state tracks and postures.
- [state-intensity.md](state-intensity.md): ranking score inputs.
- [transition-quality.md](transition-quality.md): transition scoring.
- [owner-input.md](owner-input.md): objective envelope from owner evidence.

## Local Map

- [multi-state.md](multi-state.md): owns track fields and display.
- [state-intensity.md](state-intensity.md): owns rank formula.
- [transition-quality.md](transition-quality.md): owns transition value checks.
- [owner-input.md](owner-input.md): owns raw-to-normalized task framing.

## Reading Paths

- Implementation path: owner-input, multi-state, state-intensity.
- Diagnosis path: transition-quality, state-intensity, multi-state.
- Verification path: owner-input tests, state ranking tests, status display tests.

## Cross-Links

- Related contract: [../state-graph/task-state.md](../state-graph/task-state.md).
- Owning crate or module: `crates/lkjagent-graph/src`.
