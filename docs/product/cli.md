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
| `lkjagent status` | Print daemon state, queue depth, open task, context usage. |
| `lkjagent log` | Print recent transcript events; `--follow` tails new ones. |
| `lkjagent memory <query>` | Search distilled memory through the full-text index. |
| `lkjagent skills` | List the skill library: name, trigger line, last refined. |

All commands accept `--data <dir>` to locate the store and default to the
container data directory defined in [../operations/running.md](../operations/running.md).

## Conversation Shape

The owner sends a message and reads the answer from the transcript:

```sh
lkjagent send "Profile the parser and remove the hot allocation."
lkjagent log --follow
```

Answers appear as agent.done events; questions from the agent appear as
agent.ask events and are answered with another `lkjagent send`. Replies join
the queue like any message; ordering rules live in [queue.md](queue.md).

## Output Style

CLI output is plain text, one fact per line, no decoration. Exit code 0 means
the command itself succeeded; it never claims anything about task success.

## Status

implemented.
