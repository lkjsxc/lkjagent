# Scenario Contract

## Source

Store each required scenario under evaluation/scenarios/<scenario-id> with a
scenario.tsv, matters.tsv, owner schedule, seed manifest, and checker
definitions. Commit these with product source before the live run.

## Scenario Fields

- scenario_id;
- minimum_duration_seconds;
- minimum_owner_turns;
- minimum_owner_span_seconds;
- minimum_decision_span_seconds;
- allowed_terminal_states;
- required_check_ids;
- seed_fingerprint;
- required fault injections and negative predicates.

Configuration is a run input, not part of the reusable scenario. Every run
stores canonical effective configuration bytes and their SHA-256. Experiment
rows bind that fingerprint; scenario comparisons keep controlled keys equal and
vary only declared factors.

## Terminal Expectations

matters.tsv maps each stable scenario_key to completed, waiting-owner,
waiting-external, or a named visible exhausted blocker. Runtime matters persist
that scenario key. Required final campaigns normally complete all goals.
Dedicated waiting scenarios prove exact question or wake-condition behavior.
The gate compares each actual matter with the declared expectation and never
forces false closure merely to obtain a green result.

## Checks

Scenario checks are stable IDs implemented by independent Rust checkers. A
required obligation names the current passed check that satisfied it. Historical
failed or superseded checks remain durable but do not fail the current result.

## Binding

The run copies the manifest and records a bundle SHA-256 over scenario.tsv,
matters.tsv, owner-schedule.tsv, seed-manifest.tsv, and checks.tsv. The final gate
recomputes it from source-commit files and rejects copied or modified campaign
definitions.
