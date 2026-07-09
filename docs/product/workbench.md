# Workbench

## Purpose

Define the owner terminal workbench without making it a second runtime
controller.

## Command

`lkjagent workbench [--mode append|pane]` attaches to the configured data
directory. It does not start or stop the daemon. The Docker entrypoint routes
the command the same way it routes `status`, `console`, and `watch`.

## Pure TUI Core

The app has a pure terminal model, event reducer, grapheme-aware composer,
stable transcript-entry identity, agent draft accumulation, id-based transcript
merge, follow/manual viewport state, and non-TTY renderer. It preserves composer
input while agent, tool, state, artifact, resize, interrupt, approval, save, and
quit events arrive. Terminal backends are effects at the edge.

## Modes

`append` is the default safest mode. It prints immutable refresh cards to the
primary screen. Plain terminal scrollback, tmux copy mode, and saved command
output remain usable.

`pane` is an explicit framed primary-screen renderer. Its left pane is the
owner conversation transcript only. Diagnostic rows such as step progress,
matter trace, proof counts, queue state, and recent events belong in the
right rail or other non-transcript panes.

Each refresh includes bounded sections:

- status fields from `lkjagent status`;
- durable owner turns and terminal lkjagent messages, deduplicated by row
  identity and ordered by matter causality when timestamps tie;
- pending `session:*` transcript rows are shadowed by matching durable rows after
  refresh, while distinct durable rows with identical text remain visible;
- the active matter trace or `matter: none`;
- active decision, prompt, context, tool-view, workspace, and proof counts;
- a prompt hint for owner input.

## Input

Owner input remains available while progress refreshes. Plain text enqueues an
owner turn. Slash commands reuse the console handlers for `/status`, `/watch`,
`/log`, `/queue`, `/matter`, `/record`, `/send TEXT`, `/new TEXT`, and `/quit`.
`/mode append` and `/mode pane` switch render modes without touching daemon
state. `/scroll up`, `/scroll down`, `/scroll top`, `/page up`, and `/page down`
move pane scroll state only. `/follow on` returns the transcript window to the
latest rows; `/follow off` leaves manual scroll in place. If the viewport is in
follow mode when a row arrives, the rendered bottom stays anchored.

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

## Evidence

Tests cover parser routing, reducer mode changes, line handling, closed-input
exit, grapheme cursor movement, pane scroll and follow state, scroll-down follow
restoration, pane bottom anchoring after growth, agent delta draft commit,
durable transcript merge, duplicate suppression by stable row identity, saved
ids and source paths, canonical transcript rendering without step/task duplicate
messages, status rail fallback fields, and bounded rendering.
Interactive behavior is proven by captured command logs under `tmp/agent-runs/`
or `tmp/live-runs/`, with unavailable terminals recorded as an honest skip.
