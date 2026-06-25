# File Tools

## Purpose

Define direct file and directory tools inside the workspace. These tools
replace ordinary ls, find, grep, wc, mkdir, and heredoc shell usage. Canonical
parameter table: [registry.md](registry.md).

## fs.read

Reads a ranged file span. Parameters are required `path`, optional `start`
default `1`, and optional `count` default `200`. The observation includes one
header line and exact file bytes for the selected range.

## fs.read_many

Reads several ranged file spans in one bounded observation. Parameters are
required `paths`, optional `start` default `1`, optional `count` default `80`,
and optional `total` default `400`. The paths value is newline-separated.

## fs.write

Writes one file, creating parent directories. The observation confirms path
and byte count and does not echo the content. The write is rejected before any
filesystem mutation when content contains scaffold phrases such as instructions
to replace a skeleton or add substance later.

## fs.edit

Replaces exactly one `find` string with `replace` in `path`. Zero or multiple
matches are errors and leave the file unchanged.

## fs.patch

Applies one or more exact find/replace blocks to one file. Each block uses:

```
find:
old text
replace:
new text
```

Blocks are separated with `-- lkjagent-next-edit --`. Every find string must
match exactly once before the file is written.

## fs.list

Lists sorted relative paths under `path`, bounded by `depth`, `kind`, and
`limit`. Rows include kind, bytes, and cheap line counts for text files.

## fs.tree

Renders a deterministic bounded tree for quick workspace survey. It is the
normal replacement for simple recursive `find` or `tree` shell commands.

## fs.search

Performs bounded substring search. `include` is a simple suffix or substring
filter; `case` defaults to `insensitive`. Output is `path:line: snippet` with
bounded context.

## fs.stat

Reports kind, bytes, line count for text files, and a stable checksum. It is
the normal replacement for simple wc and existence checks.

## fs.mkdir

Creates one directory inside the workspace, including parents. The path must be
a directory path, not a Markdown or TOML leaf such as `README.md` or
`catalog.toml`; file leaves are written with `fs.write` or `fs.batch_write`.

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

The dispatcher validates every file before writing any file. It rejects empty
paths, duplicate paths, workspace escapes, scaffold phrases, files over 65,536
bytes, total batches over 262,144 bytes, and file counts above the active
limit.

## Graph Policy

During maintenance, mutating file tools are blocked by the maintenance gate.
During owner work, graph policy blocks mutating file tools until a valid plan
and executable graph node are active.

## Status

implemented.
