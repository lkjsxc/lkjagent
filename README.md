# lkjagent

## Purpose

Provide one durable local agent that can work safely and truthfully inside one
owner-visible workspace.

## Product Direction

The first complete behavior is deliberately small: accept an owner turn, inspect
one UTF-8 file, apply one revision-bound edit, verify actual bytes, emit one
truthful message, and continue with the next turn.

SQLite state cells and persisted `RuntimeDecision` rows own control flow. The
model sees compact state-specific XML-like tools. Effects are admitted, journaled,
observed, and checked. The model never closes a matter.

Daily records, readable memory, multiple projects, session receipts, reports, and
a corrected TUI build on that same loop only after ordinary editing works.

## Owner Commands

```sh
cargo run --locked -p lkjagent-app -- send --new "OWNER TEXT"
cargo run --locked -p lkjagent-app -- run
cargo run --locked -p lkjagent-app -- status
cargo run --locked -p lkjagent-app -- tui
docker compose run --rm verify
```

These commands exist in the current source. Their present limitations and the
next cutover work are recorded in `docs/current-state.md`.

## Start Here

Read `AGENTS.md`, then follow its canonical documentation order. The active work
and acceptance inputs are:

- `evaluation/workgraph.tsv`
- `evaluation/acceptance.tsv`
- `evaluation/experiment-plan.tsv`

## Repository Rules

- Documentation is the contract.
- Every authored file is at most 200 lines.
- The functional core is pure and edges own effects.
- File changes stay below the configured workspace root.
- Completion requires fresh harness checks.
- Claims name only commands and evidence that actually ran.
