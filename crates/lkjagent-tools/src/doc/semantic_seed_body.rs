pub fn implementation_readme() -> String {
    doc(
        "Implementation",
        "Implementation docs own Rust as the substrate for runtime authority.",
        &[
            "[rust.md](rust.md): Rust ownership in the workspace.",
            "[functional-core.md](functional-core.md): pure reducer and effects boundary.",
        ],
        "Implementation supports typed state, tool schemas, and verification gates.",
    )
}

pub fn rust_page() -> String {
    leaf(
        "Rust",
        "Rust provides typed state, explicit errors, and workspace gates.",
        &[
            "Newtypes identify cases, tasks, evidence, tracks, and tool intents.",
            "Pure functions update state without filesystem, shell, queue, or model effects.",
            "Docker Compose verifies the workspace from a clean build context.",
        ],
    )
}

pub fn functional_core() -> String {
    leaf(
        "Functional Core",
        "The core returns decisions while adapters perform effects.",
        &[
            "Reducers transform events into state vectors and hard states.",
            "Authorization returns a decision instead of executing a tool.",
            "Audits record owned evidence after deterministic checks pass.",
        ],
    )
}

pub fn relation_main() -> String {
    leaf(
        "Project Model Implementation",
        "lkjagent connects runtime, model boundary, and Rust.",
        &[
            "The project depends on Rust for typed reducers and tool schemas.",
            "The project constrains the model through prompt frames and action validation.",
            "Rust implementation audits the evidence needed before completion.",
        ],
    )
}

fn doc(title: &str, purpose: &str, entries: &[&str], map: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\n{purpose}\n\n## Table of Contents\n\n- {}\n\n## Local Map\n\n{map}\n\n## Status\n\nimplemented\n",
        entries.join("\n- ")
    )
}

fn leaf(title: &str, purpose: &str, facts: &[&str]) -> String {
    format!(
        "# {title}\n\n## Purpose\n\n{purpose}\n\n## Contract\n\n- {}\n\n## Links\n\n- Relation: ../relations/project-model-implementation.md.\n- Graph ledger: ../.lkj-doc-graph.md.\n\n## Status\n\nimplemented\n",
        facts.join("\n- ")
    )
}
