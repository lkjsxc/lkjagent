# Verification

## Purpose

The gate commands that decide whether work is done, and the output contract
that keeps them readable to agents. A gate that did not run did not pass;
claiming otherwise violates [../agent/honest-state.md](../agent/honest-state.md).

## The Quiet Contract

Gates run quiet: a passing gate prints exactly one line, `ok ` followed by
the gate name, and exits 0. A failing gate prints the failing step, the
exit status, and a bounded tail of captured output, then exits nonzero.
Quiet output is a context-hygiene rule as much as a CI rule: a passing gate
costs an agent a handful of tokens.

## Gates

Built by [../execution/tasks/xtask-checks.md](../execution/tasks/xtask-checks.md);
all run as `cargo run -p lkjagent-xtask -- <gate>`:

| Gate | Checks |
| --- | --- |
| check-docs | doc shape, topology, TOC completeness, internal links, ASCII, width, banned tokens, task format, README coverage |
| check-lines | the 200-line cap on every tracked file |
| check-style | panic-path scan and dependency allowlist on product crates |
| benchmark check-corpus | deterministic benchmark task, fixture, and judge validity |
| quiet test | cargo fmt --check, clippy with warnings denied, all workspace tests |
| quiet verify | check-docs, check-lines, check-style, benchmark check-corpus, then quiet test |

## Compose Gates

```sh
docker compose run --rm verify
docker compose run --rm test
docker compose run --rm lint
docker compose run --rm bench
docker compose run --rm replay
```

`verify` is the final gate. It builds the image from a clean context and runs
quiet verify inside it; no source bind mounts, so the gate proves the
repository as committed, not the working tree. `test`, `lint`, `bench`, and
`replay` expose narrower Docker Compose gates for focused diagnosis. Service
design is in [compose.md](compose.md). Any claim that a runtime behavior is
implemented requires the final gate in the same handoff.

Real benchmark scoring is not part of CI because it needs an endpoint. The
operator command is documented in [../evaluation/running.md](../evaluation/running.md).

## CI

CI runs the final gate and nothing else, so local and CI verdicts cannot
diverge. The workflow file lands with
[../execution/tasks/compose-final-gate.md](../execution/tasks/compose-final-gate.md).

## Status

implemented.
