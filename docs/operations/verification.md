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
| check-docs | doc shape, topology, TOC completeness, ASCII, prose width, banned tokens, skill format, task format, README coverage |
| check-lines | the 200-line cap (120 for skills) on every tracked file |
| check-style | panic-path scan and dependency allowlist on product crates |
| quiet test | cargo fmt --check, clippy with warnings denied, all workspace tests |
| quiet verify | check-docs, check-lines, check-style, then quiet test |

## Final Gate

```sh
docker compose run --rm verify
```

Builds the image from a clean context and runs quiet verify inside it; no
source bind mounts, so the gate proves the repository as committed, not the
working tree. Service design in [compose.md](compose.md). Any claim that a
runtime behavior is implemented requires this gate in the same handoff.

## CI

CI runs the final gate and nothing else, so local and CI verdicts cannot
diverge. The workflow file lands with
[../execution/tasks/compose-final-gate.md](../execution/tasks/compose-final-gate.md).

## Status

implemented.
