# lkjagent-runtime Mode Authority

## Purpose

This directory owns pure turn authority inputs, runtime mission selection,
snapshot and event reduction, and active-mode selection.

## Table of Contents

- [authority.rs](authority.rs): pure turn authority assembly.
- [input.rs](input.rs): turn authority snapshot input.
- [mission.rs](mission.rs): runtime mission enum and mode mapping.
- [model.rs](model.rs): active-mode input, mode, and policy records.
- [reducer.rs](reducer.rs): pure runtime snapshot and event reducer.
- [select.rs](select.rs): pure mode selector.
