# Artifact Addresses

## Purpose

This file owns the model-facing tool address contract for artifact tools.

## Address Shape

- `root`: directory path of the artifact.
- `path`: optional relative file path under root.
- `kind`: artifact kind.
- `state`: missing-root, root-directory, file-under-root, file-without-root, or invalid-root.

## Tool Behavior

- artifact.plan creates a ledger identity; it does not require filesystem existence.
- artifact.apply creates or repairs a root directory; it refuses `.md` roots
  unless an explicit migration route converts an old bad root.
- artifact.next returns one of:
  - apply-root when the root is missing.
  - repair-paths when weak paths exist.
  - audit-root when root content readiness has no weak paths.
  - root-is-file when the address is a file.
- artifact.audit audits only root directories.

## File Roots

If `root` resolves to a file, artifact tools classify the address before any
filesystem audit. A file under a known artifact root keeps the owning root and
relative weak path. A file without a known root asks for directory inspection.
Neither case may render artifact.audit for the file path.

## Invalid Roots

A root ending in `.md` is a Markdown leaf shape. artifact.apply and doc.scaffold
refuse it before mutation. artifact.next and artifact.audit report a semantic
address refusal or a focused repair action instead of an OS directory error.

## Status

partially implemented
