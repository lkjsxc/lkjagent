# Workbench

## Purpose

Define the owner terminal workbench without making it a second runtime
controller.

## Command

`lkjagent workbench` attaches to the configured data directory. It does not start
or stop the daemon. The Docker entrypoint routes the command the same way it
routes `status`, `console`, and `watch`.

## Layout

The first useful implementation is a normal-screen loop, not an alternate-screen
full-screen application. Each refresh prints bounded sections:

- status fields from `lkjagent status`;
- recent durable events;
- the active task trace or `task: none`;
- active decision, prompt, context, tool-view, and proof counts when present;
- a prompt hint for owner input.

Plain terminal scrollback, tmux copy mode, and saved command output remain
usable.

## Input

Owner input must remain available while progress refreshes. Plain text enqueues
an owner message. Slash commands reuse the console handlers for `/status`,
`/watch`, `/log`, `/queue`, `/task`, `/send TEXT`, `/new TEXT`, and `/quit`.
The loop opens short store operations per input or refresh.

## Authority Limits

The workbench never selects runtime decisions, mutates hidden state, stores a
private transcript, or interprets completion. Mutations go through queue,
context-resolution, record, or other row-backed command paths.

## Evidence

Tests should cover parser routing, line handling, closed-input exit, and bounded
rendering. Interactive behavior is proven by captured command logs under
`tmp/agent-runs/` or `tmp/live-runs/`, with unavailable terminals recorded as an
honest skip.
