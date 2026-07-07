# Benchmarks

## Purpose

Define the deterministic benchmark corpus.

## Corpus Record

Corpus entries live under `evaluation/corpus/<suite>/<entry>/`. Each entry has:

- `objective.txt`: owner request text;
- `template.txt`: expected classifier template;
- `checks.json`: checks from [../checks/catalog.md](../checks/catalog.md);
- `fixtures/exchanges.txt`: scripted parse fixtures as `mode|kind|raw` lines.

Judges are check evaluators, not separate matter-family code. Entry-specific judge
logic is a corpus-definition error.

## Gate

`cargo run -p lkjagent-xtask -- bench check-corpus` validates the corpus. A
passing run prints `ok bench check-corpus`. The gate is part of `quiet verify`.

## Run Command

`cargo run -p lkjagent-xtask -- bench run --suite tiny --data DIR` drives each
suite entry through the app daemon with the configured endpoint and writes a
bounded Markdown report. `DIR/lkjagent.json` or endpoint environment variables
provide the endpoint configuration. Each entry writes an isolated artifact
subdirectory under `DIR/entries/`.

## Coverage

The tiny suite includes structured reports, docs-tree planning, file work,
questions, journal work, truncated exchanges, and wrong-envelope exchanges.

## Fixture Rule

Fixtures are recorded from real runs or constructed directly from the written
contract. Each fixture states which parse behavior it guards without endpoint
URLs or secrets.
