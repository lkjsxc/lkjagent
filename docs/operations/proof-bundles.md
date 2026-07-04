# Proof Bundles

## Purpose

Define bounded evidence bundles for smoke and live runs.

## Command

```sh
cargo run -p lkjagent-xtask -- proof collect --data data --out tmp/proof-current
```

## Contents

A proof bundle records these Markdown files:

- `summary.md`: commands, results, anomalies, and next action;
- `cases.md`: case rows and terminal state;
- `state-vector.md`: active and terminal state cells with evidence refs;
- `decisions.md`: decision ids, operation keys, and fingerprints;
- `prompt-frames.md`: prompt-frame refs and context fingerprints;
- `tool-views.md`: rendered tool names and hidden-tool diagnostics;
- `admissions.md`: admitted and rejected actions tied to decisions;
- `observations.md`: bounded effect output and artifact refs;
- `context.md`: conflicts and contaminated items suppressed from prompts;
- `checks.md`: check results and artifact fingerprints;
- `exchanges.md`: provider exchange refs and nullable token usage;
- `workspace-tree.md`: file paths under the workspace;
- `warnings.md`: orphaned exchange, observation, or artifact warnings.

It does not copy SQLite files, endpoint secrets, full prompt bodies, full model
responses, or full artifact prose. Large data stays in the workspace or logs and
is referenced by path.

## Capture Rule

Every proof activity writes a stamped `tmp/` directory with a `summary.md` that
states commands run, results, anomalies, commands not run, and next action.

## Live Attempt Ledger

### 2026-07-04

- commit: `b0f5b18`
- run id: `live-proof-20260704T224334Z-b0f5b18`
- data dir: `tmp/live-proof-20260704T224334Z-b0f5b18/data`
- proof bundle: `tmp/live-proof-20260704T224334Z-b0f5b18/proof-bundle`
- objective: 10000 word Aurora Ledger manuscript under `stories/aurora-ledger`
  with 10 chapter files, settings, no placeholders, and measured checks.
- endpoint condition: endpoint URL, model, and API key were present.
- time box: 2026-07-04T22:43:34Z to 2026-07-04T23:13:37Z, 1801 seconds.
- terminal state: not reached; task remained open at budget 11/50.
- latest decision: `case-1-decision-0019`, `model.call/1015`, pending,
  context fingerprint `fnv1a64:62886272c2ebc683`.
- rows: tasks 1, steps 16, decisions 19, prompt frames 18, exchanges 17,
  observations 11, artifacts 23, checks 32, context items 7.
- files: settings plus chapter 01 through chapter 10 existed; chapter word
  counts were 327, 317, 326, 318, 327, 340, 355, 330, 336, and 343.
- result: honest bounded live attempt, not a passing live proof; the 10000 word
  closure target was not met before the time box ended.

## Failure This Prevents

A reviewer can audit state, decisions, tools, context hygiene, and evidence
without reading secrets or unbounded model output.
