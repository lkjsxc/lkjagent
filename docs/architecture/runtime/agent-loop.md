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
   `</action>`; sampling per [../llm/sampling.md](../llm/sampling.md).
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

A task carries a continuation checkpoint interval loaded from
`task.turn-budget`, defaulting to 64 endpoint turns. The final budgeted turn is
still admitted. If the daemon reaches another endpoint turn at the boundary,
it records a `TurnBudgetCheckpoint`, refreshes runtime authority, opens the
next epoch, and continues autonomously when a legal internal route remains.
The budget boundary is not a permission prompt and does not ask the owner to
say continue.
Repeated parse faults, repeated actions, or tool errors record typed faults
and route to recovery nodes such as recover-parse, recover-repeat, or
recover-tool. Recovery changes the next action class: inspect graph state,
choose a smaller scope, use an alternate native tool, replan around a blocked
step, and use shell only from a shell-admitted node.

Some owner messages activate task-family completion requirements. Recursive
structure tasks cannot close until graph evidence proves a README-indexed
recursive tree. Documentation, counted structured content, and knowledge-base
requests enter document construction nodes that create structure anchors
before endpoint execution. File and markdown-count requests become completion
guards and compose with recursive or knowledge-base requirements instead of
replacing them. Exact or approximate wording is scored near the chosen count
target, so direct exact wording is strict while exact wording attached to a
smaller subcount does not make an approximate total strict. Approximate
wording is a scale hint and accepts the documented tolerance. Active count
guards are rendered in the graph-state prefix with an instruction to use
recursive directories for large outputs, prefer `artifact.next`, `doc.audit`,
`fs.read_many`, `fs.tree`, `workspace.index`, `fs.batch_write`, and `fs.patch`,
keep the action payload bounded, and avoid shell unless graph policy admits a
shell escape after native tools are insufficient.
Generic counted scaffolds are allowed only for explicitly generic counted
documentation tasks that name no semantic root, no exact target path, and no
large-artifact profile. That route may create `structured-output/` with design
files, main files, audit manifests, coverage maps, reading paths, per-part
ledgers, and count evidence, then close only after the graph completion gate
admits the deterministic evidence.

Named roots, exact paths, manuscripts, reports, study sets, dictionaries,
cookbooks, documentation artifacts, and other long structured work products use
the large-artifact engine instead. When the owner names a concrete root or
target path, generic roots such as `structured-output`, `output`, `artifact`,
and `work-product` are forbidden roots, not repair candidates. If either route
lacks its required evidence, the daemon waits or repairs with the missing gate
reason instead of closing.

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
