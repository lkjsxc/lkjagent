# Agent Loop

## Purpose

Specify the single continuous loop: how a turn runs, how a task opens and
closes, and which budgets bound it. Decision:
[../../decisions/single-loop.md](../../decisions/single-loop.md).

## The Turn

A turn is the atomic unit of agent activity:

1. Boundary work: deliver due queue messages as owner frames
   ([queue-intake.md](queue-intake.md)), check the compaction trigger
   ([../context/compaction.md](../context/compaction.md)).
2. Call the endpoint with the current message list and the stop sequence
   `</act>`; sampling per [../llm/sampling.md](../llm/sampling.md).
3. Parse the completion into one action per
   [../protocol/parsing.md](../protocol/parsing.md). On failure, follow
   [../protocol/recovery.md](../protocol/recovery.md).
4. Execute the action through the toolset
   ([../tools/registry.md](../tools/registry.md)).
5. Append the action and its bounded observation to the transcript and to
   the context log. Token caps per
   [../context/budgets.md](../context/budgets.md).

Steps are sequential; nothing else touches the context while a turn runs.
Recoverable parser, repeat-action, and tool faults append recovery notices
to the same transcript and leave the task open for the next endpoint turn.

## The Task

A task is opened when an owner message is delivered with no task open, and
closed by an agent.done action whose summary is recorded.

| Task state | Driven by |
| --- | --- |
| open | owner message delivered; turns proceed |
| waiting | agent.ask emitted; loop waits until another send arrives |
| closed | agent.done emitted; summary recorded; task-summary memory saved |

A task carries a turn budget (initial contract: 64 turns, config-tunable).
When exhausted, the harness injects a budget notice; the only lawful actions
after it are agent.ask or agent.done. This converts runaway loops into a
question for the owner instead of silent burn.

Some owner messages activate a completion guard. Recursive structure tasks
cannot close until the tool layer verifies a README-indexed recursive tree;
a premature agent.done returns a tool error and the task remains open. The
daemon auto-loads the recursive-structure skill body immediately after the
opening owner frame and before the first endpoint completion for that task.
When the owner asks for recursive docs or documentation, the daemon also
creates a README-indexed docs scaffold, then records that observation
before calling the endpoint. Encyclopedia, wiki, and knowledge-base creation
requests use the same pre-endpoint path with a stricter recursive-knowledge
guard and a small nucleus scaffold with maps, a starter domain, reference,
curation, execution state, an expansion queue, and a rebalance plan. That
guard requires nucleus, growth-control, and required-file evidence before
agent.done can close.

When no task is open and the queue is empty, the daemon opens a bounded
self-maintenance cycle, records `daemon_state=working`, and continues toward
one concrete improvement or an honest empty-cycle agent.done. Queue arrival
preempts maintenance at the next turn boundary.

## Pure Core

The loop body is a pure transition function in lkjagent-runtime composed
from lkjagent-context and lkjagent-protocol:

```
step : (State, Completion) -> (State, Effects)
```

Effects (tool execution, store writes, endpoint calls) are interpreted by
thin adapters. Tests drive `step` with recorded completions and assert
states and effects without any IO, per
[../../repository/functional-style.md](../../repository/functional-style.md).

## Stop Reasons

Every turn records why it ended: acted, done, ask, invalid_action,
unknown_tool, bad_params, repeat_action, endpoint_error, tool_error,
budget_notice. The taxonomy is owned by
[../protocol/recovery.md](../protocol/recovery.md).

## Status

implemented.
