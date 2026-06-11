# Verification

## Purpose

The gate commands that decide whether work is done, and the output contract
that keeps them readable to agents. A gate that did not run did not pass;
claiming otherwise violates [../agent/honest-state.md](../agent/honest-state.md).

## The Quiet Contract

Gates run quiet: a passing gate prints exactly one line, `ok ` followed by
the gate name, and exits 0. A failing gate prints the failing step, the
exit status, and a bounded tail of captured output, then exits nonzero.
Quiet output is a context-hygiene rule as much as a CI rule: a passing gate
costs an agent a handful of tokens.

## Gates

Built by [../execution/tasks/xtask-checks.md](../execution/tasks/xtask-checks.md);
all run as `cargo run -p lkjagent-xtask -- <gate>`:

| Gate | Checks |
| --- | --- |
| check-docs | doc shape, topology, TOC completeness, ASCII, prose width, banned tokens, skill format, task format, README coverage |
| check-lines | the 200-line cap (120 for skills) on every tracked file |
| check-style | panic-path scan and dependency allowlist on product crates |
| quiet test | cargo fmt --check, clippy with warnings denied, all workspace tests |
| quiet verify | check-docs, check-lines, check-style, then quiet test |

## Final Gate

```sh
docker compose run --rm verify
```

Builds the image from a clean context and runs quiet verify inside it; no
source bind mounts, so the gate proves the repository as committed, not the
working tree. Service design in [compose.md](compose.md). Any claim that a
runtime behavior is implemented requires this gate in the same handoff.

## Interim Checks

Until the xtask exists, docs-only changes run these from the repository
root; together they substitute for check-docs and check-lines:

```sh
# line cap: no output means pass
find . -path ./.git -prune -o -type f -name '*.md' -print \
  | xargs awk 'END{} {c[FILENAME]++} END{for(f in c) if (c[f]>200) print f, c[f]}'

# README topology: every docs dir has README.md and >=2 children
for d in $(find docs -type d); do
  [ -f "$d/README.md" ] || echo "missing README: $d"
  [ "$(ls "$d" | wc -l)" -ge 3 ] || echo "thin directory: $d"
done

# TOC completeness: every doc is linked from its directory README
for f in $(find docs -name '*.md' ! -name 'README.md'); do
  grep -q "($(basename "$f"))" "$(dirname "$f")/README.md" || echo "unlinked: $f"
done

# banned tokens (the -ersion family, single-letter release tags, compat framing)
grep -rniE 'v[0-9]+|[a-z]ersion|legacy|backward|deprecat' docs/ README.md AGENTS.md
```

The banned-token scan must print nothing; the others must print nothing.

## CI

CI runs the final gate and nothing else, so local and CI verdicts cannot
diverge. The workflow file lands with the xtask task.

## Status

design-only (interim checks excepted: they run today).
