# No MCP

## Purpose

Fix the boundary against the Model Context Protocol and tool-server
ecosystems in general.

## Decision

lkjagent never implements MCP, neither as client nor as server. Every
capability beyond the fixed toolset arrives as a skill: a markdown file that
teaches the model to use shell commands, scripts, or files. The skill system
is owned by [../architecture/skills/README.md](../architecture/skills/README.md).

## Consequences

- Zero protocol surface to track, no server lifecycle management, no
  capability negotiation, no per-server context tax.
- Tool descriptions never flood the window; a skill index line costs a few
  tokens and the body loads only on demand, which preserves both the budget
  in [../architecture/context/budgets.md](../architecture/context/budgets.md)
  and the prefix cache.
- Integrations are owned and refined by the agent itself during
  self-maintenance, instead of being pinned to third-party server quality.
- Anything reachable from a shell in the container is reachable by the agent;
  the sandbox, not a protocol, is the boundary.

## Rejected Directions

- MCP client support: each connected server injects tool schemas into every
  prompt, which a 32k window cannot afford, and pulls in a JSON-RPC stack the
  rest of the design deliberately avoids.
- A bespoke plugin API: same costs as MCP without the ecosystem; skills on
  shell already cover the need.
