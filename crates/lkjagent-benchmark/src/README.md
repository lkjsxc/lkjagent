# lkjagent-benchmark Source

## Purpose

This directory holds the benchmark core: static corpus definitions, judges,
metrics, report rendering, and runner configuration parsing.

## Table of Contents

- [corpus.rs](corpus.rs): task lookup and corpus validity checks.
- [error.rs](error.rs): benchmark error values.
- [fixture.rs](fixture.rs): fixture and starter workspace materialization.
- [judges/](judges/README.md): deterministic task judges.
- [lib.rs](lib.rs): public crate surface.
- [metrics.rs](metrics.rs): transcript and status metric extraction.
- [model.rs](model.rs): benchmark data types.
- [report.rs](report.rs): TSV, markdown, and comparison rendering.
- [runner.rs](runner.rs): endpoint and run configuration helpers.
- [tasks/](tasks/README.md): tiny suite task definitions.
