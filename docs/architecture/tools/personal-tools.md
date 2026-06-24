# Personal Tools

## Purpose

Define the fixed registry tools for diary, schedule, and TODO records.

## Tool Family

The personal tools are store-backed and use the live action envelope:

- `diary.record`: date optional, title required, content required, tags optional.
- `diary.find`: query, start, end, tags, and limit filters.
- `schedule.add`: title and start required; end, timezone, location, notes,
  recurrence, and tags optional.
- `schedule.list`: start, end, query, status, and limit filters.
- `schedule.update`: ID required; title, start, end, status, notes, and tags optional.
- `todo.add`: title required; details, due, priority, project, and tags optional.
- `todo.list`: status, query, due_before, project, and limit filters.
- `todo.update`: ID required; title, details, due, priority, project, status,
  and tags optional.

## Observation Shape

Create and update observations return ID, kind, title, status, normalized dates,
changed fields, tags, and projection path when a view is written. List and find
observations return bounded rows plus a truncation note when more rows exist.

## Admission

Owner personal-record tasks may create, update, list, and search records.
Maintenance may search or prune only when a separate maintenance contract admits
that behavior. Maintenance never silently creates personal records.
