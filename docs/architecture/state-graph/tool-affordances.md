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

Completion nodes allow agent.done only after `CompletionState` is ready.
Recovery nodes allow inspection, graph notes, transitions, and narrower
verification or shell escape actions when policy admits them.

The dispatcher uses the same registry as the prompt. If graph policy refuses
a tool, the observation names the active node and gives allowed alternatives.

## Status

implemented.
