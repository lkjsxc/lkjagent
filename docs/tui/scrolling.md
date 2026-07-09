# Scrolling

## Purpose

Define stable terminal layout and interactive evidence requirements.

## Wrapping

Wrap grapheme clusters against the transcript viewport width. Compute each
message height independently before deriving total rows, top, and `max_top`.
Narrow widths may wrap long ASCII and Japanese text without splitting a
grapheme or drawing outside the pane.

## Anchors

Follow mode anchors the viewport to the newest row. Manual scroll stores a
logical message and wrapped-row offset, so incoming rows and resize preserve
the visible content. Reaching the bottom restores follow. Empty ranges clamp to
zero and never render blank space while messages exist.

## Input

Agent, tool, state, resize, refresh, and paging events do not erase or move the
owner composer. Input remains responsive while endpoint and store effects run
outside the pure reducer.

## Evidence

Final acceptance uses a real PTY session against frozen source. The trace binds
visible logical IDs and sequences to SQLite, records resize and scroll state,
captures Japanese input during a slow call, and replays screen hashes from the
terminal recording.
