# lkjagent State Runtime Redesign Report

## Purpose

This temporary report gives the coding agent a concrete plan for moving
`lkjsxc/lkjagent` from a fixed plan-step engine toward a durable state-ledger
runtime with state-derived tool affordances and clean context assembly.

A deeper ZIP package was generated separately as
`tmp/lkjagent-state-runtime-redesign-report.zip`. If the ZIP exists in the local
checkout, extract and read it after this file. If it is absent, this Markdown file
is the repository-readable fallback.

## Current Ground Truth

The current product is a single-owner, single-daemon, local-LLM agent. Owner
messages become typed plans, ordered steps, deterministic effects, checks, and
honest reports.

The current model surface is intentionally narrow. Task states are `open`,
`waiting`, `blocked`, and `closed`. Step kinds are `plan`, `write`, `revise`,
`explore`, `verify`, `respond`, and `ask`. Explore has ten tools. The parser and
app dispatcher hardcode that registry.

Keep the strengths: no fake success, no model-owned completion, durable evidence,
bounded prompts, pure core logic, effectful edges, Docker Compose verification,
file line limits, and direct documentation contracts.

The blockers are structural. A fixed task-state enum cannot express arbitrary
concurrent states. A fixed step-kind selector cannot express a rich active state
vector. A hardcoded explore registry cannot flex by state, evidence need, budget,
and recovery mode. Prompt rendering cannot guarantee tool availability unless it
and admission read the same persisted decision.

## Target Architecture

Replace plan-ledger authority with state-ledger authority. The plan can remain as
one state family, but it must stop being the only control plane.

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

Known states may have typed helper modules. Unknown state keys must hydrate,
round-trip through storage, appear in diagnostics, and avoid breaking the reducer.
Never require a central enum edit merely to preserve a new state cell.

The reducer is pure. It consumes a `RuntimeSnapshot` and one event, returns state
patches plus a decision proposal, and never performs IO. SQLite, filesystem,
shell, clocks, and endpoint calls remain at the edge.

## RuntimeDecision

A decision is the per-turn authority record. It freezes selected case, active
state keys, allowed output grammar, tool view fingerprint, context selection
fingerprint, budget caps, retry frame, evidence requirements, and next effect.

Prompt rendering must render from the decision. Tool admission must validate
against the decision's stored `ToolSetView` fingerprint and action contract. A
prompt/admission mismatch is a high-priority bug.

Crash resume must first inspect unfinished persisted decisions. If a decision was
persisted before an endpoint call but no exchange, admission, or observation was
committed, resume must either retry the same decision or commit a bounded recovery
event. It must not silently render a different prompt for the same turn.

## Tool Affordances

Create one tool catalog. Each descriptor should include name, purpose, parameter
schema, required and optional parameters, observation bound, effect class,
workspace path requirements, timeout, and safety notes.

Derive a `ToolSetView` by applying policy layers: global safety, owner
configuration, workspace boundary, active state affordances, case constraints,
retry suppressors, evidence requirements, and recovery constraints.

Render only the resulting view. Admit only actions matching the same view. A tool
that the harness would reject for the turn must be absent from the prompt.

The parser should validate syntax and parameter shape against the decision's
rendered contracts. It should not know a global hardcoded list of legal tools.
Unknown tools are unknown relative to the decision, not to hidden policy.

## Context Hygiene

Prompts must be assembled from `ContextItem` rows, not transcript replay. Every
item needs source ref, semantic key, text, fingerprint, trust class, staleness,
contamination flag, evidence refs, and budget cost.

Normal prompts exclude contaminated items. Contaminated sources include failed
model bodies, refused actions, stale logs, obsolete plans, raw tool dumps,
provider anomalies, and unverified claims. Recovery prompts may include bounded
fault summaries, never unbounded failed output.

Before rendering, group context items by semantic key and detect contradictions.
If two active items conflict, render an explicit unresolved conflict with source
refs and require a resolving state edge before ordinary work continues.

Context selection must be deterministic and budgeted. Prefer current evidence,
owner objective, active state payloads, task brief, accepted memory, latest
bounded observation, and required artifact tails. Demote stale or low-trust items.

## Store Shape

Use a fresh schema if simpler. Backward compatibility is not required. Minimum
tables: `queue`, `cases`, `events`, `state_cells`, `state_history`, `decisions`,
`prompt_frames`, `tool_admissions`, `observations`, `context_items`,
`context_edges`, `artifacts`, `check_results`, `provider_exchanges`,
`token_usage`, `memory`, and `config`.

Rows own truth. Exchange files may own large request and response bodies, but
durable rows own resumable facts. Nullable provider usage means unknown, not zero.
Each turn transaction should commit events, state patches, admissions,
observations, checks, usage, and decision settlement together.

## Crate Ownership

Prefer this ownership unless the checkout has a cleaner equivalent:

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

Keep source files below 200 lines. Split by ownership, not arbitrary size.

## Documentation Tasks

Update docs first. `docs/current-state.md` must say the current checkout is
implemented for the existing plan engine but does not satisfy the new owner
target. Adjust rules and decisions that block state-ledger authority, runtime
decisions, tool admission, context items, contradictions, contamination, schema,
recovery, observability, or proof bundles.

Every docs directory must have one README table of contents and at least two
children. Avoid release-number wording and compatibility framing. Do not write
placeholders. Every documented behavior needs an implementation owner and test.

## Implementation Slices

1. Document the new contract and update agent instructions.
2. Add pure types for states, events, decisions, tools, and context.
3. Implement reducers and selectors with arbitrary state keys.
4. Implement the tool catalog, policy layers, views, and admissions.
5. Implement context selection, contamination, conflicts, and prompt frames.
6. Replace store schema, hydration, decision persistence, and crash resume.
7. Wire runtime orchestration through decisions, admissions, and observations.
8. Update CLI, status, logs, proof bundles, and diagnostics.
9. Remove obsolete plan-only paths after replacements pass verification.

## Focused Tests

Add tests for unknown state keys, deterministic transitions, decision fingerprint
stability, rendered tools matching admitted tools, unavailable tools absent from
prompts, refused actions becoming bounded events, contaminated context exclusion,
contradictions rendering as conflicts, completion refusing stale evidence, crash
resume settling decisions, workspace guards, and Docker Compose gates.

## Acceptance

The target is reached when a coding agent can add a state key and policy rule
without editing a central state enum, prove the LLM sees only admitted tools, and
show that conflicts and polluted items are handled before prompt rendering.

Do not claim success from model prose. Close work only with current tests and
proof-bundle evidence.
