# Skill: Benchmark Driven Improvement

## Purpose

Use benchmark reports to improve lkjagent without leaking task answers or
weakening judges.

## Trigger

A benchmark report is being used to choose or verify an agent improvement.

## Context

- [../../evaluation/README.md](../../evaluation/README.md): benchmark area map.
- [../../evaluation/metrics-reports.md](../../evaluation/metrics-reports.md): report fields and metric meanings.
- [../../evaluation/improvement-loop.md](../../evaluation/improvement-loop.md): required improvement loop.
- [../../evaluation/overfitting.md](../../evaluation/overfitting.md): anti-overfitting rules.
- [../../operations/verification.md](../../operations/verification.md): gates that must pass before handoff.

## Procedure

1. Run `cargo run -p lkjagent-xtask -- benchmark check-corpus` before
   trusting any score report.
2. Read the report TSV and summary markdown. Group failures by family,
   judge reason, end state, and operational counts.
3. Pick one failure cluster that maps to a durable harness behavior: tool
   ergonomics, prompt rendering, context policy, memory, skills, runtime
   recovery, or queue handling.
4. Read the owning docs contract for that behavior. Update the contract
   before editing code or skill text.
5. Make the smallest general change that explains why the failure cluster
   improves without naming hidden answers, seeds, or fixture contents.
6. Rerun the focused task with `benchmark run --suite tiny --task <id>` and
   keep the report path as evidence. Rerun the full tiny suite when the
   focused result changes.
7. Record improved, regressed, and inconclusive results from real reports in
   the handoff. Treat check-corpus as benchmark validity, not a score.

## Checks

- `cargo run -p lkjagent-xtask -- benchmark check-corpus` prints
  `ok benchmark-corpus`.
- The focused real run report exists under `data/benchmark/runs/` or the
  operator-supplied data directory.
- Any claimed improvement cites old and new report paths or states that no
  benchmark improvement is claimed.
- `cargo run -p lkjagent-xtask -- quiet verify` prints `ok verify`.

## Must Not

- Do not add hidden answers, generated cases, judge logic, or fixture text to
  prompts, skills, memory, or runtime special cases.
- Do not weaken a judge to improve a score.
- Do not claim a benchmark improvement from corpus checks, unit tests, or
  intent; only real run reports count.
- Do not optimize one task when the evidence points to a family-level
  behavior.
