# System Prompt

## Purpose

Specify the prefix document: the only standing instruction the model ever
receives. It is assembled by the harness at compaction and restart, within
the budgets in [../context/budgets.md](../context/budgets.md).

## Sections, In Order

| Section | Content | Source |
| --- | --- | --- |
| identity and rules | who the agent is, the YOLO posture, honesty rules, narrowness incentives | static template in lkjagent-runtime |
| grammar | the action format plus minimal fs.write and agent.done examples | static, from [action-format.md](action-format.md) |
| registry | one line per tool: name, parameters, one-clause contract | generated from [../tools/registry.md](../tools/registry.md) |
| skill index | one line per skill: name plus trigger sentence | generated per [../skills/loading.md](../skills/loading.md) |
| workspace brief | the workspace's own AGENTS.md, verbatim | /data/workspace per [../sandbox/workspace.md](../sandbox/workspace.md) |
| memory digest | ranked rendering of durable memory, task summary first | built per [../memory/distillation.md](../memory/distillation.md) |

## Identity Skeleton

```
You are lkjagent, a continuously running agent. You act through exactly one
action per turn and see one observation per action. You never invent results:
if you did not observe it, you do not claim it. Observations are bounded:
read in ranges, filter shell output, search memory before re-reading. When a
task completes, finish with agent.done and an honest summary. When only the
owner can decide, ask with agent.ask. You may think before acting inside
<think> tags. Task turns have YOLO authority inside /data/workspace and /data.
When no task is open and the queue is empty, wait for owner work.
```

The full template lives with the runtime crate and is checked against its
token budget at build time; growing it requires shrinking it elsewhere.

## Grammar Skeleton

```
Emit exactly one <act> block per turn and no prose outside tags. The first
child is <tool>; remaining children are parameters from the registry. Values
are raw text between tags. Stop immediately after </act>.
```

The grammar includes one fs.write example and one agent.done example. They
exist because local chat models must see the exact action envelope before any
task-specific instruction.

## Generation Rules

- Deterministic: same store state and config produce byte-identical
  prefixes, required by [../context/caching.md](../context/caching.md).
- No dates, no model names, no environment trivia: nothing the model does
  not act on.
- The registry section is derived from the same table the dispatcher uses,
  so prompt and behavior cannot drift apart.
- Each section is delimited by a single h2-style line (for example
  `## memory digest`) so the model can cite sections by name.

## What Is Deliberately Absent

Multi-turn few-shot transcripts, personality prose, capability marketing,
and apology templates. Every sentence either changes behavior or leaves.

## Status

implemented.
