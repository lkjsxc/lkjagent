# Current State

## Purpose

This file is the honest ledger of lkjagent. It states what exists, what is
design-only, and what comes next. Every change that moves behavior updates
this file in the same commit. Statuses used across docs: implemented,
design-only, not implemented, out of scope, open question.

## Summary

The repository contains the complete documentation contract, a compiling
Cargo workspace, local verification gates, the action parser, the pure
context engine, the SQLite store boundary, the LLM endpoint client, the
skill validator plus source-owned seed library, guarded recursive structure stewardship,
the tool dispatcher/adapters, the runtime step/daemon core, and the CLI
for send, status, log, console,
memory, skills, startup checks, repository-root .env loading for local
deployment values, JSON runtime config, a /data/workspace working tree, and a
resident daemon that delivers queued owner work to the endpoint and tool
dispatcher until agent.done. The console is transcript-first with a bottom
status/control deck and display-width wrapping for mixed English and Japanese
terminal text. Recursive structure tasks auto-load their seed skill before the
first endpoint turn. Runtime skills are indexed from
source or image paths, not copied into data. Recursive docs tasks create a
README-indexed scaffold before endpoint work; encyclopedia, wiki, and
knowledge-base creation tasks create a small knowledge nucleus with
current-state, concept-map, expansion-queue, and rebalance-plan anchors.
They cannot close without knowledge-nucleus evidence plus contract-shaped
markdown pages. Generic recursive structure tasks cannot close without a
verified README-indexed tree. Recursive-knowledge tasks also reject nested
docs roots and writes outside the seeded top-level docs map. shell.run
reports non-zero and signal exits as error observations with captured
output. Recoverable parse, repeat-action, and tool errors add recovery
notices to the transcript and keep the task moving instead of pausing it;
consecutive parse/repeat notices steer large or tag-like writes toward
shell.run scripts instead of repeated fs.write actions.
Empty queues open bounded self-maintenance cycles in rotation; owner queue
arrival preempts maintenance at the next turn boundary. Compose wiring is
implemented. The implementation queue is
[execution/current-blockers.md](execution/current-blockers.md).

## Area Status

| Area | Status | Contract |
| --- | --- | --- |
| Documentation tree and policies | implemented | [repository/](repository/README.md) |
| Vision and scope | implemented | [vision/](vision/README.md) |
| Decision records | implemented | [decisions/](decisions/README.md) |
| Agent manual and skills | implemented | [agent/](agent/README.md) |
| Execution queue and tasks | implemented | [execution/](execution/README.md) |
| Cargo workspace and crates | implemented | [repository/layout.md](repository/layout.md) |
| Verification xtask and quiet gates | implemented | [operations/verification.md](operations/verification.md) |
| Docker compose services | implemented | [operations/compose.md](operations/compose.md) |
| Container image skeleton | implemented | [architecture/sandbox/container.md](architecture/sandbox/container.md) |
| Daemon and agent loop | implemented | [architecture/runtime/](architecture/runtime/README.md) |
| Context engine and compaction | implemented | [architecture/context/](architecture/context/README.md) |
| Action protocol and parser | implemented | [architecture/protocol/](architecture/protocol/README.md) |
| Toolset | implemented | [architecture/tools/](architecture/tools/README.md) |
| SQLite store, transcript, and memory access | implemented | [architecture/memory/](architecture/memory/README.md) |
| Skill validator, index, loader, and seed library | implemented | [architecture/skills/](architecture/skills/README.md) |
| LLM endpoint client | implemented | [architecture/llm/](architecture/llm/README.md) |
| Container and sandbox | implemented | [architecture/sandbox/](architecture/sandbox/README.md) |
| User message queue and CLI | implemented | [product/](product/README.md) |
| Automatic idle self-maintenance | implemented | [architecture/runtime/self-maintenance.md](architecture/runtime/self-maintenance.md) |

## Out of Scope

Messaging channels, web UI, MCP, sub-agents, plan mode, heartbeat schedules,
and cron schedules. The boundaries are stated in [vision/scope.md](vision/scope.md).

## Next Step

No open blocker remains in
[execution/current-blockers.md](execution/current-blockers.md). Future changes
use the compose final gate per [operations/verification.md](operations/verification.md).

## Honesty Rules

- A behavior is implemented only when code, focused tests, and a passing gate exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
