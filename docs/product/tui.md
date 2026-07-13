# Terminal UI

## Purpose

Define one responsive workbench with exact-once messages and bounded scrolling.

## Canonical Conversation

The TUI reads `conversation_messages` ordered by monotonic sequence and logical
ID. It never builds conversation from queue plus generic events. Owner/final
messages have immutable bodies, lifecycle, causal event, and replacement link.

Local drafts use the eventual durable identity. A committed row replaces its
draft by identity, never by text equality.

## Separation

Conversation contains only owner and final agent messages. Tool calls, state,
checks, diffs, recovery, and errors use a stable-ID activity pane collapsed by
default. Queue/status chatter does not appear as conversation.

A frame loads conversation, activity, and status in one deferred SQLite read
transaction on one connection. The first native frame projection caps a page at
100 conversation rows and 200 activity rows, caps each conversation body at
16,384 bytes with an explicit truncation flag, and supports older-page cursors.

Activity exposes only stable source-qualified IDs, fixed source kinds, matter
IDs, constrained statuses, and monotonic ordering. It reads decisions, provider
exchanges, admissions, effects, observations, checks, and state cells. State
cell identity material is fingerprinted. Provider request and response refs,
prompts, parsed calls, payloads, observations, check measurements, and error
fields are never projected. Status reports matter lifecycle, unfinished runtime
work, rejected or failed rows, current and passing checks, and active cell
counts from the same transaction.

This slice is store-only and read-only. It adds no public command, terminal,
input, viewport, or rendering behavior. A later TUI may commit owner input and
durable control events but never selects runtime work.

## Input

Endpoint and filesystem work run outside the terminal reducer. Japanese text,
emoji, cursor position, and unsubmitted bytes survive refresh, activity, resize,
search, scrolling, and slow model calls. Submit clears the composer only after
durable intake succeeds.

Initial behavior does not persist an unsubmitted draft across process exit. A
restart reloads committed messages once.

## Viewport

One pure layout implementation wraps grapheme clusters against actual inner pane
width. The viewport is either Follow or a logical message plus wrapped-row
offset. Follow shows the newest row. Manual mode preserves its anchor while
content arrives or width changes.

Scrolling to computed bottom restores Follow. Resize, search, pagination, and
content shrink clamp the anchor. Existing matching messages never produce an
all-blank pane.

## Evidence

Focused tests cover identity merge, read-transaction frames, Unicode wrapping,
follow/manual transitions, resize, search, and composer preservation. Final proof
uses a real PTY during a slow call with Japanese input, resize, manual append,
bottom restoration, search clear, and post-submit restart.
