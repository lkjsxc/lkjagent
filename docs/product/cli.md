# CLI

## Purpose

Describe the thin command-line client. The CLI is the only user surface of
lkjagent. It uses lkjagent-store directly; there is no socket, no HTTP, and
no IPC protocol.

## Commands

| Command | Behavior |
| --- | --- |
| `lkjagent run` | Start the daemon in the foreground. Exactly one per store. |
| `lkjagent send <text>` | Append a user message through lkjagent-store; print its queue id. |
| `lkjagent status` | Print daemon state, queue depth, open task, question, error, context usage. |
| `lkjagent log` | Print recent transcript events; `--follow` tails new ones. |
| `lkjagent console` | Open an interactive owner console with transcript, bottom status deck, and send prompt. |
| `lkjagent memory <query>` | Search distilled memory through the full-text index. |
| `lkjagent graph` | Print the active graph case and source graph summary. |

All commands accept `--data <dir>` to locate the store and default to the
container data directory defined in [../operations/running.md](../operations/running.md).
The graph command reads source graph definitions and active case state.

## Conversation Shape

The owner sends a message and reads the result from the transcript:

```sh
lkjagent send "Profile the parser and remove the hot allocation."
lkjagent log --follow
```

Completed work appears as agent.done events. Questions from the agent
appear as agent.ask events, and the owner responds with another
`lkjagent send`. Every owner line joins the same queue; ordering rules live
in [queue.md](queue.md).

## Owner Console

`lkjagent console` renders a compact management screen that redraws while it
is open. It keeps the transcript area at the top and concentrates operational
controls at the bottom:

- transcript tail and the last useful agent output,
- pending queue previews near the transcript,
- daemon state, pending count, open task, question, error, and notice in the
  bottom deck,
- a `send>` prompt under the bottom deck.

Any non-command line typed at the prompt is enqueued with the same durability
as `lkjagent send`. `/refresh` redraws, `/help` lists commands, and `/quit`
exits the console without touching the daemon. The screen refreshes every
second, so daemon state, transcript changes, and queued work appear even when
the owner is not typing. In an interactive terminal, unfinished input stays
visible across redraws. The renderer reads terminal columns and rows, caps the
body to the visible screen, and wraps mixed English, Japanese, and other wide
characters by display width.

## Output Style

CLI output is plain text for snapshot commands and ANSI terminal text for the
interactive console. Snapshot commands print one fact per line. The console
prints a compact screen plus a prompt. Exit code 0 means the command itself
succeeded; it never claims anything about task success.

## Status

implemented.
