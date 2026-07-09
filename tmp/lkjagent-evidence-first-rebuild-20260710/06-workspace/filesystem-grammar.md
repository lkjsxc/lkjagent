# Filesystem Grammar

## Root

Use a separate externally visible workspace directory. In Compose, mount the
host workspace at /workspace and runtime data at /data. Outside the container,
the configured workspace path may be relative or absolute and is canonicalized
once.

## On-Demand Tree

    workspace/
      README.md
      inbox/
      life/
        journal/YYYY/MM/DD/entry.md
        todo/open/YYYY/MM/
        todo/waiting/YYYY/MM/
        todo/done/YYYY/MM/
        calendar/YYYY/MM/DD/
        finance/YYYY/MM/entries/
        notes/<topic>/YYYY/MM/
      knowledge/<topic>/
      projects/
      artifacts/
        reports/
        documents/
        proof/
      activity/YYYY/MM/DD/
      indexes/
      archive/
      system/
        operations/
        quarantine/
        import-review/

Create only the branch needed by real content. A new directory gets a useful
bounded README that links actual children. Do not precreate the full tree.

## Managed Versus Project Files

The 512-token target applies to agent-managed memory and navigation documents.
User project source, generated binaries, data sets, and external repositories
retain their natural formats and use project-specific checks.

## Paths

All ledger paths are normalized relative to the canonical root. Reject absolute
model paths, parent traversal, symlink escapes, control characters, reserved
system paths, and case-collision ambiguity. Open and rename through
descriptor-relative no-follow traversal, or an equivalent race-resistant API;
canonicalize-only checks are insufficient against a symlink swap.

Navigation uses bounded page files such as page-2026-07.md or topic slugs and
rolls over before the managed-document token ceiling.
