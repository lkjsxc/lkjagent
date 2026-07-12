# Working Here

## Purpose

Define how coding agents execute tracked work without premature completion.

## Case State

Before editing, record objective, constraints, assumptions, risks, evidence
requirements, candidate files, and next action. Recheck source and current-state
claims before relying on historical plans.

## Workgraph

`../../evaluation/workgraph.tsv` is the dependency input. A required row becomes
eligible only when every dependency predicate has source-bound evidence. The file
contains no status column. The acceptance checker derives completion.

After each commit, inspect the actual diff and commands, then start every safe
newly eligible row. A milestone, subagent summary, process health, or model prose
is not a stopping condition.

## Parallel Work

Use isolated worktrees for disjoint modules. Give each coding subagent exact
owned paths, frozen interfaces, exclusions, and one substantive shared suite.
The parent owns docs, architecture, shared exports, merges, and final evidence.
Inspect every subagent diff before integration; never merge two alternatives.

Runtime sub-agents are not part of the product.

## Commits

Update the owning contract before implementation and include
`docs/current-state.md` when behavior moves. Commit one coherent slice. Trailers
name exact passing commands and exact unrun commands/reasons.

Do not write a pass receipt by hand. Keep failed evidence and rejected experiment
results, then delete losing product code.

## Stop

Success requires final Docker/live/PTY evidence, clean independent review, and
exit zero from the tracked acceptance command. A blocked handoff is allowed only
when no safe eligible work remains and it names the exact resume command.
