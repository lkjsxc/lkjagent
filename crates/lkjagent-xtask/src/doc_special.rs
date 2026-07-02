use crate::model::{RepoFile, Violation};

pub fn check_special_docs(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    violations.extend(check_model_name_claims(files));
    violations.extend(check_generated_boilerplate(files));
    violations
}

fn check_model_name_claims(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    for file in files.iter().filter(|file| file.path.starts_with("docs/")) {
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
        let repeated_leaf = [
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
        if repeated_leaf || filler {
            violations.push(Violation::new(
                &file.path,
                "generated boilerplate",
                "remove repeated generated leaf prose",
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
        "crates/lkjagent-tools/tests/doc_boilerplate.rs"
            | "crates/lkjagent-tools/src/doc/content_signals.rs"
            | "crates/lkjagent-tools/src/doc/repeated_content.rs"
            | "crates/lkjagent-xtask/src/doc_special.rs"
    )
}
