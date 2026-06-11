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
- A markdown skill system shared by the harness and its builders.
- The fixed toolset: file read, write, edit, shell, memory ops, skill ops,
  and control actions. Everything else is a skill built on shell.
- An idle self-maintenance loop that distills memory and refines skills.
- Container-first operation via docker compose.
- The xtask verification gates that keep this repository lawful.

## Out of Scope

- MCP in any form. Capabilities arrive as skills, not servers.
  Decision: [../decisions/no-mcp.md](../decisions/no-mcp.md).
- Sub-agents, worker forks, parallel sessions.
  Decision: [../decisions/no-subagents.md](../decisions/no-subagents.md).
- Messaging channels (Telegram, Discord, mail) and any web UI.
- Plan mode, permission prompts, or any non-YOLO interaction.
- Heartbeat checklists and cron schedules. Idle time belongs to
  self-maintenance; scheduled wakeups are a separate concern this project
  does not take on.
- Model training, fine-tuning, or serving. The endpoint is someone else's job.
- Multi-user or multi-tenant operation. One owner, one store, one loop.

## Open Questions

- Whether the self-maintenance loop should ever propose scope changes to its
  owner. Status: open question, revisit after the loop runs for a month.
