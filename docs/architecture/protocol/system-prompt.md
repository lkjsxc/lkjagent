# System Prompt

## Purpose

Specify the prefix document: the only standing instruction the model ever
receives. It is assembled by the harness at compaction and restart, within
the budgets in [../context/budgets.md](../context/budgets.md).

## Sections, In Order

| Section | Content | Source |
| --- | --- | --- |
| identity and rules | who the agent is, graph posture, YOLO posture, honesty rules | static template in lkjagent-runtime |
| grammar | the action format plus minimal fs.write and agent.done examples | static, from [action-format.md](action-format.md) |
| registry | one line per tool: name, parameters, one-clause contract | generated from [../tools/registry.md](../tools/registry.md) |
| graph state | active case, phase, node, legal transitions, context packages, missing evidence | graph slice |
| workspace brief | the workspace's own AGENTS.md, verbatim | /data/workspace per [../sandbox/workspace.md](../sandbox/workspace.md) |
| memory digest | ranked rendering of durable memory, task summary first | built per [../memory/distillation.md](../memory/distillation.md) |

## Identity Skeleton

```
You are lkjagent, a graph-governed continuous agent. Treat every meaningful
task as a case with phases, evidence, context packages, and legal transitions.
Do not act directly from the first owner message. Follow the active graph
state notice before choosing an action. Prefer inspection and plan
construction before edits. You act through exactly one action per turn and see
one observation per action. You never invent results: if you did not observe
it, you do not claim it. Close only when graph evidence gates are satisfied.
For exact file-count tasks, create a README-indexed manifest, write batches
with shell.run, verify counts with shell commands, and repair in one script
before agent.done. For approximate file-count tasks, verify the tolerance
instead of forcing exact-count repairs.
When only the owner can decide, ask with agent.ask.
```

The full template lives with the runtime crate and is checked against its token
budget at build time; growing it requires shrinking another prefix section.

## Grammar Skeleton

```
Emit exactly one <act> block per turn and no prose outside tags. The first
child is <tool>; remaining children are parameters from the registry. Values
are raw text between tags. Stop immediately after </act>.
```

The grammar includes one fs.write example and one agent.done example. Local
chat models must see the exact action envelope before graph-specific guidance.

## Generation Rules

- Deterministic: same store state and config produce byte-identical
  prefixes, required by [../context/caching.md](../context/caching.md).
- No dates, no model names, no environment trivia: nothing the model does
  not act on.
- The registry section is derived from the same table the dispatcher uses,
  so prompt and behavior cannot drift apart.
- The graph state section is derived from the active `TaskGraphState` and
  source graph definitions.
- Each section is delimited by a single h2-style line (for example
  `## memory digest`) so the model can cite sections by name.

## What Is Deliberately Absent

Multi-turn few-shot transcripts, personality prose, capability marketing,
and apology templates. Every sentence either changes behavior or leaves.

## Status

implemented.
