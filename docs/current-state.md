# Current State

## Purpose

Keep an honest ledger that separates the product contract, behavior proven in
this checkout, and open implementation gaps.

## Contract Target

lkjagent is moving from a fixed plan-ledger engine to a durable state-ledger
runtime. Owner messages become durable cases, events, state cells, runtime
decisions, prompt frames, bounded model calls, tool admissions, deterministic
effects, observations, checks, context items, exchange logs, token usage, and
proof rows. Durable rows are the single control plane.

The runtime target supports any number of active state cells. A persisted
`RuntimeDecision` is selected from the hydrated state vector before prompt
rendering, endpoint calls, tool admission, tool execution, recovery, compaction,
or completion. The decision id and fingerprint travel through prompt frames,
provider exchanges, admissions, observations, status output, and proof bundles.

Completion remains harness-computed. The model may author bounded content or
request a decision-visible operation, but a case closes only when fresh evidence
satisfies the checks selected by the runtime.

## Proven In Current Checkout

The current checkout is proven for the existing plan engine, not for the new
state-ledger target. It no longer reads `app.active-snapshot` as runtime
authority. Focused resume tests prove that config snapshots are ignored when
rows are absent and that normalized task and step rows win over stale config.
The daemon claims a heartbeat config-row lease, uses an injected clock seam for
durable timestamps, hydrates open and waiting tasks from normalized rows,
commits turn state through rows, and records waiting answers as rows before
continuing.

The parser rejects explore `<finish>` and `<ask>` envelopes, leading or trailing
prose, duplicate action parameters, unknown action parameters, and unknown tools
from the fixed explore registry. Prompt rendering tells explore steps to finish
with the `finish` action inside `<action>`, and the engine rejects adjacent
repeated explore actions before effects run. Prompt rendering includes the
bounded task brief, including admitted memory facts. Endpoint errors use the
documented ten-failure patience before blocking a step. Endpoint clients default
to a loose finite 900-second timeout unless configuration overrides it.

Queue rows persist `force_new`, send uses it, and daemon intake can select a
forced-new row without treating it as an answer. Status, task, queue, log,
memory, and watch surfaces read rows instead of a config snapshot. The bounded
explore dispatcher runs the documented filesystem, shell, memory, plan-note, and
finish actions, stores latest observations in step inputs, persists
`memory.save` rows, suppresses exact duplicate memory facts, mirrors memory into
FTS, admits bounded row-backed memory facts into new task briefs, and resolves
`memory.find` from durable rows.

Plan-authored write steps carry `words=N` into deterministic `min_words` checks.
Endpoint calls produce exchange files and structured completion records with
usage, cache metrics, provider anomalies, closure mode, timing, generated
exchange refs, and nullable token usage rows. Check results use the active step
id, store check parameters, and keep numeric measurements as scalar values.
README coverage requires links to children, and link checks cover tracked
Markdown, crate README contract links, `./`, `../`, anchors, and directory
README inference. Deterministic effect failures commit an `effect_error` attempt
and notice without marking the step done.

Focused tests cover the row-first store path, parser, CLI, explore actions,
exchange logs, token usage, check measurements, memory rows and admission,
docs-link checks, daemon clock and lease, repeat guard, endpoint patience,
plan-word checks, prompt briefs, and effect-error settlement. This checkout also
has a first pure state-ledger domain slice in `lkjagent-core`: state keys and
cells, runtime events and patches, runtime decisions, tool-set views, action
admission with workspace path policy, context items with contamination classes,
contradiction detection, stable fingerprints, and fresh-evidence completion
helpers. `lkjagent-core` also has a pure selector that picks a runtime decision
from hydrated state and reuses unfinished decisions before selecting new work.
`lkjagent-store` creates the first state-ledger table set beside the plan-family
rows, with row helpers for cases, unknown state cells, pending runtime
decisions, and context items. `lkjagent-app` projects plan rows into
operation-specific state cells, hydrates state cells, persists or reuses a
`RuntimeDecision` before prompt rendering, and settles the decision after the
turn. The bridge projects cells such as `model:<step>`, `check:<step>`,
`case:waiting-answer`, and `completion:close-candidate`. Explore tool
descriptors now live in one core catalog used to derive the bridge
`ToolSetView`; prompt rendering prints that persisted view, parsing reads
the same decision view, app admission rows are persisted before explore effects,
and the explore dispatcher resolves tool effects through the catalog descriptor.
The daemon bridge also persists source-tagged context items, selects
clean current items for prompt briefs, detects contradictory clean items into
`context:conflict/<semantic-key>` state cells, and excludes contaminated items
from normal prompts. The core artifact slice models checked 512-token-target
units, deterministic assembly, artifact fingerprints, and fresh-fingerprint
completion evidence; the store persists artifact rows with unit metadata, and
write effects record file and unit artifact fingerprints. Endpoint exchanges now
carry decision id, context-frame fingerprint, tool-view fingerprint, active
timeout, and provider-exchange rows tied to the
runtime decision. Status output now summarizes active state cells, conflict
cells, latest decision, admissions, observations, provider exchanges, and
artifacts. Prompt-frame rows are persisted before model calls, and observation
rows are persisted after deterministic effects. Clean observations are converted
into durable context items while error observations are marked recovery-only.
Parse-fault provider exchanges create failed-model-output context items, and
endpoint errors create recovery-only context items. Active
`context:resolve/<key>` cells suppress losing conflict items before prompt
rendering. Unfinished decisions with
committed provider exchanges, admissions, or observations are recovered before a
new decision is selected; decisions without external evidence are reused. Proof
collection writes state-ledger sections for state cells, decisions, admissions,
observations, exchanges, artifacts, and context. Status reports active or stale
daemon lease rows from heartbeat config evidence. Prompt-frame rows now point to
bounded prompt body refs under `data/logs/`. Current gate results belong in the
handoff after commands are rerun against this checkout.

## State-Ledger Gap

The checkout does not yet satisfy the owner-requested state-ledger contract. The
open gaps are executable and must be closed with docs, code, tests, and current
gate evidence:

- daemon runtime hydration is still shaped around fixed tasks, steps, and step
  kinds; state-ledger tables and row helpers exist but are not yet runtime
  authority;
- runtime selection reads operation-specific state cells projected from plan
  rows; the full reducer does not yet own all state transitions;
- persisted `RuntimeDecision` rows are created and reused by the daemon, but old
  plan rows still determine which operation-specific cells are projected;
- prompt rendering, parsing, and admission use the persisted explore
  `ToolSetView`, but non-explore prompts and full policy-layer derivation are
  still bridge-limited;
- tool descriptors are catalog-backed for prompt, parser, admission, and explore
  effect selection, but full policy-layer derivation is still bridge-limited;
- prompt context has a durable context-item bridge for clean items, conflicts,
  observations, and contaminated exclusion, and prompt-frame rows own bounded
  body refs; full prompt-frame replay is still bridge-level;
- contradictions become conflict state cells and active resolution cells
  suppress losing items in the bridge, but full resolution event lineage is not
  yet wired;
- parse faults, endpoint errors, and effect errors classify contamination
  durably, but sensitive-owner-data and external-raw classes are not fully wired;
- crash resume reuses decisions with no external evidence and recovers
  externally evidenced unfinished decisions, but recovery state reports remain
  bridge-level;
- artifact units, deterministic assembly, and fresh fingerprint checks have pure
  helpers and rows, and write effects persist file and unit artifacts, but model
  generation is not yet split into checked 512-token units before assembly; and
- proof bundles expose first state-ledger sections and context suppression
  reasons, but full conflict resolution lineage and sensitive-data policy are
  not yet complete.

## Implemented Code

`lkjagent-core` owns the first pure state-ledger domain modules plus the current
plan engine, parser, renderer, checks, word counting, classifier, templates,
docs-link helpers, and recovery helpers.
`lkjagent-store` owns the plan-store schema, first state-ledger tables, row
hydration, queue access, and atomic turn state commits. `lkjagent-effects` owns
filesystem, shell, check gathering, observations, and exchange log file helpers.
`lkjagent-app` owns the
daemon interpreter, row-backed CLI renderers, endpoint adapter, waiting answer
routing, effect-error settlement, and bounded explore dispatcher. `lkjagent-llm`
owns the endpoint wire client. `lkjagent-xtask` owns repository gates, structure
audit, deterministic replay, benchmark commands, and proof bundle collection.

## Historical Evidence

Checked-in logs under `tmp/` proof folders record previous successful gates and
live proof artifacts. They are historical evidence only. They do not prove that
the current checkout passes a gate unless that gate is rerun now.

## Next Executable Step

Next, retire plan-only authority only after state-ledger parity is proven by
replay and Docker gates; until then, keep the bridge explicit and move durable
state reducers into runtime authority one state family at a time.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates
  exist in the current checkout.
- Checked-in run logs can be failure fixtures or historical proof, not current
  gate results.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
- Never claim a gate passed without running it.
