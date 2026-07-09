# Transcript Model

## Purpose

Define one durable conversation source for console, workbench, and proof.

## Identity

Each owner or agent message has a globally unique logical ID, monotonic
sequence, immutable body and fingerprint, lifecycle, causal event, and optional
replacement relation. Replacement preserves logical ordering and leaves one
current rendering.

## Ordering

Views order messages by sequence and logical ID, never by arrival timestamp.
Refreshes merge by logical ID. Pagination resumes before a known sequence and
cannot duplicate rows already present.

## Separation

Owner input and terminal agent messages are conversation. Slash commands,
status, decisions, tool calls, diagnostics, checks, and errors use separate
surfaces and tables. The TUI never promotes a generic runtime event into an
agent message.

## Drafts

Composer and uncommitted agent draft state are local presentation state. A
durable final message replaces its matching draft by identity. Restart may
restore owner composer bytes from an explicit local draft record, but it cannot
invent a committed conversation row.
