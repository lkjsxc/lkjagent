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

Records objective, steps, reason, optional constraints, assumptions, risks,
checks, and paths. It satisfies plan evidence only with a non-empty objective,
at least one step, and at least one of checks or paths.

The Chronos story-bible task renders this dispatch-valid example when the root
is known:

```text
<action>
<tool>graph.plan</tool>
<objective>Create a structured science-fiction story bible for Chronos Fracture.</objective>
<constraints>
Root directory: stories/chronos-fracture.
Root must contain README.md and catalog.toml.
Every directory must contain README.md and at least two children.
Every Markdown file must stay under 160 lines.
Do not write the full manuscript.
Do not write scaffold-only pages.
</constraints>
<steps>
1. Record the story-bible plan.
2. Create the root catalog and README.
3. Write bounded batches for setting, characters, plot, continuity, and checks.
4. Audit document structure.
5. Audit artifact readiness.
</steps>
<paths>
stories/chronos-fracture
</paths>
<reason>The owner requested a structured story bible with evidence-gated completion.</reason>
</action>
```

## graph.transition

Requests a target node. The harness evaluates typed edge guards and refuses
illegal transitions with missing evidence or policy reasons.

Targets are graph node ids only. The runtime must never render labels such as
`plan:admitted` as targets, and must not recommend `plan` when that transition
is illegal from the active node.

## graph.context

Selects graph context packages by id. The runtime validates package ids
against the source graph and active family.

## graph.note

Records structured non-evidence state: constraint, assumption, risk,
decision, question, invariant, success, or path. It never satisfies
completion evidence by itself. Generated examples must use one of those kinds,
such as decision.

A question note opens an owner-required question. `agent.ask` is admitted only
after such a question exists and remains open.

## graph.evidence

Records explicit evidence against a known requirement when the harness cannot
infer it from a tool output. The runtime links evidence to the active case.
Known requirements include `plan`, `observation`, `verification`, and
`document-structure`, plus dynamically registered requirements. Note-like
kinds such as decision, risk, planning, and recovery are rejected.

## graph.compact

Requests a graph compaction checkpoint when policy admits it. Forced
compaction is runtime-owned and never requires a model-authored `memory.save`
action that graph policy blocks.

## Status

partially implemented; dispatch validation and examples exist. Runtime
transition selection and artifact-specific evidence coverage remain open.
