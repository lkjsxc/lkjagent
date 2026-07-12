# Evaluation Harness Source

## Purpose

Implement the confined public production evaluation runner, baseline evidence
validator, bounded captures, and mechanical false-positive checks.

## Table of Contents

- [mod.rs](mod.rs): campaign CLI, copied-binary runner, and derived output.
- [clock.rs](clock.rs): monotonic deadlines, bounded capture, and process groups.
- [evidence.rs](evidence.rs): versioned baseline and negative-fixture validation.
- [pty.rs](pty.rs): explicit rejection of incomplete PTY scenarios.
- [scenario.rs](scenario.rs): four tracked aliases, hashed five-turn schedules,
  hashed seed bytes, and shell-free endpoint files. Parser tests reject changed
  owner-text and seed hashes; `exact-file-edit` is accepted by both campaign
  modes without accepting arbitrary paths or owner text.
- [snapshot.rs](snapshot.rs): confined roots, manifests, diffs, and stable SQLite facts.
