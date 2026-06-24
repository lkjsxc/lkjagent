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
- [tag-action-protocol.md](tag-action-protocol.md): singular tag-based action output.
- [append-only-context.md](append-only-context.md): append-only context with graph-aware compaction.
- [single-loop.md](single-loop.md): one continuous agent loop, no sessions.
- [sqlite-store.md](sqlite-store.md): one SQLite file for queue, transcripts, memory, state.
- [state-graph-runtime.md](state-graph-runtime.md): typed graph cases govern planning and completion.
- [no-mcp.md](no-mcp.md): no MCP; graph-selected tools and shell carry capability.
- [no-subagents.md](no-subagents.md): no sub-agents or worker forks.
- [container-first.md](container-first.md): the harness lives inside the container.
