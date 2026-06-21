# Scope

## Purpose

This file draws the boundary of lkjagent. Work inside the boundary is
welcome; work outside it is rejected regardless of quality.

## In Scope

- One Rust daemon with one continuous agent loop.
- A persistent user message queue written by a thin CLI.
- An OpenAI-compatible HTTP client for one local endpoint at a time.
- The tag-based action protocol and its strict parser.
- The append-only context engine with explicit compaction.
- A SQLite store for queue, transcripts, memory, and runtime state.
- A typed state graph shared by runtime, context, memory, compaction, and
  completion.
- The fixed toolset: file read, write, edit, shell, queue ops, memory ops,
  graph ops, and control actions. Everything else is graph guidance built on
  shell and files.
- Container-first operation via docker compose.
- The xtask verification gates that keep this repository lawful.

## Builder Helpers

Implementation-time helpers supplied by a coding harness are allowed for
reading, planning, test design, or review. Product behavior may not depend on
external subagents, MCP servers, or parallel worker sessions. Adding product
subagents requires a deliberate scope change first.

## Out of Scope

- MCP in any form. Capabilities arrive as graph policy, not servers.
  Decision: [../decisions/no-mcp.md](../decisions/no-mcp.md).
- Sub-agents, worker forks, parallel sessions.
  Decision: [../decisions/no-subagents.md](../decisions/no-subagents.md).
- Messaging channels (Telegram, Discord, mail) and any web UI.
- Plan mode, permission prompts, or any non-YOLO interaction.
- Automatic idle self-maintenance, heartbeat checklists, and cron schedules.
  The daemon heartbeat exists only for lock reclaim.
- Model training, fine-tuning, or serving. The endpoint is someone else's job.
- Multi-user or multi-tenant operation. One owner, one store, one loop.

## Open Questions

None.
