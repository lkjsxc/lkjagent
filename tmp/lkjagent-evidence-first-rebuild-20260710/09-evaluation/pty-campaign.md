# PTY Campaign

## Setup

Start the real daemon and workbench through a pseudo-terminal at a fixed initial
size. Subscribe to transcript and status from a fresh evidence directory.

## Script

Across at least fifteen minutes:

1. enter Japanese and ASCII owner turns;
2. type while a slow endpoint call is active;
3. scroll up during incoming messages;
4. resize narrow and wide;
5. scroll to bottom and verify follow restoration;
6. run quiet slash commands;
7. restart the workbench and daemon;
8. load older transcript pages;
9. finish with new rows while following.

## Trace

Capture monotonic time, input, visible logical IDs, transcript sequence, width,
height, transcript viewport height, independently wrapped row count, top,
max_top, follow, anchor, visible roles, composer hashes, input latency,
forbidden-diagnostic count, render duration, cast offset, and SHA-256 screen
hash. Redact owner text when necessary while preserving identity and timing.

Use stable event names for owner_input, resize, scroll_up, scroll_down,
Japanese input, slow-call input, slash commands, daemon_restart,
workbench_restart, and agent_update so the gate can prove each interaction
occurred.

result.tsv binds the source commit, terminal-operator scenario, unix-pty backend,
trace SHA-256, and terminal.cast SHA-256. The gate cross-checks every visible
logical message, sequence, and role against the captured SQLite backup.

The recorder includes PTY input events and full output frames in asciinema cast
format. A Rust replay test reconstructs screen bytes at trace offsets,
recomputes wrapping and hashes, and writes terminal-replay.tsv bound to source,
cast, trace, frame count, and zero mismatch counters. Final acceptance hashes
that receipt and reruns the replay node gate against the evidence root. Empty
casts or trace-only invented geometry fail.

## Gate

Reject duplicate visible logical IDs, causal inversion, blank-range scrolling,
lost composer text, follow drift, slash command in conversation, or excessive
p95 input latency.
