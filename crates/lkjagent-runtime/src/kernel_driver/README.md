# Kernel Driver Source

## Purpose

This directory contains the effect-edge driver that records a kernel snapshot,
event, decision, and prompt frame or runtime effect before execution.

## Table of Contents

- `input.rs`: turn input facts collected by callers.
- `persist.rs`: writes authority rows.
- `persist_map.rs`: maps kernel records into store detail rows.
- `turn.rs`: sequences one kernel-owned turn.
- [dense_rows.rs](dense_rows.rs): dense rows source module.
- [input.rs](input.rs): input source module.
- [mod.rs](mod.rs): mod source module.
- [persist.rs](persist.rs): persist source module.
- [persist_map.rs](persist_map.rs): persist map source module.
- [turn.rs](turn.rs): turn source module.
