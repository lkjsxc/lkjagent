use super::model::{PlannedFile, ScaffoldInput};
use super::names::slug;
use super::semantic_workspace_readme::{
    project_readme, relations_readme, request_readme, topics_readme,
};
use super::semantic_workspace_terms::{bullet_terms, title};

pub(super) fn root_file(input: &ScaffoldInput, terms: &[String]) -> PlannedFile {
    file("README.md", "Workspace", "root", root_body(input, terms))
}

pub(super) fn request_files(input: &ScaffoldInput, terms: &[String]) -> Vec<PlannedFile> {
    vec![
        file("request/README.md", "Request", "request", request_readme()),
        file(
            "request/objective.md",
            "Objective",
            "objective",
            objective_body(input),
        ),
        file("request/terms.md", "Terms", "terms", terms_body(terms)),
    ]
}

pub(super) fn project_files() -> Vec<PlannedFile> {
    vec![
        file("project/README.md", "Project", "project", project_readme()),
        file("project/operating-rules.md", "Operating Rules", "rules", leaf(
            "Operating Rules",
            "Generated structure grows only after relation and evidence ownership are clear.",
        )),
        file("project/evidence-gates.md", "Evidence Gates", "gates", leaf(
            "Evidence Gates",
            "Completion requires audit-owned topology, content readiness, and verification evidence.",
        )),
    ]
}

pub(super) fn relation_files(input: &ScaffoldInput, terms: &[String]) -> Vec<PlannedFile> {
    vec![
        file(
            "relations/README.md",
            "Relations",
            "relations",
            relations_readme(),
        ),
        file(
            "relations/topic-map.md",
            "Topic Map",
            "topic map",
            leaf(
                "Topic Map",
                &format!("Requested terms: {}.", terms.join(", ")),
            ),
        ),
        file(
            "relations/artifact-map.md",
            "Artifact Map",
            "artifact map",
            leaf(
                "Artifact Map",
                &format!("Artifact root `{}` uses kind `{}`.", input.root, input.kind),
            ),
        ),
    ]
}

pub(super) fn topic_files(terms: &[String]) -> Vec<PlannedFile> {
    if terms.len() < 2 {
        return Vec::new();
    }
    let mut files = vec![file(
        "topics/README.md",
        "Topics",
        "topics",
        topics_readme(terms),
    )];
    files.extend(terms.iter().map(String::as_str).map(topic_file));
    files
}

pub(super) fn catalog(input: &ScaffoldInput) -> PlannedFile {
    file(
        "catalog.toml",
        "Catalog",
        "scaffold metadata",
        format!(
            "title = \"{}\"\nkind = \"{}\"\nprofile = \"GenericStructuredDocs\"\ncontent_state = \"structure-only\"\n",
            input.title, input.kind
        ),
    )
}

fn root_body(input: &ScaffoldInput, terms: &[String]) -> String {
    format!(
        "# Workspace\n\n## Purpose\n\nThis workspace contains the active documentation artifact for `{}`.\n\n## Table of Contents\n\n- [request/](request/README.md): objective and owner terms.\n- [project/](project/README.md): operating rules and evidence gates.\n- [relations/](relations/README.md): topic and artifact maps.\n- [topics/](topics/README.md): owner-term topic pages.\n- [catalog.toml](catalog.toml): compact structure metadata.\n\n## Current Gate\n\nCompletion requires topology, content readiness, artifact audit, and verification evidence.\n\n## Owner Terms\n\n{}\n\n## Status\n\nstructure-only\n",
        input.title,
        bullet_terms(terms)
    )
}

fn objective_body(input: &ScaffoldInput) -> String {
    format!(
        "# Objective\n\n## Purpose\n\nThis page records the owner objective for the generated documentation root.\n\n## Objective\n\n- Root: `{}`.\n- Title: `{}`.\n- Kind: `{}`.\n\n## Next Evidence\n\nPlan the artifact and run `doc.audit` before treating this root as ready.\n",
        input.root, input.title, input.kind
    )
}

fn terms_body(terms: &[String]) -> String {
    format!(
        "# Terms\n\n## Purpose\n\nThis page preserves owner-provided terms without adding external facts.\n\n## Terms\n\n{}\n\n## Source Boundary\n\nEach term is owner-provided until a local source path or command output supplies facts.\n",
        bullet_terms(terms)
    )
}

fn topic_file(term: &str) -> PlannedFile {
    file(
        &format!("topics/{}.md", slug(term)),
        &title(term),
        "topic",
        topic_body(term),
    )
}

fn topic_body(term: &str) -> String {
    format!(
        "# {}\n\n## Purpose\n\nThis page tracks the owner-provided term `{term}` as a requested documentation topic.\n\n## Known Source\n\ncontent_state=owner-term-only\nsource_type=owner-term\nobserved_term={term}\n\n## Relation\n\n- The term is part of the active documentation request.\n- It must connect to the artifact map before expansion.\n\n## Next Evidence\n\nAdd sourced local facts or owner-confirmed details before treating this page as content-ready.\n",
        title(term)
    )
}

fn leaf(title: &str, fact: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\n{fact}\n\n## Evidence\n\ncontent_state=structure-only\nnext_audit=doc.audit\n\n## Links\n\n- Request: ../request/README.md.\n- Relations: ../relations/README.md.\n"
    )
}

fn file(path: &str, title: &str, role: &str, body: String) -> PlannedFile {
    PlannedFile {
        path: path.to_string(),
        title: title.to_string(),
        role: role.to_string(),
        body,
    }
}
