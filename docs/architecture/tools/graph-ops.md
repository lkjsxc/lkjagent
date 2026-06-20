# Graph Operations

## Purpose

Specify graph tools exposed to the model. The runtime owns mutation and guard
evaluation; these tools request or record typed state through tag actions.

## graph.state

Shows active case id, objective, family, phase, node, selected context
packages, evidence, missing requirements, allowed tools, and legal next
transitions. The harness also injects graph state notices automatically.

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
completion evidence by itself.

## graph.evidence

Records explicit evidence against a known requirement when the harness cannot
infer it from a tool output. The runtime links evidence to the active case.

## graph.compact

Requests a graph compaction checkpoint. Automatic compaction remains owned by
the runtime and is admitted only when graph policy allows it.

## Status

implemented.
