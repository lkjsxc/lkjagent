# Evaluation Harness Source

## Purpose

Implement deterministic scenario, clock, fault, snapshot, manifest, PTY, and
false-positive checks for evaluation gates.

## Table of Contents

- [mod.rs](mod.rs): node composition and replacement smoke replay.
- [clock.rs](clock.rs): monotonic fake clock and ordered fault injector.
- [evidence.rs](evidence.rs): raw fact validation and negative fixtures.
- [hash.rs](hash.rs): SHA-256 fingerprints for source and raw bytes.
- [pty.rs](pty.rs): raw PTY recording validation and replay binding.
- [scenario.rs](scenario.rs): anchored scenario and seed validation.
- [scenario-seed.rs](scenario_seed.rs): source seed path and byte validation.
- [snapshot.rs](snapshot.rs): SQLite Online Backup and raw manifests.
