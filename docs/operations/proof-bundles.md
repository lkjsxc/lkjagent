# Proof Bundles

## Purpose

Define bounded derived views over raw node and acceptance evidence.

## Node Evidence

Before source freeze, each workgraph node stores raw command output under
`tmp/lkjagent-progress/nodes/<node-id>/raw/`. Its result binds node, source
commit, sequence, exact Docker gate, exit code, dependency receipt hashes,
evidence hashes, and separate verifier note.

## Acceptance Evidence

After source freeze, evidence lives under
`tmp/lkjagent-acceptance/<source-commit>/`. Campaign directories contain raw
SQLite backup, workspace bytes and manifest, events, decisions, admissions,
checks, metrics, process lifecycle, provider manifest, redacted logs, scenario
inputs, and verifier report. PTY evidence also contains the terminal recording,
trace, and replay result.

## Derived Views

Bounded reports may summarize matters, state cells and edges, decisions,
prompt frames, tool views, admissions, effects, observations, context exclusions,
checks, exchanges, token usage, document revisions, workspace tree, and
warnings. Every row names raw refs and fingerprints.

## Raw Authority

Reports do not copy secrets, full prompt bodies, full model responses, or large
artifact prose. They also do not replace raw SQLite, workspace bytes, event
traces, provider hashes, Git commits, or terminal recordings. A missing raw
input remains a failure regardless of summary text.

## Commit Binding

Raw evidence is committed after the frozen source commit. Public CI validates
the evidence commit. The final material receipt is committed next, followed by
a separate four-file verifier commit and its public verification workflow.

Harness-generated manifests are not pass receipts. They are sorted indexes of
raw paths and SHA-256 values. Gates reopen the SQLite Online Backup, rehash
workspace and PTY bytes, and recompute scenario bundles before deriving any
result.
