# Layout

## Purpose

Define path ownership in the repository.

## Top Level

| Path | Owner |
| --- | --- |
| `crates/lkjagent-core/` | pure matter, state, prompt, parse, check, and ladder logic |
| `crates/lkjagent-store/` | SQLite schema and access |
| `crates/lkjagent-llm/` | endpoint client |
| `crates/lkjagent-effects/` | filesystem, shell, checks, and exchange logs |
| `crates/lkjagent-app/` | daemon loop and owner CLI |
| `crates/lkjagent-xtask/` | repository gates, bench, replay, and proof |
| `docs/` | implementation contract |
| `evaluation/corpus/` | deterministic benchmark corpus |
| `data/` | runtime store, logs, and workspace |
| `tmp/` | ignored scratch and proof capture |

## Ownership Rule

A source module links to its owning docs page in comments only when the link adds
clarity. Behavior changes update the owning docs page and
[../current-state.md](../current-state.md) in the same commit when the ledger
truth changes.
