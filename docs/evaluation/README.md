# Evaluation

## Purpose

Define deterministic failure replay, scenario checks, experiments, live
campaigns, and independent acceptance.

## Table of Contents

- [benchmarks.md](benchmarks.md): anchored scenarios, fixtures, and independent
  checkers.
- [replay.md](replay.md): failure-derived deterministic replay.
- [harness.md](harness.md): fake time, fault schedules, raw capture, manifests,
  PTY recording, and false-positive rejection.
- [live-proof.md](live-proof.md): frozen-source campaigns, raw evidence, and
  terminal conditions.
- [protocol-experiments.md](protocol-experiments.md): controlled factor matrix,
  repeats, metrics, and adoption rules.

## Evidence Rule

Raw SQLite, event traces, provider manifests, workspace bytes, PTY recordings,
and Git bindings are authority. Summaries and receipts are derived. A missing
endpoint, terminal, public CI result, or raw input blocks its dependent gate; a
skip file is not a pass.
