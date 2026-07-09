# Operation Graph

## Purpose

Define semantic executable work without task, step, or template authority.

## Nodes

Every operation stores an opaque ID, matter, semantic kind, typed inputs and
outputs, budget, admission requirement, state, current flag, and unique
idempotency key. Large outputs use named semantic units rather than anonymous
serial fragments.

Common operation families include owner intake, clarification, deterministic
workspace effects, model content, tool action, observation, check, recovery,
report, timed wake, and maintenance.

## Edges

Typed acyclic edges express `requires`, `produces`, `invalidates`, `supersedes`,
`retries`, and `blocks`. Self edges and duplicate triples are invalid. A
selector cannot skip an unsatisfied dependency.

## Compilation

Owner intent becomes a matter, required obligations, and feasible operations.
Deterministic rules compile known intents. The model may propose a decomposition
for unfamiliar goals, but the harness validates every node, edge, budget,
output path, and required check before it becomes current.

## Replanning

Replanning appends validated events. It preserves completed evidence, explains
superseded work, prevents duplicate output paths, and proves that the revised
graph fits remaining context, output, effect, retry, and elapsed budgets.
