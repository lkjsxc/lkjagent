# Regression Fixtures

## Purpose

Regression fixtures own replay cases derived from owner-reported failures. A
fixture names input, forbidden output, expected guard, and completion evidence.

## Fixture Set

- multi-topic docs for lkjagent, model endpoint, and domain examples.
- documentation init for lkjagent, named model topic, and Rust.
- improve-structure no-op after docs init.
- directory with `.md` suffix.
- disconnected root README topic blurbs.
- model-specific durable naming.
- repeated generated boilerplate across scaffold leaves.
- mock content across sibling files.
- Japanese cookbook drifting into bread paths.
- parser recovery loop and repeated invalid actions.
- filesystem and shell parameter consistency.
- maintenance no-op loop.
- queue interruption.
- context compaction consistency.
- store-backed active case projection versus `graph.state` output.
- audit loop between `artifact.audit`, `doc.audit`, and `graph.state`.
- document topology pass mistaken for artifact readiness.
- direct audit-owned `graph.evidence` attempts.
- shallow story content with role labels but no role-specific facts.
- provider reasoning-only anomaly before action parsing.
- generated log index entries pointing at absent turn directories.
- recovery examples that contradict the current decision.

## Gate

The replay gate must prove these cases fail on the old pattern and pass on the
runtime contract before the related blocker can close. The checked-in active run
is failure evidence, not success proof.

## Status

partially implemented
