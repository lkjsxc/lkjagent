# System Prompt

## Purpose

Specify the prefix document: the standing instruction the model receives. It is
assembled by the harness at compaction and restart, within the budgets in
[../context/budgets.md](../context/budgets.md).

## Sections, In Order

| Section | Content | Source |
| --- | --- | --- |
| identity and rules | who the agent is, graph posture, YOLO posture, honesty rules | static template in lkjagent-runtime |
| grammar | the singular action envelope plus concrete action examples | static, from [action-format.md](action-format.md) |
| registry | tool schemas, conditional requirements, and executable examples | generated from [../tools/registry.md](../tools/registry.md) |
| runtime authority | decision id, active mode, admitted tools, blocked tools, and next action | runtime decision |
| graph state | active case, phase, node, legal transitions, packages, missing evidence | graph slice |
| workspace brief | the workspace's own AGENTS.md, verbatim | /data/workspace per [../sandbox/workspace.md](../sandbox/workspace.md) |
| memory digest | ranked rendering of durable memory, task summary first | built per [../memory/distillation.md](../memory/distillation.md) |

## Identity Skeleton

```text
You are lkjagent, a graph-governed continuous agent. Treat every meaningful
task as a case with phases, evidence, context packages, and legal transitions.
Follow the current runtime authority card before choosing an action. You act
through exactly one action per turn and see one observation per action. You
never invent results: if you did not observe it, you do not claim it. Close only
when the central completion reducer admits agent.done. When only the owner can
decide, ask with agent.ask.
```

The full template lives with the runtime crate and is checked against its token
budget at build time; growing it requires shrinking another prefix section.

## Grammar Skeleton

```text
Emit exactly one <action> block and no prose outside it. The first child is
<tool>. Every other child is an attribute-free parameter tag from the registry.
Values are raw text between opening and closing tags. Do not emit <actions>,
<act>, tag attributes, JSON tool calls, or hidden reasoning. Stop immediately
after </action>.
```

The grammar includes one concrete `fs.write` example and one concrete
`agent.done` example. Local chat models must see the exact action envelope
before graph-specific guidance.

## Example Rules

- Model-facing examples use current task data: current root, current missing
  evidence, current artifact kind, and current admitted tool surface.
- No example uses generic placeholder values.
- A valid example must parse, pass registry validation, pass conditional
  validation, and be admitted by the referenced runtime decision.
- A recovery example for an attribute-like path fault uses `<paths>` for
  `graph.plan` and `<root>` for artifact or document root tools.

## Generation Rules

- Deterministic: same store state and config produce byte-identical prefixes,
  required by [../context/caching.md](../context/caching.md).
- No dates, model names, or environment trivia: nothing the model does not act
  on.
- The registry section is derived from the same table the dispatcher uses.
- The runtime authority section is rendered from a persisted decision id.
- The graph state section is guidance input and cannot admit tools after
  runtime admission refuses them.
- Each section is delimited by a single h2-style line, such as
  `## memory digest`, so the model can cite sections by name.

## What Is Deliberately Absent

Multi-turn few-shot transcripts, hidden-reasoning requests, personality prose,
capability marketing, apology templates, and JSON tool-call instructions. Every
sentence either changes behavior or leaves.

## Status

partially implemented. Runtime prompt examples now use the `<action>` envelope,
but concrete registry examples for all visible tools and persisted decision-id
authority cards on every endpoint turn remain open.
