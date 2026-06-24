# Personal Search

## Purpose

Define bounded listing and full-text search for personal records.

## Listing

List operations accept kind, status, date range, project, query, tags, and limit
filters as allowed by each tool. Limits are required and default to small
values. Results are sorted deterministically: schedule by time, TODOs by status
and due date, diary by date and ID.

## Full-Text Search

FTS covers title, body, tags, and project. Search results return stable IDs,
kind, title, status, normalized dates, tags, and short snippets. Large bodies do
not enter context unless a bounded read tool requests one record by ID.

## Context Budget

Search and list observations are compact. They report total matched when known,
returned row count, and a truncation note when more rows exist. The model must
ask for a narrower list instead of dumping all records.
