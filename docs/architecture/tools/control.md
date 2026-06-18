# Control Actions

## Purpose

The contracts for agent.done and agent.ask: the two actions that close or
suspend work. Both use the standard act shape of
[../protocol/action-format.md](../protocol/action-format.md). Canonical
parameter table: [registry.md](registry.md).

## agent.done

| Parameter | Rule |
| --- | --- |
| summary | required |

Closes the open task. The summary states what was asked, what was done, and
how it was verified. agent.done also ends a maintenance cycle per
[../runtime/self-maintenance.md](../runtime/self-maintenance.md); an empty
cycle ends with a one-line summary.

## agent.ask

| Parameter | Rule |
| --- | --- |
| question | required |

Delivers a question to the owner and moves the task into waiting. At most
one question may be outstanding; a second agent.ask while one waits is a
tool error. The owner responds through the same `lkjagent send` path per
[../runtime/queue-intake.md](../runtime/queue-intake.md).

## Budget Interaction

When the task turn budget is exhausted, the harness appends a budget
notice, and only agent.ask or agent.done are lawful next actions, per
[../runtime/agent-loop.md](../runtime/agent-loop.md) and the taxonomy in
[../protocol/recovery.md](../protocol/recovery.md). Asking buys owner
guidance; done closes honestly with what was achieved.

## Think Is Not a Tool

The think preamble of
[../protocol/action-format.md](../protocol/action-format.md) is unparsed
free text for the model's own reasoning. It carries no parameters,
dispatches nothing, and never yields an observation. The dispatcher
refuses think as a tool name like any other unknown name.

## Status

implemented.
