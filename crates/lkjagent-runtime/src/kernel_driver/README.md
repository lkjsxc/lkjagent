# Kernel Driver Source

## Purpose

This directory contains the effect-edge driver that records a kernel snapshot,
event, decision, and prompt frame or runtime effect before execution.

## Table of Contents

- `input.rs`: turn input facts collected by callers.
- `persist.rs`: writes authority rows.
- `persist_map.rs`: maps kernel records into store detail rows.
- `turn.rs`: sequences one kernel-owned turn.
