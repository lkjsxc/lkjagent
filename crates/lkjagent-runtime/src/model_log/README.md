# Model Log Helpers

## Purpose

This directory holds focused renderer helpers for the single current model handoff
Markdown file.

## Table of Contents

- [exchange.rs](exchange.rs): provider request, response, and error file writers.
- [exchange_support.rs](exchange_support.rs): provider exchange hashing and write helpers.
- [export.rs](export.rs): provider export manifest writer.
- [export_json.rs](export_json.rs): export file-list and missing-file JSON helpers.
- [index.rs](index.rs): provider exchange index writer.
- [ledger.rs](ledger.rs): evidence, fault, transcript, and verification sections.
- [sections.rs](sections.rs): top-level snapshot, objective, tracks, and plan sections.
- [text.rs](text.rs): Markdown escaping and compact count formatting helpers.
- [turn_files.rs](turn_files.rs): parsed action, admission, and observation files.
