# Current State

## Purpose

Keep an honest ledger that separates the target contract, behavior proven in
this checkout, and open implementation gaps.

## Contract Target

lkjagent is a workspace-first personal agent harness for one owner, one local
LLM, one visible workspace, and one SQLite ledger. Owner turns become semantic
facts: matters, records, decisions, state cells, relation edges, events,
artifacts, checks, context items, token usage, and proof rows. Durable rows and
persisted `RuntimeDecision` rows are the single control plane.

Record-like owner turns write files under `data/workspace` by default. The
workspace contains journals, notes, calendar-like records, finance entries,
project records, development evidence, generated artifacts, transcripts,
indexes, and proof logs. If lkjagent says it recorded something, a workspace
path, fingerprint, and ledger row must exist.

The LLM-visible interface is compact XML-like text with source refs. Tool calls
use a strict attribute-less XML-like action grammar. JSON is allowed for flat
internal data configuration and exchange files, but not for model context or
model action output.

## Proven In Current Checkout

The checkout has substantial state-ledger bridge code: state cells, state edges,
runtime events, runtime decisions, prompt-frame rows, tool-set views, admission
rows, observations, context items, context conflict edges, artifact rows,
workspace record rows, workspace path aliases, workspace rebalance audit rows,
provider exchanges, token usage rows, proof collection, and row-backed status
surfaces.

The store can persist cases, events, state cells, decisions, context items,
workspace records, record fingerprint history, artifacts, prompt frames,
admissions, observations, exchanges, checks, and token usage. The core crate has
pure reducers, selectors, transition guards, tool descriptors, context hygiene,
artifact units, workspace manifests, graph queries, and parser helpers. The app
crate has bridge interpreters, endpoint exchange capture, record commands,
workspace rebuild and rebalance commands, console, watch, status, workbench, and
row-backed inspection paths.

The model action parser now accepts one `<lkjagent_action>` envelope with no
attributes and child tags for decision id, context fingerprint, tool name, and
arguments. It rejects JSON-shaped bodies, attributes, unknown tags, duplicate
scalar tags, duplicate argument names, stale decisions, context mismatches,
unknown tools, wrong primitive classes, unclosed tags, crossed tags, and bad
entities. Prompt cards, recovery cards, scripted endpoint helpers, experiment
fixtures, and stop tags render the same XML-like action grammar. Internal JSON
exchange files and flat data config are allowed as ledger evidence, not as
model-visible action output.

Token usage rows now keep total input, cached input, derived uncached input,
output, and cache status beside bridge prompt, completion, and cached columns.
Status and proof bundles render cached input separately and print unknown cache
values as unknown, not zero.

Endpoint configuration now reads flat `data/lkjagent.json` keys such as
`endpoint_url`, `endpoint_model`, `endpoint_timeout_seconds`, and
`endpoint_api_key_env`. When an older nested endpoint object is present, loading
or diagnostics rewrites it to the flat shape while environment variables still
take precedence.

The Docker image now copies explicit repository inputs instead of a broad build
context copy. Compose proof has run an owner record turn through `send`, the
long-running agent, and the workspace write path with an isolated data bind.

Live profile runs now accept endpoint settings from environment variables or
flat data config, copy root config into per-profile data directories, run each
available-endpoint profile for the requested elapsed time unless blocked, and
write explicit skip evidence when endpoint inputs are absent.

Workspace rebalance now refreshes generated README child links, records path
aliases and audit rows, rebuilds record-backed indexes after moves, and keeps
old paths resolvable through the alias table.

The terminal workbench reducer now stores a composer cursor, inserts and deletes
by Unicode grapheme clusters, maps left and right keys to composer movement, and
persists Japanese owner and agent transcript entries to workspace transcript
files. The TUI snapshot now builds one durable transcript stream from queue owner
turns and event rows ordered by row time, and the Transcript pane merges that
stream with in-session entries. Daemon-written agent messages remain visible
after refresh or TUI restart without splitting owner and agent blocks.

Earlier owner-turn and TUI transcript slices recorded Docker Compose and quiet
gate evidence. Those historical gates are useful proof records but must be
rerun after current behavior changes before claiming this checkout is fully
verified.

Owner CLI, status, watch, console, and TUI labels now expose matters rather than
tasks. Plan-family rows still exist as bridge storage, but owner-visible list,
show, queue, log, status, and exchange paths use matter and operation wording.

Explicit record-like owner turns now route before model calls. Japanese record
requests, todo-like text, calendar-like text, finance notes, notes, project
notes, and artifact records write workspace Markdown files directly, update
`workspace_records`, write fingerprint history, mark queue rows recorded, create
workspace state cells and edges, and avoid creating plan-family rows for those
turns. Record writes create README files along the touched workspace path.

Owner-turn routing now has focused pure coverage for existing-matter answers,
existing-matter continuations, artifact requests, inspection turns, system
operations, direct records, and Japanese diary/save wording. Queue rows persist
the deterministic route lane, desired durability, title seed, and transformation
permission; waiting-answer delivery refreshes the route to `existing_matter` and
`queue_answer`, and queue inspection commands display the route evidence.
Inspection routes now execute without endpoint access by closing a ledger matter
with a read-only summary of pending queue, active matters, and record count.
System-operation routes also avoid endpoint access; until an allowlisted executor
exists they block honestly with unsupported-executor and no-command-run evidence.
Artifact-request routes now build a write/verify/respond matter with a concrete
workspace artifact path and file-exists check. The route supplies intended
evidence, while bridge closure safety must still be proven by current focused
and Docker-backed gates. Continuation turns such as "also" or "this matter"
attach to an open matter at cycle boundaries, record an owner event, update the
matter brief, and feed the active step inputs without calling the endpoint.

The implementation still contains a plan-family bridge. Existing task and step
rows are treated as body storage and continuity evidence while state cells and
runtime decisions take over turn authority. Transitional commands may expose
that bridge until the semantic matter surface is complete.

The bridge completion-safety slice now has focused and Docker-backed coverage
for the packet's false closure shape. Core completion rejects blocked, active,
pending, and unsuperseded skipped bridge steps, requires artifact/check evidence
for artifact-like templates, and allows superseded skipped verify steps only
after repair and verify evidence. Runtime projection emits `completion:blocked`
instead of `completion:close-candidate` for blocked bridge states. The app has a
SQLite-backed regression proving an open file-work matter with a blocked plan
step and zero artifacts, check rows, workspace records, tool admissions, or
observations becomes blocked rather than closed. This slice passed `cargo run -p
lkjagent-xtask -- quiet verify` and Docker Compose `verify` in this run.

The native state store slice now commits runtime events and their state patches
inside one transaction. Duplicate runtime event ids are ignored before
reduction, so replaying an already inserted event cannot apply a second patch or
state history row. This slice has focused store coverage and passed quiet test
and Docker Compose `verify` in this run.

Runtime decisions now carry the selected state key. Selector tests cover model
and payload-defined cells, store tests prove the key survives unfinished-decision
hydration, and an app regression proves settlement suppresses a payload-defined
cell by its recorded key instead of reverse-parsing the operation string.

Native model-free state resolution now supports payload-defined cells with
`operation_key` `state.resolve`. The runtime selects this operation from state,
settles the selected state key without a bridge step, does not call the endpoint,
and leaves the matter open for the next state candidate instead of blocking it.

Payload-defined model-free effect commands now support a narrow
`workspace.write_text` command. The selector copies the effect command into the
persisted decision, the engine turns it into a workspace write command, the app
runs the existing workspace-safe write edge, and a focused regression proves the
file and artifact rows are created without endpoint output or bridge step
execution.

Bridge check results now carry parameters, decision id, evidence fingerprint,
artifact refs, and native state-edge freshness evidence. Completion requires
matching check name and parameters plus decision/evidence/artifact refs. Store
hydration suppresses stale passed rows, and artifact replacement suppresses old
check-to-artifact edges.

Record-like owner turns now write owner-readable workspace records without
creating tasks, refresh rows/history, write README path coverage, rebuild record
indexes, and record index artifact evidence. Focused coverage checks files,
rows, history, fingerprints, README links, index contents, and index artifacts.

Prompt context admits bounded workspace record/index metadata; unsafe XML-like
tool actions persist rejections; status/TUI show blocked, refused, and stale-edge
counts. `run --once` executes a bounded daemon turn; refreshed workspace smoke
passed with `missing: none`. Bounded live profiles ran with exchange evidence.
Native model-free effects support `workspace.write_text` and
`workspace.append_text`, both path-checked and persisted as artifact rows.
Deterministic verify projects `completion:check-pending/<step>` carrying
`check.run/<step>`; hydration suppresses passed rows lacking native outcome cells.

## Known Gaps

- Closure still reads transitional check rows after native hydration admits them.
- The plan-family bridge still participates in runtime projection. Until native
  state cells fully own selection, unsafe bridge steps must block close candidates.
- Native operation execution remains limited to state resolution, workspace text
  effects, and bridge-backed effects.

## Next Executable Step

Implement next bridge-retirement: reduce remaining model and close-candidate
projection from plan-family bridge rows.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates
  exist in the current checkout.
- Checked-in run logs can be failure fixtures or historical proof, not current
  gate results.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first request.
- Never claim a gate passed without running it.
