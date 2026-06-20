# lkjagent-runtime Active Mode

## Purpose

This directory owns deterministic active-mode selection. One mode decides which
policy layer is visible for a turn: owner task, recovery, maintenance,
compaction, or closed idle.

## Table of Contents

- [mod.rs](mod.rs): module exports.
- [model.rs](model.rs): active-mode input, mode, and policy records.
- [policy.rs](policy.rs): allowed tools and policy-layer flags per mode.
- [render.rs](render.rs): compact active-mode notice rendering.
- [select.rs](select.rs): pure mode selector.
