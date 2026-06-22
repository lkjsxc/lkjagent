# Maintenance

## Purpose

This directory owns autonomous maintenance policy. Maintenance is idle-only,
preemptable by owner work, and required to produce a real structural effect, a
new durable finding, or a truthful suppressed no-op.

## Table of Contents

- [structural-health.md](structural-health.md): metrics for documentation and runtime health.
- [commonality-discovery.md](commonality-discovery.md): extraction of repeated concepts.
- [bias-detection.md](bias-detection.md): skew checks across docs, prompts, and artifacts.
- [no-op-suppression.md](no-op-suppression.md): repeated maintenance loop suppression.

## Local Map

- State guards live in [../state/track-guards.md](../state/track-guards.md).
- Maintenance prompt mode lives in
  [../prompting/state-selected-prompts.md](../prompting/state-selected-prompts.md).
- Current blockers live in [../execution/current-blockers.md](../execution/current-blockers.md).

## Status

design-only
