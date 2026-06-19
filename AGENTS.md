# AGENTS.md

## Purpose

This file is the entry point for automated coding agents working on lkjagent.
It states what lkjagent is, the non-negotiable rules, the read order, task
routing, verification, and where the deeper contracts live.

## What lkjagent Is

A minimal, continuously running agent harness in Rust for a local LLM with a
configurable context window. One daemon, one agent loop, a persistent user message
queue, a tag-based action protocol, an append-only cache-friendly context,
SQLite persistence, a unified markdown skill system, and a container-first
YOLO posture. The full picture is [docs/architecture/overview.md](docs/architecture/overview.md).

## Non-Negotiable Rules

1. docs/ is the implementation contract. Update docs and implementation in the
   same change, including [docs/current-state.md](docs/current-state.md) when behavior moves.
2. Every file stays at or below 200 lines. Split by ownership before adding.
   See [docs/repository/line-limits.md](docs/repository/line-limits.md).
3. Docs use ASCII prose, kebab-case filenames, one H1 then a Purpose section.
   See [docs/repository/documentation-standards.md](docs/repository/documentation-standards.md).
4. Every docs directory holds one README.md that is a table of contents plus
   at least two children. Every doc is linked from its directory README.
5. No release shorthand and no compatibility framing anywhere. State the
   current contract directly. Nothing is preserved for compatibility.
6. Functional style: pure cores, effects at the edges, no panic paths
   (unwrap, expect, panic, todo, unimplemented) in product crates.
   See [docs/repository/functional-style.md](docs/repository/functional-style.md).
7. Honest state only: no fake success, no placeholder behavior, no mock
   implementations. The canonical rule is [docs/agent/honest-state.md](docs/agent/honest-state.md).
8. No MCP and no sub-agents in the product. Do not add them.
9. Commit small and often per [docs/repository/commit-protocol.md](docs/repository/commit-protocol.md).
   Tested and Not-tested trailers must match commands actually run.
10. Never claim a gate passed without running it.

## Read Order

1. [docs/current-state.md](docs/current-state.md)
2. [docs/agent/README.md](docs/agent/README.md)
3. [docs/execution/current-blockers.md](docs/execution/current-blockers.md)
4. The contracts linked from the chosen task or skill

## Task Routing

When the user names a task, do that task and load the matching skill from
[docs/agent/skills/README.md](docs/agent/skills/README.md). Otherwise take the
first open blocker in [docs/execution/current-blockers.md](docs/execution/current-blockers.md)
and follow its task file under [docs/execution/tasks/](docs/execution/tasks/README.md).

Skills here use the same format the harness itself runs on:
[docs/architecture/skills/format.md](docs/architecture/skills/format.md).

## Verification

The verification contract is [docs/operations/verification.md](docs/operations/verification.md).

- Docs-only change: run the doc checks (xtask check-docs and check-lines once
  built; until then, the shell checks listed in the verification contract).
- Rust change: cargo fmt --check, focused crate tests, then quiet verify.
- Any final claim: docker compose run --rm verify (image build, no source mounts).

Quiet gates print exactly one line on success: ok followed by the gate name.

## Handoff

Follow [docs/agent/handoff.md](docs/agent/handoff.md). Name what changed and
why, the docs updated, the commands run with their actual results, the
commands not run with reasons, and the next executable step.
