# Proof Bundles

## Purpose

Define bounded evidence bundles for smoke and live runs.

## Command

```sh
cargo run -p lkjagent-xtask -- proof collect --data data --out tmp/proof-current
```

## Contents

A proof bundle records these Markdown files:

- `summary.md`: counts for tasks, steps, and check results;
- `status.md`: task state, template, and budget rows;
- `attempts.md`: attempt count;
- `workspace-tree.md`: file paths under the workspace;
- `word-counts.md`: Markdown file word counts by path;
- `warnings.md`: bounded warnings.

It does not copy SQLite files, endpoint secrets, full prompt bodies, full model
responses, or full artifact prose. Large data stays in the workspace or logs and
is referenced by path.

## Capture Rule

Every proof activity writes a stamped `tmp/` directory with a `summary.md` that
states commands run, results, anomalies, and next action.
