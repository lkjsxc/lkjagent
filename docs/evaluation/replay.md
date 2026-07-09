# Replay

## Purpose

Define deterministic replay of baseline and campaign failures.

## Inputs

A replay fixture contains scrubbed owner events, endpoint exchanges, clock
steps, fault injections, initial workspace bytes, configuration fingerprint,
and expected check measurements. It contacts no live endpoint.

## Execution

Replay uses a fresh store and the production reducer, selector, parser,
admission, effects boundary, workspace service, and checkers relevant to the
failure. Clock and external outcomes are injected at their existing pure or
effect interfaces.

## Assertions

The gate compares events, decisions, failure lineage, admissions, effects,
observations, checks, messages, workspace manifests, and terminal matter state.
It rejects copied seeded history, duplicate effects, repeated failure tuples,
stale checks, false response success, and synthetic quiescence work.

## Failure Rule

A live or PTY failure becomes a red, minimized, named replay fixture before the
source fix. Raw campaign evidence remains immutable; the fixture records its
source hashes and redaction procedure.
