pub fn readme_body(title: &str, role: &str, entries: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\n{role} for this generated documentation root.\n\n## Table of Contents\n\n{entries}\n\n## Local Map\n\n{entries}\n\n## Reading Paths\n\n- Implementation path: read this README, then follow each linked child.\n- Diagnosis path: inspect the graph manifest, then audit failed paths.\n- Verification path: run `doc.audit` for this root.\n\n## Cross-Links\n\n- Related contract: `.lkj-doc-graph.md`.\n- Owning crate or module: `crates/lkjagent-tools/src/doc.rs`.\n"
    )
}

pub fn leaf_body(title: &str, role: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\nThis file records the {role} role for the generated documentation tree.\n\n## Contract\n\n- Keep this file semantic and linked from its local README.\n- Record concrete facts, decisions, and verification evidence.\n\n## Implementation Hooks\n\n- Source: `crates/lkjagent-tools/src/doc.rs`\n- Tests: `crates/lkjagent-tools/tests/typed_tools.rs`\n- Verification: `docker compose run --rm verify`\n\n## Failure Modes\n\n- The file is unlinked from its directory README.\n- The file becomes a placeholder without role-specific content.\n\n## Status\n\nscaffolded\n"
    )
}
