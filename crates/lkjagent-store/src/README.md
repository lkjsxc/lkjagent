# Source

## Purpose

Map the modules compiled into the native direct store.

## Table of Contents

- [lib.rs](lib.rs): native store exports and errors.
- [native-schema.rs](native_schema.rs): exact direct-runtime schema validation.
- [native-schema.sql](native-schema.sql): native SQLite objects.
- [transactions.rs](transactions.rs): native intake through close boundaries.
- [direct-transactions.rs](direct_transactions.rs): direct tool and fault settlement.

Retired plan, queue, record, workspace projection, and bridge row source files
remain in this directory but are excluded from `lib.rs`.
