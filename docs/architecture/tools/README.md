# Tools

## Purpose

The fixed toolset the dispatcher executes: file access, workspace summaries,
direct verification gates, document topology tools, shell escape hatch,
queue operations, memory operations, graph operations, and task control.
There is no plugin surface and no MCP client per
[../../decisions/no-mcp.md](../../decisions/no-mcp.md).
The registry table in [registry.md](registry.md) is the single source for
dispatch validation and for the registry section of the system prompt, so
prompt and behavior cannot drift.

## Table of Contents

- [registry.md](registry.md): the canonical tool table, dispatch order, and the observation frame shape.
- [fs.md](fs.md): fs.read, fs.write, fs.edit, fs.list, fs.search, fs.stat, fs.mkdir, and fs.batch_write.
- [workspace.md](workspace.md): workspace.summary and repository-shape output.
- [verification-tools.md](verification-tools.md): verify.cargo and verify.xtask direct gates.
- [doc-tools.md](doc-tools.md): doc and artifact topology tools.
- [artifact-addresses.md](artifact-addresses.md): root, path, and artifact address rules.
- [shell.md](shell.md): shell.run, the escape hatch and its capture rules.
- [queue-ops.md](queue-ops.md): queue.list and queue mutation actions.
- [memory-ops.md](memory-ops.md): memory.save, memory.find, and memory.prune contracts.
- [graph-ops.md](graph-ops.md): graph.state, graph.plan,
  graph.transition, graph.context, graph.note, graph.evidence, and graph.compact.
- [control.md](control.md): agent.done and agent.ask, the actions that close or suspend a task.
