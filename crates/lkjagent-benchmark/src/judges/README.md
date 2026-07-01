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
- [owner-address.rs](owner_address.rs): artifact address regression judge.
- [owner-docs.rs](owner_docs.rs): semantic documentation topology judges.
- [owner-loop-ops.rs](owner_loop_ops.rs): uploaded loop-regression judges.
- [owner-ops.rs](owner_ops.rs): action recovery, status, and model log judges.
- [owner-uploaded.rs](owner_uploaded.rs): uploaded run-log fixture matrix judge.
- [program.rs](program.rs): bounded shell program judges.
- [large_artifact.rs](large_artifact.rs): durable large-artifact judge helpers.
- [long_novel.rs](long_novel.rs): long novel source module.
- [owner_continuation.rs](owner_continuation.rs): owner continuation source module.
- [owner_doc_topics.rs](owner_doc_topics.rs): owner doc topics source module.
- [story_manuscript.rs](story_manuscript.rs): story manuscript source module.
