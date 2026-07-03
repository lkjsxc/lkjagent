# Step Kinds

## Purpose

Define the bounded units of work the engine can execute.

## Kind Set

`engine.step-kinds.count=7` names the complete set: `plan`, `write`, `revise`,
`explore`, `verify`, `respond`, and `ask`. No task work happens outside these
kinds.

## Contracts

| Kind | Model block | Engine effect |
| --- | --- | --- |
| `plan` | `<plan>` | parse plan lines and materialize valid steps |
| `write` | `<content>` | write or append content at the planned path |
| `revise` | `<content>` | replace one planned file with corrected content |
| `explore` | `<action>` | run one bounded tool, including `finish` |
| `verify` | none, or `<verdict>` for judged checks | record check results |
| `respond` | `<message>` | append an owner-facing event |
| `ask` | `<message>` | ask the owner and set the task to waiting |

A write step for Aurora Ledger might say: write the next section of
`stories/aurora-ledger/manuscript/chapter-03.md`, target the objective-derived
word range, and return only `<content>...</content>`. The owner target is data
from the objective; generation size is controlled by `llm.max-tokens.write`.

## Explore And Ask

Explore completion is the `finish` tool inside `<action>`, with a `summary`
parameter. Asking the owner is not an explore action; it requires an `ask` step
that returns `<message>` and parks the task as `waiting`.

## Checks

Step and task checks use the [completion catalog](completion.md). A verify step
that needs model judgment uses `<verdict>`, but deterministic checks are the
normal path.

## Failure This Prevents

Illegal action loops cannot form for scripted work. The model sees one expected
block for the selected kind, so there is no rejected tool search space.
