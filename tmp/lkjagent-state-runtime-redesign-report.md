# lkjagent State Runtime Redesign Report

## Purpose

This temporary report gives the coding agent a concrete execution plan for moving
`lkjsxc/lkjagent` from a fixed plan-step engine toward a durable state-ledger
runtime with state-derived tool affordances and clean context assembly.

The detailed ZIP package was generated separately as
`tmp/lkjagent-state-runtime-redesign-report.zip`. If a local checkout contains
that ZIP, extract and read it first. If the ZIP is absent, this Markdown file is
the authoritative fallback.

## Current Ground Truth

The current product is a single-owner, single-daemon, local-LLM agent that turns
owner messages into typed plans, executes ordered steps, verifies deterministic
checks, and reports honestly.

The current docs and code intentionally narrow the model surface. Task states are
`open`, `waiting`, `blocked`, and `closed`. Step kinds are `plan`, `write`,
`revise`, `explore`, `verify`, `respond`, and `ask`. Explore has a fixed registry
of ten tools. The parser and app dispatcher both hardcode this registry.

The current strengths must survive the redesign: no fake success, no model-owned
completion, durable evidence, bounded prompts, pure core logic, effectful edges,
Docker Compose verification, file line limits, and direct documentation contracts.

The current blockers for the owner target are structural. A fixed task-state enum
cannot represent arbitrary concurrent states. A fixed step-kind selector cannot
express a rich active state vector. A hardcoded explore registry cannot flex tool
admission by state, evidence need, budget, and recovery mode. Prompt rendering
cannot guarantee that the visible tool list exactly equals the harness admission
view unless both are derived from the same persisted decision.

## Target Architecture

Replace plan-ledger authority with state-ledger authority. The plan can remain as
one state family, but it must stop being the only control plane.

The runtime loop should become:

```text
Durable rows -> RuntimeSnapshot
RuntimeSnapshot + RuntimeEvent -> RuntimeDecision
RuntimeDecision -> PromptFrame or EffectCommand
PromptFrame + RuntimeDecision -> ModelCall
ModelAction + RuntimeDecision -> ToolAdmission
ToolAdmission -> EffectCommand
EffectObservation -> RuntimeEvent
RuntimeEvent -> durable rows
```

Persist `RuntimeDecision` before prompt rendering, endpoint calls, action
admission, tool execution, recovery, compaction, or completion. The decision id
and fingerprint must appear in prompt frames, admissions, observations, provider
exchanges, logs, status output, and proof bundles.

## State Vector

Represent runtime state as a map keyed by stable `StateKey`, not as one enum.
`StateKey` should be a namespace plus name, for example `task:active`,
`plan:frontier`, `tool:fs-read-needed`, `context:conflict-open`, or
`recovery:after-parse-fault`.

Each `StateCell` should contain key, status, priority, confidence, payload schema,
payload JSON, evidence refs, source event id, created and updated timestamps,
optional expiry, optional cooldown, and optional conflict group.

Known states may have typed helper modules. Unknown state keys must still hydrate,
round-trip through storage, appear in diagnostics, and avoid breaking the reducer.
Never require a central enum edit merely to preserve a new state cell.

The reducer is pure. It consumes a `RuntimeSnapshot` and one event, returns state
patches plus a decision proposal, and never performs IO. SQLite, filesystem,
shell, clocks, and endpoint calls remain at the edge.

## RuntimeDecision

A decision is the per-turn authority record. It freezes selected case, active
state keys, allowed output grammar, tool view fingerprint, context selection
fingerprint, budget caps, retry frame, evidence requirements, and intended next
effect.

Prompt rendering must not recompute policy independently. It must render from the
decision. Tool admission must not recompute a different policy. It must validate
against the decision's stored `ToolSetView` fingerprint and action contract.

Crash resume must first look for an unfinished persisted decision. If a decision
was persisted before an endpoint call but no exchange/admission/observation was
committed, resume must either safely retry the same decision or mark a bounded
recovery event. It must not silently render a different prompt for the same turn.

## Tool Affordances

Create one tool catalog. Each tool descriptor should include name, purpose,
parameter schema, required parameters, optional parameters, observation bound,
effect class, workspace path requirements, default timeout, and safety notes.

Derive a `ToolSetView` by applying policy layers in this order: global safety,
owner configuration, workspace boundary, active state affordances, case/task
constraints, retry suppressors, evidence requirements, and recovery constraints.

Render only the resulting view. Admit only actions matching the same view. A tool
that the harness would reject for the turn must be absent from the prompt. A tool
shown in the prompt but rejected by admission is a high-priority bug.

The parser should validate syntax and parameter shape against the decision's
rendered contracts. It should not know a global hardcoded list of legal tools.
Unknown tools are unknown relative to the decision, not relative to a hidden
runtime registry.

## Context Hygiene

Prompts must be assembled from `ContextItem` rows, not transcript replay. Every
item needs source ref, semantic key, text, fingerprint, trust class, staleness,
contamination flag, evidence refs, and budget cost.

Normal prompts exclude contaminated items. Contaminated sources include failed
model bodies, refused actions, stale logs, obsolete plans, raw tool dumps,
provider anomalies, and unverified claims. Recovery prompts may include bounded
fault summaries, but never unbounded failed output.

Before rendering, group context items by semantic key and detect contradictions.
If two active items conflict, do not render both as normal facts. Render an
explicit unresolved conflict with source refs and require a resolving state edge
before ordinary work continues.

Context selection must be deterministic and budgeted. It should prefer current
evidence, owner objective, active state payloads, task brief, accepted memory,
latest bounded observation, and required artifact tails. It should demote stale
or low-trust items.

## Store Shape

Use a fresh schema if that is simpler. Backward compatibility is not required.
Minimum tables: `queue`, `cases`, `events`, `state_cells`, `state_history`,
`decisions`, `prompt_frames`, `tool_admissions`, `observations`, `context_items`,
`context_edges`, `artifacts`, `check_results`, `provider_exchanges`,
`token_usage`, `memory`, and `config`.

Rows own truth. Exchange files may own large request/response bodies, but durable
rows own resumable facts. Store nullable provider usage as unknown, never zero.

Every transaction that advances work should commit events, state patches,
admissions, observations, checks, usage, and decision settlement together. A
crash must not create false completion or replay stale prompt context.

## Crate Ownership

Prefer this crate split unless the checkout already has an equivalent with better
ownership:

- `lkjagent-core`: state vector, events, reducer, selectors, decisions, checks.
- `lkjagent-protocol`: envelopes, parser, parse faults, action structures.
- `lkjagent-tools`: descriptors, policies, tool views, admissions.
- `lkjagent-context`: context items, contradictions, contamination, prompt frames.
- `lkjagent-store`: schema, rows, hydration, transactions, recovery.
- `lkjagent-effects`: filesystem, shell, checks, observations, exchange files.
- `lkjagent-runtime`: daemon loop, endpoint orchestration, effect dispatch.
- `lkjagent-cli`: owner commands and status renderers.
- `lkjagent-llm`: provider-neutral endpoint client.
- `lkjagent-xtask`: audits, replay, corpus checks, proof bundles.

Keep source files below 200 lines. Split by ownership, not by arbitrary size.

## Documentation Tasks

Update docs first. `docs/current-state.md` must state that the current checkout is
implemented but does not satisfy the new owner target. Replace rules and decision
records that ban graph/state authority, admission matrices, runtime modes, or a
broader tool policy when those bans block the state-ledger target.

Add docs for state runtime, runtime decisions, tool catalog, tool admission,
context items, contradiction handling, contamination, schema, recovery,
observability, and proof bundles. Each docs directory must have one README table
of contents and at least two children.

Avoid release-number wording and compatibility framing. Do not write placeholders.
Every documented behavior needs an implementation owner and an acceptance test.

## Implementation Slices

First slice: document the new contract and update agent instructions so the
coding agent is not trapped by old prohibitions.

Second slice: add pure types for `StateKey`, `StateCell`, `RuntimeEvent`,
`RuntimeSnapshot`, `RuntimeDecision`, `ToolDescriptor`, `ToolSetView`,
`ContextItem`, and `ContextPack`. Add serialization and fingerprint tests.

Third slice: implement pure reducers and selectors. Prove arbitrary state counts,
unknown state keys, deterministic transitions, and completion refusal without
fresh evidence.

Fourth slice: implement tool catalog and policy layers. Prove rendered tools and
admitted tools are exactly the same view.

Fifth slice: implement context selection, contamination exclusion, contradiction
detection, and prompt rendering from persisted decisions.

Sixth slice: replace store schema and hydration. Persist decisions before calls
and reuse or settle them on crash resume.

Seventh slice: move runtime orchestration out of the old app loop. Wire effects,
endpoint calls, admissions, observations, and transactions through decisions.

Eighth slice: update CLI/status/proof bundle surfaces to expose active states,
decisions, tool views, context conflicts, contamination exclusions, and evidence.

Ninth slice: remove obsolete plan-only paths after replacements pass focused and
Docker Compose verification.

## Focused Tests

Add tests before broad rewrites where possible:

- unknown state keys hydrate and round-trip without enum edits;
- state transitions are pure and deterministic;
- decision fingerprints change when state, context, or tool view changes;
- prompt-rendered tools exactly match admission-accepted tools;
- unavailable tools are absent from prompt frames;
- refused actions become bounded events, not prompt pollution;
- contaminated context is excluded from normal prompts;
- contradictions render as unresolved conflicts, not simultaneous facts;
- completion cannot close without current evidence rows;
- crash resume reuses or settles persisted decisions honestly;
- workspace path guards cannot be bypassed by flexible tool policy;
- Docker Compose verify, test, lint, replay, and benchmark gates pass when claimed.

## Acceptance

The target is reached when a coding agent can add a new state key and policy rule
without editing a central state enum, can prove the LLM sees only tools accepted
by admission for that decision, and can show that context conflicts and polluted
items are handled before prompt rendering.

Do not claim success from model prose. Close work only with current tests and
proof bundle evidence.
