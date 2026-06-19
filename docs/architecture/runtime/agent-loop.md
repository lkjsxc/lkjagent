# Agent Loop

## Purpose

Specify the single continuous loop: how a graph case opens, how a turn runs,
how completion closes, and which budgets bound it. Decision:
[../../decisions/single-loop.md](../../decisions/single-loop.md).

## The Turn

A turn is the atomic unit of agent activity:

1. Boundary work: deliver due queue messages, create or resume graph cases
   ([queue-intake.md](queue-intake.md)), check the compaction trigger
   ([../context/compaction.md](../context/compaction.md)).
2. Render graph state and call the endpoint with the current message list and the stop sequence
   `</act>`; sampling per [../llm/sampling.md](../llm/sampling.md).
3. Parse the completion into one action per
   [../protocol/parsing.md](../protocol/parsing.md). On failure, follow
   [../protocol/recovery.md](../protocol/recovery.md).
4. Execute the action through the toolset
   ([../tools/registry.md](../tools/registry.md)).
5. Append the action and its bounded observation to the transcript, graph
   evidence, and the context log. Token caps per
   [../context/budgets.md](../context/budgets.md).

Steps are sequential; nothing else touches the context while a turn runs.
Recoverable parser, repeat-action, and tool faults append recovery notices
to the same transcript and leave the task open for the next endpoint turn.

## The Task

A task case is opened when an owner message is delivered with no active case,
and closed by an agent.done action admitted by the graph completion gate.

| Case phase | Driven by |
| --- | --- |
| planning | owner message delivered; graph classifier and planner run |
| execution | plan and context package selection complete |
| verification | checks or observed evidence are required |
| recovery | parse, tool, endpoint, budget, pressure, or completion gate failure |
| waiting | agent.ask emitted; loop waits until another send arrives |
| closed | agent.done admitted; summary, evidence, and memory links recorded |

A task carries a turn budget loaded from `task.turn-budget`, defaulting to
64 turns. The final budgeted turn is still admitted. If the daemon reaches
another endpoint turn after the budget is exhausted, it records a budget
notice and sets the task to waiting with a concrete owner question instead
of silently burning turns. The next owner send resumes the task with the
configured fresh turn budget.

Some owner messages activate task-family completion requirements. Recursive
structure tasks cannot close until graph evidence proves a README-indexed
recursive tree. Documentation, counted structured content, and knowledge-base
requests enter document construction nodes that create structure anchors
before endpoint execution. File and markdown-count requests become completion
guards and compose with recursive or knowledge-base requirements instead of
replacing them. Exact or approximate wording is scored near the chosen count
target, so direct exact wording is strict while exact wording attached to a
smaller subcount does not make an approximate total strict. Approximate
wording accepts the documented tolerance. Active count guards are rendered in
the graph-state prefix with an instruction to use one compact `shell.run`
command with direct `/bin/sh` loops and `printf` templates for bulk creation
and count verification, keep the act payload under about 1200 characters,
and avoid hardcoded `/workspace` paths, brace expansion, cat heredocs, bash
scripts, literal bodies, or one `fs.write` per file.
For counted documentation tasks without recursive or benchmark scaffolds, the
daemon writes a generic `structured-output/` tree before the first endpoint
turn. The scaffold profiles the owner's objective by detected language and
broad deliverable kind, gives each main file a kind-aware segment brief, adds
a root machine-readable audit manifest, a root acceptance-audit section, and
a per-part role ledger to the main index,
verifies the requested count, root index, root file budget, count-linked
audit manifest, acceptance audit, optional directory indexes, docs coverage
map, all design memo sections, all main-file sections, required content
blocks, and part
ledger, records graph evidence, saves the same verification evidence in the
task summary, and closes the task without asking the endpoint to repeat the
same bulk generation. The recorded evidence includes the `structured-output`
path, target file count, index file count, design memo count, main file count,
file-budget status, audit-manifest status, acceptance-audit status,
coverage-map status,
`index_scope=all`, `section_scope=all`, content-block status, required
design-section status, required main-section status, first and last main
status, part ledger status, and `verification=ok`.

When no task is open and the queue is empty, the daemon opens a bounded
graph maintenance case only after a directive is due, records
`daemon_state=working`, and continues toward one concrete improvement or an
honest empty-cycle agent.done. Task-summary saves defer all directives for
the maintenance cooldown so a completed owner task returns to visible idle
before maintenance resumes. Queue arrival preempts maintenance at the next
turn boundary.

## Pure Core

The loop body is a pure transition function in lkjagent-runtime composed
from lkjagent-context and lkjagent-protocol:

```
step : (State, GraphState, Completion) -> (State, GraphState, Effects)
```

Effects (tool execution, store writes, endpoint calls) are interpreted by
thin adapters. Tests drive `step` with recorded completions and assert
states and effects without any IO, per
[../../repository/functional-style.md](../../repository/functional-style.md).

## Stop Reasons

Every turn records why it ended: acted, done, ask, invalid_action,
unknown_tool, bad_params, repeat_action, endpoint_error, tool_error,
budget_notice, compaction, maintenance. The taxonomy is owned by
[../protocol/recovery.md](../protocol/recovery.md).

## Status

implemented.
