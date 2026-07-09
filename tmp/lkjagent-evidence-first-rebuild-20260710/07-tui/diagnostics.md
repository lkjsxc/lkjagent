# Diagnostics

## Ordinary Conversation

Show only owner messages, agent reports, and explicit questions. Hide queue
internals, state counters, follow flags, debug transitions, tool traces, step
events, and proof counts.

## Compact Status

A quiet status line may show current matter title and a human phrase such as
working, waiting for your answer, retrying endpoint, or no current work. Do not
show state: queue: 1 new=false or similar implementation text.

## Detail Panes

Tool, state, context, workspace, proof, and error details are opt-in panes.
They read durable projections and link to raw evidence. Errors show bounded
owner-actionable summaries.

## Commands

Slash commands do not become conversation messages. Their output appears in a
temporary command panel unless the command explicitly creates a record or owner
turn.

## Line Mode

Line mode follows the same filtering. It must not print queue debug after every
submit.
