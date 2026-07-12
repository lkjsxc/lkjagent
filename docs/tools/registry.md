# Tool Registry

## Purpose

Define one descriptor source for every prompt, parser, admission, and effect.

## Descriptor

A descriptor owns stable name, purpose, field order, required flags, value
classes, byte/count bounds, safe example, state affordances, admission rules,
stable effect key, result bound, and denial code.

Prompt cards, parser validation, persisted decision projection, admission,
dispatch, generated docs, and contract tests consume the same descriptor.

## Initial Catalog

- `list_directory`: bounded no-follow directory listing.
- `search_text`: bounded UTF-8 search below one path.
- `read_file`: numbered page plus current SHA-256 revision.
- `edit_file`: exact single-match replacement against an observed revision.
- `create_file`: create one observed-absent UTF-8 file without overwrite.

Shell, file delete/move, whole-file overwrite, record writing, and memory tools
are absent until their complete state/effect/check path exists.

## State Views

- Orient: list, search, read, with an action envelope for the first admitted call.
- Modify: read, edit, create.
- Review: native checks and no model tools.
- Respond: no tools and final grammar only.
- Protocol recovery: smallest useful subset of the intended state.
- Stale-file recovery: read, then edit after a fresh revision.
- Wait and idle: no model call.

The global catalog is never rendered. Hidden names do not appear in examples or
denial prose. Review, respond, wait, and idle project an empty view. Recovery
projects nothing unless the selector names one intended direct tool; retired
`fs.*` explore names are never a fallback for direct decisions.

## Persisted Projection

A decision stores exact immutable descriptor projections, not only names or a
hash. Admission uses that stored shape. If the executable cannot honor its effect
key after restart, the decision blocks instead of changing meaning.
