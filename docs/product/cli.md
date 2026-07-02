# CLI

## Purpose

Define the owner command surface and output discipline.

## Commands

| Command | Behavior |
| --- | --- |
| `lkjagent run` | run the daemon in the foreground |
| `lkjagent send TEXT [--new]` | enqueue an owner message and print its queue id |
| `lkjagent status` | print daemon, task, step, budgets, queue, and tokens |
| `lkjagent log [--limit N] [--follow]` | print transcript events |
| `lkjagent task list` | list tasks with state and summary |
| `lkjagent task show ID` | show plan, diagnoses, checks, and exchange refs |
| `lkjagent queue list` | list owner messages |
| `lkjagent queue show ID` | show one owner message and delivery state |
| `lkjagent memory QUERY` | search memory rows |
| `lkjagent watch` | open the terminal console |
| `lkjagent help [group]` | print usage |

## Output Rules

- Commands print line-oriented, machine-readable text by default.
- A successful mutating command prints the created id or one concise success
  line.
- A failed command prints the command, reason, and next useful action when one
  exists.
- Quiet gates are owned by xtask, not by the owner CLI.

## Data Directory

Commands accept the configured data directory consistently. Status and read-only
inspection commands work while the daemon is stopped because they read the
store directly.

## Removed Surfaces

There are no graph, personal-records, model-log, audit, or verification command
groups in the owner CLI. The task, queue, memory, log, status, and proof-bundle
surfaces expose the same facts without a second control plane.
