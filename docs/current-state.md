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

The implementation still contains a plan-family bridge. Existing task and step
rows are treated as body storage and continuity evidence while state cells and
runtime decisions take over turn authority. Transitional commands may expose
that bridge until the semantic matter surface is complete.

## Known Gaps

- Some CLI output still exposes owner work through plan-family vocabulary.
- Record-like owner turns are not yet guaranteed to write workspace files before
  any claim of recording.
- Semantic owner-turn routing to existing matters, direct records, artifacts,
  inspection, and system operations needs focused tests.
- Workspace READMEs, indexes, and rebalance link-repair evidence need stronger
  coverage.
- The workbench composer needs grapheme-aware cursor movement and durable agent
  transcript rendering tests.
- Token accounting needs first-class total input, cached input, uncached input,
  output, and unknown cache status in schema, status, and proof.
- Flat `data/lkjagent.json` migration from older nested config needs proof.
- Docker layering must avoid broad `COPY . .` and final compose verification
  must be rerun in this checkout.
- Live campaigns must run for real elapsed time when an endpoint is available,
  or write explicit skip evidence.

## Next Executable Step

Implement the smallest coherent slice that makes the docs true: route explicit
record-like owner turns to direct workspace record writes before any model call
or recording claim.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates
  exist in the current checkout.
- Checked-in run logs can be failure fixtures or historical proof, not current
  gate results.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first request.
- Never claim a gate passed without running it.
