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

## Failure This Prevents

A reviewer can audit state, decisions, tools, context hygiene, and evidence
without reading secrets or unbounded model output.
