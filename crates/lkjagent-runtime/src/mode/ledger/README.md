# lkjagent-runtime Mode Ledger

## Purpose

This directory owns runtime decision-record assembly and the field projection
used by store-facing authority history.

## Table of Contents

- [ledger.rs](ledger.rs): persisted decision-record assembly.
- [ledger_data.rs](ledger_data.rs): decision record, kind, and fingerprint data.
- [ledger_event.rs](ledger_event.rs): event labels and fingerprint helper.
- [ledger_fields.rs](ledger_fields.rs): decision-to-ledger field projection.
