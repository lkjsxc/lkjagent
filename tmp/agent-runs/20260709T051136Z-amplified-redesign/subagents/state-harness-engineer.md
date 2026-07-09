# State Harness Engineer Report

## Scope

- Lane: `tmp/lkjagent-yolo-amplified-thinking-packet-20260708/10-subagents/state-harness-engineer.md`.
- Mode: read-only source/docs inspection; report artifact only.
- Product docs/source edits performed: none.
- Report path: `tmp/agent-runs/20260709T051136Z-amplified-redesign/subagents/state-harness-engineer.md`.

## Current Facts

- `docs/current-state.md` says durable state rows plus persisted `RuntimeDecision`
  rows are the single control plane, and completion is engine-computed from
  fresh state, artifacts, checks, observations, and blocker evidence.
- Required candidate files exist:
  `runtime_state.rs`, `runtime_transition.rs`, `runtime_selector.rs`,
  `runtime_completion.rs`, and `runtime_bridge.rs`.
- Candidate files are under the 200-line rule: `runtime_state.rs` 161 lines,
  `runtime_transition.rs` 167 lines, `runtime_selector.rs` 147 lines,
  `runtime_completion.rs` 31 lines, `runtime_bridge.rs` 44 lines.
- `runtime_state.rs` defines generic state keys, statuses, evidence refs,
  cells, snapshots, active cell selection, active edge hydration, and stable
  snapshot fingerprints.
- `runtime_transition.rs:46` defines transition rows with node ref, previous
  state, next state, actor, reason fields, evidence refs, context fingerprint,
  tool call id, retry count, and correlation id.
- `runtime_transition.rs:103` rejects terminal reopen, illegal state steps,
  success/failure/supersession without evidence, and progress while blocking or
  dependency edges remain.
- `runtime_candidate.rs:8` includes workspace-family operation candidates for
  todo, calendar, routine, index, proof, dev, project, and finance.
- `runtime_candidate.rs:47` gives payload-defined `operation_key` cells a
  generic candidate path before namespace-specific matching.
- `runtime_candidate.rs:60` selects owner intake, owner answer, recovery,
  effect, model, check, completion blocked, completion close, workspace family,
  custom payload, cooldown, and idle candidates.
- `runtime_selector.rs` persists selected state key, effect command, state
  vector fingerprint, context-frame fingerprint, model budget, evidence
  requirements, and recovery policy on the selected decision.
- `runtime_bridge.rs:13` inserts the case, checks unfinished decisions through
  recovery, hydrates state, projects a runtime cell when needed, selects a new
  decision, and persists it before returning.
- `runtime_bridge.rs:36` settles runtime decisions and suppresses the selected
  state cell through `runtime_projection.rs`.
- `runtime_projection.rs:11` projects bridge state into runtime cells only if no
  executable state candidate already exists.
- `runtime_completion.rs:18` has a narrow pure helper that closes only when
  requirements are non-empty and every requirement has matching passed evidence
  by check name and artifact fingerprint.
- Broader bridge completion safety lives in `engine_completion.rs`: every step
  must be done or superseded, file/artifact templates cannot close with empty
  artifact evidence, check results must match name and params, carry a decision
  id and evidence fingerprint, and artifact-backed checks need artifact refs.
- Focused tests exist for transition guards, selector ordering and blockers,
  completion freshness, bridge completion safety, DB blocked-shape safety, and
  state-key settlement after native/payload decisions.

## Contradictions

- Packet completion design lists many close inputs: active cells, edges,
  decision status, admissions, observations, artifacts, check rows, workspace
  records, prompt fingerprints, unresolved recovery, queue rows, and owner
  questions. `runtime_completion.rs` itself checks only check name,
  artifact fingerprint, and pass flag. Some missing concerns are enforced in
  `engine_completion.rs` and app/store bridges, but the pure
  `runtime_completion` module is not the full documented close predicate.
- Transition design says `active` requires one persisted runtime decision,
  `waiting-owner` requires a concrete owner question, `recovering` requires a
  bounded diagnosis, `verifying` requires fresh facts or artifact refs, and
  `failed` requires blocker plus exhausted/refused recovery. Current
  `validate_transition` enforces generic legal steps, terminal protection,
  terminal evidence, and blocker/dependency edges, but not those
  state-specific evidence semantics.
- State taxonomy includes `matter`, `operation`, `workspace`, `context`, `tool`,
  `recovery`, `tui`, `evaluation`, and `maintenance`. Selector candidates cover
  many practical namespaces, but there is no explicit first-class namespace path
  for `matter`, `operation`, `context`, `tool`, `tui`, `evaluation`, or
  `maintenance` except via payload-defined `operation_key` cells.
- Docs say completion cells include `completion:check-pending/*`,
  `completion:check-passed/*`, `completion:check-failed/*`,
  `completion:blocked`, and `completion:close-candidate`. Source inspection
  confirmed selector branches for `completion:blocked` and
  `completion:close-candidate`; the check-passed/check-failed projection
  contract needs separate store/reducer inspection before claiming full parity.
- `runtime_projection.rs` suppresses a selected state cell after any settled
  decision. This matches current tests for payload/native state settlement, but
  it is risky for multi-phase state cells unless every operation re-projects
  unresolved follow-up state before suppression.

## Exact Docs Edits

- None performed due report-only scope.
- If implementation proceeds, update `docs/runtime/recovery-and-completion.md`
  and `docs/engine/completion.md` to name the exact module boundary:
  `runtime_completion.rs` is artifact-check freshness only, while
  `engine_completion.rs` and state/store reducers enforce bridge blockers,
  recovery, queue, prompt, and decision-status closure gates.
- If the intended contract is a single pure close predicate, instead edit those
  docs to point at an expanded `runtime_completion` input struct and remove any
  implication that bridge-only checks are sufficient.
- Update `docs/state/reducer-and-selectors.md` if first-class selector branches
  are added for currently payload-only families such as `context`, `tool`,
  `tui`, `evaluation`, or `maintenance`.

## Exact Source Edits

- None performed due report-only scope.
- Candidate source edit: expand `runtime_transition.rs` with typed
  state-specific evidence guards:
  `Active` requires `RuntimeDecision` evidence for decision/operation nodes;
  `WaitingOwner` requires owner-question evidence; `Recovering` requires
  diagnosis evidence; `Verifying` requires check/artifact/fact evidence;
  `Failed` requires blocker plus exhausted/refused recovery evidence.
- Candidate source edit: replace or wrap `runtime_completion::can_close` with a
  `CompletionSnapshot`/`CompletionInputs` struct covering required checks,
  active blocker/dependency edges, unfinished decisions, unresolved recovery,
  pending queue/owner questions, artifact fingerprints, and workspace record
  freshness. Keep existing `can_close` as a low-level helper only if docs say so.
- Candidate source edit: add explicit selector mapping for any taxonomy family
  that must be contractually supported without payload `operation_key`; otherwise
  document payload-defined operations as the intended extension path.
- Candidate source edit: audit `suppress_decision_cell` so suppression happens
  only after the settled operation has emitted replacement/terminal state for
  multi-phase cells.

## Tests To Add Or Update

- Add `runtime_transitions.rs` cases for `Active` without decision evidence,
  `WaitingOwner` without question evidence, `Recovering` without diagnosis,
  `Verifying` without fact/artifact/check evidence, and `Failed` without
  blocker/exhausted recovery evidence.
- Add `runtime_completion.rs` tests where matching check evidence exists but an
  unfinished decision, unresolved recovery cell, blocker edge, pending owner
  question, or stale artifact prevents closure.
- Add an app/store integration test proving check rows without active native
  completion cells cannot close a case after hydration.
- Add a multi-phase state-cell test proving settlement suppression does not
  drop unresolved follow-up work.
- Keep existing focused tests: `runtime_transitions`, `runtime_selector`,
  `context_completion`, `runtime_artifact`, `completion_safety`,
  `completion_db_safety`, and `state_key_settlement`.

## Commands To Run

- Targeted after implementation:
  `cargo test -p lkjagent-core runtime_transitions runtime_selector context_completion runtime_artifact completion_safety`
- Targeted app/store after implementation:
  `cargo test -p lkjagent-app completion_db_safety state_key_settlement recovery`
- Required docs/shape gates after edits:
  `cargo run -p lkjagent-xtask -- check-docs`
  `cargo run -p lkjagent-xtask -- check-lines`
  `cargo run -p lkjagent-xtask -- check-files`
  `cargo run -p lkjagent-xtask -- check-style`
- Required final gates:
  `cargo run -p lkjagent-xtask -- smoke replay`
  `cargo run -p lkjagent-xtask -- quiet verify`
  `docker compose run --rm verify`
- Existing stamped evidence in this run directory shows exit 0 for
  `check-docs`, `check-lines`, and `quiet verify`, but this lane did not rerun
  product gates because the task was report-only.

## Risks

- Highest risk: docs imply one coherent completion predicate, but the current
  implementation is distributed across `runtime_completion`, bridge engine
  completion, projection, store hydration, and app recovery paths.
- Strengthening transition guards could break tests or callers that currently
  construct sparse transitions through `RuntimeTransition::new`.
- Adding first-class state-family branches can accidentally create a second
  registry unless they remain derived from persisted state cells and decisions.
- Suppressing selected cells on every settled decision can hide unfinished
  multi-step state unless replacement cells or edges are emitted transactionally.
- Docker final verification can expose environment-specific failures not seen in
  targeted core/app tests.

## Acceptance Items Affected

- State harness acceptance: state families, transitions, decision persistence,
  selector coverage, and state-to-tool/state-to-prompt routing.
- Recovery/completion acceptance: model prose cannot close work; unfinished,
  blocked, failed, stale, or unresolved work keeps the matter open or blocked.
- Evidence/proof acceptance: final claims require focused tests, quiet verify,
  and Docker Compose verification after the last source/doc change.
- Single-control-plane acceptance: all prompt/tool/effect/completion choices
  must derive from state cells, edges, and persisted `RuntimeDecision` rows.
