# File Tools

## Purpose

Define direct file and directory tools inside the workspace. These tools
replace ordinary ls, find, grep, wc, mkdir, and heredoc shell usage. Canonical
parameter table: [registry.md](registry.md).

## fs.read

Reads a ranged file span. Parameters are required `path`, optional `start`
default `1`, and optional `count` default `200`. The observation includes one
header line and exact file bytes for the selected range.

## fs.write

Writes one file, creating parent directories. The observation confirms path
and byte count and does not echo the content.

## fs.edit

Replaces exactly one `find` string with `replace` in `path`. Zero or multiple
matches are errors and leave the file unchanged.

## fs.list

Lists sorted relative paths under `path`, bounded by `depth`, `kind`, and
`limit`. Rows include kind, bytes, and cheap line counts for text files.

## fs.search

Performs bounded substring search. `include` is a simple suffix or substring
filter; `case` defaults to `insensitive`. Output is `path:line: snippet` with
bounded context.

## fs.stat

Reports kind, bytes, line count for text files, and a stable checksum. It is
the normal replacement for simple wc and existence checks.

## fs.mkdir

Creates one directory inside the workspace, including parents.

## fs.batch_write

Writes several files from a simple line protocol:

```
path: docs/a.md
content:
# A
-- lkjagent-next-file --
path: docs/b.md
content:
# B
```

The dispatcher rejects empty paths, duplicate paths, and workspace escapes.

## Graph Policy

During maintenance, mutating file tools are blocked by the maintenance gate.
During owner work, graph policy blocks mutating file tools until a valid plan
and executable graph node are active.

## Status

implemented.
