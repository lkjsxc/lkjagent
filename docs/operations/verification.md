# Verification

## Purpose

Define gate commands and the quiet output contract.

## Quiet Contract

A passing gate prints exactly one line: `ok ` followed by the gate name. A
failing gate prints the failing step, exit status, and bounded output tail. A
gate that did not run did not pass.

## Gates

All local gates run as `cargo run -p lkjagent-xtask -- <gate>`.

| Gate | Checks |
| --- | --- |
| `check-docs` | docs shape, topology, links, ASCII, bans, docs count |
| `check-lines` | per-file line cap |
| `check-files` | product source and docs file-count budgets |
| `check-style` | panic-path scan and dependency allowlist |
| `bench check-corpus` | deterministic corpus shape, fixtures, and check validation |
| `smoke replay` | deterministic replay through store, effects, and checks |
| `quiet test` | fmt, clippy, and workspace tests |
| `quiet verify` | check-docs, check-lines, check-files, check-style, quiet test |

## Docker Gates

```sh
docker compose run --rm verify
docker compose run --rm test
docker compose run --rm lint
docker compose run --rm bench
docker compose run --rm replay
```

Workgraph nodes use the shell profile so each named gate runs from the same
locked Docker build context:

```sh
docker compose --profile shell run --rm shell \
  cargo run --locked -p lkjagent-xtask -- gate <node-id>
```

Each node gate owns substantive assertions for its completion predicate. An
unknown node, an empty test filter, a skipped command, or an editable pass label
fails. Pre-freeze raw output and machine-readable results live under
`tmp/lkjagent-progress/nodes/<node-id>/raw/` and remain distinct from final
acceptance evidence.

`docker compose run --rm verify` is the final deterministic gate. It builds the
image from explicit Dockerfile copies, not broad `COPY . .`, and runs
`quiet verify` without source bind mounts. The Docker context excludes runtime
data, logs, tmp evidence, target output, local model files, and secrets while
retaining `Cargo.lock`.

## Claims

Any claim that runtime behavior is implemented names the focused test, quiet
gate, and Docker gate that ran. Unavailable Docker or live endpoints are
recorded as honest skips with exact command and error, not as passes.
