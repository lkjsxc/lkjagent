# Root Path Contract

## Purpose

This file owns the difference between document roots, artifact roots, weak
paths, and Markdown leaves.

## Definitions

- document root: a workspace directory that contains README.md and may contain catalog.toml.
- artifact root: a workspace directory with catalog.toml or an artifact ledger record.
- weak path: a relative path under an artifact root that needs content repair.
- Markdown leaf: a `.md` file, never a root for doc.audit, artifact.audit, doc.scaffold, or artifact.apply.

## Rules

- doc.scaffold root resolves to a directory path and must not end in `.md`.
- artifact.apply root resolves to a directory path and must not end in `.md`.
- doc.audit root is a directory.
- artifact.audit root is a directory.
- artifact.next may accept a root directory or a file path under a known root,
  but it normalizes the address before producing the next action.
- fs.write and fs.batch_write own Markdown file paths.
- Completion reads audit-owned root readiness plus weak-path readiness.

## Failure Rendering

When a root parameter points at a file, the tool reports `root_is_file` and
renders one copyable next action using the owning root directory when it can be
discovered. If the root cannot be discovered, the next action is fs.list or
workspace.summary, not artifact.audit.

## Invariants

- No generated directory may end with `.md`.
- No audit tool renders a Markdown file path in a root-only slot.
- A structure-only or owner-term-only page is weak content until content audit
  records concrete evidence.

## Status

partially implemented
