# Recovery Ladders

## Purpose

Define how typed faults move through graph recovery without turning recoverable
runtime failures into owner waiting.

## Fault Classes

The runtime records every fault before selecting recovery. The closed fault
classes are parse, parameter, tool, repeat, endpoint, budget, context,
verification, compaction, payload, and completion.

Each record contains kind, active case, active node, action fingerprint,
parameter shape when present, summary, count, timestamp, ladder position,
selected route, and the next admitted action class.

## Route Table

| Fault class | First route | Forced action class | Escalation |
| --- | --- | --- | --- |
| parse | `recover-parse` | one valid action block | smaller action surface |
| parameter | `recover-params` | same tool with registry schema | alternate inspection |
| tool | `recover-tool` | admitted native inspection | alternate native tool |
| repeat | `recover-repeat` | different tool class | blocked handoff or smaller scope |
| endpoint | `recover-endpoint` | retry or workspace summary | blocked handoff |
| budget | `recover-budget` | continuation checkpoint | blocked handoff |
| context | `recover-context` | runtime context repair | blocked handoff |
| verification | `recover-verification` | admitted verification or repair | blocked handoff |
| compaction | `recover-compaction` | runtime compaction snapshot | blocked handoff |
| payload | `recover-by-bounded-write` | artifact plan or batch write | smaller batch |
| completion | `recover-completion` | audit or missing evidence repair | blocked completion handoff |

## Ladder Rules

Recovery follows this deterministic order:

1. correct format or parameters and retry once.
2. inspect graph or workspace state and choose a different native tool.
3. reduce scope and create a smaller plan step.
4. route to the matching recovery node and record the decision.
5. use shell only from a shell-admitted recovery node.
6. mark the specific plan step blocked and continue an independent step.

Repeated fingerprints are evidence that the next action class must change. A
second `graph.next` for the same fault records diagnostic exhaustion and forces
`graph.recover`, a legal transition, an unused non-mutating native tool, a
smaller plan step, or a blocked handoff.

## Parameter Recovery

Parameter faults use `recover-params`, not generic parse recovery. Safe aliases
may normalize when the meaning is clear and the normalized path stays inside
the workspace. The observation records normalization.

`graph.note` accepts only constraint, assumption, risk, decision, question,
invariant, success, and path. `graph.evidence` targets known missing
requirements and cannot satisfy audit-owned requirements.

## Escape Requirements

Recovery suggestions must be admitted by the active mode. If the current node
blocks the only productive mutation tool, the controller transitions to a node
that admits that tool or produces a blocked handoff with exact evidence.

Waiting is forbidden when an internal transition, alternate native tool, or
smaller independent step can continue. `agent.ask` is allowed only for a
concrete owner-required question.

Compaction is runtime-owned. Forced compaction never asks the model to run
`memory.save` when policy blocks it.

## Long Payloads

Long-payload and completion-oversize faults route to artifact planning or
bounded batch writes, not raw `fs.write` retries. Batch repair must name exact
paths and line-limit-safe content.

## Status

partially implemented. Fault notices, graph routes, repeat refusals, escape
visibility, and a pure recovery plan table exist. Durable retry counts and live
shape-change enforcement for every fault class remain open.
