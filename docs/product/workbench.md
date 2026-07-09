# Workbench

## Purpose

Define the owner terminal workbench without making it a second runtime
controller.

## Command

`lkjagent workbench` attaches to the configured data directory. It does not
start or stop the daemon. The Docker entrypoint routes
the command the same way it routes `status`, `console`, and `watch`.

## Pure TUI Contract

The target has a pure terminal model, event reducer, grapheme-aware composer,
stable transcript-entry identity, agent draft accumulation, id-based transcript
merge, clamped follow/manual viewport windows, and non-TTY renderer. Transcript
views render owner/agent conversation entries only; tool, state, system, and
error diagnostics stay in status, side, or saved diagnostic surfaces. Slash
commands are commands, not owner transcript messages. The reducer preserves
composer input while agent, tool, state, artifact, resize, interrupt, approval,
save, and quit events arrive. Terminal backends are effects at the edge.

## Renderer

The workbench has one pane-oriented renderer. When stdin and stdout are TTYs it
uses the ratatui alternate-screen backend; otherwise it uses the line pane
renderer for scripts and tests. Its left pane is the owner conversation
transcript only. Diagnostic rows such as operation progress, matter trace, proof
counts, queue state, and recent events belong in the right rail or other
non-transcript panes.

Each refresh includes bounded sections:

- status fields from `lkjagent status`;
- durable owner turns and terminal lkjagent messages, deduplicated by row
  identity and ordered by matter causality when timestamps tie;
- pending `session:*` transcript rows are shadowed only by the matching durable
  row id or path after refresh, while distinct rows with identical text remain
  visible;
- the active matter trace or `matter: none`;
- active decision, prompt, context, tool-view, workspace, and proof counts;
- a prompt hint for owner input.

## Input

Owner input remains available while progress refreshes. Plain text enqueues an
owner turn. Slash commands reuse the console handlers for `/status`, `/watch`,
`/log`, `/queue`, `/matter`, `/record`, `/send TEXT`, `/new TEXT`, and `/quit`.
`/scroll up`, `/scroll down`, `/scroll top`, `/page up`, and `/page down` move
pane scroll state only. `/follow on` returns the transcript window to the
latest rows; `/follow off` leaves manual scroll in place. If the viewport is in
follow mode when a row arrives, the rendered bottom stays anchored. Manual scroll
is clamped to the rendered pane length so the owner cannot scroll into endless
blank space.

## Japanese And Mixed-Width Text

The composer stores UTF-8 text plus a grapheme cursor index. Insert, delete,
backspace, left, right, home, end, multiline, and submit operations do not split
Japanese characters, emoji graphemes, or IME-composed text. Cursor placement uses
Unicode display width instead of byte counts. Tests cover Japanese strings,
emoji, ASCII, newline handling, display width, and durable transcript saves.

## Authority Limits

The workbench never selects runtime decisions, mutates hidden state, stores a
private transcript, or interprets completion. Mutations go through queue,
context-resolution, record, or other row-backed command paths.

## Required Evidence

Focused tests cover parser routing, reducer mode changes, line handling, input
exit, grapheme cursor movement, pane scroll and follow state, scroll-down follow
restoration, pane bottom anchoring after growth, agent delta draft commit,
durable transcript merge, duplicate suppression by stable row identity, saved
ids and source paths, slash-command non-transcription, conversation-only
transcript display, clamped viewport windows, canonical transcript rendering
without operation duplicates, status rail fallback fields, and bounded
rendering. Final behavior requires the frozen-source PTY campaign defined in
[../tui/scrolling.md](../tui/scrolling.md); a skipped terminal run is a blocker.
