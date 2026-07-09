# TUI Gates

## Canonical Conversation

- Queue and internal event tables are not reinterpreted as messages.
- Every logical owner and final agent message appears once.
- Draft replacement survives refresh and restart without duplicate display.
- Identical text with distinct logical IDs remains distinct.

## Ordering

- One transactional sequence orders tied timestamps and multiple matters.
- Pagination and restart do not reshuffle existing messages.
- Final response follows its verified effects.

## Scroll

- top never exceeds max_top.
- Up from follow moves one visual row above bottom.
- New content keeps bottom anchored in follow mode.
- Manual scroll preserves an anchor and does not jump.
- Reaching bottom re-enables follow.
- Resize rewraps Japanese and long paragraphs correctly.
- max_top equals independently wrapped rows minus the transcript viewport,
  saturated at zero; implementation-reported blank range is not trusted.

## Ordinary View

No queue debug, state counters, tool traces, step rows, or slash commands appear
in conversation.

## PTY

Final-commit fifteen-minute trace has no duplicate ID, causal inversion, blank
scroll range, lost input, follow violation, or excessive input latency.
terminal.cast contains real PTY input/output for the full span, and Rust replay
reproduces trace screen hashes, wrapping, and geometry.
