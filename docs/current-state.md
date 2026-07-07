# Current State

## Purpose

Keep an honest ledger that separates the product contract, behavior proven in
this checkout, and open implementation gaps.

## Contract Target

lkjagent is moving from a fixed plan-ledger engine to a one-workspace durable
state-ledger runtime. Owner messages become durable cases, events, state cells,
state edges, runtime decisions, prompt frames, bounded model calls, tool
admissions, deterministic effects, observations, checks, workspace records,
context items, exchange logs, token usage, artifacts, and proof rows. Durable
rows are the single control plane.

The target supports many active cells and dense edge evidence. A selector builds
bounded candidates from state, records, artifacts, context, and stale evidence,
then persists one `RuntimeDecision` before prompts, endpoint calls, tool
admission, effects, recovery, compaction, or completion. Completion remains
harness-computed through fresh checks.

## Proven In Current Checkout

The current checkout is proven for the existing plan engine, not for the new
state-ledger target. It no longer reads `app.active-snapshot` as runtime
authority. Focused resume tests prove that config snapshots are ignored when
rows are absent and that normalized task and step rows win over stale config.
The daemon claims a heartbeat config-row lease, uses an injected clock seam for
durable timestamps, hydrates open and waiting tasks from normalized rows,
commits turn state through rows, and records waiting answers as rows before continuing.

The parser rejects explore `<finish>` and `<ask>` envelopes, leading or trailing
prose, old `<action>` envelopes, missing `<tool_name>`, duplicate or unknown
tool-call fields, and unknown tools absent from the persisted decision
`ToolSetView`. `lkjagent-core` also contains a staged JSON action parser for one
bounded action envelope with duplicate-key rejection, decision-id validation,
unknown-field rejection, and per-tool primitive argument checks; the daemon has
not yet switched active prompts to that JSON grammar. Prompt rendering labels safe filled tool
examples separately from schema-only placeholders, which parse but admission
rejects unchanged; internal `Action` domain names remain. Prompt rendering
includes the bounded task brief. Endpoint errors use the documented ten-failure
patience before blocking a step, and endpoint clients default to a loose finite
900-second timeout unless configured.

Queue rows persist `force_new`, send uses it, and daemon intake can select a
forced-new row without treating it as an answer. Status, task, queue, bounded
log, follow log, memory, watch, and console read rows instead of config; console
flushes prompt and replies directly and exposes local `/help`. The bounded
explore dispatcher runs filesystem, shell, memory, plan-note, and finish actions,
stores observations, persists `memory.save` rows, suppresses exact duplicate
memory facts, mirrors memory into FTS, admits bounded row-backed memory facts
into new task briefs, and resolves `memory.find` from durable rows.

Plan-authored write steps carry `words=N` into deterministic `min_words` checks.
Endpoint calls produce exchange files and structured completion records with
usage, cache metrics, provider anomalies, closure mode, timing, generated
exchange refs, and nullable token usage rows. Check results use the active step
id, store check parameters, and keep numeric measurements as scalar values.
README coverage requires links to children, and link checks cover tracked
Markdown, crate README contract links, `./`, `../`, anchors, and directory
README inference. Static gate collection ignores generated runtime state under
`data/`, `tmp/`, local workspace directories, lock files, and SQLite sidecars in
git-backed and plain checkouts. Deterministic effect failures commit an
`effect_error` attempt and notice without marking the step done.

Focused tests cover the row-first store path, parser, CLI, explore actions,
exchange logs, token usage, check measurements, memory rows and admission,
docs-link checks, daemon clock and lease, repeat guard, endpoint patience,
plan-word checks, prompt briefs, and effect-error settlement. This checkout also
has a first pure state-ledger domain slice in `lkjagent-core`: state keys and
cells, runtime events and patches, runtime decisions, tool-set views, action
admission with workspace path policy, context items with contamination classes,
contradiction detection, stable fingerprints, and fresh-evidence completion
helpers. `lkjagent-core` also has a pure selector that builds bounded
`SelectorCandidate` values from cells, payload operation keys, priority,
deadlines, cooldowns, and blocking edges before persisting the winner; unfinished
decisions still win first. `lkjagent-store` adds row helpers for cases, events,
state cells, decisions, context items, workspace record metadata, and record
fingerprint history. `lkjagent-app` projects plan rows
through durable runtime events into operation-specific state cells, mirrors task
snapshots into `case:snapshot` state cells, hydrates runnable snapshots from
state cells before plan rows, uses the same path for status, leaves active
operation cells as decision authority until settlement, persists or reuses a
`RuntimeDecision` before prompt rendering, derives turn work from the persisted
decision operation, and settles it after the turn. The bridge projects cells
such as `model:<step>`, `check:<step>`, `case:waiting-answer`, and
`completion:close-candidate`. Explore tool
descriptors now live in one core catalog used to derive the bridge
`ToolSetView`; prompt rendering prints that persisted view, parsing reads
the same decision view, non-explore prompt protocol follows the decision
envelope, app admission rows are persisted before explore effects, and the
explore dispatcher resolves tool effects through the catalog descriptor.
The daemon bridge also persists source-tagged context items, selects
clean current items for prompt briefs, detects contradictory clean items through
runtime events into `context:conflict/<semantic-key>` state cells, writes
contradiction and resolution `context_edges`, and excludes contaminated items
from normal prompts. The first generic state-edge slice adds pure edge refs,
relations, reducer patch operations, snapshot edge visibility, and `state_edges`
rows for relation evidence. The core artifact slice models checked 512-token-target
units, deterministic assembly, artifact fingerprints, and fresh-fingerprint
completion evidence; the store persists artifact rows with unit metadata, and
write effects split large bodies into checked units, assemble them before file
writes, and record file and unit artifact fingerprints. Write and revise prompts
use the 512-token artifact-unit target with close-tag headroom. Endpoint exchanges
now carry decision id, context-frame fingerprint, tool-view fingerprint, active
timeout, and provider-exchange rows tied to the
runtime decision. Status output summarizes active cells, conflicts, decisions,
admissions, observations, provider exchanges, and artifacts. Prompt-frame rows
are persisted before model calls and replay bounded bodies from refs; prompt
cards include context lanes with source refs, item ids, budgets, and lane
fingerprints. Parse faults, endpoint errors, effect errors, shell observations,
and secret-like bodies become classified context items. Recovery prompts now name
the decision, attempt, bounded fault diagnosis, and next expected envelope.
The owner-facing `context resolve` command writes active resolution cells.
Unfinished decisions with committed external evidence recover before new
selection; decisions without external evidence are reused. Proof collection
writes state-ledger sections for cells, decisions, candidates, records, prompt
frames, admissions, observations, exchanges, artifacts, checks, and context.
Status, doctor, workspace, watch, and workbench read rows. Workbench accepts
owner input, append or pane mode, scroll commands, follow on/off, and status rail
fallbacks. Generic records add, list, show, link, archive, and resolve path
aliases. Wrappers are today, journal, todo, calendar, finance, project, and dev;
record kinds project index, todo, calendar, finance, routine, proof, dev, and
project state cells. Workspace rebuild writes derived indexes and artifact rows;
workspace rebalance writes manifest, preview, apply audit, and alias rows. Xtask
writes deterministic protocol ledgers and live-profile run or skip evidence.
## State-Ledger Parity

The checkout satisfies the executable state-ledger bridge contract in this
repository. Durable rows are the runtime control plane for operation selection,
prompt frames, admissions, observations, context hygiene, recovery, artifacts,
status, and proof evidence.

The current task body remains the fixed `TaskSnapshot` shape because product
templates and checks still use task and step records. The daemon mirrors that
body into `case:snapshot` state cells, hydrates runnable snapshots from state
before plan rows, and uses plan-family rows as durable task-body storage. Plan
rows seed operation projection events only when no active operation cell exists.
Once projected, state cells and persisted `RuntimeDecision` rows control turn execution.

Runtime selection reads operation-specific state cells projected through durable
events. Context conflict cells, owner resolution cells, recovery report cells,
and task snapshot cells are reducer-applied events with state history rows. The
turn interpreter follows the persisted decision operation and settlement
suppresses the operation cell.

Prompt rendering, parsing, and admission use the persisted decision envelope and
explore `ToolSetView`. Tool field specs are catalog-backed for prompt, parser,
admission, and explore effect selection. Prompt-frame rows own replayable bounded
body refs plus structured prompt card rows and fingerprints.

Prompt context has durable context items, conflict cards, contamination and stale
exclusions, contradiction edges, and context lane plans with included and
excluded ids, source refs, reasons, budgets, and fingerprints. Parse faults,
endpoint errors, effect errors, shell observations, and secret-like bodies
classify contamination durably before prompt admission.

Artifact manifests, units, deterministic assembly, fresh fingerprint checks, and
artifact rows are wired into the generic artifact target. Manuscript remains an
old template kept for existing checks, while generic manifests support nested
units, source refs, checks, and workspace rebalancing. Proof bundles expose state-vector,
decisions, prompt-frame, admission, observation, exchange, artifact, context,
suppression, and conflict-edge sections.

## Implemented Code

`lkjagent-core` owns the first pure state-ledger domain modules plus the current
plan engine, parser, renderer, checks, word counting, classifier, templates,
docs-link helpers, graph queries, workspace manifests, artifact manifests, and
recovery helpers. `lkjagent-store` owns the plan-store schema, state-ledger
tables, state-edge rows, workspace manifest, alias, audit rows, hydration, queue
access, and atomic turn state commits. `lkjagent-effects`
owns filesystem, shell, check gathering, observations, and exchange log file helpers.
`lkjagent-app` owns the
daemon interpreter, row-backed CLI renderers, endpoint adapter, waiting answer
routing, effect-error settlement, and bounded explore dispatcher. `lkjagent-llm`
owns the endpoint wire client. `lkjagent-xtask` owns repository gates, structure
audit, deterministic replay, benchmark commands, and proof bundle collection.

## Historical Evidence

Checked-in `tmp/` logs are historical proof or failure fixtures unless rerun here.

## Next Executable Step

Run longer story proof trials when endpoint budget is available.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates
  exist in the current checkout.
- Checked-in run logs can be failure fixtures or historical proof, not current
  gate results.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first task.
- Never claim a gate passed without running it.
