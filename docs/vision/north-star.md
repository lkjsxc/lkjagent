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

1. Long structured artifacts: manuscripts, document trees, reports, study
   material, and other multi-file or multi-thousand-word outputs.
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

The product succeeds when a configured checkout can run the daemon, accept the
Aurora Ledger manuscript request, and produce ten chapter files under the
requested root with at least 10,000 measured manuscript words. The task closes
only after engine-computed checks pass. A failed task ends as blocked with a
bounded report and evidence.

The same engine also handles a docs-tree request, a question, and a file-work
request without task-family code paths outside templates and checks.
