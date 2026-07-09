# TUI Rebuild

## Task A: Conversation Schema

Add monotonic sequence, logical message ID, causal refs, draft/final state, and
replacement relation. Queue intake and report commit messages transactionally.

## Task B: Remove Synthesis

Stop interpreting queue rows and task event strings as conversation messages.
Remove stepdone hiding as the deduplication mechanism. Both backends query the
canonical table.

## Task C: Shared Core

Unify message merge, pagination, viewport, wrapping, scroll, follow, filtering,
and reducer logic. Keep only terminal IO backend differences.

## Task D: Scroll

Use actual wrapped display rows and viewport height. Fix up-from-bottom, clamp
stored state, preserve manual anchors, restore follow at max, and recompute on
resize.

## Task E: Responsiveness

Move SQLite, filesystem, command, and endpoint work off the UI reducer thread.
Coalesce only replaceable drafts and refresh notices.

## Task F: Quiet View

Remove queue debug after submit. Make diagnostics opt-in. Keep conversation
owner/agent/question only.

## Task G: Proof

Add store integration and real PTY campaign with Japanese input, slow call,
resize, scroll, restart, more than forty messages, and trace validation.
