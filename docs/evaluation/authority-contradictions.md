# Authority Contradictions

## Purpose

Define fixtures for conflicts between graph suggestions, model actions, and
runtime tool admission.

## Contract

Known-bad traces contain a preferred next action that dispatch refuses or a
recovery instruction that blocks the tool needed to escape. Known-good traces
contain one runtime authority object naming admitted tools, blocked tools,
reason, and exact next valid action.

## Required Cases

- `tool_admission_graph_plan_contradiction`.
- Completion node with missing content and blocked mutation.
- Recovery mode that admits read, audit, and repair tools.

## Pass Condition

The preferred next action never names a blocked tool. If graph policy and
runtime policy conflict, the runtime decision wins and records a policy fault.

## Verification

Run `cargo test -p lkjagent-runtime tool_admission`.

## Status

partially implemented.
