# Bootstrap Prompt For The Coding Agent

## Mission

Work on `lkjsxc/lkjagent`. First improve the documentation. Then improve the
implementation so it matches the new documented contract.

The owner wants the project to move toward a durable state-ledger runtime. The
runtime should support any number of active state cells, derive model-visible tool
affordances from those states, keep prompt context clean, and prevent conflicts
between what the model sees and what the harness accepts.

Read `tmp/lkjagent-state-runtime-redesign-report.md` before editing. If
`tmp/lkjagent-state-runtime-redesign-report.zip` exists in the checkout, extract
it and read it after the Markdown report.

## First Commands

```sh
git status --short
git rev-parse --short HEAD
find docs -maxdepth 4 -type f | sort
find crates -maxdepth 4 -type f | sort
find tmp -maxdepth 2 -type f | sort
rg -n "reforge|fable5|able5|state-graph|runtime-authority|active-mode|graph-free|single control plane|explore toolset|admission" docs crates data AGENTS.md README.md Cargo.toml tmp || true
```

Record the results in working notes. Trust the local checkout for exact paths and
test commands. Trust this prompt and the temporary report for the owner target.

## Read Order

1. `docs/current-state.md`
2. `AGENTS.md`
3. `docs/vision/README.md`
4. `docs/product/README.md`
5. `docs/engine/README.md`
6. `docs/context/README.md`
7. `docs/tools/README.md`
8. `tmp/lkjagent-state-runtime-redesign-report.md`
9. `tmp/lkjagent-state-runtime-redesign-report.zip`, when present

## Owner Target

Build toward these outcomes:

1. The runtime can hold arbitrary active state cells.
2. State is durable, evidence-backed, and open to future state keys.
3. The active state vector determines output grammar and available tools.
4. A persisted `RuntimeDecision` governs prompt rendering and action admission.
5. A tool rejected by the harness for a turn is not shown to the model.
6. Tool permissions are flexible through layered policy.
7. Context comes from durable source-tagged facts, not transcript replay.
8. Contradictions render as unresolved conflicts until resolved.
9. Contaminated context is excluded from normal prompts.
10. Completion remains harness-computed and evidence-gated.

## Documentation Work

Update `docs/current-state.md` first. State that the current checkout is proven
for the existing plan engine but does not yet satisfy the new state-ledger target.

Then adjust the docs and decision records so they describe state-ledger authority,
runtime decisions, tool admission, context items, contradiction handling,
contamination handling, schema, recovery, observability, and proof bundles.

Every docs directory needs one `README.md` table of contents and at least two
children. Keep documentation files under 200 lines. Avoid release-number wording
and compatibility framing.

## Implementation Direction

Prefer this ownership unless the local checkout already has a cleaner split:

- `lkjagent-core`: state vector, events, reducer, selectors, decisions, checks.
- `lkjagent-protocol`: envelopes, parser, parse faults, action structures.
- `lkjagent-tools`: descriptors, policy layers, tool views, admissions.
- `lkjagent-context`: context items, contradictions, contamination, prompt frames.
- `lkjagent-store`: schema, rows, hydration, transactions, recovery.
- `lkjagent-effects`: filesystem, shell, checks, observations, exchange files.
- `lkjagent-runtime`: daemon loop, endpoint orchestration, effect dispatch.
- `lkjagent-cli`: owner commands and renderers.
- `lkjagent-llm`: provider-neutral endpoint client.
- `lkjagent-xtask`: gates, audits, corpus checks, replay, proof bundles.

Keep source files under 200 lines. Split by ownership.

## Runtime Shape

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
admission, tool execution, recovery, compaction, or completion. Carry the
decision id and fingerprint through prompt frames, admissions, observations,
provider exchanges, model logs, status output, and proof bundles.

## State Model

Use a map-like state vector keyed by stable state keys. Known states may have
typed helpers, but storage, hydration, reducer logic, and diagnostics must support
unknown keys.

Each state cell should include key namespace and name, status, priority,
confidence, payload schema, payload JSON, evidence refs, source event id, update
time, optional expiry or cooldown, and optional conflict group.

## Tool Affordances

Create one tool catalog and derive a `ToolSetView` for each decision. Render only
that view to the model. Admit only actions that match the same persisted view.

Layer tool policy through global safety, owner configuration, workspace boundary,
active state affordances, case constraints, retry suppressors, evidence needs,
and recovery constraints.

## Context Hygiene

Build prompts from context items. Every item needs source, fingerprint, trust
class, staleness, and contamination status. Normal prompts exclude contaminated
items. Recovery prompts may include bounded fault summaries, never unbounded
failed output.

Before rendering, detect conflicting assertions with the same semantic key. If a
conflict is unresolved, render it as an explicit unresolved conflict with source
refs and let the reducer choose a resolving operation.

## Tests Required

Add focused tests for arbitrary state count, unknown state keys, deterministic
state transitions, decision fingerprint stability, prompt-rendered tools matching
admission, unavailable tools absent from prompts, contradiction handling,
contamination exclusion, persisted decisions after crash resume, fresh evidence
for completion, and flexible tool policy without workspace escapes.

## Operating Rules

Commit small coherent slices. Each commit must name exact commands run and exact
commands not run. Do not claim a gate passed unless it ran. Use Docker Compose
for final verification. Avoid fake runtime behavior, unfinished stubs, and
shortcuts.
