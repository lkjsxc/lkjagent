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
`artifact.next`, `artifact.plan`, graph evidence tools for non-audit-owned
requirements, and structured handoff tools when repair cannot safely continue.
`artifact.plan` is admitted for semantic artifact work only. A code-change task
that names one target file can receive an exact `fs.write` surface after plan
evidence instead of being routed into artifact identity.

## Prohibited States

- Missing evidence blocks the read or audit tool that would observe it.
- Missing plan evidence blocks `graph.plan` without an admitted alternate.
- Recovery blocks every mutation tool while content repair is required.
- Verification-only mode hides artifact creation or repair tools.
- Preferred next action names a blocked tool.
- The exact valid example uses a blocked tool.

## Fixture

`tool_admission_graph_plan_contradiction`,
`effective_policy_contradiction`, and `completion-with-blocked-mutation` prove
escape tools remain admitted and rendered examples use admitted tools.

## Verification

Run `cargo test -p lkjagent-runtime tool_admission`,
`cargo test -p lkjagent-tools --test effective_policy_contradiction`, and
`cargo test -p lkjagent-tools effective_policy_repair`.

## Status

implemented for kernel-derived admission views, artifact work, non-artifact
single-file writes, recovery escape tools, and completion refusal paths.
