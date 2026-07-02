# Personal As Templates

## Purpose

Record the decision to model personal records as workspace files.

## Context

Journal, schedule, and task-list work needs durable output but not dedicated
tables or tools.

## Decision

Personal record tasks use plan templates that write Markdown under `journal/`,
`schedule/`, and `todos/`. Memory rows may record distilled summaries.

## Consequences

Personal records use the same task, step, file, check, and proof path as other
work. Backup and inspection are ordinary filesystem operations.

## Rejected Alternatives

Dedicated personal tables and commands would create a separate persistence
surface with separate gates and failure modes.
