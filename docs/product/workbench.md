# Workbench

## Purpose

Define the owner terminal workbench without making it a second runtime
controller.

## Command

`lkjagent workbench [--mode append|pane]` attaches to the configured data
directory. It does not start or stop the daemon. The Docker entrypoint routes
the command the same way it routes `status`, `console`, and `watch`.

## Pure TUI Core

The app has a pure terminal model, event reducer, grapheme-aware composer, and
non-TTY renderer. It preserves composer input while agent, tool, state,
artifact, resize, interrupt, approval, save, and quit events arrive. Terminal
backends are effects at the edge.

## Modes

`append` is the default safest mode. It prints immutable refresh cards to the
primary screen. Plain terminal scrollback, tmux copy mode, and saved command
output remain usable.

`pane` is an explicit framed primary-screen renderer. It groups durable rows
into transcript, right-rail, and input-hint panes without raw terminal mode or
alternate-screen ownership unless that mode is explicitly selected and tested.

Each refresh includes bounded sections:

- status fields from `lkjagent status`;
- durable transcript events from owner, lkjagent, tools, records, and artifacts;
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
latest rows; `/follow off` leaves manual scroll in place.

## Japanese And Mixed-Width Text

The composer stores UTF-8 text plus a byte cursor that is clamped to Unicode
grapheme boundaries. Insert, backspace, left, right, multiline, and submit
operations do not split Japanese characters, emoji graphemes, or IME-composed
text. Tests cover Japanese strings, emoji, ASCII, newline handling, and durable
transcript saves.

## Authority Limits

The workbench never selects runtime decisions, mutates hidden state, stores a
private transcript, or interprets completion. Mutations go through queue,
context-resolution, record, or other row-backed command paths.

## Evidence

Tests cover parser routing, reducer mode changes, line handling, closed-input
exit, grapheme cursor movement, pane scroll and follow state, durable transcript
merge, status rail fallback fields, and bounded rendering. Interactive behavior
is proven by captured command logs under `tmp/agent-runs/` or `tmp/live-runs/`,
with unavailable terminals recorded as an honest skip.
