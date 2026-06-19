# Metrics And Reports

## Purpose

Define benchmark scoring, operational metrics, and durable report files.

## Metrics

Correctness metrics are task pass or fail, points earned, points possible,
and bounded judge failure reason. They are produced by deterministic judges
outside the agent workspace.

Operational metrics are process signals from status and transcript data:
turn count, parse errors, repeat-action notices, tool errors, shell actions,
file writes or edits, and questions. They explain failure clusters but do not
replace correctness.

Each real run writes reports under `data/benchmark/runs/<run-id>/` or under
the operator-supplied benchmark data directory. Generated reports are ignored
local state and are not committed.

The machine report is `report.tsv`. Each row records run id, timestamp, git
state, model label, summarized endpoint host, suite, task id, family,
difficulty, pass or fail, points, judge reason, turn count, elapsed wall time,
end state, operational counts, workspace path, and transcript path. The
human report is `summary.md` with score and per-task result rows.

## Status

implemented.
