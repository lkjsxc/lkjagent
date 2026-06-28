# Tool Escape Hatches

## Purpose

Define tools that remain available when recovery needs observation or repair.

## Contract

Recovery policy admits the smallest escape set that can observe missing state,
repair exact gaps, verify the repair, or emit a structured handoff. Escape
tools are runtime-authority decisions, not graph-node side effects.

## Observation Tools

`fs.read`, `fs.stat`, `fs.list`, `fs.tree`, `doc.audit`, `artifact.audit`,
`graph.state`, and `queue.list` are observation tools when they are needed to
escape the current fault.

## Repair Tools

`fs.write`, `fs.batch_write`, `artifact.next`, `artifact.plan`,
`graph.transition`, graph evidence tools, and `agent.done` with blocked
handoff shape are repair or handoff tools when admitted by authority.

## Invariants

- Missing completion evidence cannot block the audit that would produce it.
- Missing artifact content cannot block the batch write that repairs it.
- Missing artifact root cannot force verification-only mode.

## Fixture

`completion-with-blocked-mutation` proves repair tools remain available.

## Verification

Run `cargo test -p lkjagent-tools effective_policy_repair`.

## Status

partially implemented.
