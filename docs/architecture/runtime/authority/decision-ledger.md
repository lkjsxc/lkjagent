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

Every decision records:

- decision id, snapshot id, event id, case id, and prompt frame head;
- mission, active mode, decision kind, graph node, and graph phase;
- admitted tools, blocked tools, exact next tool class, and either a valid
  non-content example or a content write contract;
- runtime effect command when no model-authored content is needed;
- missing evidence, existing evidence, and audit-owned evidence gaps;
- artifact root, artifact kind, profile, weak cursor, batch cursor, and current
  weak path set;
- latest observation, latest successful observation, fault class, recovery
  route, repeat count, and next route;
- compaction policy, trigger reason, pre and post snapshot ids, and resume target;
- maintenance plan, cooldown outcome, and no-op reason;
- completion state, failed gates, close allowance, and refusal text;
- provider anomaly state, retry count, exchange id, and pause target;
- prompt card data, context package ids, authority fingerprint, and staleness
  fingerprint;
- persistence plan for model-log, status, admission, effect, and observation
  rows;
- rule explanation naming the first matching mission-priority rule.

## Invariants

- One event produces one persisted decision.
- Prompt, model-log, status, admission, dispatch, compaction, maintenance, and
  close paths cite the same decision id.
- Model-call decisions have a non-empty admitted tool set.
- Empty admitted tool sets are limited to deterministic runtime effects, owner
  wait, accepted close, and closed idle.
- Completion refusal includes one next admitted audit or repair tool.
- Content-write prompts render contracts and never prefill body prose.
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

partially implemented. `RuntimeDecisionRecord` exists. SQLite tables plus store APIs record authority snapshots,
events, decisions, transitions, effects, and tool admissions. Runtime turn authority refresh writes normalized
snapshot, event, decision, and transition rows. Prompt authority cards include the persisted decision id and
fingerprint, and dispatch admission views read the same id. Broader route coverage remains open.
