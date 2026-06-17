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
skill validator plus seed library, the tool dispatcher/adapters, and the
runtime step/daemon core. CLI behavior described under
[product/](product/README.md) is design-only. The implementation queue is
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
| Docker compose services | design-only | [operations/compose.md](operations/compose.md) |
| Container image skeleton | implemented | [architecture/sandbox/container.md](architecture/sandbox/container.md) |
| Daemon and agent loop | implemented | [architecture/runtime/](architecture/runtime/README.md) |
| Context engine and compaction | implemented | [architecture/context/](architecture/context/README.md) |
| Action protocol and parser | implemented | [architecture/protocol/](architecture/protocol/README.md) |
| Toolset | implemented | [architecture/tools/](architecture/tools/README.md) |
| SQLite store, transcript, and memory access | implemented | [architecture/memory/](architecture/memory/README.md) |
| Skill validator, index, loader, and seed library | implemented | [architecture/skills/](architecture/skills/README.md) |
| LLM endpoint client | implemented | [architecture/llm/](architecture/llm/README.md) |
| Container and sandbox | design-only | [architecture/sandbox/](architecture/sandbox/README.md) |
| User message queue and CLI | design-only | [product/](product/README.md) |
| Self-maintenance loop | design-only | [architecture/runtime/self-maintenance.md](architecture/runtime/self-maintenance.md) |

## Out of Scope

Messaging channels, web UI, MCP, sub-agents, plan mode, heartbeat schedules,
and cron schedules. The boundaries are stated in [vision/scope.md](vision/scope.md).

## Next Step

Take the first open blocker in
[execution/current-blockers.md](execution/current-blockers.md): implement the
queue CLI per [execution/tasks/queue-cli.md](execution/tasks/queue-cli.md).

## Honesty Rules

- A behavior is implemented only when code, focused tests, and a passing gate exist.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
