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
- [architecture/](architecture/README.md): runtime, graph, context, protocol, tools, memory, llm, sandbox.
- [evaluation/](evaluation/README.md): mechanical benchmark tasks, judges, reports, and improvement loop.
- [decisions/](decisions/README.md): durable decision records with rejected directions.
- [repository/](repository/README.md): layout, line limits, doc standards, style, commits, workflow.
- [operations/](operations/README.md): verification gates, compose design, running the harness.
- [agent/](agent/README.md): manual for the coding agents that build lkjagent.
- [execution/](execution/README.md): operating rules, blocker queue, and executable tasks.

## All Files

- `current-state.md`
- `vision/README.md` `vision/north-star.md` `vision/principles.md` `vision/scope.md`
- `product/README.md` `product/daemon.md` `product/cli.md` `product/queue.md` `product/observability.md`
- `architecture/README.md` `architecture/overview.md`
- `architecture/document-structure/README.md` `architecture/document-structure/tree-contract.md`
  `architecture/document-structure/network-contract.md` `architecture/document-structure/naming.md`
  `architecture/document-structure/scaffold-profiles.md` `architecture/document-structure/audit.md`
  `architecture/document-structure/completion-gates.md`
- `architecture/action-reliability/README.md` `architecture/action-reliability/parameter-contract.md`
  `architecture/action-reliability/recovery.md` `architecture/action-reliability/normalization.md`
  `architecture/action-reliability/error-messages.md`
- `architecture/state-model/README.md` `architecture/state-model/multi-state.md`
  `architecture/state-model/state-intensity.md` `architecture/state-model/transition-quality.md`
  `architecture/state-model/owner-input.md`
- `architecture/observability/README.md` `architecture/observability/token-ledger.md`
  `architecture/observability/console-deck.md` `architecture/observability/gpt-log.md`
  `architecture/observability/status-format.md`
- `architecture/runtime/README.md` `architecture/runtime/agent-loop.md` `architecture/runtime/daemon-process.md`
  `architecture/runtime/queue-intake.md` `architecture/runtime/self-maintenance.md`
- `architecture/state-graph/README.md` `architecture/state-graph/model.md`
  `architecture/state-graph/transitions.md` `architecture/state-graph/context-packages.md`
  `architecture/state-graph/task-state.md` `architecture/state-graph/compaction.md`
  `architecture/state-graph/completion.md` `architecture/state-graph/maintenance.md`
  `architecture/state-graph/policy.md` `architecture/state-graph/tool-affordances.md`
  `architecture/state-graph/recovery-ladders.md` `architecture/state-graph/graph-guidance.md`
  `architecture/state-graph/document-construction.md`
- `architecture/context/README.md` `architecture/context/layout.md` `architecture/context/budgets.md`
  `architecture/context/compaction.md` `architecture/context/caching.md` `architecture/context/hygiene.md`
- `architecture/protocol/README.md` `architecture/protocol/action-format.md` `architecture/protocol/parsing.md`
  `architecture/protocol/system-prompt.md` `architecture/protocol/recovery.md`
- `architecture/tools/README.md` `architecture/tools/registry.md` `architecture/tools/fs.md`
  `architecture/tools/shell.md` `architecture/tools/queue-ops.md` `architecture/tools/memory-ops.md`
  `architecture/tools/graph-ops.md` `architecture/tools/workspace.md`
  `architecture/tools/verification-tools.md` `architecture/tools/doc-tools.md`
  `architecture/tools/control.md`
- `architecture/memory/README.md` `architecture/memory/store.md` `architecture/memory/retrieval.md`
  `architecture/memory/transcripts.md` `architecture/memory/distillation.md`
- `architecture/llm/README.md` `architecture/llm/endpoint.md` `architecture/llm/model-target.md`
  `architecture/llm/sampling.md`
- `architecture/sandbox/README.md` `architecture/sandbox/container.md` `architecture/sandbox/workspace.md`
  `architecture/sandbox/safety.md`
- `evaluation/README.md` `evaluation/mechanical-benchmarks.md` `evaluation/task-contract.md`
  `evaluation/metrics-reports.md` `evaluation/running.md` `evaluation/improvement-loop.md`
  `evaluation/overfitting.md`
- `decisions/README.md` `decisions/rust-workspace.md` `decisions/openai-endpoint.md`
  `decisions/xml-action-protocol.md` `decisions/append-only-context.md` `decisions/single-loop.md`
  `decisions/sqlite-store.md` `decisions/state-graph-runtime.md` `decisions/no-mcp.md`
  `decisions/no-subagents.md` `decisions/container-first.md`
- `repository/README.md` `repository/layout.md` `repository/line-limits.md`
  `repository/documentation-standards.md` `repository/functional-style.md`
  `repository/commit-protocol.md` `repository/workflow.md`
- `operations/README.md` `operations/verification.md` `operations/compose.md` `operations/running.md`
- `agent/README.md` `agent/work-loop.md` `agent/handoff.md` `agent/honest-state.md`
- `execution/README.md` `execution/operating-rules.md` `execution/current-blockers.md`
- `execution/current-work/README.md` `execution/current-work/owner-reported-failures.md`
  `execution/current-work/action-fault-recovery.md` `execution/current-work/document-structure-redesign.md`
  `execution/current-work/context-accounting.md` `execution/current-work/multi-state-runtime.md`
  `execution/current-work/gpt-log.md` `execution/current-work/verification-plan.md`
- `execution/tasks/README.md` `execution/tasks/bootstrap-workspace.md` `execution/tasks/xtask-checks.md`
  `execution/tasks/llm-client.md` `execution/tasks/protocol-parser.md` `execution/tasks/context-engine.md`
  `execution/tasks/sqlite-store.md` `execution/tasks/state-graph-runtime.md` `execution/tasks/tool-runtime.md`
  `execution/tasks/agent-loop.md` `execution/tasks/queue-cli.md` `execution/tasks/self-maintenance.md`
  `execution/tasks/compose-final-gate.md`
