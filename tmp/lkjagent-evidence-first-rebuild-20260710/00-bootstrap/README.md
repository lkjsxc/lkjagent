# Bootstrap

## Purpose

Control the downstream coding agent without relying on its own confidence.

## Contents

- BOOTSTRAP_PROMPT.md: self-contained start and resume instruction.
- REQUIREMENT_MAP.md: owner requirement to contract, node, and proof traceability.

- START_HERE.md: primary prompt.
- EXECUTION_RULES.md: hard operating constraints.
- FIRST_ACTIONS.md: deterministic start.
- EXTERNAL_CONTROL.md: anchored manifest and verifier flow.
- SUBAGENT_WAVES.md: parallel dispatch and merge rules.
- STOP_POLICY.md: the only permitted stopping conditions.
- workgraph.xml: attribute-free execution graph.

## Important Difference

Earlier packets used editable checklists that the implementation agent marked
complete from narrow tests. This packet uses an anchored acceptance contract,
fresh evidence after the final source commit, and an independent verifier.
