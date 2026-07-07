# North Star

## Purpose

State what lkjagent is for, who it serves, and how success is measured.

## Product

lkjagent is a continuously running personal agent for one owner, one local LLM,
one workspace, and one SQLite store. The owner sends plain-language work to a
thin CLI. The daemon turns each message into a durable task, executes a typed
plan step by step, verifies results with deterministic checks, and reports the
truth.

The product serves these work types in priority order:

1. Structured artifacts: notes, document trees, reports, study material,
   workspace manifests, transcripts, and other checked outputs.
2. Ordinary workspace file work: create, revise, organize, and summarize.
3. Questions and small tasks answered directly into the transcript.
4. Personal records as plain workspace files maintained by the same plan engine.

## Reader

This repository is read and written by LLM agents. Files are optimized for
machine reading first: short pages, explicit ownership, table-of-contents
READMEs, quiet gates, and direct contracts.

## Weak Model Assumption

The model may have a modest context window, no reliable JSON tool calling,
imperfect instruction following, and a strong tendency to repeat patterns from
its prompt. The harness carries control flow so the model can author bounded
content instead of navigating policy.

## Measured Success

The product succeeds when a configured checkout can run the daemon, accept one
workspace objective, and produce row-backed evidence across records, state
cells, prompt frames, tool admissions, observations, checks, and artifacts. A
structured artifact success uses an `ArtifactManifest`, nested units, source
refs, fresh artifact fingerprints, and harness-computed checks before closure.

The same engine must handle personal records, a software project report, a
docs-tree request, a question, and file work without creating a second runtime
authority.
