# Non-Goals

## Excluded

- MCP support, plugin marketplaces, browser automation, or product subagents.
- Approval prompts inside the daemon container.
- A second task engine beside the state reducer.
- Global tool exposure to the model.
- A cloud-only workspace or opaque memory database.
- Automatic bulk scaffolding of hypothetical folders.
- Compatibility branches for the current bridge schema.
- Endless model calls when no owner work or due maintenance exists.
- Self-improvement that edits product policy without an owner request.

## Allowed Boundaries

External coding agents may use subagents while developing the repository.
Scripted endpoint fixtures may test deterministic behavior, but final daily-use
acceptance requires real endpoint sessions and real filesystem, SQLite, and PTY
effects.
