# Task Contract

## Purpose

Define the parts of a benchmark task and the validity rules enforced by the
corpus check.

## Parts

Each task has a stable task id, suite, family, difficulty, tags, prompt,
optional follow-up prompt, optional public starter files, hidden judge kind,
deterministic seed, points, timeout, and known fixtures.

Public prompt text is copied to the owner queue. Public starter files are
copied into the task workspace before the daemon starts. Hidden judge logic,
oracle answers, generated cases, and fixture definitions stay in
`crates/lkjagent-benchmark` and are never copied into the agent workspace.

Generated cases use stable seed fields in the task definition. A seed is part
of the task identity: changing it changes the judged distribution and requires
docs, tests, and report interpretation to move together.

Each task carries at least one known-good fixture and at least two known-bad
fixtures. One bad fixture covers a plausible edge failure. Another covers
public-test-only or hard-coded-style behavior. `benchmark check-corpus`
materializes these fixtures, runs the hidden judge, and fails unless every
good workspace passes and every bad workspace fails.

## Status

implemented.
