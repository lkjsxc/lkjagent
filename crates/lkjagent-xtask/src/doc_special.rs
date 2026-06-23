use std::collections::BTreeSet;

use crate::model::{RepoFile, Violation};

pub fn check_special_docs(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    violations.extend(check_task_shapes(files));
    violations.extend(check_model_name_claims(files));
    violations.extend(check_crate_readmes(files));
    violations.extend(check_generated_boilerplate(files));
    violations
}

fn headings(file: &RepoFile) -> Vec<String> {
    file.text
        .lines()
        .filter_map(|line| line.strip_prefix("## "))
        .map(str::to_string)
        .collect()
}

fn check_task_shapes(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    let required = [
        "Purpose",
        "Status",
        "Depends On",
        "Files To Read",
        "Files To Touch",
        "Focused Gate",
        "Acceptance",
        "Must Not",
    ];
    for file in files.iter().filter(|file| is_task(file)) {
        if headings(file) != required {
            violations.push(Violation::new(
                &file.path,
                "task shape",
                "headings must match the task template",
            ));
        }
    }
    violations
}

fn is_task(file: &RepoFile) -> bool {
    file.path.starts_with("docs/execution/tasks/")
        && file.path.ends_with(".md")
        && !file.path.ends_with("/README.md")
}

fn check_model_name_claims(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    for file in files.iter().filter(|file| file.path.starts_with("docs/")) {
        if file.path.starts_with("docs/regressions/") {
            continue;
        }
        for (index, line) in file.text.lines().enumerate() {
            if let Some(pattern) = model_name_pattern(line) {
                violations.push(Violation::new(
                    &file.path,
                    "model names",
                    format!(
                        "line {} contains '{pattern}'; use provider-neutral wording",
                        index + 1
                    ),
                ));
            }
        }
    }
    violations
}

fn model_name_pattern(line: &str) -> Option<&'static str> {
    for pattern in ["GPT-", "Qwen3.5", "Claude-", "Gemini-"] {
        if line.contains(pattern) {
            return Some(pattern);
        }
    }
    if line.to_ascii_lowercase().contains("latest model") {
        return Some("latest model");
    }
    None
}

fn check_generated_boilerplate(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    for file in files
        .iter()
        .filter(|file| !runtime_output(&file.path) && !boilerplate_allowlist(&file.path))
    {
        let old_block = [
            "Keep this file semantic and linked from its local README",
            "Record concrete facts, decisions, and verification evidence",
            "Implementation Hooks",
            "Failure Modes",
            "scaffolded",
        ]
        .iter()
        .all(|phrase| file.text.contains(phrase));
        let filler = [
            "defines the artifact role, the observed constraints",
            "Example one names a path, an invariant",
        ]
        .iter()
        .any(|phrase| file.text.contains(phrase));
        if old_block || filler {
            violations.push(Violation::new(
                &file.path,
                "generated boilerplate",
                "remove repeated generated leaf prose or put it in the explicit fixture allowlist",
            ));
        }
    }
    violations
}

fn runtime_output(path: &str) -> bool {
    path.starts_with("data/logs/") || path.starts_with("data/workspace/")
}

fn boilerplate_allowlist(path: &str) -> bool {
    matches!(
        path,
        "docs/regressions/generated-boilerplate.md"
            | "crates/lkjagent-tools/tests/doc_boilerplate.rs"
            | "crates/lkjagent-tools/src/doc/content_signals.rs"
            | "crates/lkjagent-tools/src/doc/repeated_content.rs"
            | "crates/lkjagent-xtask/src/doc_special.rs"
    )
}

fn check_crate_readmes(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    let crates = crate_names(files);
    for crate_name in crates {
        let root = format!("crates/{crate_name}");
        violations.extend(require_readme(files, &root, true));
        let source_dirs = source_dirs(files, &root);
        for source_dir in source_dirs {
            violations.extend(require_readme(files, &source_dir, false));
        }
    }
    violations
}

fn crate_names(files: &[RepoFile]) -> BTreeSet<String> {
    files
        .iter()
        .filter_map(|file| file.path.strip_prefix("crates/"))
        .filter_map(|rest| rest.split('/').next())
        .map(str::to_string)
        .collect()
}

fn source_dirs(files: &[RepoFile], root: &str) -> BTreeSet<String> {
    let mut dirs = BTreeSet::new();
    let prefix = format!("{root}/src/");
    for file in files.iter().filter(|file| file.path.starts_with(&prefix)) {
        let mut current = format!("{root}/src");
        dirs.insert(current.clone());
        let rest = file.path.trim_start_matches(&prefix);
        let mut parts = rest.split('/').peekable();
        while let Some(part) = parts.next() {
            if parts.peek().is_some() {
                current.push('/');
                current.push_str(part);
                dirs.insert(current.clone());
            }
        }
    }
    dirs
}

fn require_readme(files: &[RepoFile], dir: &str, needs_contract: bool) -> Vec<Violation> {
    let readme_path = format!("{dir}/README.md");
    let Some(readme) = files.iter().find(|file| file.path == readme_path) else {
        return vec![Violation::new(
            dir,
            "crate readme",
            "add README.md for this crate directory",
        )];
    };
    let mut violations = Vec::new();
    if needs_contract && !readme.text.contains("Doc contract:") {
        violations.push(Violation::new(
            &readme.path,
            "crate readme",
            "name the Doc contract",
        ));
    }
    if !readme.text.contains("## Table of Contents") {
        violations.push(Violation::new(
            &readme.path,
            "crate readme",
            "add a Table of Contents",
        ));
    }
    violations
}
