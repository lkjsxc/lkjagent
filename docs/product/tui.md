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

The native frame projection remains store-only and read-only. The public `tui`
command polls it in one deferred transaction on the UI thread at bounded cadence.
The application merges older pages by exact message ID and sequence without
duplicates. Activity remains the newest bounded page and is not paginated yet.
The terminal edge commits owner input but never selects or dispatches runtime
work.

## Input

Endpoint and filesystem work run on a background worker constructed inside its
thread. The worker waits for bounded wake signals and never holds a frame read
transaction during endpoint I/O. Japanese text, paste, emoji, grapheme cursor
position, and unsubmitted bytes survive refresh, activity, resize, search,
scrolling, and slow model calls. Submit retains bytes until typed durable intake
returns the exact canonical message ID, then clears only the submitted revision.
Intake failure retains text and displays a bounded error.

Initial behavior does not persist an unsubmitted draft across process exit. A
restart reloads committed messages once.

## Viewport

The pure layout wraps grapheme clusters against the supplied inner pane width
using terminal display columns. The viewport is either Follow or a durable
anchor containing message ID and wrapped-row offset. Follow shows the newest
row. Manual mode preserves its anchor while content arrives or width changes.

Scrolling to computed bottom restores Follow. Scrolling above the loaded top
loads the next older conversation page and preserves the logical message and
wrapped-row anchor. Resize, search, pagination, and content shrink clamp the
anchor. Existing matching messages never produce an all-blank pane.

Ctrl-F enters search and Escape clears it without changing the composer. F2
toggles activity. Arrow and page keys scroll, End restores Follow, and Ctrl-C
exits. Character, paste, backspace, delete, cursor, Home, and End input are
grapheme safe. Raw mode, alternate screen, and cursor state use an unwind-safe
terminal guard.

## Evidence

Focused store and application tests cover identity merge, read-transaction
frames, Unicode wrapping and composer edits, durable submit outcomes, renderer
clipping and separation, terminal cleanup, pagination merge, input while a test
endpoint is blocked, and native binary entry/cleanup in a Unix PTY. Final proof
still uses a configured endpoint PTY during a slow call with
Japanese input, resize, manual append, bottom restoration, search clear, and
post-submit restart.
