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

Fake time is a monotonic sequence supplied by the fixture. Typed faults are
identified by injection ID and boundary, consumed exactly once in declared
order, and may advance fake time. A monotonic regression, skipped faults, or a
second consumption fails replay before outcome comparison.

## Assertions

The gate compares events, decisions, failure lineage, admissions, effects,
observations, checks, messages, workspace manifests, and terminal matter state.
It rejects copied seeded history, duplicate effects, repeated failure tuples,
stale checks, false response success, and synthetic quiescence work.

Snapshot tests write a live SQLite source, take a quiesced Online Backup, then
mutate the source. The backup must retain only pre-boundary rows and pass its
own integrity check. File manifests are rebuilt from current bytes rather than
copied from expected rows.

## Failure Rule

A live or PTY failure becomes a red, minimized, named replay fixture before the
source fix. Raw campaign evidence remains immutable; the fixture records its
source hashes and redaction procedure.
