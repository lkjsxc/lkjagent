# Evaluation Harness

## Purpose

Define the deterministic support layer used by replay, experiment, live, and
PTY gates without allowing summaries to declare their own success.

## Time And Faults

Tests receive a monotonic fake clock. Clock steps are explicit inputs and a
monotonic regression fails. A fault schedule orders injection ID, boundary, typed
outcome, and clock advance. Each injection is consumed once; an omitted,
reordered, or repeated injection fails replay.

## Scenario Source

Every directory under `evaluation/scenarios/` contains `scenario.tsv`,
`matters.tsv`, `owner-schedule.tsv`, `seed-manifest.tsv`, and `checks.tsv`.
The harness recomputes one SHA-256 bundle fingerprint from those source bytes.
Campaign results must name that fingerprint and their actual source commit.

Scenario rows declare duration, owner and decision floors, allowed terminal
states, required checks, required faults, and negative predicates. The checker
compares raw matter and check rows with those declarations. An editable result
label is never an input to the decision.

## Snapshot And Manifest

At a quiesced read boundary the recorder uses the SQLite Online Backup API to
create `run.sqlite3`. It then writes a sorted workspace manifest containing
normalized path, document ID, revision ID, and SHA-256 for every current file.
Each run manifest binds all raw files. A campaign manifest also binds the exact
binary, build log, source and plan records, matrix, adoption rows, and run trees.

## PTY Recording

The PTY recorder writes asciinema JSON input and output frames with ordered
monotonic offsets. Replay reads the cast, rebuilds frame state, and binds its
receipt to the cast and scenario fingerprints. Empty output, absent owner
input, invented trace-only geometry, unordered time, or missing Japanese input
fails the recorder check. The fixture separates its acceptance and completion
writes so scheduler-dependent PTY read coalescing cannot erase the required
raw frame boundary.

## False Positives

Committed negative fixtures cover idle reported as complete, blocked reported
as complete, skipped commands, zero-test filters, and generated placeholders.
The node gate must prove each fixture is rejected for its mechanical defect.
It also proves one computed fixture succeeds, so a validator that rejects
everything cannot pass.

## Command Authority

`bench check-corpus` validates the anchored scenario sources. `smoke replay`
runs the complete deterministic harness. `experiment run` requires clean source
inputs, an unchanged strict-ancestor plan, and a detached offline release build
with isolated Cargo configuration and remapped paths. A Git-backed gate compares
the rebuilt binary exactly; the Git-free Docker gate rebuilds the same source bytes.
The runner records fresh-store production-endpoint probes and conditionally
adds repeats four and five when the first three outcomes differ. Failed attempts
retain logs and snapshots; explicit resume archives failures and skips completed
tuple rows without reusing their stores. `gate
domain-experiments` recomputes input, exchange, database, complete table export,
workspace, metric, and manifest claims. Probe rows explicitly mark the fault
schedule, recovery hypotheses, semantic success, and live floor when unexercised
or unmeasured; final fault and live work owns those predicates.
