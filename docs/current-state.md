# Current State

## Purpose

This file is the honest ledger of lkjagent. It states what exists, what is
partial, and what is open. A behavior is implemented only when code, focused
tests, quiet gates, and required Docker gates prove it.

## Summary

lkjagent has a working Rust workspace with parser, protocol registry, graph,
context, store, LLM, tools, runtime, CLI, benchmark, and xtask crates. The
persisted-decision daemon path is implemented and proved by focused tests and
Docker verify. Runtime authority now records dense facts, obligations, resolver
plans, progress edges, deterministic effects, and typed completion gate inputs
for decisions. A live high-school romance request proves a remaining gap:
story-bible scaffolds are produced, but manuscript-scale chapter prose is not
reliably written. The checked-in active model run remains historical failure
evidence until an owner chooses to replace it.

The durable target is a deterministic state-transition runtime for a weak local
LLM:

```text
RuntimeSnapshot + RuntimeEvent -> RuntimeFacts
RuntimeFacts -> Vec<Obligation>
Vec<Obligation> + RuntimeFacts -> ResolverPlan
ResolverPlan -> RuntimeDecision
RuntimeDecision -> PromptFrame | RuntimeEffectCommand
RuntimeDecision + ModelAction -> ToolAdmission
ToolAdmission -> EffectObservation -> RuntimeEvent
```

The persisted runtime decision is the sole authority for mission, mode,
admitted tools, blocked tools, context policy, compaction, recovery,
completion, write contracts, deterministic effects, and the next action surface.
Model output supplies only bounded semantic intent or bounded file content
inside that selected surface.

## Implemented Surfaces

| Area | Evidence |
| --- | --- |
| Workspace and gates | `Cargo.toml`, `crates/lkjagent-xtask`, and `docker-compose.yml` exist. |
| Parser | `lkjagent-protocol` parses singular action turns and emits structured faults. |
| Registry | `lkjagent-tools` validates tools, parameters, and required-any groups. |
| Graph | `lkjagent-graph` stores typed cases, evidence requirements, tracks, and transitions. |
| Store | Queue, state, event, memory, authority, prompt-frame, observation, artifact, compaction, and provider-exchange surfaces exist. |
| Runtime kernel | Snapshot, event, decision, admission, effect, render, fault, provider, adapter, reducer, and driver records exist. |
| Endpoint loop | Provider calls record model-log files, token usage when present, anomalies, and bounded retry facts. |
| CLI | `lkjagent --help` and `lkjagent help` print usage before config loading, and `--data` is accepted before or after the command. |
| Artifact lifecycle | Artifact plan, audit, next, cursors, weak paths, invalid roots, story readiness, and completion refusals are ledger-backed. |
| Maintenance | Maintenance gates, owner preemption checks, no-op cooldown facts, and closed-idle rules have focused coverage. |
| Benchmarks | Owner recovery, artifact, memory, accounting, model-log, batch-schema, compaction, repeated-recovery, and novel signatures are in the corpus. |

## Active Data Log Fixture

`data/logs/current-model-run.md`, `data/logs/index.ndjson`, and latest turn
directories remain checked-in historical failure fixtures. Fresh smoke evidence
is separate until an owner chooses to replace the fixture. `index.ndjson` uses
`/data/logs/...` as a repository-relative log root and must resolve to present
turn directories after that normalization:

- active case `1` is at node `evidence-plan` in phase `recovery`;
- owner task is `Create a long novel. named "iwanna". with detailed and
  structured settings.`;
- observed root is `stories/novel-named`;
- `doc.audit` repeatedly reported `missing_root` for that root;
- authority refused a local `fs.mkdir` path that was not admitted;
- the model attempted duplicate `settings.md` `fs.batch_write` batches that did
  not create root identity;
- repeat recovery and `graph.recover` changed shape but routed back to
  same-root `doc.audit`;
- `graph.state` showed active case `1` while the runtime remained in recovery;
- reasoning-only provider responses were classified as provider anomalies;
- document audit, artifact readiness audit, and final verification remained
  pending.

The checked-in generated log fixture remains historical failure evidence.

## Runtime Authority Target Flow

The decision is persisted before prompt rendering, endpoint calls, dispatch,
recovery, compaction, maintenance, or close attempts. Prompt frames, provider
exchange rows, pending actions, admissions, observations, model-log exports,
and status expose the same authority ids and staleness fingerprints.

Live output is one singular tag action only. Top-level JSON, top-level
line-action syntax, nested file objects, object-literal batches, `<actions>`,
and `<think>` output are refused. `fs.batch_write` accepts line protocol only
inside `<files>`.

## Artifact Contract

The artifact lifecycle is:

```text
OwnerObjective -> ArtifactIdentity -> ArtifactPlan -> WriteContract
-> ModelAuthoredBatch -> DocumentAudit -> ArtifactAudit
-> WeakPathCursor -> MoreWriteContracts -> Verification -> CompletionGate
```

Prompt-visible scaffold writers are not live tools. `artifact.next` is
non-mutating and returns write contracts, not body prose. Missing roots become
root identity write contracts and force `fs.batch_write`; they do not repeat
same-root `doc.audit` before write progress. `fs.batch_write` mutates only
after contract validation. Audit-owned evidence comes from `doc.audit` and
`artifact.audit`, not direct `graph.evidence`.

## Compaction Contract

Context compaction can happen at state boundaries, not only at token thresholds.
A compaction snapshot preserves mission, artifact id, root, weak cursor, latest
audit, recovery route, provider anomaly budget, completion blockers, and the
next action surface.

## Verification Evidence

Fresh pre-change ground truth is committed under
`tmp/runtime-smoke-ground-truth-20260629T051817Z/` and summarized in
[execution/current-work/runtime-smoke-ground-truth.md](execution/current-work/runtime-smoke-ground-truth.md).
It proves that `Compact Compass` still false-closed before the sweep and that
`iwanna` still degraded to `stories/novel-named` with noisy recovery.

The current implementation has focused tests for compact-title classification,
owner-title roots, non-ASCII title identity, counted story scale, dense runtime
rows, total resolver plans, deterministic audit effects, prompt/admission
fingerprints, typed completion input packets, durable missing-root repair,
`candidate_action=artifact.audit`, exact root-repair admission, story
scale-readiness refusal, readiness repair routing, and line-protocol recovery.
Workspace tests, benchmark, quiet verify, and Docker verify pass. Final smoke at
`tmp/runtime-smoke-final-iwanna-20260629T131603Z/` and
`tmp/runtime-smoke-final-compact-20260629T134111Z/` proves both named long-novel
routes preserve owner roots, avoid generic roots, reach story-semantic readiness,
and close through `agent.done` without the observed noisy loop.

## Known Story Manuscript Gap

Live runs for an English high-school romance novella named `The Bell Rings
Twice` created `stories/bell-rings-twice` and story-bible files, but not the
requested chapter manuscript files. The strongest run under
`/tmp/lkjagent-high-school-romance-long-20260630T072532Z` reached only
`english_word_count 741` after continuation guidance and still reported
`check stories/bell-rings-twice/manuscript/chapter-one.md False`. Direct
single-chapter requests under `/tmp/lkjagent-romance-chapter-test-20260630T073614Z`
and `/tmp/lkjagent-romance-chapter-prequeued-20260630T074037Z` created
`structured-output/README.md` instead of the requested story path. The gap is
manuscript-scale classification, cursoring, and readiness, not root identity.

## Active Target

The active row in [execution/current-blockers.md](execution/current-blockers.md)
is story manuscript generation gap. Next executable step: follow
[execution/tasks/story-manuscript-generation-gap.md](execution/tasks/story-manuscript-generation-gap.md).

## Out of Scope

Messaging channels, web UI, MCP, runtime sub-agents, heartbeat schedules, and
cron schedules remain outside this product.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates exist.
- Checked-in run logs can be failure fixtures without proving current success.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
