# Console Responsive Layout Handoff

## Purpose

Record the current problem, completed console work, commit scope, validation
evidence, and remaining risks for the `lkjagent console` responsive layout
task.

## Language Note

The owner asked in Japanese. This tracked Markdown file uses ASCII English
because the repository `check-docs` gate rejects non-ASCII text in every
tracked `.md` file.

## Current Problems

- The reported symptom is that the process "stops with an error"; this turn
  did not include a new stack trace, stderr tail, or failing command.
- The strongest confirmed problem in this session is commit scope, not a
  failing console test: the working tree already contains broad compact graph,
  config, runtime, tool, and docs edits that were present before the console
  layout work.
- A blanket `git add .` would commit unrelated graph/config/runtime changes.
  That would violate the user plan and the repository rule against reverting
  or absorbing changes not made in this task.
- `docs/current-state.md` was already heavily dirty before this task. The
  console work added a current-state sentence in the worktree, but staging the
  whole file would also stage the pre-existing graph rewrite.
- The safe commit needs to include only the console layout code, console docs,
  tests, and this handoff file. Any current-state handling must avoid pulling
  unrelated graph changes into the same commit.

## Original Console Issue

- The target is `lkjagent console`, a terminal-first console, not a web UI.
- Width wrapping already existed, including display-width handling for English
  and Japanese or CJK text.
- Height behavior was incomplete: when transcript content was short, the
  bottom status deck floated upward instead of staying above the final prompt
  row.
- The `send>` prompt was not effectively anchored to the last terminal row
  because the rendered body did not always fill `rows - 1` lines.
- Live resize behavior depended on `COLUMNS` and `LINES`; it did not query the
  terminal size on every redraw first.
- Long typed prompt buffers could run past the visible prompt row.

## Behavior Now Implemented

- Terminal size resolution moved into `crates/lkjagent-cli/src/console/size.rs`.
- Each redraw asks `stty size` against `/dev/tty` first.
- If live terminal size is unavailable or invalid, sizing falls back to
  `COLUMNS` and `LINES`.
- If environment sizing is unavailable or invalid, sizing falls back to 80x24.
- The existing minimum clamp remains 40 columns by 12 rows.
- `render_screen_for_size(..., ScreenSize)` keeps the explicit size test path.
- `render_snapshot(data_dir, notice, columns, rows)` keeps its public shape.
- `ScreenSize` is still re-exported from the render path for existing imports.
- The screen body now renders exactly `rows - 1` lines.
- The transcript area receives the remaining height after the bottom deck.
- Sparse transcript screens get blank spacer rows above the bottom deck.
- The bottom deck is directly above the prompt row.
- The draw layer truncates unfinished typed input to the prompt row width.
- Existing display-width wrapping remains in place for mixed-width text.
- The implementation stays stdio plus ANSI; no Ratatui, OpenTUI, or other TUI
  dependency was added.

## Files Changed For The Console Work

- `crates/lkjagent-cli/src/console/size.rs`
- `crates/lkjagent-cli/src/console/render.rs`
- `crates/lkjagent-cli/src/console.rs`
- `crates/lkjagent-cli/tests/console_render.rs`
- `crates/lkjagent-cli/src/console/README.md`
- `DESIGN.md`
- `docs/product/cli.md`
- `docs/current-state.md`, staged only as a minimal HEAD-based console
  sentence; the worktree still contains unrelated graph edits in that file

## Tests Added Or Strengthened

- Sparse transcript padding keeps the bottom deck anchored.
- The rendered body has exactly `rows - 1` lines.
- A 40x12 narrow screen keeps the bottom deck visible.
- Wide and CJK text stays within terminal width.
- Long typed input is truncated to the prompt line width.
- Size parsing covers valid `stty size` output and invalid fallback behavior.

## Validation Already Run

- `cargo fmt`
- `cargo fmt --check`
- `cargo test -p lkjagent-cli --test console_render`
- `cargo test -p lkjagent-cli`
- `cargo run -p lkjagent-xtask -- check-docs`
- `cargo run -p lkjagent-xtask -- check-lines`
- `cargo run -p lkjagent-xtask -- quiet verify`
- `docker compose run --rm verify`

All of the above passed in this workspace before this handoff file was added.

## Commit Scope Guidance

- Commit the safe console files and this handoff file.
- Do not commit unrelated compact graph/config/runtime/tool changes that were
  already dirty before this console task.
- Do not use `git add .`.
- `docs/current-state.md` is safe only when staged as a minimal console
  sentence against `HEAD`; do not stage the full worktree copy.

## Remaining Risks

- The working tree still contains many unrelated dirty files. Future agents
  must inspect `git status --short` before committing anything else.
- The current-state worktree content is useful, but it shares a file with
  unrelated graph changes. That file is the main place where accidental commit
  scope can go wrong.
- The user mentioned an error stop without a fresh log. If the error persists
  after this commit, the next step is to capture the exact failing command,
  exit code, and stderr tail.
- The console behavior is verified by render and crate tests, plus local and
  compose gates. It has not been manually exercised in a live resized terminal
  session in this handoff.

## Next Executable Step

If another error appears, run the exact failing command again and capture:

```sh
git status --short
cargo test -p lkjagent-cli --test console_render
```

Then compare the failure to this file. If the failure is commit related, start
with `git diff --cached --name-only` and confirm no graph/config/runtime files
are staged by accident.
