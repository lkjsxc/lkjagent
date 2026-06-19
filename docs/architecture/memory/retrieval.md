# Retrieval

## Purpose

How distilled memory comes back: the memory.find action the model calls
mid-task, and the digest builder that fills the prefix region. Both read
the same memory table with the same ranking.

## memory.find

memory.find takes a query string and returns matching memory rows as a
bounded observation. Matching and ranking:

1. The query runs against memory_fts, the FTS5 mirror over title, tags,
   and content ([store.md](store.md)).
2. The base score is FTS5 bm25.
3. The score is weighted by kind: task-summary highest, then incident,
   then lesson, then fact. Summaries of past work and records of past
   failures outrank general knowledge at equal lexical relevance.
4. Ties break by recency: latest updated_at first.

## Why Lexical

Relevance is lexical by design, per
[../../decisions/sqlite-store.md](../../decisions/sqlite-store.md). An
agent recalls by the exact names of files, tools, and decisions, and those
names are what entries are titled and tagged with per
[distillation.md](distillation.md). FTS5 plus good distillation beats an
embedding stack here: nothing extra to host, no opaque scores, and sqlite3
can explain any result.

## The Digest Builder

The digest builder runs whenever the prefix is rebuilt, which is at
compaction ([../context/compaction.md](../context/compaction.md)) and at
daemon startup. It selects top-ranked entries, then validates the rendered
prefix text against the 2,048-token memory digest budget from
[../context/budgets.md](../context/budgets.md):

- The open task's task summary is always first.
- Remaining space fills in rank order; a rendered entry that would overflow
  the cap is skipped.
- If the first rendered entry alone exceeds the cap, it is truncated with an
  explicit marker so daemon startup can continue.
- The rendered digest lands in the rebuilt prefix and is what the model
  sees without asking.

Whole-entry selection remains the normal path, so entries stay near 200
tokens per [distillation.md](distillation.md).

## Status

implemented.
