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
- `selector-candidates.md`: bounded candidate operations, reasons, and blockers;
- `admissions.md`: admitted and rejected tool calls tied to decisions;
- `observations.md`: bounded effect output and artifact refs;
- `context.md`: conflicts and contaminated items suppressed from prompts;
- `checks.md`: check rows with step id, name, params, and measured value;
- `exchanges.md`: provider exchange refs and nullable token usage;
- `records.md`: workspace record fingerprints and archived state;
- `workspace-tree.md`: file paths under the workspace;
- `warnings.md`: orphaned exchange, observation, or artifact warnings.

It does not copy SQLite files, endpoint secrets, full prompt bodies, full model
responses, or full artifact prose. Artifact fingerprints are summarized in
`artifacts.md`; check parameters are bounded in `checks.md`. Large data stays in
the workspace or logs and is referenced by path.

## Capture Rule

Every proof activity writes a stamped `tmp/` directory with a `summary.md` that
states commands run, results, anomalies, commands not run, and next action.

## Live Attempt Ledger

Earlier live attempts are archived as historical stress evidence. They remain
useful for checking endpoint patience, prompt-frame capture, artifact rows, and
honest time-box reporting. They are not current proof targets. Current live
proof uses the daily-use profiles in [../evaluation/live-proof.md](../evaluation/live-proof.md).

## Failure This Prevents

A reviewer can audit state, decisions, tools, context hygiene, and evidence
without reading secrets or unbounded model output.
