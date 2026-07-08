# Current State

## Purpose

Keep an honest ledger that separates the target contract, behavior proven in
this checkout, and open implementation gaps.

## Target Contract

lkjagent is a workspace-first personal agent harness for one owner, one local
LLM, one visible workspace, and one SQLite ledger. Durable rows and persisted
`RuntimeDecision` rows are the single control plane. Workspace files are the
owner-readable memory surface; they never replace ledger authority.

Every owner turn writes visible evidence: a transcript entry for ordinary turns
or an inbox trace when routing is ambiguous. Record-like turns write Markdown
under `data/workspace` by default and upsert workspace record rows, fingerprint
history, README coverage, index artifacts, state cells, and queue route
evidence. Artifact turns create concrete workspace paths, artifact rows, checks,
and response paths before any success report.

The model-visible interface is compact XML-like text with source refs. Tool
calls use one attribute-less `<lkjagent_action>` envelope selected by the active
runtime decision. Prompts render selected context lanes, selected tool views,
and bounded recovery diagnoses; they do not render JSON, whole transcripts, raw
failed output, or the global tool catalog.

Completion is engine-computed from fresh state, artifacts, checks, observations,
and blocker evidence. A later response cannot run after earlier blocked, active,
failed, pending, or unsuperseded skipped work without repair or supersession
evidence.

## Proven In Current Checkout

The store persists cases, owner queue rows, runtime events, state cells, state
edges, runtime decisions, context items, prompt frames, tool admissions,
observations, provider exchanges, token usage, checks, artifacts, workspace
records, record history, path aliases, rebalance audits, and proof data.

The core crate contains pure reducers, selectors, transition guards, completion
checks, context hygiene, tool descriptors, XML-like action parsing, artifact
units, workspace manifests, and graph queries. The app crate contains bridge
interpreters, endpoint exchange capture, record commands, workspace rebuild and
rebalance commands, console, watch, status, workbench, row-backed inspection,
and bounded `run --once` paths.

Current model action parsing accepts one attribute-less `<lkjagent_action>` with
child tags for decision id, context fingerprint, tool name, and repeated
arguments. It rejects JSON-shaped bodies, attributes, unknown tags, duplicate
scalars, duplicate argument names, stale decisions, context mismatches, unknown
tools, bad primitive classes, unclosed tags, crossed tags, bad entities, and
placeholder-like executable values.

Runtime decisions carry selected state keys, selected tool views, context-frame
fingerprints, expected envelopes, evidence requirements, and recovery policy.
Native model-free operations support state resolution and workspace text effects
for `workspace.write_text` and `workspace.append_text` through path-checked
workspace effects and artifact rows.

Record-like owner turns have focused coverage for direct workspace writes,
record rows, history, fingerprints, README links, index contents, and index
artifacts. CLI send and daemon intake now scaffold the workspace and write
owner-turn transcript or inbox trace files. Owner-turn routing has focused
coverage for existing-matter answers, continuations, artifact requests,
inspection, unsupported system operations, direct records, ambiguous inbox
routing, and Japanese diary or save wording.

Prompt context can admit bounded workspace record and index metadata with source
fingerprints and suppress duplicate clean context items by semantic key, body,
source type, and source fingerprint. Default explore tool views exclude
`shell.run`; shell is available only through an explicit persisted shell-capable
decision view. Unsafe XML-like tool actions persist rejections. Status and proof
surfaces expose cached token usage, blocked counts, stale-edge counts, prompt
frames, tool-view fingerprints, artifact refs, check refs, and proof rows.

The terminal workbench has a pure reducer, durable transcript stream with stable
queue/event ids, agent draft accumulation for streaming deltas, id-based
transcript merge, saved transcript ids and source paths, Japanese and
grapheme-aware composer operations, append and pane modes, row-backed status
fallbacks, and focused coverage for transcript merge and rendering surfaces.

Baseline commands in this work session passed `cargo run -p lkjagent-xtask --
check-docs`, `cargo run -p lkjagent-xtask -- quiet verify`, and `docker compose
run --rm verify`. Packet static scripts require `python3`; the checked data DB
had no workspace records, artifacts, check results, admissions, or observations.

## Known Gaps

- Current acceptance still needs broad daily-use campaign evidence across
  journal, todo, calendar, finance, notes, artifacts, questions, project work,
  TUI, and recovery flows.
- Artifact creation still needs packet-level proof that files, rows, checks, and
  response paths are all present before owner-facing success.
- Recovery state coverage is incomplete for parse, admission, effect, endpoint,
  and check failures as one coherent ladder.
- Tool-view selection still needs broader state-by-state coverage, but focused
  tests now prove the default explore prompt hides shell and explicit shell views
  still admit shell with contamination tracking.
- Prompt context still needs packet-level no-JSON rendering and failed-output
  containment evidence beyond focused duplicate-context coverage.
- TUI bottom-follow behavior still needs packet-level focused tests and
  evidence; transcript identity, draft accumulation, and id-based merge now have
  focused coverage.
- Deterministic replay, quiet verify, Docker Compose verify, and live or
  scripted campaigns must run again after source changes before final claims.

## Next Executable Step

Make the docs-first product architecture commit, then implement focused source
slices with tests and gate evidence until the packet acceptance matrix is fully
checked in the progress ledger.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates
  exist in the current checkout.
- Checked-in run logs can be failure fixtures or historical proof, not current
  gate results.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first request.
- Never claim a gate passed without running it.
