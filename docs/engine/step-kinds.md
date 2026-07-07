# Step Kinds

## Purpose

Define known plan operation helpers without making them the full runtime state
space.

## Known Helpers

The current plan engine recognizes helpers for plan, write, revise, explore,
verify, respond, and ask work. In the state-ledger target these become operation
keys or typed helpers selected by `RuntimeDecision`, not a closed list of all
possible runtime states.

## Contracts

| Helper | Model block | Engine effect |
| `plan` | `<plan>` | parse plan lines into events and plan state |
| `write` | `<content>` | write or append content at the planned path |
| `revise` | `<content>` | replace one planned file with corrected content |
| `explore` | `lkjagent_action` block when tools are exposed | admit one tool call |
| `verify` | none, or `<verdict>` for judged checks | record check results |
| `respond` | `<message>` | append an owner-facing event |
| `ask` | `<message>` | ask the owner and set waiting state |

The exact envelope for a turn comes from the persisted decision. A helper cannot
render tools or blocks that are absent from that decision.

## Explore And Ask

Explore output is a decision-visible `<lkjagent_action>` tool call only when
the `ToolSetView` exposes it. Asking the owner is a selected operation that
parks waiting state and records the question as a clean context item.

## Checks

Step and case checks use the [completion catalog](completion.md). Deterministic
checks are the normal path; sparse model judgment is bounded evidence when the
decision selects it.

## Failure This Prevents

Known plan helpers remain convenient while unknown state keys and future
operation keys can hydrate, render diagnostics, and influence decisions without a
central enum edit.
