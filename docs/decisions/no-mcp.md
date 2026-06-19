# No MCP

## Purpose

Fix the boundary against the Model Context Protocol and tool-server
ecosystems in general.

## Decision

lkjagent never implements MCP, neither as client nor as server. Capability
beyond the fixed toolset arrives as graph guidance that teaches the model to
use shell commands, scripts, or files through the fixed action protocol.

## Consequences

- Zero protocol surface to track, no server lifecycle management, no
  capability negotiation, no per-server context tax.
- Tool descriptions never flood the window; graph-selected context packages
  render only the phase-relevant instruction slice, preserving both the budget
  in [../architecture/context/budgets.md](../architecture/context/budgets.md)
  and the prefix cache.
- Integrations are owned and refined by the agent itself during
  self-maintenance, instead of being pinned to third-party server quality.
- Anything reachable from a shell in the container is reachable by the agent;
  the sandbox, not a protocol, is the boundary.

## Rejected Directions

- MCP client support: each connected server injects tool schemas into every
  prompt, which the runtime window cannot afford, and pulls in a JSON-RPC stack the
  rest of the design deliberately avoids.
- A bespoke plugin API: same costs as MCP without the ecosystem; graph
  guidance on shell already covers the need.
