# TUI Verification

## Pure Tests

- logical message replacement and exactly-once rendering;
- causal sequence under tied timestamps;
- Unicode wrapping and cursor placement;
- scroll bounds for empty, short, long, and shrinking content;
- bottom follow across append and resize;
- manual anchor preservation;
- diagnostic exclusion.

## Store Integration

Run real queue intake, daemon decisions, crash resume, and transcript queries.
Assert one owner and one final agent row per logical exchange. Inject duplicate
event text and prove it does not create duplicate conversation messages.

## PTY Campaign

Use a real pseudo-terminal for at least fifteen minutes. Schedule owner input
throughout, including during a slow endpoint call. Exercise scroll, resize,
Japanese input, follow restoration, slash commands, and restart.

## Automated Trace

Record input timestamp, render timestamp, viewport size, top, max_top, follow,
anchor, visible logical IDs, and screen hash. The gate rejects duplicate IDs,
causal inversions, top above max_top, bottom-anchor drift, or excessive input
latency.

## Manual Inspection

Capture a bounded terminal recording for human review, but never use visual
approval as a substitute for trace invariants.
