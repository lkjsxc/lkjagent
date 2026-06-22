# Track Update Policy

## Purpose

This file owns how runtime events raise, lower, and decay weighted tracks.
Updates are deterministic and live in pure reducers.

## Event Mapping

- Parser fault raises parse recovery and action parameter reliability.
- Valid action lowers parse recovery.
- Documentation audit failure raises document structure and semantic coverage.
- Relation audit failure raises structure connectivity.
- Mock-content audit failure raises mock content risk.
- Model-name audit failure raises model-specific naming.
- Artifact objective mismatch raises artifact drift.
- Owner task arrival raises queue interruption.
- Queue classification lowers queue interruption.
- Post-compaction mismatch raises context snapshot mismatch.

## Decay Rules

- Passing audit lowers only the track owned by that audit.
- Classification lowers queue interruption.
- Valid action lowers parser pressure but does not erase evidence gaps.
- Completion readiness rises only from fresh audit and verification events.

## Status

implemented
