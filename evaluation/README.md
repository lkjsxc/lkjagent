# Evaluation Inputs

## Purpose

Own tracked scenario inputs, execution plans, false-positive fixtures, and
source-bound evidence for lkjagent.

## Plans

- `workgraph.tsv`: dependency and command plan without editable status.
- `acceptance.tsv`: required predicates without outcome labels.
- `experiment-plan.tsv`: concrete model-sensitive cells committed before runs.
- `fault-schedule.tsv`: deterministic fault timing for supported scenarios.

## Fixtures And Runners

- `scenarios/`: declared workspace seeds, owner schedules, and expected checks.
- `false-positive-fixtures/`: evidence that every checker must reject.
- `corpus/`: deterministic parser and check inputs.
- `experiment_runner/`: current experiment support pending direct-loop cutover.
- `sqlite-online-backup.py`: online SQLite capture helper.
- `pty-recorder.py`: terminal capture helper.

## Evidence

Sanitized synthetic evidence is committed below `evaluation/evidence/<source>/`.
Private raw runs stay ignored. Plans are inputs, not completion evidence. Result
rows are derived by the acceptance checker described in
`../docs/evaluation/live-proof.md`.
