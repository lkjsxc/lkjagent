# Xtask Structure

## Purpose

This directory owns non-mutating structure audits for documentation and
workspace trees.

## Table of Contents

- [mod.rs](mod.rs): public audit entrypoint and CLI adapter.
- [catalog.rs](catalog.rs): catalog coverage and stale path checks.
- [findings.rs](findings.rs): finding data and messages.
- [plan.rs](plan.rs): pure structure plan builder.
- [readme.rs](readme.rs): README topology and weak content checks.
- [render.rs](render.rs): terminal rendering for audit and plan commands.
