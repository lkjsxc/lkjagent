# Current State

## Purpose

This file is the honest ledger of lkjagent. It states what exists, what is
partial, and what is open. A behavior is implemented only when code, focused
tests, quiet gates, and required Docker gates prove it.

## Summary

lkjagent has a working Rust workspace with parser, protocol registry, graph,
context, store, LLM, tools, runtime, CLI, benchmark, and xtask crates. The
runtime-authority kernel exists and has local verification evidence from the
previous cutover, but the checked-in active model run is failure evidence, not a
fresh success proof for the current redesign.

The durable target is stricter than the checked-in run: one persisted runtime
decision must govern each endpoint turn, prompts must render one compact
authority card and one exact next action, live model output must use only the
singular tag action format, long artifacts must advance by audited
micro-batches, and completion must require current audit-owned readiness.

## Implemented Surfaces

| Area | Evidence |
| --- | --- |
| Workspace and gates | `Cargo.toml`, `crates/lkjagent-xtask`, and `docker-compose.yml` exist. |
| Parser | `lkjagent-protocol` parses singular action turns and emits structured faults. |
| Registry | `lkjagent-tools` validates tools, required parameters, and required-any groups. |
| Graph | `lkjagent-graph` stores typed cases, evidence requirements, tracks, and transitions. |
| Store | Queue, state, event, memory, authority, prompt-frame, observation, artifact, compaction, and provider-exchange surfaces exist. |
| Runtime kernel | Snapshot, event, decision, admission, effect, render, fault, provider, adapter, reducer, and driver records exist. |
| Endpoint loop | Provider calls record model-log files, token usage when present, anomalies, and bounded retry facts. |
| CLI | `lkjagent --help` and `lkjagent help` print usage before config loading, and `--data` is accepted before or after the command. |
| Artifact lifecycle | Artifact plan, apply, audit, next, cursors, weak paths, invalid roots, story readiness, and completion refusals are ledger-backed. |
| Maintenance | Maintenance gates, owner preemption checks, no-op cooldown facts, and closed-idle rules have focused coverage. |
| Benchmarks | Owner-reported recovery, artifact, memory, accounting, model-log, batch-schema, compaction, repeated-recovery, and long-novel signatures are in the corpus. |

## Active Data Log Fixture

`data/logs/current-model-run.md`, `data/logs/index.ndjson`, and latest turn
directories prove checked-in failure facts until a fresh smoke run proves
repair:

- active case `1` is at node `document` in phase `execution`;
- owner task is `Create a long novel. with structured settings.`;
- pre-owner maintenance repeats empty memory searches, no-op pruning, and
  maintenance close attempts instead of staying closed idle;
- the active run root is the long objective slug
  `stories/long-novel-with-structured-settings`, which the redesign replaces
  with a short semantic alias such as `stories/novel`;
- active tracks are `document-structure`, `action-param-reliability`, and
  `observability-ledger`;
- evidence ledger contains `plan` and `observation`; audit-owned
  `document-structure` and `artifact-readiness` remain missing;
- `artifact.apply` created a `NarrativeManuscript` scaffold and was repeated
  after the root already existed;
- `doc.audit` failed content readiness with structure-only story pages;
- an attempted batch exceeded the file-count limit and was refused before
  mutation;
- reasoning-only provider responses were recorded as provider anomalies;
- document audit and artifact readiness audit remain pending in the fixture.

## Runtime Authority Target Flow

```text
DurableReadModel -> RuntimeSnapshot
RuntimeSnapshot + RuntimeEvent -> RuntimeDecision
RuntimeDecision -> PromptFrame or RuntimeEffectCommand
RuntimeDecision + ModelAction -> ToolAdmission
ToolAdmission -> RuntimeEffectCommand
RuntimeEffectCommand -> EffectObservation
EffectObservation -> RuntimeEvent
```

The decision is persisted before prompt rendering, endpoint calls, dispatch,
recovery, compaction, maintenance, or close attempts. Prompt frames, provider
exchange rows, pending actions, admissions, observations, model-log exports,
and status expose the same authority ids and staleness fingerprints.

## Verification Evidence

The previous kernel cutover recorded passing local and Docker gates in this
file before this redesign reopened work. That historical evidence proves only
that the old cutover was internally consistent at that time. It does not prove
that the active long-novel fixture is repaired, that prompts are compact, that
object-literal batch formats are absent from model-facing context, or that short
artifact aliases are implemented.

New success claims for this redesign require the focused tests named in
[execution/current-blockers.md](execution/current-blockers.md), `quiet verify`,
and `docker compose run --rm verify`.

## Active Target

The dependency queue is [execution/current-blockers.md](execution/current-blockers.md).
The first open blocker is the truth sweep and fixture root reconciliation.

## Out of Scope

Messaging channels, web UI, MCP, runtime sub-agents, heartbeat schedules, and
cron schedules remain outside this product.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates exist.
- Checked-in run logs can be failure fixtures without proving current success.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
