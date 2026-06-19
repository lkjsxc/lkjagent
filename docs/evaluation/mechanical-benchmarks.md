# Mechanical Benchmarks

## Purpose

Explain why lkjagent uses deterministic benchmarks and what the tiny suite
is allowed to claim.

## Contract

lkjagent improves through tasks whose success can be checked by code. A
benchmark task asks for a concrete artifact: an exact number, a certificate,
a DFA table, an executable shell program, a repaired file, or a linked file
tree. The judge recomputes the answer or checks the artifact independently.

Mechanical benchmarks are not a replacement for owner judgment. They are the
repeatable loop that shows whether changes to tools, prompts, memory,
context, skills, or runtime behavior make general task completion more
reliable.

The tiny suite is small enough for repeated local runs on the target local
model class. Its score is a trend signal over task families. A single low
score is benchmark data, not proof that the harness is broken. A harness
error, endpoint error, timeout, or missing endpoint config is operational
data and must not be reported as a model capability score.

## Status

implemented.
