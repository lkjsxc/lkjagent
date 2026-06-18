# Skill: Recursive Structure

## Purpose

Create and maintain recursively navigable project trees where each directory
has a local index, each file has one owner concept, and task state survives
outside chat.

## Trigger

A task asks to organize, scaffold, split, or maintain a recursive project file structure.

## Context

- `AGENTS.md` states local rules that outrank this skill.
- `README.md`, `docs/README.md`, or another root index shows the current navigation contract.
- `docs/current-state.md` and `docs/execution/current-blockers.md` show durable state when present.
- `find . -maxdepth 3 -type f | sort` gives a bounded map before edits.

## Procedure

1. Run `pwd`, `find . -maxdepth 3 -type f | sort`, and `git status --short`.
2. Read applicable `AGENTS.md`, root README, docs README, current-state, and blocker files when present.
3. Pick the smallest root that owns the request, such as `docs/`, `src/`, or a named product area.
4. Write or update that root `README.md` as the table of contents before adding child files.
5. Split by ownership: create a child directory when one file would own multiple peer concepts.
6. Put a `README.md` in every new directory before adding leaf files under it.
7. Keep each leaf to one purpose, one status or evidence section, and links to its parent or owner doc.
8. Record durable work state in current-state, blockers, tasks, or traceability files when those exist.
9. If no state spine exists, create only the minimal state files needed for the current task.
10. Build at least six directories, twelve markdown files, and three nested directory levels.
11. Run the repository gate named by AGENTS or README; if absent, run the checks below.

## Checks

- `find <root> -type d ! -exec test -f '{}/README.md' ';' -print` prints no required directory.
- `find <root> -type f -name '*.md' -exec wc -l {} + | sort -n | tail -20` stays under the local cap.
- `find <root> -type d | wc -l` is at least 6, and markdown file count is at least 12.
- `git status --short` shows only paths intended for this structural slice.
- The repository gate exits 0, or the handoff names why no gate exists.

## Must Not

- Do not create a deep directory without an index at each level.
- Do not split by file type when ownership gives a clearer boundary.
- Do not leave status, tasks, assumptions, or acceptance only in chat.
- Do not call agent.done after only one file or one directory level.
- Do not delete or overwrite existing source unless the owner explicitly requested it.
- Do not claim repository-wide structure when checks covered only a subroot.
