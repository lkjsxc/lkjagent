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
under canonical workspace family paths: dated journal entries, state-grouped
TODOs, dated calendar notes, month-grouped finance notes, and readable work or
knowledge records. The owner command is route evidence; the record body is a
structured owner-readable note unless verbatim storage is explicit. Recording
also upserts workspace record rows, fingerprint history, README coverage, index
artifacts, state cells, and queue route evidence. Artifact turns create concrete
workspace paths, artifact rows, checks, and response paths before any success
report.

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
arguments. It preserves multiline and Japanese values, bounds argument value
size, and rejects JSON-shaped bodies, attributes, unknown tags, duplicate
scalars, duplicate argument names, stale decisions, context mismatches, unknown
tools, bad primitive classes, unclosed tags, crossed tags, bad entities, empty
executable values, and placeholder-like executable values.

Runtime decisions carry selected state keys, selected tool views, context-frame
fingerprints, expected envelopes, evidence requirements, and recovery policy.
Parse, admission, effect, endpoint, and check failures now write
`recovery.failure` state cells keyed by kind and decision. Native model-free
operations support state resolution and workspace text effects for
`workspace.write_text` and `workspace.append_text` through path-checked
workspace effects and artifact rows. Runtime projection and decision dispatch now
preflight earlier unfinished step blockers before later model response work.

Record-like owner turns have focused coverage for direct workspace writes,
record rows, history, fingerprints, README links, index contents, and index
artifacts. Journal records now use `YYYY/MM/DD/entry.md`, TODO records use state
paths, calendar records use dated paths, finance records use month paths, and
record bodies are structured separately from owner-turn transcript evidence.
Artifact-request routing now blocks closure unless the final response names the
artifact path after file, artifact row, and check row evidence exists. CLI send
and daemon intake now scaffold the workspace and write owner-turn transcript or
inbox trace files. Owner-turn routing has focused coverage for existing-matter
answers, continuations, artifact requests, inspection, unsupported system
operations, direct records, ambiguous inbox routing, and Japanese diary or save
wording.

Prompt context can admit bounded workspace record and index metadata with source
fingerprints, suppress duplicate clean context items by semantic key, body,
source type, and source fingerprint, and replace JSON-like context bodies with
source-linked suppression markers before prompt rendering. Default explore tool
views exclude `shell.run`; shell is available only through an explicit persisted
shell-capable decision view. Unsafe XML-like tool actions persist rejections.
Status and proof surfaces expose cached token usage, blocked counts, stale-edge
counts, prompt frames, tool-view fingerprints, artifact refs, check refs, and
proof rows.

The terminal workbench has a pure reducer, durable transcript stream with stable
queue/event ids, agent draft accumulation for streaming deltas, id-based
transcript merge, saved transcript ids and source paths, Japanese and
grapheme-aware composer operations, append and pane modes, row-backed status
fallbacks, and focused coverage for transcript merge and rendering surfaces.

Baseline and post-change commands for the 2026-07-08 packet passed `cargo run
-p lkjagent-xtask -- check-docs`, `cargo run -p lkjagent-xtask -- quiet
verify`, and `docker compose run --rm verify`. Packet static scripts ran with a
`python3` shim for `python`. The checked data DB had workspace_records=1,
workspace_record_history=2, artifacts=6, check_results=0, tool_admissions=5,
and observations=5.

Focused acceptance evidence now covers owner record writes, transcript traces,
artifact files and response-path gating, blocked-step preflight, prompt-context
non-JSON suppression, state-selected tool views, XML action parsing and
admission, recovery failures, and TUI transcript identity and follow behavior.
Scripted evidence under `tmp/agent-runs/20260708T180141Z/` records the workspace
probe, protocol experiment matrix, and proof bundle. Live endpoint profiles under
`tmp/live-runs/20260708T180320Z/` are honest skips because endpoint input was
intentionally absent.

## Known Gaps

- Scripted workspace evidence covers todo, note, transcript, index, proof, and
  blocked artifact-request behavior; a full live endpoint daily-use run remains
  skipped until endpoint configuration is present.
- TUI behavior is proven by reducer and transcript tests, not an interactive
  terminal capture in this environment.
- The worktree still contains preexisting dirty `data/logs` deletions and
  untracked `data/` runtime files that are not part of this packet.

## Next Executable Step

Commit the current-state and evidence ledger, preserving exact command evidence
and residual risks for the final handoff.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates
  exist in the current checkout.
- Checked-in run logs can be failure fixtures or historical proof, not current
  gate results.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first request.
- Never claim a gate passed without running it.
