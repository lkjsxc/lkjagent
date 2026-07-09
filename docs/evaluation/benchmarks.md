# Benchmarks

## Purpose

Define anchored scenarios and deterministic checks independent of runtime prose.

## Scenario Shape

Each scenario stores ID, minimum duration and owner schedule, allowed terminal
states, required check IDs, seed fingerprint, required fault injections, and
negative predicates. Matters use stable scenario keys so checks compare every
declared goal with one fresh-store result.

The source directory contains `scenario.tsv`, `matters.tsv`,
`owner-schedule.tsv`, `seed-manifest.tsv`, and `checks.tsv`. The harness hashes
their exact bytes in that order. Results bind the recomputed bundle and the run
source commit; copied or modified scenario inputs fail.

Fixtures contain exact owner turns, workspace seed bytes, endpoint exchanges,
fault schedules, and expected measurements. They do not contain editable pass
labels or implementation-owned success summaries.

## Required Families

- daily life and recall with Japanese input and a local-day boundary;
- two similar projects with source separation and bounded code work;
- a 1,500-word artifact under an initial 768-token output cap;
- protocol, admission, effect, crash, context, and no-progress faults;
- canonical TUI identity, paging, resize, input, and restart behavior.

## Checks

Stable Rust checkers read raw rows and bytes. They prove obligations, paths,
fingerprints, current check lineage, strategy changes, provider exchanges,
workspace visibility, transcript identity, and negative predicates. Runtime
messages never decide a checker result.

## Replay Gate

Every reproduced failure lands as a minimized fixture before its fix. The
focused node gate and Docker Compose rerun that fixture through production
boundaries or a pure checker with the same schema.

The gate also executes committed invalid evidence fixtures. It must reject idle
duration presented as work, blocked state presented as completion, skipped
commands, zero-test filters, and generated placeholders while accepting a
computed valid fixture.
