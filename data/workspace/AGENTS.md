# Workspace Brief

## Purpose

Standing instructions for agents operating inside `/data/workspace` during
lkjagent diagnostic runs.

## Rules

- Treat this tree as generated workspace evidence, not repository docs.
- Keep all edits inside `/data/workspace` unless the runtime admits a broader
  project action.
- Do not present scaffolds, empty indexes, generic leaves, or unsupported audit
  claims as completed user content.
- Every directory created for an artifact needs one `README.md` that links its
  immediate children.
- Every generated content root needs a compact catalog and objective-specific
  readiness evidence.
- Failed outputs stay in `diagnostic-output/` with findings and next repair
  actions.
- Prefer bounded batch writes and repair cursors over single large writes.
- A close summary names only files, audits, and checks that actually happened.
