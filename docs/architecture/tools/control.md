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

Closes the active graph case. The summary states what was asked, what was
done, and how it was verified. agent.done also ends a maintenance case per
[../runtime/self-maintenance.md](../runtime/self-maintenance.md); an empty
case ends with a one-line summary.

Recursive structure tasks carry graph evidence requirements. When the owner
asks for recursive or highly structured organization, agent.done is refused
until graph evidence proves a README-indexed tree with enough depth and
breadth. Encyclopedia, wiki, and knowledge-base creation requests use a
recursive-knowledge task family: the docs tree must contain a small nucleus
with required map, starter-domain, reference, curation, expansion-queue, and
rebalance-plan files. The graph gate requires contract-shaped markdown pages,
growth-control sections, and enough links to behave like a navigable nucleus.

Owner tasks that state a file or markdown file count also carry completion
requirements. Exact wording requires one candidate tree to contain exactly
that many files. README-indexed roots are preferred candidates; when none
exist, one visible top-level directory with no visible sibling files is also
a candidate. Approximate wording such as about, around, roughly,
approximately, or common Japanese equivalents accepts a 10 percent tolerance
with a minimum tolerance of one file. Markdown-count requests count only .md
files; general file-count requests count every non-hidden regular file. Count
guards compose with recursive and knowledge-base guards. The refusal names
the closest candidate and directs the next action toward one compact
shell.run repair script.

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
notice and moves the case into a graph recovery or waiting phase per
[../runtime/agent-loop.md](../runtime/agent-loop.md) and the taxonomy in
[../protocol/recovery.md](../protocol/recovery.md).

## Think Is Not a Tool

The think preamble of
[../protocol/action-format.md](../protocol/action-format.md) is unparsed
free text for the model's own reasoning. It carries no parameters,
dispatches nothing, and never yields an observation. The dispatcher
refuses think as a tool name like any other unknown name.

## Status

implemented.
