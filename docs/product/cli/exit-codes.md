# Exit Codes

## Purpose

Define CLI process exit codes and their meaning.

## Codes

| Code | Meaning |
| --- | --- |
| `0` | The CLI command completed its own work. |
| `1` | Runtime, store, configuration, or filesystem failure. |
| `2` | Usage error: missing command, unknown command, bad option, or bad argument. |

Exit code `0` does not mean an owner task succeeded. Task success is visible in
the transcript, authority ledger, artifact audit, and completion gate evidence.

## Output Streams

- Successful snapshot commands print to stdout.
- Usage and failure messages print to stderr.
- Help text prints to stdout.
- Commands that enqueue work print the durable id to stdout and return `0` once
  the queue mutation is stored.

## Examples

```text
lkjagent help             -> 0
lkjagent send "hello"     -> 0 after durable enqueue
lkjagent missing-command  -> 2
lkjagent status           -> 1 when the store cannot be opened
```

## Tests

CLI tests must assert exit code, stdout, and stderr for help, missing command,
unknown command, bad option, store failure, and successful snapshot cases.
