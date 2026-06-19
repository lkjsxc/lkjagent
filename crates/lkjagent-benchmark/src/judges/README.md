# Benchmark Judges

## Purpose

This directory holds deterministic mechanical judges for benchmark task
families. Judges read only the candidate workspace and hidden constants in
this crate.

## Table of Contents

- [arithmetic.rs](arithmetic.rs): exact CRT judge.
- [automata.rs](automata.rs): DFA parser and equivalence judge.
- [bundle.rs](bundle.rs): README-indexed bundle judge.
- [correction.rs](correction.rs): latest-instruction answer judge.
- [graph.rs](graph.rs): shortest-path certificate judge.
- [mod.rs](mod.rs): judge dispatcher.
- [program.rs](program.rs): bounded shell program judges.
