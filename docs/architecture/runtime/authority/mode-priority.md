# Mode Priority

## Purpose

Define the deterministic priority order that selects exactly one runtime
mission for one authority decision.

## Decision Owner

`lkjagent-runtime` owns priority selection. Graph policy, endpoint output,
maintenance ticks, verification requests, and context pressure are inputs.

## Persisted Names

Persisted mission names use snake case. Rust variants may use idiomatic
CamelCase, but store rows, prompt cards, benchmark fixtures, and status output
use these strings:

```text
hard_runtime_compaction
owner_recovery
schema_repair
artifact_repair
verification_repair
owner_execution
owner_verification
owner_completion
idle_maintenance
closed_idle
```

## Mission Truth Table

Rows are evaluated in order. The first row whose required facts are true wins.
No later row may admit tools or render prompt policy for that turn.

| Rank | Mission | Required facts | Blocks |
| --- | --- | --- | --- |
| 1 | `hard_runtime_compaction` | hard context pressure is present | all model actions |
| 2 | `owner_recovery` | active owner case has an unresolved non-schema fault | lower missions |
| 3 | `schema_repair` | latest parse or schema fault blocks model action | lower missions |
| 4 | `artifact_repair` | current artifact ledger has weak paths or open cursor | lower missions |
| 5 | `verification_repair` | verification failed or required verification evidence is missing | lower missions |
| 6 | `owner_execution` | active owner case needs plan, context, execute, observe, or evidence work | maintenance and idle |
| 7 | `owner_verification` | owner case is ready to run requested verification gates | maintenance and idle |
| 8 | `owner_completion` | completion was requested or all close inputs are ready for gating | maintenance and idle |
| 9 | `idle_maintenance` | no queue row, active case, recoverable owner fault, or compaction pressure exists | idle |
| 10 | `closed_idle` | no mission row above matches | none |

## Output

The selector emits the mission, reason, resume data requirements, admitted tool
classes, blocked tool classes, and next valid action class. Active mode is
derived from the selected mission and is not selected independently.

## Prohibited States

- Maintenance continues after owner queue depth becomes non-zero.
- Verification runs before the artifact exists or readiness is plausible.
- Hard compaction asks the model to preserve state through `memory.save`.
- Recovery yields to normal progress while the previous fault is unresolved.
- Prompt rendering and dispatch admission use different mission decisions.

## Fixture

`maintenance_noop_claim` proves idle work cannot outrank owner work.
`compaction_resume_missing` proves hard pressure must snapshot first.

## Verification

Run `cargo test -p lkjagent-runtime --test authority_reducer` and focused
active-mode tests that assert the table order from persisted snapshot facts.

## Status

partially implemented. Hard compaction, owner recovery, owner execution, idle
maintenance, and closed idle are represented in current code. Schema repair,
artifact repair, verification repair, owner verification, and owner completion
still need direct mission selection from the durable runtime snapshot.
