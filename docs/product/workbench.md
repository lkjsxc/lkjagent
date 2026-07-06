# Workbench

## Purpose

Define the owner terminal workbench without making it a second runtime
controller.

## Command

`lkjagent workbench [--mode append|pane]` attaches to the configured data
directory. It does not start or stop the daemon. The Docker entrypoint routes the
command the same way it routes `status`, `console`, and `watch`.

## Modes

`append` is the default safest mode. It prints immutable refresh cards to the
primary screen. Plain terminal scrollback, tmux copy mode, and saved command
output remain usable.

`pane` is an explicit framed primary-screen renderer. It groups the same durable
rows into transcript, right-rail, and input-hint panes without raw terminal mode
or alternate-screen ownership. A later raw terminal pane may use terminal
features only after docs and tests make that mode explicit.

Each refresh includes bounded sections:

- status fields from `lkjagent status`;
- recent durable events;
- the active task trace or `task: none`;
- active decision, prompt, context, tool-view, and proof counts when present;
- a prompt hint for owner input.

## Input

Owner input must remain available while progress refreshes. Plain text enqueues
an owner message. Slash commands reuse the console handlers for `/status`,
`/watch`, `/log`, `/queue`, `/task`, `/send TEXT`, `/new TEXT`, and `/quit`.
`/mode append` and `/mode pane` switch render modes without touching daemon
state. `/scroll up`, `/scroll down`, `/scroll top`, `/page up`, and `/page down`
move pane scroll state only. `/follow on` returns the transcript window to the
latest rows; `/follow off` leaves manual scroll in place. The loop opens short
store operations per input or refresh.

## Authority Limits

The workbench never selects runtime decisions, mutates hidden state, stores a
private transcript, or interprets completion. Mutations go through queue,
context-resolution, record, or other row-backed command paths.

## Evidence

Tests should cover parser routing, reducer mode changes, line handling,
closed-input exit, pane scroll and follow state, status rail fallback fields, and
bounded rendering. Interactive behavior is proven by captured command logs under
`tmp/agent-runs/` or `tmp/live-runs/`, with unavailable terminals recorded as an
honest skip.
