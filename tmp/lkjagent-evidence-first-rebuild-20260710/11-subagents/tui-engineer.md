# TUI Engineer

## Objective

Render one causal conversation exactly once with correct scrolling.

## Work

- canonical conversation schema and sequence;
- transactional owner, question, draft, and final messages;
- remove queue and event synthesis;
- shared transcript, pagination, viewport, wrapping, and reducer core;
- visual-row scroll bounds and follow;
- background input, store, command, and draft producers;
- quiet ordinary view.

## Tests

Tied timestamps, two matters, identical text with distinct IDs, duplicate event
text, streaming replacement, restart, more than forty rows, Japanese wrapping,
resize, manual anchor, bottom follow, and slow store latency.

## Output

Run real PTY trace. Do not use snapshot-only tests to claim interactive success.
