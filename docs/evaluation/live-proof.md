# Live Proof

## Purpose

Define tracked source-bound evidence and the only successful stopping condition.

## Evidence Root

Sanitized synthetic bundles live at:

```text
evaluation/evidence/<source>/<campaign>/<run-id>/
```

Private raw data stays ignored. Every attachment is verified with `git ls-files`.
`SOURCE` is a full frozen commit and must remain an ancestor of HEAD. Commits after
that source may change only `evaluation/evidence/SOURCE/`; any product, plan, or
other path change makes the evidence stale.

A bundle includes command log, source/status, binary/config/plan hashes,
workspace before/after manifests, diff, compressed SQLite backup, stable table
exports, process log, redacted provider data, derived result, and raw manifest.

`result.tsv` is checker-generated. It records predicate ID, category, derived
status, evidence path, measured value, checker hash, and predicate-schema hash.
An input pass label is rejected outside `result.tsv`. Result statuses are allowed
only so the checker can write them; verification ignores and recomputes them.

## Acceptance Command

```text
cargo run --locked -p lkjagent-xtask -- acceptance verify \
  --source SOURCE --evidence evaluation/evidence/SOURCE
```

The command has a nonzero incomplete mode. It validates source binding, tracked
plans and attachments, final workgraph ancestry, concrete experiment cells,
secret patterns, and nine negative fixtures. Static derivation binds exact source and
test symbols rather than status text. It covers the compact authority,
documentation shape, line limit, reachable-secret scan, safe descriptor-relative
reads, exact edits, compact envelopes, hidden-tool admission, provider anomaly
and transport classification, native decision and transaction boundaries, and
direct reduction and selection. These are confined primitive claims only.
Endpoint connectivity, the public application loop, workspace-root behavior,
campaigns, records, TUI behavior, completion, and every evidence-dependent real
behavior remain missing, so the command cannot yet return success.

The checker will read plans, Git history/trailers, tracked files, command exits,
source/binary/image hashes, SQLite, workspace manifests, prompt/tool audits,
experiments, campaigns, PTY traces, secret scan, and independent review. It
recomputes every required row in `../../evaluation/acceptance.tsv`.

## Campaigns

Five meaningful development campaigns and five frozen-source campaigns each run
at least 900 seconds: file, recovery, daily life/recall, multiple projects/report,
and PTY. Each uses fresh roots, real endpoint, production public commands,
predeclared owner schedule, and corresponding durable work. Sleeping duration is
not progress.

The frozen campaigns use one no-cache clean-source binary without product changes
between runs.

## PTY

PTY evidence stores raw input/output frames, dimensions, timestamps, composer
events, canonical message IDs/sequences, viewport mode/anchor/max-top, and screen
hashes. A textual description without frames and IDs fails.

## Secret Safety

Capture strips authorization headers. Pre-commit scans the index against actual
loaded secrets without printing them. Final verification scans every Git object
reachable from HEAD for current or known prior secret fingerprints,
authorization patterns, high-confidence credentials, and private owner content.
Reports contain only
path/object hash.

## Independent Review

After final Docker and campaign evidence, a read-only reviewer recomputes all
predicates and reports blockers. Resolve findings and regenerate affected
evidence. Success requires clean review followed by checker exit zero.
