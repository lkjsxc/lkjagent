# Tool Affordances

## Purpose

Document the graph-level tool policy that keeps the model deliberate.

## Phases

Planning and context nodes allow graph planning, notes, context selection,
bounded file reads, multi-file reads, listing, tree, search, stat, workspace
summary, workspace index, memory find, and owner ask. They block mutating file
tools, shell.run, and agent.done.

Execution nodes allow the planned mutation tools and evidence recording.
Verification nodes allow verify.cargo, verify.xtask, doc.audit, bounded read,
multi-file read, search tools, graph evidence, and shell.run only as an
escape hatch.

Document construction nodes allow planning, audit, contract selection, batch
writes, and bounded single-file writes for semantic indexes or sections.
Payload-risk recovery redirects large raw writes to `artifact.next`, audit, or
smaller contract-bound sections.

Completion nodes allow agent.done only after `CompletionState` is ready.
Recovery nodes expose the smallest useful action surface for the active fault.
`recover-parse` favors `graph.recover` and exact copyable action examples.
`recover-params` blocks `graph.next` loops and mutation. `recover-tool`
allows one state inspection, then alternate native tools or smaller scope.
`recover-repeat` requires a different action fingerprint. Shell is allowed
only from `recover-by-shell-escape`.

`recover-by-smaller-scope` must admit `graph.plan` or transition to a
planning node that admits it. `recover-by-artifact-plan` admits artifact
planning, next-batch contracts, audits, or contract-bound writes for content
tasks.
`recover-by-bounded-write` admits batch writes or next-batch planning and
blocks another raw large write after payload risk.

`recover-by-artifact-plan` admits `artifact.plan`, `artifact.audit`,
`artifact.next`, `doc.audit`, `fs.batch_write`, `fs.mkdir`, `fs.list`,
`fs.tree`, and `fs.stat`.
`recover-by-bounded-write` admits `artifact.next`, bounded `fs.batch_write`,
and audit tools.
`recover-by-alternate-tool` must not leave an owner task with only diagnostic
tools when artifact repair or bounded writing is required.

The dispatcher uses the same registry as the prompt. If graph policy refuses
a tool, the observation names the active node, phase, reason, preferred next
action, and one copyable allowed example. No rendered valid example may be
rejected by the same registry or dispatcher.

When graph policy contradicts missing completion evidence or recovery needs,
runtime authority follows [escape-hatches.md](escape-hatches.md). The graph
recommendation stays visible as evidence, but it does not decide close
eligibility or block required audit and repair tools.

## Status

partially implemented; core affordance checks exist. Full recovery-node
escape coverage and artifact-plan affordances remain open.
