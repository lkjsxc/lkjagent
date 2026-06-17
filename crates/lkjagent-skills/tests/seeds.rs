use std::collections::BTreeSet;
use std::fs;
use std::io::{Error, ErrorKind};
use std::process::{Command, Stdio};

use lkjagent_skills::model::SkillSource;
use lkjagent_skills::validate::validate;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn seed_and_builder_skills_validate_with_shared_rules() -> TestResult<()> {
    let known = repo_paths()?;
    let mut failures = Vec::new();
    for path in known.iter().filter(|path| is_skill_path(path)) {
        let text = fs::read_to_string(path)?;
        let source = SkillSource {
            path,
            text: &text,
            known_paths: &known,
        };
        let report = validate(&source);
        if !report.is_valid() {
            failures.push(format!("{path}: {}", report.messages().join("; ")));
        }
    }
    assert_eq!(failures, Vec::<String>::new());
    Ok(())
}

#[test]
fn seed_procedure_commands_exist_in_container() -> TestResult<()> {
    for command in ["pwd", "find", "rg", "git", "cargo", "curl"] {
        let status = Command::new("sh")
            .args(["-c", &format!("command -v {command}")])
            .stdout(Stdio::null())
            .status()?;
        assert!(status.success(), "{command} is missing");
    }
    Ok(())
}

fn repo_paths() -> TestResult<BTreeSet<String>> {
    let output = Command::new("git")
        .args(["ls-files", "--cached", "--others", "--exclude-standard"])
        .output()?;
    if !output.status.success() {
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            "git ls-files failed",
        )));
    }
    let text = String::from_utf8(output.stdout)?;
    Ok(text.lines().map(str::to_string).collect())
}

fn is_skill_path(path: &str) -> bool {
    (path.starts_with("docs/agent/skills/") || path.starts_with("crates/lkjagent-skills/seeds/"))
        && path.ends_with(".md")
        && !path.ends_with("/README.md")
}
