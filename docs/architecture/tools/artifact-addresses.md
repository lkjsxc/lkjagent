# Artifact Addresses

## Purpose

This file owns the model-facing tool address contract for artifact tools.

## Address Shape

- `root`: requested artifact address.
- `path`: optional relative weak path under the normalized root.
- `kind`: artifact kind.
- `state`: root-directory, missing-root, file-under-known-root,
  file-without-known-root, or invalid-root.

## Root And Path Table

| Tool | Accepts root | Accepts path | Result |
| --- | --- | --- | --- |
| artifact.plan | missing or existing directory identity | no | creates ledger identity only for directory-shaped roots. |
| artifact.apply | missing or existing directory | no | writes or repairs a root; `.md` roots are refused before mutation. |
| artifact.next | directory or known Markdown leaf | yes | normalizes to root plus weak path, then emits repair, apply, or audit. |
| artifact.audit | existing directory | no | audits root readiness and records ledger evidence. |
| fs.write | Markdown file path | no | writes one leaf; never owns artifact root identity. |
| fs.batch_write | Markdown file paths | no | writes leaves; never owns artifact root identity. |
| completion | normalized root | ledger weak paths | reads audit readiness and weak-path state only. |

## Tool Behavior

artifact.next returns apply-root when the root is missing, repair-paths when
weak paths exist, audit-root when root content has no weak paths, or a semantic
address refusal when the request is a file or invalid root.

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
