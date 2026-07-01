# Current State

## Purpose

This file is the honest ledger of lkjagent. It states what exists, what is
partial, and what is open. A behavior is implemented only when code, focused
tests, quiet gates, and required Docker gates prove it.

## Summary

lkjagent has a working Rust workspace with parser, protocol registry, graph,
context, store, LLM, tools, runtime, CLI, benchmark, and xtask crates. The
persisted-decision daemon path records dense facts, obligations, resolver
plans, progress edges, deterministic effects, and typed completion gate inputs.

The current redesign target is a generic large-artifact engine: a single owner
objective becomes durable plans, atom graphs, exact-path write contracts,
real-file audit, deterministic assembly, readiness projection, and completion
gates for long structured work products.

The durable runtime shape remains:

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
completion, write contracts, deterministic effects, and the next action
surface. Model output supplies only bounded semantic intent or bounded file
content inside that selected surface.

## Proven Surfaces

| Area | Evidence |
| --- | --- |
| Workspace and gates | `Cargo.toml`, `crates/lkjagent-xtask`, and `docker-compose.yml` exist. |
| Parser | `lkjagent-protocol` parses singular action turns and emits structured faults. |
| Registry | `lkjagent-tools` validates tools, parameters, and required-any groups. |
| Graph | `lkjagent-graph` stores typed cases, evidence requirements, tracks, and transitions. |
| Store | Queue, state, event, memory, authority, prompt-frame, observation, artifact, compaction, and provider-exchange surfaces exist. |
| Runtime kernel | Snapshot, event, decision, admission, effect, render, fault, provider, adapter, reducer, and driver records exist. |
| Endpoint loop | Provider calls record model-log files, token usage when present, anomalies, and bounded retry facts. |
| Token accounting | Endpoint usage rows aggregate into latest, task, session, and all-time scopes with unknown counts for status and console. |
| CLI core | Metadata-rendered help, group help, `--data` before or after commands, `watch` and `console`, task inspection, queue inspection, and shared status-console decks are implemented. |
| Resolver table | Named total resolver rules select rule ids, plans, blocked handoffs, and progress keys without a resolver fallback module. |
| Artifact lifecycle | Artifact plan, audit, next, cursors, weak paths, invalid roots, durable atom graphs, write contracts, readiness projection, story readiness, and completion refusals are ledger-backed. |
| Maintenance | Maintenance gates, owner preemption checks, no-op cooldown facts, and closed-idle rules have focused coverage. |
| Benchmarks | Owner recovery, artifact, memory, accounting, model-log, batch-schema, compaction, repeated-recovery, and novel signatures are in the corpus. |

## Partial Or Open Surfaces

| Area | Current boundary |
| --- | --- |
| CLI UX | The parser, command tree, and shared status-console deck are implemented. Further UX hardening follows resolver and content evidence needs. |
| Live endpoint proof | Full live 10,000-word daemon completion remains operator-driven evidence. Store-backed planning, atom selection, audit, assembly, and completion refusal are implemented. |
| Smoke harness | Deterministic replay and explicit live smoke skip commands exist; fresh live endpoint completion remains operator-driven evidence. |

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
- duplicate `settings.md` `fs.batch_write` attempts did not create root
  identity;
- repeat recovery and `graph.recover` changed shape but routed back to
  same-root `doc.audit`;
- `graph.state` showed active case `1` while recovery remained open;
- reasoning-only provider responses were recorded as provider anomalies;
- document audit, artifact readiness audit, and final verification remained
  pending.

The checked-in generated log fixture remains historical failure evidence.

## Runtime Authority Target Flow

The decision is persisted before prompt rendering, endpoint calls, dispatch,
recovery, compaction, maintenance, or close attempts. Prompt frames, provider
exchange rows, pending actions, admissions, observations, model-log exports,
and status expose the same authority ids and staleness fingerprints.

Live output is one singular tag action only. Endpoint requests set
`reasoning_effort=none` so the provider spends output budget on final action
text, not hidden thinking tokens. Top-level JSON, top-level line-action syntax,
nested file objects, object-literal batches, `<actions>`, and `<think>` output
are refused. `fs.batch_write` accepts line protocol only inside `<files>`.

## Artifact Contract

The artifact lifecycle is:

```text
OwnerObjective -> ObjectiveFrame -> ArtifactPlan -> ArtifactAtomGraph
-> NextAtomSelection -> WriteContract -> ModelAuthoredContent
-> ContractValidation -> AtomAudit -> DeterministicAssembly
-> ReadinessProjection -> CompletionGate
```

Prompt-visible scaffold writers are not live tools. `artifact.next` is
non-mutating and returns atom write contracts, not body prose. Missing roots
become root identity write contracts and force `fs.batch_write`; they do not
repeat same-root `doc.audit` before write progress. `fs.batch_write` mutates
artifact paths only after active contract validation. Audit-owned evidence
comes from `doc.audit` and `artifact.audit`, not direct `graph.evidence`.

## Story Manuscript Evidence

Story manuscript work has typed chapter-prose facts separate from story-bible
reference files. Exact `stories/.../manuscript/*.md` requests route to story
content artifacts, counted-document scaffolding is vetoed for manuscript prose
requests, runtime facts carry manuscript target words, chapter counts, missing
paths, scene atom gaps, next write path, progress words, and provider anomaly
shrink state, and `artifact.next` chooses manuscript scene or chapter contracts
before optional story-bible repair after identity exists.

Readiness and completion reject story-bible-only output for manuscript tasks.
Provider anomaly and endpoint max-token recovery preserve the exact next
manuscript path and shrink the write contract instead of broadening admission or
blocking with a generic handoff. Historical endpoint smoke at
`/tmp/lkjagent-manuscript-direct-20260630T095705Z` proved the direct chapter
route avoided `structured-output` but stopped at a waiting handoff before the
current shrink-only recovery rule.

A follow-up live owner request for `Second Period, First Love` proved the
current boundary. After `reasoning_effort=none`, the daemon created the exact
root, README, and story-bible files without `structured-output`, but timed out
in recovery after only two short chapter files and 508 manuscript words. A
direct per-chapter endpoint fallback generated ten chapters and 11,456
manuscript words at `/tmp/lkjagent-user-romance-complete-20260630T122402Z`.
That artifact proves the model can write the prose, not that the daemon can
complete the task. The open target is live proof that the daemon-owned,
resumable scene and chapter write surface can finish the full manuscript with
real word-count evidence, deterministic assembly where needed, and central
completion closure.

## Verification Evidence

Fresh pre-change ground truth is committed under
`tmp/runtime-smoke-ground-truth-20260629T051817Z/` and summarized in
[execution/current-work/runtime-smoke-ground-truth.md](execution/current-work/runtime-smoke-ground-truth.md).
It proves that `Compact Compass` false-closed before the sweep and that
`iwanna` degraded to `stories/novel-named` with noisy recovery.

Later smoke at `tmp/runtime-smoke-final-iwanna-20260629T131603Z/` and
`tmp/runtime-smoke-final-compact-20260629T134111Z/` proves both named long-novel
routes preserve owner roots, avoid generic roots, reach story-semantic
readiness, and close through `agent.done` without the observed noisy loop. That
evidence does not prove full 10,000-word daemon manuscript completion.

## Redesign Queue Status

The executable redesign queue in
[execution/current-blockers.md](execution/current-blockers.md) tracks the
large-artifact engine work. Remaining product risk is live endpoint completion
of a full 10,000-word daemon manuscript, which is not claimed here.

## Out of Scope

Messaging channels, web UI, MCP, runtime sub-agents, heartbeat schedules, and
cron schedules remain outside this product.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates exist.
- Checked-in run logs can be failure fixtures without proving current success.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
