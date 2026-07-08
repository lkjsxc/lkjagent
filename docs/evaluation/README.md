# Evaluation

## Purpose

Define deterministic benchmarks, recorded replay, scripted daily-use campaigns,
and the live proof contract.

## Acceptance Posture

Acceptance evidence is committed as raw or bounded proof under `tmp/agent-runs/`
or `tmp/live-runs/` when a campaign runs. If an endpoint or terminal is
unavailable, the run writes an honest skip file with the exact command and reason
instead of a pass. Deterministic gates and Docker Compose verification are rerun
after source changes before final claims.

## Table of Contents

- [benchmarks.md](benchmarks.md): corpus records and judges as checks.
- [replay.md](replay.md): recorded-exchange smoke replay.
- [live-proof.md](live-proof.md): daily-use scripted and live proof criteria and capture.
- [prose-trial.md](prose-trial.md): archived prose-stress evidence notes.
- [protocol-experiments.md](protocol-experiments.md): prompt and protocol experiment records.
