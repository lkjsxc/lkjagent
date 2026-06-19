# Tools

## Purpose

The fixed toolset the dispatcher executes: file access, shell execution,
queue operations, memory operations, skill operations, and task control.
Every other capability is a skill driving shell.run. There is no plugin
surface and no MCP client per [../../decisions/no-mcp.md](../../decisions/no-mcp.md).
The registry table in [registry.md](registry.md) is the single source for
dispatch validation and for the registry section of the system prompt, so
prompt and behavior cannot drift.

## Table of Contents

- [registry.md](registry.md): the canonical tool table, dispatch order, and the observation frame shape.
- [fs.md](fs.md): fs.read, fs.write, and fs.edit, the direct file-access contracts.
- [shell.md](shell.md): shell.run, the general-purpose escape hatch and its capture rules.
- [queue-ops.md](queue-ops.md): queue.list and queue mutation actions.
- [memory-ops.md](memory-ops.md): memory.save and memory.find, the durable-knowledge contracts.
- [skill-ops.md](skill-ops.md): skill.use and source-owned skill loading.
- [control.md](control.md): agent.done and agent.ask, the actions that close or suspend a task.
