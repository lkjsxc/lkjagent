pub fn readme_body(title: &str, role: &str, entries: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\nThis README is the local map for the {role}.\n\n## Table of Contents\n\n{entries}\n\n## Local Map\n\n{entries}\n\n## Current Gate\n\nCompletion requires a later audit to prove topology, path hygiene, and content readiness.\n\n## Status\n\nstructure-only\n"
    )
}

pub fn leaf_body(title: &str, role: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\nThis page reserves the `{role}` path for the active documentation objective.\n\n## Structure State\n\ncontent_state=structure-only\npath_role={role}\n\n## Source Boundary\n\n- Known source: generated structure plan.\n- Missing evidence: owner terms, local source paths, observed facts, decisions, or commands for this page.\n\n## Next Evidence\n\nLink this path to a request term, source file, audit result, or verification command before treating it as content-ready.\n\n## Links\n\n- Local README: README.md.\n- Catalog metadata: catalog.toml.\n"
    )
}
