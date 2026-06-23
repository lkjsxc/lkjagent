use super::names::slug;

pub(super) fn request_readme() -> String {
    readme(
        "Request",
        "Request pages preserve the objective and raw terms before expansion.",
        &[
            "objective.md: normalized owner objective.",
            "terms.md: owner terms and source boundary.",
        ],
    )
}

pub(super) fn project_readme() -> String {
    readme(
        "Project",
        "Project pages own artifact operating rules for this generated root.",
        &[
            "operating-rules.md: construction rules.",
            "evidence-gates.md: audit and verification gates.",
        ],
    )
}

pub(super) fn relations_readme() -> String {
    readme(
        "Relations",
        "Relation pages connect requested topics before broad expansion.",
        &[
            "topic-map.md: requested term relations.",
            "artifact-map.md: artifact root and readiness map.",
        ],
    )
}

pub(super) fn topics_readme(terms: &[String]) -> String {
    let entries = terms
        .iter()
        .map(|term| format!("{}.md: owner-provided topic `{term}`.", slug(term)))
        .collect::<Vec<_>>();
    readme(
        "Topics",
        "Topic pages track owner terms without invented facts.",
        &entries,
    )
}

fn readme(title: &str, purpose: &str, entries: &[impl AsRef<str>]) -> String {
    let list = entries
        .iter()
        .map(|entry| {
            format!(
                "- [{}]({})",
                entry.as_ref(),
                link_from_entry(entry.as_ref())
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "# {title}\n\n## Purpose\n\n{purpose}\n\n## Table of Contents\n\n{list}\n\n## Local Map\n\nFollow these pages before adding broad documentation categories.\n\n## Status\n\nstructure-only\n"
    )
}

fn link_from_entry(entry: &str) -> &str {
    entry.split(':').next().unwrap_or(entry)
}
