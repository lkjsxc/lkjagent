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
- Encyclopedia, wiki, or knowledge-base tasks need a nucleus, expansion
  queue, and rebalance plan before broad topic branches.

## Procedure

1. Run `pwd`, `find . -maxdepth 3 -type f | sort`, and then
   `git rev-parse --is-inside-work-tree >/dev/null 2>&1 && git status --short || printf 'git=absent\n'`.
2. Read applicable `AGENTS.md`, root README, docs README, current-state, and blocker files when present.
3. Pick the smallest root that owns the request, such as `docs/`, `src/`, or a named product area.
4. Keep each action compact. Use shell.run for multi-file scaffolds and fs.write only for one concise file.
5. Write or update that root `README.md` before adding child files.
   Every README.md must include the exact heading `## Table of Contents`.
6. Split by ownership: create a child directory when one file would own multiple peer concepts.
7. Put a `README.md` in every new directory before adding leaf files under it.
8. Keep each leaf to one purpose, one status or evidence section, and links to its parent or owner doc.
9. For encyclopedia-like tasks, start with a small nucleus: root README,
   concept map, starter domain, reference, curation, expansion queue, and
   rebalance plan.
10. Record durable work state in current-state, blockers, tasks, or traceability files when those exist.
11. If no state spine exists, create only the minimal state files needed for the current task.
12. For generic recursive structure tasks, build at least six directories,
    twelve markdown files, and three nested directory levels.
13. For encyclopedia-like tasks, grow in one-to-three page batches from
    expansion-queue, then update concept-network and rebalance-plan.
14. Rebalance before broadening when one branch has more than three sibling
    leaves, stale links, or unclear ownership.
15. For auto-scaffolded encyclopedia tasks, inspect the seeded map first;
    do not create a second top-level root such as `docs/ontology/`.
16. Use explicit paths with `mkdir -p` before writes. shell.run uses /bin/sh,
    so do not rely on Bash-only `{a,b}` brace expansion.
17. For bulk creation or repair, use one compact shell.run script with short README bodies.
18. Run the repository gate named by AGENTS or README; if absent, run the checks below.
19. If the git command prints `git=absent`, report git as absent; do not call it clean.

## Checks

- `find <root> -type d ! -exec test -f '{}/README.md' ';' -print` prints no required directory.
- `grep -R -L '## Table of Contents' <root>/**/README.md <root>/README.md` prints no file.
- `find <root> -type f -name '*.md' -exec wc -l {} + | sort -n | tail -20` stays under the local cap.
- `find <root> -type d | wc -l` is at least 6, and markdown file count is at least 12.
- Encyclopedia-like tasks have expansion-queue and rebalance-plan before broad topic expansion.
- `git rev-parse --is-inside-work-tree >/dev/null 2>&1 && git status --short || printf 'git=absent\n'` does not fail.
- The repository gate exits 0, or the handoff names why no gate exists.

## Must Not

- Do not create a deep directory without an index at each level.
- Do not replace `## Table of Contents` with decorative or domain-specific headings.
- Do not use shell brace expansion; write each directory path explicitly.
- Do not put a `docs/` directory under another `docs/` directory.
- Do not add top-level encyclopedia roots outside the seeded map.
- Do not split by file type when ownership gives a clearer boundary.
- Do not leave status, tasks, assumptions, or acceptance only in chat.
- Do not call agent.done after only one file or one directory level.
- Do not delete or overwrite existing source unless the owner explicitly requested it.
- Do not claim repository-wide structure when checks covered only a subroot.
- Do not claim git is clean unless git status actually ran inside a git tree.
