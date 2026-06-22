# Mock Content Detection

## Purpose

Mock-content detection owns deterministic rejection of shallow, repeated,
placeholder-like files. A topology pass is not a content-quality pass.

## Failure Signals

- sibling files share the same heading sequence and differ mainly by title.
- text contains scaffold phrases such as `This section explains`.
- text contains `coming soon`, `TODO`, or status-only filler.
- project-specific term density is low for lkjagent docs.
- a file has no links to state, prompts, tools, implementation, or verification.
- root README contains independent blurbs instead of child links and relations.

## Repair Rule

Repair either replaces the file with concrete project content or deletes and
merges it into the owning page. Adding more files is not a repair unless the
relation graph and README links improve.

## Status

implemented
