pub fn domain_readme() -> String {
    doc(
        "Domain Examples",
        "Domain examples test whether objectives stay specific without unsupported encyclopedic claims.",
        vec![
            "[asia-foods.md](asia-foods.md): cultural specificity and drift guard example.",
            "[minecraft.md](minecraft.md): structured domain documentation example.",
            "[factorio.md](factorio.md): planning-domain documentation example.",
        ],
        "These pages exemplify objective and artifact contracts for external topics.",
    )
}

pub fn asia_foods() -> String {
    domain_leaf("Asia Foods", "Asia foods")
}

pub fn minecraft() -> String {
    domain_leaf("Minecraft", "Minecraft")
}

pub fn factorio() -> String {
    domain_leaf("Factorio", "Factorio")
}

pub fn project_model_domain_examples() -> String {
    leaf(
        "Project, Model, and Domain Examples",
        "Requested external domains test the runtime without becoming disconnected blurbs.",
        vec![
            "Asia foods exercises cultural specificity and artifact drift detection.",
            "Minecraft exercises structured domain scope and objective-match auditing.",
            "Factorio exercises planning-domain relations and controlled expansion.",
        ],
        "../domain-examples/README.md",
    )
}

fn domain_leaf(title: &str, subject: &str) -> String {
    leaf(
        title,
        &format!("{subject} is a scoped domain example for objective-contract tests."),
        vec![
            "The page records why the topic was requested instead of inventing broad facts.",
            "Expansion requires source-backed or owner-provided domain scope.",
            "Relation coverage ties the topic back to lkjagent audits and artifact planning.",
        ],
        "../relations/project-model-domain-examples.md",
    )
}

fn doc(title: &str, purpose: &str, entries: Vec<&str>, map: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\n{purpose}\n\n## Table of Contents\n\n- {}\n\n## Local Map\n\n{map}\n\n## Status\n\nimplemented\n",
        entries.join("\n- ")
    )
}

fn leaf(title: &str, purpose: &str, facts: Vec<&str>, relation: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\n{purpose}\n\n## Contract\n\n- {}\n\n## Links\n\n- Relation: {relation}.\n- Graph ledger: ../.lkj-doc-graph.md.\n\n## Status\n\nimplemented\n",
        facts.join("\n- ")
    )
}
