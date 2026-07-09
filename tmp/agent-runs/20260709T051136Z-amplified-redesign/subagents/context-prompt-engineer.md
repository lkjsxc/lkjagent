# Context Prompt Engineer Report

## Scope

Report-only lane for reducing prompt duplication and contradiction while keeping
JSON out of normal context. Product docs and source were not edited.

## Current Facts

- `runtime_context.rs` defines context trust, staleness, contamination, source
  identity, and contradiction detection over current clean items.
- `select_normal_context` calls `select_context_plan(items, &[])`, so direct
  callers do not get contradiction suppression unless they separately detect and
  pass conflicts.
- `detect_contradictions` groups clean current items by `semantic_key` and exact
  `body`; two distinct bodies under one key become a conflict.
- `contamination_for_observation` marks failed observations as `RecoveryOnly`,
  `shell.run` as `ExternalRaw`, raw logs as `RawToolLog`, and secret-like bodies
  as `SensitiveOwnerData`.
- `runtime_context_plan.rs` suppresses stale, contaminated, conflicted, and
  duplicate clean items. Duplicate identity is
  `semantic_key + body + source_type + source_fingerprint`.
- Current context lanes are only `relevant-records` and
  `excluded-context-notes`; the packet/doc region list is broader.
- Lane source refs are `source_type:source_id`; lane fingerprints include ids,
  refs, budgets, and lane contents.
- `context_bridge.rs` prepares prompt context by inserting objective and
  workspace context, applying resolutions, detecting conflicts, writing
  conflict cells/edges, selecting a plan, rendering text, and fingerprinting the
  rendered text.
- Normal prompt context is appended to the task brief as:
  `context_items:\n<rendered text>`.
- Included context lines render as
  `semantic_key [source_type:source_id fp=source_fingerprint] body`.
- JSON-like bodies are not excluded in the plan. They are included, then
  `prompt_safe_body` replaces the body with
  `[json-like context suppressed item=... source=type:id]`.
- Conflict summaries currently render item ids only:
  `Unresolved conflict <key> items=<ids>`.
- `runtime_prompt_kernel.rs` builds eight prompt-card rows in order: kernel,
  objective, state, facts, conflicts, recovery, tools, output.
- Prompt-card reasons list included/excluded context id reasons plus lane
  fingerprints and source refs. They do not render XML-like context cards.
- `prompt_bridge.rs` persists `prompt-frame.json` audit bodies containing JSON
  metadata, card plan, context plan, system text, and user text. This is allowed
  as an audit/log artifact, not normal prompt context.
- Targeted tests passed:
  `cargo test -p lkjagent-core --test context_completion` -> 4 passed.
- Targeted tests passed:
  `cargo test -p lkjagent-core --test prompt_kernel` -> 2 passed.
- Targeted tests passed:
  `cargo test -p lkjagent-app --test context_no_json` -> 1 passed.
- Targeted tests passed:
  `cargo test -p lkjagent-app --test context_items` -> 2 passed.
- Targeted tests passed:
  `cargo test -p lkjagent-app --test prompt_frame` -> 2 passed.

## Contradictions

- Docs say context renders as compact XML-like cards with lane names and
  fingerprints; source renders line-oriented prose inside `task.brief`, not
  `<context_card>` blocks.
- Docs/packet say JSON-like bodies are excluded or projected before prompt
  rendering; source includes the item in the plan and suppresses only the body
  while preserving an included `clean-current` reason.
- Docs/packet say conflict summaries include source refs and resolution
  direction; source conflict prompt text includes only semantic key and item ids.
- Docs describe lanes for identity, objective, state, facts, workspace, recovery,
  tool cards, and output; `ContextFramePlan` currently records only relevant
  records and excluded context notes.
- Packet selection algorithm requires ranking by state-key match, freshness,
  owner priority, evidence quality, workspace proximity, and recency; source
  preserves input order after suppression/dedupe and does not rank.
- `docs/current-state.md` claims JSON-like context bodies are replaced before
  prompt rendering. That is true for rendered body text, but not for plan
  admission/reasons because the JSON-like item remains included.
- `docs/context/prompt-assembly.md` says active state payloads are consumed by
  prompt assembly; this lane did not find active state payload rendering in
  `context_bridge.rs` or `render.rs` beyond decision fingerprints/card reasons.

## Exact Docs Edits

- If preserving current source behavior, update
  `docs/context/README.md` model-visible shape to say context currently renders
  as source-ref prose lines plus prompt-card rows, with XML-like action/output
  cards handled separately by the protocol renderer.
- If preserving current source behavior, update
  `docs/context/prompt-assembly.md` admission rules to say JSON-like clean
  context is admitted for audit ids/reasons but its body is replaced by a
  source-linked suppression marker before entering the rendered prompt.
- If preserving current source behavior, update
  `docs/context/prompt-assembly.md` layout to list the two currently recorded
  lanes, or mark the broader lane table as target contract rather than proven
  behavior.
- If preserving current source behavior, update
  `docs/context/contradictions.md` rendering section to say current prompt
  summaries contain semantic key and item ids; source refs are available in
  context plan/card rows, not in the prompt summary line.
- If implementing the packet contract instead, keep docs mostly intact and
  change source as listed below.
- After either path, update `docs/current-state.md` to distinguish rendered-body
  JSON suppression from plan-level exclusion/projection.

## Exact Source Edits

- `crates/lkjagent-core/src/runtime_context.rs`: either add a
  `JsonLikeContext` contamination/suppression class or expose an
  `is_json_like_context_body` helper so plan admission can mark JSON-like items
  before rendering.
- `crates/lkjagent-core/src/runtime_context_plan.rs`: use the JSON-like helper
  in `suppression_reason` if the intended contract is exclusion/projection at
  selection time; otherwise add a distinct included reason such as
  `clean-current-json-body-suppressed`.
- `crates/lkjagent-core/src/runtime_context_plan.rs`: add deterministic ranking
  fields or a rank function if the packet's ranking contract is adopted.
- `crates/lkjagent-core/src/runtime_context_plan.rs`: expand `build_lanes` if
  the docs' lane table is intended to be real frame data, not only prompt-region
  documentation.
- `crates/lkjagent-app/src/context_bridge.rs`: render conflict summaries with
  source refs/fingerprints, not only item ids, if keeping the docs/packet
  contradiction contract.
- `crates/lkjagent-app/src/context_bridge.rs`: render admitted context as
  attribute-free `<context_card>` blocks if the XML card contract is intended for
  normal context, not just tool/output cards.
- `crates/lkjagent-core/src/runtime_prompt_kernel.rs`: if context cards move
  into the kernel layer, include per-card source refs/fingerprints or point the
  facts card to rendered context card ids.
- `crates/lkjagent-app/src/prompt_bridge.rs`: no required product change found;
  JSON prompt-frame logs are consistent with the no-JSON policy because they are
  audit artifacts.

## Tests To Add Or Update

- Add an app test proving prompt-visible conflict summaries include source refs
  and fingerprints, or update docs to match item-id-only summaries.
- Add a core/app test that distinguishes JSON-like body suppression from
  plan-level admission, then assert the chosen contract explicitly.
- Add a prompt rendering test for XML-like `<context_card>` blocks if the packet
  XML card contract is adopted.
- Add a context-plan ranking test with shuffled candidates if ranking by
  freshness/priority/evidence is adopted.
- Add prompt-frame lint coverage that scans rendered ordinary context for JSON
  object/array lines outside recovery/audit contexts.
- Add a test proving prompt-frame fingerprint changes when selected context
  changes beyond the current objective-only prompt-frame test.

## Commands To Run

- `cargo test -p lkjagent-core --test context_completion`
- `cargo test -p lkjagent-core --test prompt_kernel`
- `cargo test -p lkjagent-app --test context_no_json`
- `cargo test -p lkjagent-app --test context_items`
- `cargo test -p lkjagent-app --test prompt_frame`
- After source/docs edits: `cargo run -p lkjagent-xtask -- check-docs`
- After source/docs edits: `cargo run -p lkjagent-xtask -- quiet verify`
- Final completion gate when claiming behavior completion:
  `docker compose run --rm verify`

## Risks

- Changing JSON-like items from included-with-suppressed-body to excluded may
  change context fingerprints, prompt-card reasons, and proof expectations.
- Moving normal context to XML-like cards may duplicate tool/output card shapes
  unless card kinds and output envelopes remain visually distinct.
- Adding source refs to conflict summaries could increase prompt size and leak
  source identifiers unless bounded by lane budgets.
- Ranking candidates can reorder existing prompt frames and invalidate tests
  that assume insertion order.
- Docs currently read more complete than the source; accepting them unchanged
  risks overstating implemented prompt-kernel behavior.

## Acceptance Items Affected

- No duplicate clean context bodies in a prompt frame: currently covered for
  plan selection; app-level duplicate prompt-frame rendering needs explicit
  coverage if risk increases.
- JSON-like context bodies suppressed/projected: covered for rendered prompts,
  but contract wording needs clarification or plan-level source changes.
- Contradictions produce summaries: covered; summary lacks source refs.
- Failed model output excluded from normal context: covered through
  contamination tests.
- Source refs and fingerprints present: covered for included context lines and
  lane/card audit data; conflict summaries need source refs if required.
- Prompt-frame fingerprint changes when selected context changes: partially
  covered by non-empty objective fingerprint; add direct selected-context-change
  test.
