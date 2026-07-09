# Tool Protocol Engineer Report

## Scope

- Lane file read: `tmp/lkjagent-yolo-amplified-thinking-packet-20260708/10-subagents/tool-protocol-engineer.md`.
- Required context read: `docs/current-state.md`, packet `README.md`, protocol packet files under `06-tools-protocol/`, candidate source files.
- Report only. No product docs or source were edited.

## Current Facts

- `docs/current-state.md` says action parsing accepts one attribute-less
  `<lkjagent_action>` with decision id, context fingerprint, tool name, and
  repeated arguments, and rejects JSON bodies, attributes, duplicate fields,
  stale decisions, context mismatches, unknown tools, bad primitives, bad XML,
  empty executable values, and placeholder executable values.
- `crates/lkjagent-core/src/runtime_tool_call.rs` implements strict action
  parsing against a supplied `RuntimeDecision`.
- `parse_tool_call` checks the decision id, context fingerprint, and selected
  `decision.tool_view`; unknown tools are absent from the decision view, not
  looked up in a hidden global catalog.
- `runtime_action_xml.rs` rejects malformed, unclosed, crossed, attributed, and
  bad-entity XML-like tags.
- `runtime_tool_call.rs` bounds each argument value to 8192 chars and converts
  `ToolValueClass::Count` to a JSON number.
- `runtime_admission.rs` validates `OutputEnvelope::Action`, selected view
  membership, required params, unknown params, placeholder values, empty values,
  workspace-relative paths, and numeric count values.
- `runtime_tool_view.rs` derives field specs and value classes from parameter
  names: `path`, `command`, `count|limit|budget`, `query`, otherwise text.
- `runtime_tool_catalog.rs` has a 10-tool explore catalog, but
  `default_explore_tool_view()` deliberately excludes `shell.run`.
- `runtime_tool_cards.rs` renders the decision-visible `ToolSetView`, decision
  id, context fingerprint, no-prose/no-JSON/no-attribute rules, and one safe
  filled example when available.
- `parse_expected_for_decision()` parses action output using the persisted
  decision, while legacy `parse_expected(StepKind::Explore, ...)` uses a
  synthetic decision with the full `explore_tool_view()`.
- `admission_bridge.rs` persists admissions from `Command::RunExplore` commands
  after parsing/model handling, writes admitted or rejected `tool_admissions`
  rows, and stops before effects when admission rejects.
- App daemon flow persists prompt and provider exchange, applies parsed model
  output to commands, persists admissions, then dispatches effects.
- Focused tests already cover parser fault classes, Japanese/multiline/large
  values, decision-view-only parsing, default shell hiding, prompt/admission
  fingerprint parity, and rejected admission persistence.

## Contradictions

- Packet `06-tools-protocol/admission.md` requires rejection for repeated
  identical calls for the same unresolved state. I found no corresponding check
  in `runtime_admission.rs` or `admission_bridge.rs`.
- Packet `06-tools-protocol/admission.md` requires rejection for non-idempotent
  tools hidden by recovery policy. I found docs mentioning recovery policy, but
  no admission-time recovery-policy field check in the candidate files.
- `docs/tools/toolset-view-and-admission.md` says admission runs budget
  remaining, state suppressors, and recovery constraints. Candidate admission
  currently checks only envelope/view/schema/placeholder/value-class/path.
- `docs/tools/toolset-view-and-admission.md` says a prompt/admission mismatch is
  a high-severity runtime event. I found rejection persistence and recovery
  facts, but no distinct high-severity runtime event for mismatch.
- `docs/engine/turn-cycle.md` says persistence commits events, state patches,
  checks, usage, observations, admissions, and decision settlement together.
  `daemon.rs` currently performs multiple persistence calls in sequence; this
  may be acceptable today, but the wording overstates atomicity unless the
  lower-level calls share a transaction elsewhere.
- `docs/current-state.md` says prompt rendering does not render the global tool
  catalog. This is true for `render_prompt_for_decision`, but
  `parse_expected(StepKind::Explore, ...)` still uses full `explore_tool_view()`
  for legacy/non-decision parsing. Tests should keep this legacy path from being
  used as runtime authority.

## Exact Docs Edits

- No docs edited by this lane.
- If implementing the missing contract, update
  `docs/tools/toolset-view-and-admission.md` to specify exactly where repeat
  call suppression, budget remaining, state suppressors, and recovery-policy
  hiding are enforced, or narrow the admission section to match current code.
- Update `docs/engine/turn-cycle.md` if commit atomicity is not actually one DB
  transaction spanning prompt/exchange/admission/effects/settlement.
- Update `docs/current-state.md` only after tests prove the missing admission
  constraints or after narrowing the claimed contract.

## Exact Source Edits

- No source edited by this lane.
- If implementing the missing contract, add admission context to
  `runtime_admission.rs`, for example a pure input containing previous
  unresolved admissions, budget remaining, selected state suppressors, and
  recovery policy.
- Extend `admission_bridge.rs` to load that context from durable rows before
  calling admission, while keeping `admit_action` pure or adding a new pure
  `admit_action_with_context`.
- Add explicit mismatch/runtime-event persistence where a parsed action was not
  visible in the prompt view, if that is distinct from the existing rejected
  `tool_admissions` row plus recovery fact.
- Keep `parse_expected_for_decision()` as the runtime parser authority; avoid
  routing daemon/runtime model output through legacy `parse_expected()`.

## Tests To Add Or Update

- Add a core admission test that a repeated identical non-settled action for the
  same decision/state key is rejected with a stable reason.
- Add an app integration test proving the second identical unresolved action
  persists a rejected `tool_admissions` row and does not dispatch effects.
- Add a recovery-policy test proving a tool hidden by recovery policy is absent
  from the prompt view and rejected if emitted anyway.
- Add a budget/suppressor admission test or narrow docs if those checks are not
  intended admission responsibilities.
- Add a test for high-severity mismatch event persistence, or remove/narrow that
  doc claim.
- Keep existing tests: `crates/lkjagent-core/tests/tool_call.rs`,
  `tool_call_edges.rs`, `parse_contract.rs`, `admission.rs`,
  `render_tool_cards.rs`, `crates/lkjagent-app/tests/explore.rs`,
  `admission_rejection.rs`, and `tool_views.rs`.

## Commands To Run

- `cargo test -p lkjagent-core --test tool_call`
- `cargo test -p lkjagent-core --test tool_call_edges`
- `cargo test -p lkjagent-core --test parse_contract`
- `cargo test -p lkjagent-core --test admission`
- `cargo test -p lkjagent-core --test render_tool_cards`
- `cargo test -p lkjagent-app --test explore`
- `cargo test -p lkjagent-app --test admission_rejection`
- `cargo test -p lkjagent-app --test tool_views`
- `cargo run -p lkjagent-xtask -- check-docs`
- `cargo run -p lkjagent-xtask -- check-lines`
- `cargo run -p lkjagent-xtask -- quiet verify`
- `docker compose run --rm verify`

## Risks

- Admission currently accepts context-free actions once parser/view/schema/value
  checks pass; repeated unresolved effects may be possible unless later layers
  prevent them.
- Splitting parse and admission means placeholder rejection happens after parse;
  this is fine only if all effects remain gated behind persisted admission.
- Legacy synthetic explore parsing can mask decision-view issues in tests that
  do not use `parse_expected_for_decision()`.
- The docs may promise stronger atomic persistence and recovery-policy gating
  than the candidate source currently enforces.

## Acceptance Items Affected

- Final gates: `check-docs`, `check-lines`, focused tool protocol tests,
  `quiet verify`, and Docker Compose verification.
- Required evidence: focused tool protocol coverage, proof bundle/run directory,
  sanitized SQLite evidence for `tool_admissions`, and no unchecked final ledger
  items.
- Stop condition: cannot claim the packet admission contract is complete until
  repeat-call, recovery-policy, budget/suppressor, and mismatch-event claims are
  either implemented with tests or narrowed in docs with evidence.
