use std::path::Path;

use crate::model::{RepoFile, Violation};

const ALLOWED_EXTERNAL: &[&str] = &[
    "crossterm",
    "ratatui",
    "reqwest",
    "rusqlite",
    "serde",
    "serde_json",
    "toml",
    "unicode-segmentation",
    "unicode-width",
];
const FORBIDDEN_RUST: &[&str] = &[".unwrap(", ".expect(", "panic!", "todo!", "unimplemented!"];
const FORBIDDEN_SKILL_RUST: &[&str] = &[
    "SkillRegistry",
    "struct Skill",
    "enum Skill",
    "trait Skill",
    "skills::",
];

pub fn check_style(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    for file in files.iter().filter(|file| is_product_rust(&file.path)) {
        violations.extend(check_rust_file(file));
    }
    for file in files.iter().filter(|file| is_product_manifest(&file.path)) {
        violations.extend(check_manifest(file));
    }
    for file in files {
        violations.extend(check_skill_surface(file));
    }
    violations
}

fn is_product_rust(path: &str) -> bool {
    path.starts_with("crates/lkjagent-")
        && !path.starts_with("crates/lkjagent-xtask/")
        && path.ends_with(".rs")
}

fn is_product_manifest(path: &str) -> bool {
    path.starts_with("crates/lkjagent-")
        && !path.starts_with("crates/lkjagent-xtask/")
        && path.ends_with("/Cargo.toml")
}

fn check_rust_file(file: &RepoFile) -> Vec<Violation> {
    let mut violations = Vec::new();
    for (index, line) in file.text.lines().enumerate() {
        let line_number = index + 1;
        for token in FORBIDDEN_RUST {
            if line.contains(token) {
                violations.push(Violation::new(
                    &file.path,
                    "panic path",
                    format!("line {line_number} contains '{token}'; return an error value instead"),
                ));
            }
        }
        for token in FORBIDDEN_SKILL_RUST {
            if line.contains(token) {
                violations.push(Violation::new(
                    &file.path,
                    "skill surface",
                    format!(
                        "line {line_number} contains '{token}'; model guidance belongs in the graph"
                    ),
                ));
                break;
            }
        }
    }
    violations
}

fn check_skill_surface(file: &RepoFile) -> Vec<Violation> {
    if file.path.split('/').any(|segment| segment == "skills") {
        return vec![Violation::new(
            &file.path,
            "skill surface",
            "remove product-level skills directories; use graph nodes and context packages",
        )];
    }
    Vec::new()
}

fn check_manifest(file: &RepoFile) -> Vec<Violation> {
    let mut violations = Vec::new();
    let mut in_dependencies = false;
    for line in file.text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_dependencies = matches!(
                trimmed,
                "[dependencies]" | "[dev-dependencies]" | "[build-dependencies]"
            );
            continue;
        }
        if !in_dependencies || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some(name) = dependency_name(trimmed) {
            if !name.starts_with("lkjagent-") && !ALLOWED_EXTERNAL.contains(&name) {
                violations.push(Violation::new(
                    &file.path,
                    "dependency allowlist",
                    format!("dependency '{name}' is not documented as allowed"),
                ));
            }
        }
    }
    violations
}

fn dependency_name(line: &str) -> Option<&str> {
    line.split('=')
        .next()
        .map(str::trim)
        .filter(|name| !name.is_empty())
}

const CURRENT_CRATES: &[&str] = &[
    "lkjagent-core",
    "lkjagent-store",
    "lkjagent-llm",
    "lkjagent-effects",
    "lkjagent-app",
    "lkjagent-xtask",
];
const REMOVED_CRATES: &[&str] = &[
    "lkjagent-runtime",
    "lkjagent-graph",
    "lkjagent-tools",
    "lkjagent-cli",
    "lkjagent-context",
    "lkjagent-protocol",
    "lkjagent-benchmark",
];

pub fn run_structure(args: &[String], root: &Path) -> i32 {
    if !args.is_empty() && args != ["audit"] {
        return structure_failure("use: structure audit");
    }
    match audit_structure(root) {
        Ok(()) => {
            println!("ok structure audit");
            0
        }
        Err(error) => structure_failure(&error),
    }
}

pub fn audit_structure(root: &Path) -> Result<(), String> {
    for name in CURRENT_CRATES {
        if !root.join("crates").join(name).is_dir() {
            return Err(format!("missing crate {name}"));
        }
    }
    for name in REMOVED_CRATES {
        if root.join("crates").join(name).exists() {
            return Err(format!("removed crate still present {name}"));
        }
    }
    Ok(())
}

fn structure_failure(message: &str) -> i32 {
    eprintln!("structure audit failed");
    eprintln!("exit status: 1");
    eprintln!("{message}");
    1
}
