# Overfitting

## Purpose

Set rules that keep benchmark-driven work aimed at general capability.

## Rules

Do not put oracle answers, fixture text, hidden generated cases, judge code,
or report-specific fixes into prompts, graph policy, memory, or runtime branches.
The agent sees public prompts and starter files only.

Use failure clusters instead of single task answers. A graph failure can
justify better certificate-checking habits. It does not justify memorizing
one graph. A shell hidden-case failure can justify stronger test-generation
guidance. It does not justify special-casing the input values.

When a task becomes too familiar to guide useful work, add a new seed or a new
task in the same family and keep the older task for trend continuity. The new
task needs the same good and bad fixture bar before it can affect reports.

Benchmark changes must be reviewed like product changes: docs, code, tests,
`benchmark check-corpus`, and `quiet verify` move together. A judge that gets
weaker to make a current run look better is invalid.

## Status

implemented.
