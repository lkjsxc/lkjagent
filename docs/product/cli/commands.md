# Commands

## Purpose

List the accepted command tree and what each command observes or mutates.

## Global Form

```text
lkjagent [--data DIR] <group> [command] [options]
```

`--data DIR` selects the runtime data directory. Help commands are read before
configuration so a broken config cannot hide usage text.

## Daemon And Queue

| Command | Behavior |
| --- | --- |
| `lkjagent run` | Start the resident daemon in the foreground. |
| `lkjagent send <text>` | Append an owner message and print its queue id. |
| `lkjagent queue list [--limit N]` | Print queued owner messages by state. |
| `lkjagent queue show <id>` | Print one queued owner message. |

`send` is durable without a running daemon. It does not wait for completion.

## Status And Transcript

| Command | Behavior |
| --- | --- |
| `lkjagent status` | Print the stable status fields in [status.md](status.md). |
| `lkjagent log [--limit N] [--full] [--follow]` | Print transcript events. |
| `lkjagent watch` | Open the interactive terminal screen in [console.md](console.md). |

The current binary uses `console` for the interactive screen. The CLI core task
must either move the parser to `watch` or list `console` as an explicit accepted
name in this file before claiming the command tree implemented.

## Work Inspection

| Command | Behavior |
| --- | --- |
| `lkjagent task list [--status S] [--limit N]` | Print durable task cases. |
| `lkjagent task show <id>` | Print one task case, authority ids, and evidence gaps. |
| `lkjagent graph` | Print the active graph case and source graph summary. |
| `lkjagent memory <query>` | Search distilled memory entries. |

Task and queue inspection commands are read-only store queries.

## Model Logs

| Command | Behavior |
| --- | --- |
| `lkjagent model-log` | Print the current model handoff path. |
| `lkjagent model-log --print` | Print the current Markdown handoff content. |
| `lkjagent model-log list [--limit N]` | List provider exchange summaries. |
| `lkjagent model-log show --case C --turn T` | Print one exchange summary. |
| `lkjagent model-log export --case C --turn T` | Export one Markdown handoff. |
| `lkjagent model-log raw-case --case C [--limit N]` | Print raw case exchange rows. |

## Personal Records

| Command | Behavior |
| --- | --- |
| `lkjagent personal list [--kind K] [--status S] [--project P] [--limit N]` | Inspect personal records. |
| `lkjagent personal render` | Regenerate bounded personal Markdown projections. |

Personal commands are local store operations. They do not add schedules,
heartbeats, or cron behavior.

## Help

- `lkjagent help` prints the command summary.
- `lkjagent help <group>` prints group usage.
- Unknown commands print usage on stderr and exit with code `2`.
