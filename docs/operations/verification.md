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

## Repository Determinism

`Cargo.lock`, `Dockerfile`, `docker-compose.yml`, `.dockerignore`, workflow
files, Cargo manifests, copied source, docs, and evaluation inputs are tracked.
The repository gate parses Docker copy sources and rejects any untracked input.
It also rejects non-scalar or incomplete `data/lkjagent.json`, source and docs
over 200 lines, product source over 190 files, and banned panic, unsafe,
unfinished, mock, placeholder, retired-authority, or release-style source.

The canonical isolated run is:

```sh
tmp/lkjagent-evidence-first-rebuild-20260710/13-scripts/clean_checkout_gate.sh .
```

That anchored script requires a clean tree, exports `HEAD` with `git archive`,
uses no ignored local files or environment build overrides, builds the verify,
test, and lint images with `--no-cache`, and runs all three services. Pull
requests and pushes to main call the same script directly.

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

Any implementation claim names the focused test, quiet gate, and Docker gate
that ran. Unavailable Docker, endpoint, terminal, or public CI capability is a
blocker for dependent nodes. Record the exact command and raw error; never
record a skip as a pass.
