# Decisions

## Purpose

Durable decision records for lkjagent. Each record states the decision, its
consequences, and the rejected directions, so no agent relitigates settled
ground. A decision changes only by editing its record in the same commit as
the change it permits.

## Record Shape

Every record uses the headings: Purpose, Decision, Consequences, Rejected
Directions. Records state the current contract directly and never describe
historical evolution.

## Table of Contents

- [rust-workspace.md](rust-workspace.md): Rust cargo workspace of small focused crates.
- [openai-endpoint.md](openai-endpoint.md): one OpenAI-compatible HTTP endpoint, no embedded inference.
- [xml-action-protocol.md](xml-action-protocol.md): tag-based action output, never JSON.
- [append-only-context.md](append-only-context.md): append-only context with explicit compaction.
- [single-loop.md](single-loop.md): one continuous agent loop, no sessions.
- [sqlite-store.md](sqlite-store.md): one SQLite file for queue, transcripts, memory, state.
- [no-mcp.md](no-mcp.md): no MCP; skills and shell carry every capability.
- [no-subagents.md](no-subagents.md): no sub-agents or worker forks.
- [container-first.md](container-first.md): the harness lives inside the container.
- [unified-skills.md](unified-skills.md): one skill format for the harness and its builders.
