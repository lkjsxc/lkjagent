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
`recovery.failure` state cells keyed by kind and decision. Parse diagnoses now
include concrete repair guidance for envelope, action grammar, decision, context,
tool, and argument faults. Native model-free operations support state resolution
and workspace text effects for `workspace.write_text` and
`workspace.append_text` through path-checked workspace effects and artifact rows.
Runtime projection and decision dispatch now preflight earlier unfinished step
blockers before later model response work.

Record-like owner turns have focused coverage for direct workspace writes,
record rows, history, fingerprints, README links, index contents, and index
artifacts. Journal records now use `YYYY/MM/DD/entry.md`, TODO records use state
paths, calendar records use dated paths, finance records use month paths and the
`budget-month.md` index, and record bodies are structured separately from
owner-turn transcript evidence. Oversize record bodies are split into linked
`.parts/part-NNN.md` files with a size justification in the main record.
Artifact-request routing now blocks closure unless the final response names the
artifact path after file, artifact row, and check row evidence exists. CLI send
and daemon intake now scaffold the workspace and write owner-turn transcript or
inbox trace files. Owner-turn routing has focused coverage for existing-matter
answers, continuations, artifact requests, inspection, unsupported system
operations, direct records, ambiguous inbox routing, Japanese diary or save
wording, and todo, calendar, finance, note, project, and development wrapper
writes.

Prompt context can admit bounded workspace record and index metadata with source
fingerprints, suppress duplicate clean context items by semantic key, body,
source type, and source fingerprint, and replace JSON-like context bodies with
source-linked suppression markers before prompt rendering. Prompt card facts now
carry compact lane fingerprints and source refs. Default explore tool views
exclude `shell.run`; shell is available only through an explicit persisted
shell-capable decision view. Unsafe XML-like tool actions persist rejections.
Status and proof surfaces expose cached and uncached token usage separately,
raw provider usage metadata, blocked counts, stale-edge counts, prompt frames,
tool-view fingerprints, artifact refs, check refs, and proof rows.

The terminal workbench has a pure reducer, durable transcript stream with stable
queue/event ids, agent draft accumulation for streaming deltas, id-based
transcript merge, saved transcript ids and source paths, Japanese input,
grapheme-indexed composer movement, delete, home/end, multiline submit,
display-width cursor placement, append and pane modes, row-backed status
fallbacks, and focused coverage for transcript merge and rendering surfaces.
Workbench pane mode now renders a canonical conversation transcript in the left
pane and keeps step progress, matter trace, recent events, and proof diagnostics
out of that transcript pane.

Baseline and post-change commands for the 2026-07-08 packet passed `cargo run
-p lkjagent-xtask -- check-docs`, `cargo run -p lkjagent-xtask -- quiet
verify`, and `docker compose run --rm verify`. Packet static scripts ran with a
`python3` shim for `python`. Docker builds use explicit crate copies and BuildKit
cache mounts; Compose exposes agent, daemon, live-campaign, shell, and check
profiles with writable data and tmp paths. Runtime `data/logs` rows are no
longer committed as product evidence.

Runtime configuration is flat JSON only. `data/lkjagent.json` accepts scalar
and primitive-array keys such as `endpoint_url`, `endpoint_model`,
`workspace_root`, prompt budget, and live-campaign duration; nested objects are
startup errors. Live-profile endpoint detection counts only environment
variables or those flat endpoint keys, so old nested endpoint shapes no longer
silently enable live campaigns.

Focused acceptance evidence now covers owner record writes, transcript traces,
artifact files and response-path gating, blocked-step preflight, prompt-context
non-JSON suppression, state-selected tool views, XML action parsing and
admission, recovery failures, and TUI transcript identity and follow behavior.
Scripted evidence under `tmp/agent-runs/20260708T180141Z/` records the workspace
probe, protocol experiment matrix, and proof bundle. Live endpoint profiles under
`tmp/live-runs/20260708T180320Z/` are honest skips because endpoint input was
intentionally absent. After `.env` use was authorized, a 900-second-per-profile
endpoint run under `tmp/live-runs/20260708Tstandardenv/` ran without exposing
secret values; personal-workspace, software-project, structured-artifact, and
protocol-stress all closed with elapsed_seconds=900.

The July 9 continuation reran packet static inspection, `check-docs`,
`check-lines`, all `lkjagent-core` and `lkjagent-app` integration tests,
deterministic smoke replay, a scripted workspace probe, proof collection,
protocol experiments, `quiet verify`, and `docker compose run --rm verify` from
clean `66027735`. Fresh evidence is under
`tmp/coding-agent-runs/20260709T041854Z-more-more-continuation/`.

## Known Gaps

- Scripted workspace evidence covers todo, note, transcript, index, proof, and
  blocked artifact-request behavior.
- Standard 900-second endpoint evidence exists for four profiles; adoption remains
  deferred until the owner accepts those metrics as default behavior.
- TUI behavior is proven by reducer, transcript, display-width, and packet
  capture-script tests, not an interactive operator session in this environment.

## Next Executable Step

Use the amplified evidence run as the baseline for future workspace-first
changes. If the owner wants more live confidence, run another endpoint profile
using configured credentials and record cost and elapsed-time evidence without
exposing secrets.

## Honesty Rules

- A behavior is implemented only when code, focused tests, and passing gates
  exist in the current checkout.
- Checked-in run logs can be failure fixtures or historical proof, not current
  gate results.
- Missing evidence never proves absence; verify before claiming.
- When docs and code disagree, fixing the disagreement is the first request.
- Never claim a gate passed without running it.
