# Decision Ledger

## Purpose

This file owns the persistent runtime decision record emitted for each authority event.

## Contract

Every runtime path emits exactly one `RuntimeDecision` for one `RuntimeEvent` before an effect runs. The decision is
persisted and becomes the source for prompt rendering, dispatch admission, refusal text, compaction, maintenance,
and close behavior.

## Inputs

- `RuntimeSnapshot`.
- `RuntimeEvent`.
- mission priority table.
- completion, recovery, compaction, maintenance, and tool policies.
- registry-rendered exact examples.

## Outputs

- decision id, case id, and event id.
- mission, active mode category, and active node.
- admitted tools, blocked tools, forced next action, and recommended next actions.
- exact valid example and missing evidence.
- completion decision, recovery plan, compaction requirement, and maintenance allowance.
- prompt card, persistence writes, and authority fingerprint.

## Invariants

- One event produces one persisted decision.
- Prompt and dispatch use the same decision id or an immutable view derived from it.
- Completion refusal includes one next admitted audit or repair tool.
- Recovery decisions shrink or change the action surface after repeated faults.
- Decisions record enough fields to explain why a requested tool was refused.

## Failure Cases

- Prompt policy, graph policy, and dispatch fallback each decide a different tool surface.
- A stale maintenance action executes after owner work enters the queue.
- `agent.done` closes through a path that skipped the central completion gate.
- A refusal gives prose but no exact valid next action.

## Verification

- reducer tests for every event class.
- store tests for decision history and admission joins.
- prompt and dispatch integration tests asserting the same decision id.

## Status

partially implemented. `RuntimeDecisionRecord` exists. SQLite tables plus store APIs record authority events,
decisions, and tool admissions. Runtime turn authority refresh writes normalized event and decision rows. Prompt
authority cards include the persisted decision id and fingerprint, and dispatch admission views read the same id.
Broader route coverage remains open.
