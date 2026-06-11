# Layout

## Purpose

The path ownership map. Every tracked path belongs to exactly one row here;
adding a top-level path means adding a row in the same commit.

## Top Level

| Path | Owns | Contract |
| --- | --- | --- |
| README.md | project overview and read order | self-contained |
| AGENTS.md | entry point for coding agents | self-contained |
| docs/ | the implementation contract | [../README.md](../README.md) |
| crates/ | the Rust workspace | this file, below |
| docker-compose.yml | service wiring | [../operations/compose.md](../operations/compose.md) |
| Dockerfile | the harness image | [../architecture/sandbox/container.md](../architecture/sandbox/container.md) |
| .github/ | GitHub metadata, uses _README.md | [../../.github/_README.md](../../.github/_README.md) |
| LICENSE | Apache License 2.0 | unmodified |

crates/, docker-compose.yml, and Dockerfile are design-only until their
tasks in [../execution/tasks/](../execution/tasks/README.md) land.

## Crates

| Crate | Owns | Doc contract |
| --- | --- | --- |
| crates/lkjagent-protocol | action grammar parse and render | [../architecture/protocol/](../architecture/protocol/README.md) |
| crates/lkjagent-context | window layout, budgets, compaction | [../architecture/context/](../architecture/context/README.md) |
| crates/lkjagent-store | SQLite queue, events, memory, state | [../architecture/memory/](../architecture/memory/README.md) |
| crates/lkjagent-llm | endpoint HTTP client | [../architecture/llm/](../architecture/llm/README.md) |
| crates/lkjagent-skills | skill parse, index, load | [../architecture/skills/](../architecture/skills/README.md) |
| crates/lkjagent-tools | tool execution adapters | [../architecture/tools/](../architecture/tools/README.md) |
| crates/lkjagent-runtime | daemon, loop, intake, maintenance | [../architecture/runtime/](../architecture/runtime/README.md) |
| crates/lkjagent-cli | the lkjagent binary | [../product/cli.md](../product/cli.md) |
| crates/lkjagent-xtask | repository checks and quiet gates | [../operations/verification.md](../operations/verification.md) |

Dependency direction flows toward purity: cli and runtime depend on the
others; protocol, context, and skills depend on nothing in the workspace;
nothing depends on cli or xtask.

## README Coverage

Every crate root and every source subdirectory carries a README.md with a
Purpose section and a table of contents for its files, mirroring the docs
convention in [documentation-standards.md](documentation-standards.md).
The xtask check-docs gate enforces coverage once built.

## Runtime Paths (inside the container)

| Path | Owns | Contract |
| --- | --- | --- |
| /data | store, skill library, config | [../architecture/sandbox/workspace.md](../architecture/sandbox/workspace.md) |
| /workspace | the mounted project the agent works on | [../architecture/sandbox/workspace.md](../architecture/sandbox/workspace.md) |

Runtime paths never appear inside the repository; .gitignore excludes /data
and store files defensively.
