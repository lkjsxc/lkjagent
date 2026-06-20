# Graph Policy

## Purpose

Define the typed policy attached to the source graph and enforced by runtime.

## Fields

`GraphPolicy` covers context pressure thresholds, planning requirements,
recovery limits, allowed tools by node, shell demotion, completion gates,
maintenance cadence, document defaults, and compaction preservation.

`NodePolicy` attaches local affordances to each node: entry and exit
conditions, fields read, fields updated, allowed tools, preferred tools,
blocked tools, package compression, recovery ladder, and completion or
maintenance contribution.

Policy is Rust data. The model sees a rendered slice of its consequences:
allowed tools, blocked tools, missing evidence, legal transitions, and next
recommended action class.

## Enforcement

The dispatcher validates an action against the registry, then checks the
active graph dispatch policy before routing the tool. Refusals are bounded
observations that name the active node and suggest allowed tools.

`shell.run` is refused unless both the active node lists the tool and graph
policy marks the node shell-admitted. Mutating file tools are refused before
plan and context guards pass. `agent.done` is checked by the completion gate
instead of the normal allowlist.

## Status

implemented.
