# Console

## Purpose

Define the local interactive terminal screen opened by `lkjagent watch`.

## Boundary

The console is a terminal view over the local store. It is not a web UI, not a
metrics endpoint, and not a second runtime loop. Redraws are read-only until the
owner submits input at the prompt.

## Layout

The screen has three zones:

1. Transcript tail and last useful agent output at the top.
2. Pending queue preview below the transcript when owner work is waiting.
3. Bottom status deck above the `send>` prompt.

The bottom deck uses the same facts as `lkjagent status`:

- daemon state, pending count, active task, and top state tracks;
- authority mission, next action, admitted tools, and missing evidence;
- artifact root, readiness, weak cursor, and next path;
- context usage and token aggregates;
- current model-log path;
- owner question, daemon error, and local notice when present.

## Prompt Commands

| Input | Behavior |
| --- | --- |
| plain text | Enqueue the text as an owner message. |
| `/refresh` | Redraw from the store. |
| `/help` | Show console commands. |
| `/quit` | Exit without touching daemon state. |

The console redraws while open. Unfinished owner input remains visible across
redraws in an interactive terminal.

## Sizing

Each redraw reads terminal rows and columns, falls back to `LINES` and
`COLUMNS`, and clamps to safe minimums. Transcript lines are wrapped by display
width. Mixed-width text stays inside the terminal. The bottom deck is anchored
above `send>` and never overlaps the prompt.

## Accepted Names

The current binary opens this screen through both `lkjagent watch` and the
explicit `lkjagent console` command name. There are no hidden terminal-console
command names.
