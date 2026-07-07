# North Star

## Purpose

State what lkjagent is for, who it serves, and how success is measured.

## Product

lkjagent is a continuously running personal agent for one owner, one local LLM,
one visible workspace, and one SQLite ledger. The owner sends plain-language
turns to a thin CLI or workbench. The daemon routes each turn into a matter,
record, artifact request, decision, state update, or inspection, then verifies
results with deterministic checks and reports the truth.

The product serves these work types in priority order:

1. Daily records: journal, todo, calendar-like, finance, contact, routine, and
   note files written under `data/workspace`.
2. Ordinary workspace file work: create, revise, organize, summarize, index, and
   rebalance.
3. Structured artifacts: document trees, reports, study material, transcripts,
   exports, and proof bundles.
4. Questions and retrieval answered from source-linked workspace and ledger
   evidence.
5. Software project records, repository evidence, and verification logs.

## Reader

This repository is read and written by LLM agents. Files are optimized for
machine reading first: short pages, explicit ownership, table-of-contents
READMEs, quiet gates, and direct contracts.

## Weak Model Assumption

The model may have a modest context window, no reliable JSON tool calling,
imperfect instruction following, and a strong tendency to repeat patterns from
its prompt. The harness carries control flow so the model can author bounded
content or request an admitted XML-like action instead of navigating policy.

## Measured Success

The product succeeds when a configured checkout can run the daemon, accept
ordinary daily owner turns, write the requested workspace files, and produce
row-backed evidence across matters, records, state cells, prompt frames, tool
admissions, observations, checks, token usage, and artifacts. A structured
artifact success uses an `ArtifactManifest`, nested units, source refs, fresh
artifact fingerprints, and harness-computed checks before closure.

The same engine must handle personal records, a software project report, a
docs-tree request, a question, and file work without creating a second runtime
authority.
