# lkjagent Documentation

## Purpose

This tree is the implementation contract for lkjagent. Code follows docs.
Start with [current-state.md](current-state.md), then follow the README of the
area being changed. Each docs directory has one README that acts as a table of
contents plus at least two children. Large contracts are split into
directories with short focused files.

## Table of Contents

- [current-state.md](current-state.md): honest ledger of what exists versus what is design-only.
- [vision/](vision/README.md): north star, principles, and scope boundaries.
- [product/](product/README.md): observable behavior of the daemon, CLI, and queue.
- [architecture/](architecture/README.md): runtime, context, protocol, tools, memory, skills, llm, sandbox.
- [evaluation/](evaluation/README.md): mechanical benchmark tasks, judges, reports, and improvement loop.
- [decisions/](decisions/README.md): durable decision records with rejected directions.
- [repository/](repository/README.md): layout, line limits, doc standards, style, commits, workflow.
- [operations/](operations/README.md): verification gates, compose design, running the harness.
- [agent/](agent/README.md): manual for the coding agents that build lkjagent, plus their skills.
- [execution/](execution/README.md): operating rules, blocker queue, and executable tasks.

## All Files

- `current-state.md`
- `vision/README.md` `vision/north-star.md` `vision/principles.md` `vision/scope.md`
- `product/README.md` `product/daemon.md` `product/cli.md` `product/queue.md` `product/observability.md`
- `architecture/README.md` `architecture/overview.md`
- `architecture/runtime/README.md` `architecture/runtime/agent-loop.md` `architecture/runtime/daemon-process.md`
  `architecture/runtime/queue-intake.md` `architecture/runtime/self-maintenance.md`
- `architecture/context/README.md` `architecture/context/layout.md` `architecture/context/budgets.md`
  `architecture/context/compaction.md` `architecture/context/caching.md` `architecture/context/hygiene.md`
- `architecture/protocol/README.md` `architecture/protocol/action-format.md` `architecture/protocol/parsing.md`
  `architecture/protocol/system-prompt.md` `architecture/protocol/recovery.md`
- `architecture/tools/README.md` `architecture/tools/registry.md` `architecture/tools/fs.md`
  `architecture/tools/shell.md` `architecture/tools/queue-ops.md` `architecture/tools/memory-ops.md`
  `architecture/tools/skill-ops.md` `architecture/tools/control.md`
- `architecture/memory/README.md` `architecture/memory/store.md` `architecture/memory/retrieval.md`
  `architecture/memory/transcripts.md` `architecture/memory/distillation.md`
- `architecture/skills/README.md` `architecture/skills/format.md` `architecture/skills/loading.md`
  `architecture/skills/lifecycle.md` `architecture/skills/library.md`
- `architecture/llm/README.md` `architecture/llm/endpoint.md` `architecture/llm/model-target.md`
  `architecture/llm/sampling.md`
- `architecture/sandbox/README.md` `architecture/sandbox/container.md` `architecture/sandbox/workspace.md`
  `architecture/sandbox/safety.md`
- `evaluation/README.md` `evaluation/mechanical-benchmarks.md` `evaluation/task-contract.md`
  `evaluation/metrics-reports.md` `evaluation/running.md` `evaluation/improvement-loop.md`
  `evaluation/overfitting.md`
- `decisions/README.md` `decisions/rust-workspace.md` `decisions/openai-endpoint.md`
  `decisions/xml-action-protocol.md` `decisions/append-only-context.md` `decisions/single-loop.md`
  `decisions/sqlite-store.md` `decisions/no-mcp.md` `decisions/no-subagents.md`
  `decisions/container-first.md` `decisions/unified-skills.md`
- `repository/README.md` `repository/layout.md` `repository/line-limits.md`
  `repository/documentation-standards.md` `repository/functional-style.md`
  `repository/commit-protocol.md` `repository/workflow.md`
- `operations/README.md` `operations/verification.md` `operations/compose.md` `operations/running.md`
- `agent/README.md` `agent/work-loop.md` `agent/handoff.md` `agent/honest-state.md`
- `agent/skills/README.md` `agent/skills/doc-contract-edit.md` `agent/skills/rust-crate-slice.md`
  `agent/skills/protocol-change.md` `agent/skills/context-engine.md` `agent/skills/memory-store.md`
  `agent/skills/skill-system.md` `agent/skills/verification-gate.md` `agent/skills/agent-maintenance.md`
  `agent/skills/benchmark-driven-improvement.md`
- `execution/README.md` `execution/operating-rules.md` `execution/current-blockers.md`
- `execution/tasks/README.md` `execution/tasks/bootstrap-workspace.md` `execution/tasks/xtask-checks.md`
  `execution/tasks/llm-client.md` `execution/tasks/protocol-parser.md` `execution/tasks/context-engine.md`
  `execution/tasks/sqlite-store.md` `execution/tasks/tool-runtime.md` `execution/tasks/agent-loop.md`
  `execution/tasks/queue-cli.md` `execution/tasks/skill-runtime.md` `execution/tasks/self-maintenance.md`
  `execution/tasks/compose-final-gate.md`
