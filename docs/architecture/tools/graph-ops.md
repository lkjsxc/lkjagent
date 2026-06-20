# Graph Operations

## Purpose

Specify graph tools exposed to the model. The runtime owns mutation and guard
evaluation; these tools request or record typed state through tag actions.

## graph.state

Shows active case id, objective, family, phase, node, selected context
packages, evidence, missing requirements, allowed tools, and legal next
transitions. The harness also injects graph state notices automatically.

## graph.next

Shows legal next transitions, missing guards, allowed tools, blocked tools,
and the preferred next action class. It is diagnostic, not a recovery strategy
by itself. After one `graph.next` for the same fault, the controller forces a
different action class.

## graph.audit

Reports whether the active graph case is internally consistent, whether plan
and completion gates are ready, what evidence is missing, and whether shell is
currently admitted.

## graph.recover

Reports the recovery ladder for the active fault state. It favors state
inspection, smaller scope, alternate native tools, replanning, and only then a
shell-admitted escape.

## graph.plan

Records objective, optional constraints, assumptions, risks, steps, checks,
paths, and reason. It satisfies plan evidence only with a non-empty objective,
at least one step, and checks or paths.

## graph.transition

Requests a target node. The harness evaluates typed edge guards and refuses
illegal transitions with missing evidence or policy reasons.

## graph.context

Selects graph context packages by id. The runtime validates package ids
against the source graph and active family.

## graph.note

Records structured non-evidence state: constraint, assumption, risk,
decision, question, invariant, success, or path. It never satisfies
completion evidence by itself. Generated examples must use one of those kinds,
such as decision.

## graph.evidence

Records explicit evidence against a known requirement when the harness cannot
infer it from a tool output. The runtime links evidence to the active case.

## graph.compact

Requests a graph compaction checkpoint when policy admits it. Forced
compaction is runtime-owned and never requires a model-authored `memory.save`
action that graph policy blocks.

## Status

implemented.
