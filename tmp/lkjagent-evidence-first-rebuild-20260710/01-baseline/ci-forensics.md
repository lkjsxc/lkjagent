# CI Forensics

## Clean Checkout Defect

Dockerfile copies Cargo.toml, Cargo.lock, and README.md before building. Cargo.lock
exists in the supplied local directory but is ignored by Git and absent from the
tracked file list.

Every local Docker verification claim therefore used a file that a fresh GitHub
checkout does not contain. A clean build can fail before Rust compilation even
though the local run passed.

## Workflow Drift

The workflow sets LKJAGENT_WORKSPACE, while Compose does not consume that key.
Compose mounts data and keeps the workspace under data instead of exposing a
separate workspace mount.

## Mandatory Corrections

- Track Cargo.lock and build with locked dependencies.
- Add a gate that checks every Docker COPY source is tracked.
- Run final verification from git archive HEAD, not the working directory.
- Make CI and local Compose use the same data and workspace configuration keys.
- Require the public workflow to pass after the final commit.
