# Mock Content

## Purpose

This fixture owns rejection of shallow sibling files that differ mainly by title
or repeat a generic scaffold template.

## Setup

```text
three or four Markdown files
same headings in the same order
generic body text
no project-specific links
no relation page updates
```

## Expected Behavior

- Mock-content audit fails.
- Completion is blocked.
- Repair replaces the files with concrete project content or merges them into
  one owner page.
- Adding more similarly shaped files is rejected as a non-repair.

## Forbidden Behavior

- Passing because every file has an H1 and a README link.
- Passing because a graph manifest mentions the paths.
- Closing after topology-only audit.

## Status

implemented
