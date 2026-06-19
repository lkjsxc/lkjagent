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
recursive tree. Documentation and knowledge-base requests enter document
construction nodes that create nucleus anchors before endpoint execution.
File and markdown-count requests become completion guards and compose with
recursive or knowledge-base requirements instead of replacing them. Exact
wording requires the exact count. Approximate wording accepts the documented
tolerance.

When no task is open and the queue is empty, the daemon opens a bounded
graph maintenance case, records `daemon_state=working`, and continues toward
one concrete improvement or an honest empty-cycle agent.done. Queue arrival
preempts maintenance at the next turn boundary.

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
