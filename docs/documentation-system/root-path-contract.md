# Root Path Contract

## Purpose

This file owns the difference between document roots, artifact roots, weak
paths, and Markdown leaves.

## Definitions

- document root: a workspace directory that contains README.md and may contain catalog.toml.
- artifact root: a workspace directory with catalog.toml or an artifact ledger record.
- weak path: a relative path under an artifact root that needs content repair.
- Markdown leaf: a `.md` file, never a root for doc.audit, artifact.audit, doc.scaffold, or artifact.apply.

## Root And Path Table

| Tool | Root input | Path input | Reducer result |
| --- | --- | --- | --- |
| artifact.plan | directory identity | none | records identity only when the root is not a Markdown leaf. |
| artifact.apply | directory root | none | creates or repairs the directory; refuses `.md` roots before mutation. |
| artifact.next | root or known leaf | optional weak path | normalizes to owning root and weak path before choosing repair or audit. |
| artifact.audit | directory root | none | audits directories only; file roots get a semantic refusal. |
| doc.scaffold | directory root | none | creates README, catalog, and child files; refuses `.md` roots. |
| doc.audit | directory root | none | audits directories only; file roots get a semantic refusal or read action. |
| fs.write | workspace file | none | writes Markdown leaves and never creates artifact roots. |
| fs.batch_write | workspace files | none | writes Markdown leaves and never creates artifact roots. |
| completion | normalized root | weak paths | reads audit-owned readiness and ledger weak paths, not raw files. |

## Classification Outputs

The shared reducer reports `address_status`, `detected_path_kind`,
`normalized_root`, optional `weak_path`, `next_action`, and `valid_example` for
root-only and root-plus-path tools. It must distinguish existing roots with and
without `catalog.toml`, missing directory roots, missing `.md` roots, Markdown
files under known roots, Markdown files without known roots, directories whose
name ends in `.md`, non-directory filesystem objects, empty roots, and paths
outside the workspace.

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
