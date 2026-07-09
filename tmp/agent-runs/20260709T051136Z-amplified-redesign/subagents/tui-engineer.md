# TUI Engineer Report

## Scope

- Packet lane: `tmp/lkjagent-yolo-amplified-thinking-packet-20260708/10-subagents/tui-engineer.md`.
- Required reads completed: `docs/current-state.md`, packet README, `07-tui/*.md`,
  `13-acceptance/tui-gates.md`, candidate source, and focused TUI tests.
- Scope honored: report only; no product docs or source edited.

## Current Facts

- `tui_snapshot::load` builds transcript entries from `queue` plus non-owner,
  non-answer `events`, sorted newest-first then reversed to old-to-new
  (`tui_snapshot.rs:52-82`).
- Snapshot transcript entry ids are source-table row ids: `queue:<id>` and
  `event:<id>`; source path records `sqlite:queue:<id>` or `sqlite:events:<id>`.
- Snapshot maps `stepdone`, `taskclosed`, and `question` to agent, blocked kinds
  to error, and other event kinds to state (`tui_snapshot.rs:99-106`).
- `tui_transcript::merged_entries` merges durable snapshot entries, session
  entries, and the current agent draft by exact id only (`tui_transcript.rs:49-67`).
- Streaming agent deltas accumulate in one `draft-agent` entry, then commit to a
  session id such as `agent:session:1` (`tui_transcript.rs:92-115`).
- Workbench reducer tracks `Viewport::Follow` vs `Viewport::Manual { top_line }`
  and clamps manual top line against `latest.lines().count() - visible_height`
  (`workbench_state.rs:24-28`, `131-162`).
- Workbench pane renderer splits `== section ==` blocks, sends `status` to the
  right rail, filters transcript by search, and windows by `state.scroll` or
  bottom-follow (`workbench_render.rs:23-51`, `62-103`).
- Current focused command passed:
  `cargo test -p lkjagent-app --test tui_snapshot --test tui_transcript_identity --test tui_state --test workbench_viewport`
  -> 1 + 12 + 4 + 3 tests passed.

## Contradictions

- Packet acceptance says owner messages do not duplicate across queue, event,
  and transcript sources. Current snapshot avoids owner event duplicates by
  excluding `events.kind IN ('owner','answer')`, and it does not read a third
  transcript source. There is no regression that inserts same owner text across
  all named sources.
- Packet identity says keys derive from source table, row id, kind, and event
  sequence. `TranscriptEntry` has only id/source/text/path, and current ids omit
  kind and event sequence.
- Product docs claim "duplicate suppression by stable row identity"; current
  merge suppresses only exact id equality. A local completed agent entry
  `agent:session:N` and later durable row `event:N` for the same answer can both
  render.
- Workbench state clamp uses raw `latest` line count, but pane rendering scrolls
  a section-split and search-filtered transcript. Search can shrink rendered
  lines below the reducer's max, allowing blank manual windows despite state
  clamping.
- Display cleanliness is partially true for pane mode because status is separated
  into the right rail. Append mode still prints the full watch body plus header
  booleans (`follow=`, `search=`), so "ordinary views" remains ambiguous.

## Exact Docs Edits

- None made in this report-only lane.
- Later implementation should update `docs/product/workbench.md` only after code
  and tests prove the exact duplicate, ordering, debug hiding, and scroll claims.
- If no implementation change is made, revise the Evidence paragraph to state
  the narrower truth: exact-id merge and reducer-level scroll coverage, not
  cross-source owner/durable-agent duplicate suppression.

## Exact Source Edits

- None made in this report-only lane.
- Likely source edits for a follow-up implementation:
  - `tui_types.rs`: extend transcript identity or add durable sequence metadata
    without exceeding the 200-line file limit.
  - `tui_snapshot.rs`: select and carry a stable sequence/order key; decide
    whether owner/answer events are hidden or deduped by source relationship.
  - `tui_transcript.rs`: merge durable and ephemeral agent messages using a
    stable logical key, or remove completed local echoes once durable rows arrive.
  - `workbench_state.rs` and `workbench_render.rs`: share the rendered line-count
    calculation used for clamp/window, especially after section filtering/search.
  - `workbench_render.rs`: clarify or remove ordinary append-mode debug/header
    fields if the owner view must hide them by default.

## Tests To Add Or Update

- Add snapshot test with same owner content represented as queue row, owner event,
  and any transcript artifact/source that the product treats as transcript.
- Add TUI merge test where `agent_draft` or completed `agent:session:N` and a
  durable `event:M` for the same model answer are present; expected one line.
- Add ordering test for same timestamp rows using durable sequence when available,
  with source priority only as fallback.
- Add workbench render/state test for search-filtered pane scroll: manual scroll
  beyond filtered line count should clamp to a nonblank final window.
- Add empty, one-line, exact-height, resize-smaller, and resize-larger viewport
  tests named in `07-tui/scrolling.md`.
- Add ordinary-view cleanliness test asserting queue internals/debug booleans are
  absent from the primary transcript view selected by the docs.

## Commands To Run

- `cargo test -p lkjagent-app --test tui_snapshot --test tui_transcript_identity --test tui_state --test workbench_viewport`
- `cargo test -p lkjagent-app workbench_render::tests`
- `cargo test -p lkjagent-app`
- `cargo run -p lkjagent-xtask -- check-docs`
- `cargo run -p lkjagent-xtask -- check-lines`
- `cargo run -p lkjagent-xtask -- quiet verify`
- Final completion claim only: `docker compose run --rm verify`

## Risks

- Deduping by text would hide genuinely separate repeated owner turns; use stable
  source relationship or sequence-aware identity, not content-only equality.
- Adding transcript metadata can push small files over the 200-line limit unless
  tests or helpers are split deliberately.
- Treating append-mode debug as forbidden may conflict with its current purpose
  as plain refresh cards; docs should define whether append is ordinary view or
  diagnostic refresh.
- Scroll correctness depends on the same projection being used for clamp and
  rendering; duplicating projection logic in state/render can regress again.

## Acceptance Items Affected

- Owner messages do not duplicate: partially covered, not fully proven.
- Agent messages do not duplicate across draft/durable rows: not proven for real
  different durable/session ids.
- Rows render in durable order: partially covered by timestamp/id order, not
  durable sequence.
- Internal queue debug text hidden: partially covered in pane, ambiguous in append.
- Bottom scroll clamped: reducer covered for raw body, not rendered filtered pane.
- Follow mode anchored on new rows: covered by focused render tests.
- Manual scroll preserved until bottom: covered by focused reducer/render tests.
- Resize does not create blank overscroll: not specifically covered.
