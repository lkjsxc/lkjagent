use crate::model::RepoFile;

use super::brief::workspace_brief_findings;
use super::catalog::catalog_findings;
use super::findings::StructureFinding;
use super::readme::readme_findings;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructurePlan {
    pub root: String,
    pub fanout_cap: usize,
    pub files_to_create: Vec<String>,
    pub files_to_move: Vec<String>,
    pub files_to_delete: Vec<String>,
    pub readme_updates: Vec<String>,
    pub catalog_updates: Vec<String>,
    pub relation_updates: Vec<String>,
    pub backlink_updates: Vec<String>,
    pub weak_content_findings: Vec<StructureFinding>,
    pub line_limit_findings: Vec<StructureFinding>,
    pub findings: Vec<StructureFinding>,
    pub verification_commands: Vec<String>,
}

pub fn build_plan(files: &[RepoFile], root: &str, fanout_cap: usize) -> StructurePlan {
    let normalized = root.trim_end_matches('/').to_string();
    let mut findings = readme_findings(files, &normalized, fanout_cap);
    findings.extend(workspace_brief_findings(files, &normalized));
    findings.extend(catalog_findings(files, &normalized));
    let line_limit_findings = line_limit_findings(files, &normalized);
    findings.extend(line_limit_findings.clone());
    let weak_content_findings = findings
        .iter()
        .filter(|finding| finding.rule == "structure scaffold-content")
        .cloned()
        .collect();
    StructurePlan {
        root: normalized,
        fanout_cap,
        files_to_create: Vec::new(),
        files_to_move: Vec::new(),
        files_to_delete: Vec::new(),
        readme_updates: Vec::new(),
        catalog_updates: Vec::new(),
        relation_updates: Vec::new(),
        backlink_updates: Vec::new(),
        weak_content_findings,
        line_limit_findings,
        findings,
        verification_commands: verification_commands(root),
    }
}

fn line_limit_findings(files: &[RepoFile], root: &str) -> Vec<StructureFinding> {
    let prefix = format!("{root}/");
    files
        .iter()
        .filter(|file| file.path == root || file.path.starts_with(&prefix))
        .filter(|file| file.line_count() > 200)
        .map(|file| {
            StructureFinding::new(
                &file.path,
                "structure line-limit",
                format!("has {} lines, limit is 200", file.line_count()),
            )
        })
        .collect()
}

fn verification_commands(root: &str) -> Vec<String> {
    vec![
        format!("cargo run -p lkjagent-xtask -- structure audit --root {root}"),
        "cargo run -p lkjagent-xtask -- check-lines".to_string(),
    ]
}
