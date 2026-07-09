# Evaluation Sources

## Purpose

Hold source-authored scenario, fault, seed, and false-positive inputs consumed
by mechanical evaluation gates.

## Contents

- `fault-schedule.tsv`: ordered deterministic fault injections.
- `scenarios/`: anchored goals, schedules, checks, and seed bytes.
- `false-positive-fixtures/`: evidence summaries that must be rejected.
- `sqlite-online-backup.py`: quiesced SQLite Online Backup recorder.
- `pty-recorder.py`: raw pseudo-terminal input and output recorder.
