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
fixtures, and stop tags render the same XML-like action grammar.

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
files.

Owner CLI, status, watch, console, and TUI labels now expose matters rather than
tasks. Plan-family rows still exist as bridge storage, but owner-visible list,
show, queue, log, status, and exchange paths use matter and operation wording.

Explicit record-like owner turns now route before model calls. Japanese record
requests, todo-like text, calendar-like text, finance notes, notes, project
notes, and artifact records write workspace Markdown files directly, update
`workspace_records`, write fingerprint history, mark queue rows recorded, create
workspace state cells and edges, and avoid creating plan-family rows for those
turns. Record writes create README files along the touched workspace path.

The implementation still contains a plan-family bridge. Existing task and step
rows are treated as body storage and continuity evidence while state cells and
runtime decisions take over turn authority. Transitional commands may expose
that bridge until the semantic matter surface is complete.

## Known Gaps

- Semantic owner-turn routing to existing matters, artifact requests,
  inspection, and system operations needs focused tests beyond direct records.
- Final compose verification must be rerun after the last code change in this
  checkout.

## Next Executable Step

Implement the smallest coherent slice that makes the docs true: run final
repository and Docker verification, then capture proof and handoff evidence.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates
  exist in the current checkout.
- Checked-in run logs can be failure fixtures or historical proof, not current
  gate results.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first request.
- Never claim a gate passed without running it.
