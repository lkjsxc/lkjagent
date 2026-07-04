# Crash And Resume

## Purpose

Define transaction boundaries and daemon boot recovery for the state ledger.

## Turn Transactions

The store commits events and their state patches together. Prompt frames,
decisions, admissions, observations, checks, exchange refs, and usage rows carry
the same decision id when they belong to one turn. A gate that did not commit a
row did not happen.

## Before Endpoint Calls

Persist `RuntimeDecision` and `PromptFrame` before calling the endpoint. A crash
after prompt rendering but before model response resumes from the persisted
decision and either retries the same call or records a bounded endpoint-loss
recovery event.

## Before Tool Execution

Persist `ToolAdmission` before running a tool. A crash after admission but
before observation resumes from the admission and either reruns idempotent work
or records a recovery event for non-idempotent work.

## Boot

At boot the daemon opens the store, enables WAL and foreign keys, reclaims a
stale lease when allowed, searches for unfinished decisions, settles recovery
when needed, hydrates a `RuntimeSnapshot` from rows, and only then selects new
work.

## Failure Boundary

Resume reads durable rows and ignores config snapshots, orphan exchange bodies,
and prompt-only state. It cannot mark completion without committed fresh check
rows.

## Failure This Prevents

Crashes cannot create false completion, stale prompt authority, or unexplained
refused-tool behavior.
