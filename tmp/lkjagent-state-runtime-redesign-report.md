# lkjagent State Runtime Redesign Report

## Purpose

Provide an implementation-ready temporary handoff for the next coding pass.

## Summary

The repository already documents a durable state-ledger target and has an early
pure core slice for state cells, events, decisions, tool views, admissions,
context items, contradiction detection, contamination classes, fingerprints, and
fresh completion evidence. The integrated daemon is still plan-ledger shaped and
must be wired to persisted runtime decisions before prompt rendering, endpoint
calls, action admission, effects, recovery, and completion.

## Next Work

Improve documentation first, then implement the store, selector, runtime,
tooling, context, artifact, endpoint, proof, and cleanup slices described by the
structured package prepared for this handoff.
