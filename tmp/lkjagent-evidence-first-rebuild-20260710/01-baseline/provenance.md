# Provenance

## Supplied Snapshot

- Archive: lkjagent-no-target-no-env-20260709T191753Z.zip
- Local HEAD: ae5ff551457adce869dee6159200c85a63aab3de
- Local timestamp: 2026-07-09T11:56:00Z
- Public main observed through GitHub: 2affb801baf4f7e3c402c0bd3d665ec3ae501fe7
- Local branch position: 73 commits ahead of bundled origin/main
- Dirty path: data/README.md deleted in the supplied worktree
- Extraction omitted archive entry tmp/bin/python because it was an absolute
  symlink to /usr/bin/python3; no repository evidence depended on it.

## Repository Shape

- Six Rust workspace crates.
- About 272 Rust files and 100 documentation files.
- Many source files sit between 190 and 200 lines.
- More than 650 tracked files exist under tmp, plus many ignored run artifacts.
- Cargo.lock exists locally but is ignored and not tracked.

## Inspection Limits

This environment had no cargo, rustc, Docker, or sqlite3 command. Static source
inspection, Git history, GitHub repository reads, archive evidence, and
read-only SQLite inspection through Python were available. The downstream agent
must rerun all compilation and container gates.
