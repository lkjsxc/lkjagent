# CLI

## Purpose

Define the command-line contract for operating lkjagent. The CLI is the only
owner surface besides files in the local data directory. It reads and writes the
SQLite store directly; there is no socket, HTTP server, web UI, or product MCP.

## Status

The command tree, metadata-rendered help, `watch` terminal command, explicit
`console` command, task inspection, queue inspection, shared status deck,
prefixed status facts, model-log display, and cumulative token accounting are
implemented. The current blocker ledger is closed; later CLI work must be
attached to a new current-work page before code changes.

## Table of Contents

- [commands.md](commands.md): accepted command groups, arguments, and target names.
- [status.md](status.md): stable `lkjagent status` sections and fields.
- [console.md](console.md): interactive terminal watch behavior.
- [token-output.md](token-output.md): cumulative token accounting display.
- [exit-codes.md](exit-codes.md): command success, usage, and failure codes.

## Global Rules

- `lkjagent --help` and `lkjagent help` print help without loading config.
- `--data DIR` is accepted before or after the command group.
- `--` ends option parsing for command text that starts with `--`.
- Snapshot commands print plain text and one fact per line.
- Interactive terminal output may use ANSI text but remains local only.
- Exit code `0` means the CLI command succeeded, not that an owner task
  completed.
- Missing endpoint usage renders as `unknown`, never as zero.

## Operator Flow

```sh
lkjagent run
lkjagent send "Create hello.md with a short hello."
lkjagent status
lkjagent log --limit 20
lkjagent watch
```

Inside Docker, the same contract is used through `docker compose run --rm agent
<command>`. The resident daemon is still one process and one loop.
