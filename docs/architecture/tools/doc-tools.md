# Document Tools

## Purpose

Define graph-native document construction helpers for structured artifacts.

## doc.scaffold

Creates a compact README-indexed tree under `root`. Parameters are `root`,
optional `kind`, optional `count`, optional `mode` (`exact` or `approx`),
required `title`, and optional newline sections. It is for small scaffolded
trees, not large generated prose.

## doc.audit

Audits a document root for README presence, linked children where present,
and optional file-count target. Passing audits can satisfy document-structure
evidence when the active graph gate accepts that evidence.

## Status

implemented.
