# Responsiveness

## Architecture

Run terminal input, store subscription, model draft updates, and command effects
as separate producers into one bounded UI event channel. The pure reducer must
never perform SQLite, filesystem, endpoint, or shell work.

## Input

Composer edits apply immediately. Japanese IME, grapheme movement, multiline
submit, resize, and interrupt remain responsive while the daemon works.

## Refresh

Subscribe to transcript sequence and bounded status projections. Coalesce
redundant refresh notifications, but never drop owner input or final messages.
Slow proof and workspace panels refresh independently from conversation.

## Backpressure

Combine rapid model deltas by logical draft ID. Preserve the latest draft and
all final or error events. Bound diagnostics history and load older rows on
demand.

## Metrics

Capture p50 and p95 key-to-render latency, refresh time, store query time, event
queue depth, dropped coalescible events, and model delta rate.
