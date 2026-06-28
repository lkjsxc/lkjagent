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
If graph completion is refused because evidence is missing, the observation
names the missing kinds and points the next action to `graph.evidence` with
the first missing kind, an observed verification summary, and a path.

Owner tasks that state a file or markdown file count also carry completion
requirements. English file, document, and docs wording plus common Japanese
file wording activate general file-count guards; markdown and .md wording
activate markdown-count guards. ASCII digits, full-width digits, and
comma-like digit separators are accepted. When several numbers appear, the
target is the number closest to the file or document wording, while numbers
attached to non-file units such as words, pages, chapters, children, or lines
are ignored, so line-limit, child-count, and section-count instructions do not
create file-count tasks.
Exact or approximate wording is scored near the chosen target, so
direct exact wording requires one candidate tree to contain exactly that many
files, while exact wording for a smaller subcount does not make an
approximate total strict. README-indexed roots are preferred candidates; when
none exist, a clean set of visible top-level output directories with no
visible top-level files is also a candidate. Approximate wording such as
about, around, roughly, approximately, or common Japanese equivalents accepts
a 10 percent tolerance with a minimum tolerance of one file. Markdown-count
requests count only .md files; general file-count requests count every
non-hidden regular file. Count guards compose with recursive and
knowledge-base guards. The refusal names the closest candidate and directs
the next action toward doc.audit, fs.list, fs.stat, fs.batch_write, or
artifact.next or contract-bound repair before shell is considered.

## agent.ask

| Parameter | Rule |
| --- | --- |
| question | required |

Delivers a question to the owner and moves the task into waiting. At most
one question may be outstanding; a second agent.ask while one waits is a
tool error. Graph policy admits this action only after an open owner-required
question exists. The owner responds through the same `lkjagent send` path per
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
