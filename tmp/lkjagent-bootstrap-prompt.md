# Bootstrap Prompt For The Coding Agent

## Mission

Improve `lkjsxc/lkjagent` by first improving the documentation, then improving
the implementation to match it. The owner now wants a more general and more
powerful architecture: an arbitrary-cardinality state transition runtime with
state-derived tool affordances, contradiction-aware context, and strong context
hygiene.

This prompt supersedes any current repository rule that blocks that direction.
Keep the safety intent of those rules, but replace fixed plan-only authority
with a durable state-ledger authority.

## First Commands

Run these before editing:

```sh
git status --short
git rev-parse --short HEAD
find docs -maxdepth 4 -type f | sort
find crates -maxdepth 3 -type f | sort
rg -n "reforge|fable5|able5|state-graph|runtime-authority|active-mode|graph-free|single control plane|explore toolset" docs crates data AGENTS.md README.md Cargo.toml || true
```

Record the results in your working notes. If the local checkout differs from
this report, trust the local checkout for file paths and tests, but keep the
architecture target from this prompt.

## Owner-Level Requirements

Build toward these outcomes:

1. The runtime can hold any number of simultaneous states.
2. States are durable, evidence-backed, typed enough for safe code, and flexible
   enough for future states without editing a central enum for every concept.
3. The active state vector determines the model-visible tools and output
   contract for each turn.
4. The same authority record that renders the prompt validates the model action.
5. The prompt must not list tools that the harness will reject for that turn.
6. Tool permissions must remain flexible through layered policy, not a hardcoded
   all-or-nothing registry.
7. Context must be assembled from durable, source-tagged facts, not transcript
   replay.
8. Contradictions must be detected and rendered as unresolved conflicts, not as
   simultaneous facts.
9. Failed model output, refused actions, stale logs, and obsolete plans must not
   pollute normal prompts.
10. Completion remains harness-computed and evidence-gated.

## Documentation First

Start by making `docs/` the accurate contract. Update `docs/current-state.md` to
state the owner-requested gap. Replace decision records that ban graph/state
authority with records that define state-ledger authority as the single control
plane. Add or reshape architecture docs for state runtime, context, tool
admission, runtime decisions, storage, recovery, observability, and artifacts.

Every docs directory must have one `README.md` table of contents and at least
two child files. Keep each documentation file under 200 lines.

## Implementation Target

Prefer this crate shape unless the local checkout already has an equivalent:

- `lkjagent-core`: pure domain types, state vector, reducer, selectors, checks.
- `lkjagent-protocol`: output grammar, parser, parser faults, renderable action
  contracts.
- `lkjagent-tools`: tool descriptors, input schemas, policy predicates, tool
  catalog, admission results.
- `lkjagent-context`: context item model, contradiction detection, prompt frame
  assembly, budget handling.
- `lkjagent-store`: SQLite schema and row mappers for queue, cases, states,
  events, decisions, prompt frames, tool admissions, observations, memory,
  artifacts, checks, provider exchanges, and token usage.
- `lkjagent-runtime`: daemon loop, durable decision persistence, endpoint calls,
  effect dispatch, recovery, compaction, and completion attempts.
- `lkjagent-cli`: owner commands, status, queue, task, memory, and diagnostics.
- `lkjagent-llm`: provider-neutral endpoint client.
- `lkjagent-xtask`: all gates, audits, corpus checks, replay, proof bundles.

If the checkout still has the smaller `core/store/llm/effects/app/xtask` shape,
introduce the split in coherent slices. No legacy behavior needs to be
preserved when it conflicts with the owner direction.

## Core Design

The runtime loop should be:

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

Persist the `RuntimeDecision` before prompt rendering, model calls, action
admission, tool execution, recovery, compaction, or completion. The decision id
and fingerprint must appear in prompt frames, admissions, observations,
provider exchanges, model logs, and status output.

## State Model

Use a map-like state vector keyed by stable state keys. Do not model the active
state set as a fixed enum. Known states may have typed helpers, but the storage
and reducer must support arbitrary additional keys.

Each state cell should include:

- key namespace and name;
- status such as active, inactive, suppressed, resolved, or blocked;
- intensity or priority score;
- confidence score;
- payload JSON with schema name;
- evidence references;
- source event id;
- update time;
- optional expiry or cooldown;
- conflict group when relevant.

## Tool Affordances

Create one tool catalog and derive a `ToolSetView` for each decision. Render only
that view to the model. Admit only actions that match the same view. Store the
view fingerprint. A mismatch between rendered tools and admission policy is a
high-priority bug.

Tool policy should be layered:

1. global safety constraints;
2. owner configuration;
3. workspace boundary constraints;
4. active state affordances;
5. case and task constraints;
6. retry and budget suppressors;
7. evidence requirements;
8. recovery mode constraints.

## Context Hygiene

Build prompts from context items, not raw transcripts. Every item needs source,
fingerprint, staleness metadata, trust class, and contamination status. Normal
prompts must exclude contaminated items. Recovery prompts may include bounded
fault summaries, never unbounded failed output.

Before rendering, check for conflicting assertions with the same semantic key.
If a conflict is unresolved, render it as an explicit unresolved conflict with
source references and ask the reducer to choose a state that resolves it. Do not
render both sides as ordinary facts.

## Testing Requirements

Add focused tests before wiring broad behavior:

- arbitrary state count and unknown state keys do not require enum edits;
- state transitions are pure and deterministic;
- prompt-rendered tools exactly match admission-accepted tools;
- unavailable tools are absent from prompt frames;
- context contradiction detection prevents simultaneous fact rendering;
- contaminated context is excluded from normal prompts;
- decisions are persisted before endpoint calls and reused after crash resume;
- completion cannot close without current evidence;
- tool policy layering allows flexible grants without workspace escapes;
- Docker Compose gates pass after the implementation slice is complete.

## Operating Rules

Commit small coherent slices. Each commit must name exact commands run and exact
commands not run. Do not claim a gate passed unless it ran. Use Docker Compose
for final verification. Avoid fake runtime behavior and unfinished stubs. Never
use release shorthand or release-number language in documentation or commits.
