# lkjagent-runtime Mode Authority

## Purpose

This directory is the compatibility adapter for turn authority inputs, runtime
mission selection, snapshot and event reduction, and active-mode selection.
Mission selection delegates shared facts to the transition kernel.

## Table of Contents

- [authority.rs](authority.rs): pure turn authority assembly.
- [examples.rs](examples.rs): context-aware valid action examples for authority cards.
- [input.rs](input.rs): turn authority snapshot input.
- [mission.rs](mission.rs): runtime mission enum and mode mapping.
- [model.rs](model.rs): active-mode input, mode, and policy records.
- [reducer.rs](reducer.rs): pure runtime snapshot and event reducer.
- [select.rs](select.rs): pure mode selector.
