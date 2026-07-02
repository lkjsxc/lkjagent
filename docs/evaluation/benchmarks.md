# Benchmarks

## Purpose

Define the deterministic benchmark corpus.

## Corpus Record

Corpus entries live under `evaluation/corpus/<suite>/<entry>/`. Each entry has:

- `objective.txt`: owner request text;
- `template.txt`: expected classifier template;
- `checks.json`: checks from [../checks/catalog.md](../checks/catalog.md);
- `fixtures/exchanges.txt`: scripted parse fixtures as `mode|kind|raw` lines.

Judges are check evaluators, not separate task-family code. Entry-specific judge
logic is a corpus-definition error.

## Gate

`cargo run -p lkjagent-xtask -- bench check-corpus` validates the corpus. A
passing run prints `ok bench check-corpus`. The gate is part of `quiet verify`.

## Run Command

`cargo run -p lkjagent-xtask -- bench run --suite tiny --data DIR` writes a
bounded Markdown report. The command validates the corpus before writing the
report. Real endpoint scoring is rebuilt on top of this report surface.

## Coverage

The tiny suite includes manuscript planning, docs-tree planning, file work,
questions, journal work, truncated exchanges, and wrong-envelope exchanges.

## Fixture Rule

Fixtures are recorded from real runs or constructed directly from the written
contract. Each fixture states which parse behavior it guards without endpoint
URLs or secrets.
