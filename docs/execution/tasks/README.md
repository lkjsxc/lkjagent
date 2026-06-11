# Tasks

## Purpose

One executable task file per implementation slice, written so a session can
start from the file alone. The queue order lives in
[../current-blockers.md](../current-blockers.md); this README owns the
shared template.

## Template

Every task file uses exactly these headings:

```
# <Task Title>

## Purpose
## Status
## Depends On
## Files To Read
## Files To Touch
## Focused Gate
## Acceptance
## Must Not
```

- Status is one line: open, in progress with the blocking question, or done
  with the closing commit.
- Files To Read lists contracts in reading order; Files To Touch lists
  expected paths, marking new ones.
- Focused Gate is the exact command list run during the work.
- Acceptance is the evidence that closes the task: commands plus the output
  facts that must hold.
- Must Not carries the task-specific traps.

## Table of Contents

- [bootstrap-workspace.md](bootstrap-workspace.md): create the cargo workspace and image skeleton.
- [xtask-checks.md](xtask-checks.md): build the gates that enforce the repository rules.
- [protocol-parser.md](protocol-parser.md): the action grammar parser and renderer.
- [context-engine.md](context-engine.md): budgets, admission, accounting, compaction decisions.
- [sqlite-store.md](sqlite-store.md): queue, events, memory, state, retrieval.
- [llm-client.md](llm-client.md): the chat-completions client and backoff.
- [skill-runtime.md](skill-runtime.md): validator, index, loader, seed skills.
- [tool-runtime.md](tool-runtime.md): dispatch and the ten tool adapters.
- [agent-loop.md](agent-loop.md): the turn loop composing every crate.
- [queue-cli.md](queue-cli.md): the lkjagent binary and its commands.
- [self-maintenance.md](self-maintenance.md): idle directives and their bounds.
- [compose-final-gate.md](compose-final-gate.md): Dockerfile, compose services, CI.
