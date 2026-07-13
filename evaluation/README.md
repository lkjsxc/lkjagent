# Evaluation Inputs

## Purpose

Own tracked scenario inputs, execution plans, false-positive fixtures, and
source-bound evidence for lkjagent.

## Plans

- `workgraph.tsv`: dependency and command plan without editable status.
- `acceptance.tsv`: required predicates without outcome labels.
- `experiment-plan.tsv`: outcome-free cells, controls, repeats, and threshold hash.
- `fault-schedule.tsv`: ordered deterministic fault injections.

## Fixtures And Runners

- `scenarios/`: anchored goals, schedules, checks, and seed bytes. Five exact
  aliases cover file, daily-life, project, artifact, and slow Japanese PTY runs.
- `false-positive-fixtures/`: evidence summaries that must be rejected.
- `corpus/`: deterministic parser and check inputs.
- `experiment_runner/`: current experiment support pending direct-loop cutover.
- `sqlite-online-backup.py`: quiesced SQLite Online Backup recorder.
- `pty-recorder.py`: disposable raw pseudo-terminal input and output recorder.
- [`../crates/lkjagent-xtask/src/evaluation_harness/`](../crates/lkjagent-xtask/src/evaluation_harness/README.md): confined public production runner and evidence validator.

## Evidence

Sanitized evidence is committed below `evaluation/evidence/<source>/`. Private
raw captures use disposable mode-0700 roots outside the repository. Scripted or
local mechanics remain `semantic_status=not-evaluated`; plans are inputs, not
completion evidence. Result rows remain derived by the acceptance checker in
`../docs/evaluation/live-proof.md`.
