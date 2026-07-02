# Proof Bundles

## Purpose

Define bounded evidence bundles for smoke and live runs.

## Command

```sh
cargo run -p lkjagent-xtask -- proof collect --data data --out tmp/proof-current
```

## Contents

A proof bundle records:

- task, step, attempt, event, check, and token summaries from SQLite;
- workspace tree entries and word counts for artifact roots;
- exchange-log indexes with paths, outcomes, and diagnoses;
- warnings for orphaned exchange directories and stale waiting tasks;
- command output used by the runbook summary.

It does not copy SQLite files, endpoint secrets, full prompt bodies, full model
responses, or full artifact prose. Large data stays in the workspace or logs and
is referenced by path.

## Capture Rule

Every proof activity writes a stamped `tmp/` directory with a `summary.md` that
states commands run, results, anomalies, and next action.
