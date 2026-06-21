# Tool Policy

## Purpose

Define the tool classes admitted by runtime authority for each active mission.

## Decision Owner

`lkjagent-runtime` owns tool policy. `lkjagent-tools` receives the effective
policy and refuses any action outside it.

## Inputs

The policy reads active mission, evidence gaps, recovery class, artifact gaps,
last refused action, verification state, and compaction state.

## Output

The output lists admitted tool names, blocked tool names, refusal reason,
policy contradiction if any, and one next executable action.

## Escape Classes

Observation tools include `fs.read`, `fs.stat`, `fs.list`, `doc.audit`, and
`artifact.audit`. Repair tools include `fs.write`, `fs.batch_write`,
`artifact.next`, `artifact.apply`, graph evidence tools, and structured
handoff tools when repair cannot safely continue.

## Prohibited States

- Missing evidence blocks the read or audit tool that would observe it.
- Recovery blocks every mutation tool while content repair is required.
- Verification-only mode hides artifact creation or repair tools.
- Preferred next action names a blocked tool.

## Fixture

`tool_admission_graph_plan_contradiction` and
`completion-with-blocked-mutation` prove escape tools remain admitted.

## Verification

Run `cargo test -p lkjagent-runtime tool_admission` and
`cargo test -p lkjagent-tools effective_policy_repair`.

## Status

partially implemented.
