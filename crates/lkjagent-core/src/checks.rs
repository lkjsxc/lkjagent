use crate::model::{CheckResult, CheckSpec};
use crate::runtime_artifact::count_words;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileFact {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandFact {
    pub cmd: String,
    pub success: bool,
}

pub fn evaluate(spec: &CheckSpec, files: &[FileFact], commands: &[CommandFact]) -> CheckResult {
    match spec {
        CheckSpec::FileExists { path } => {
            let exists = file(files, path).is_some_and(|content| !content.trim().is_empty());
            result("file_exists", exists, exists.to_string())
        }
        CheckSpec::MinWords { path, n } => {
            let measured = file(files, path).map_or(0, count_words);
            result("min_words", measured >= *n, measured.to_string())
        }
        CheckSpec::MinWordsTotal { glob, n } => {
            let measured = files
                .iter()
                .filter(|fact| glob_match(glob, &fact.path))
                .map(|fact| count_words(&fact.content))
                .sum::<usize>();
            result("min_words_total", measured >= *n, measured.to_string())
        }
        CheckSpec::MaxLines { path, n } => {
            let measured = file(files, path).map_or(0, |content| content.lines().count());
            result("max_lines", measured <= *n, measured.to_string())
        }
        CheckSpec::FileCount { glob, min, max } => {
            let measured = files
                .iter()
                .filter(|fact| glob_match(glob, &fact.path))
                .count();
            let upper_ok = max.is_none_or(|value| measured <= value);
            result(
                "file_count",
                measured >= *min && upper_ok,
                measured.to_string(),
            )
        }
        CheckSpec::Contains { path, needle } => {
            let passed = file(files, path).is_some_and(|content| content.contains(needle));
            result("contains", passed, passed.to_string())
        }
        CheckSpec::Absent { path, needle } => {
            let passed = file(files, path).is_none_or(|content| !content.contains(needle));
            result("absent", passed, passed.to_string())
        }
        CheckSpec::Command { cmd } => {
            let passed = commands.iter().any(|fact| fact.cmd == *cmd && fact.success);
            result("command", passed, passed.to_string())
        }
        CheckSpec::Judged { path, .. } => result("judged", file(files, path).is_some(), path),
        CheckSpec::ReadmeCoverage { root } => crate::checks_links::readme_coverage(files, root),
        CheckSpec::LinksResolve { root } => crate::checks_links::links_resolve(files, root),
    }
}

fn file<'a>(files: &'a [FileFact], path: &str) -> Option<&'a str> {
    files
        .iter()
        .find(|fact| fact.path == path)
        .map(|fact| fact.content.as_str())
}

fn result(name: &str, passed: bool, measured: impl Into<String>) -> CheckResult {
    CheckResult {
        name: name.to_string(),
        params: None,
        decision_id: None,
        evidence_fingerprint: None,
        artifact_refs: Vec::new(),
        passed,
        measured: measured.into(),
    }
}

fn glob_match(glob: &str, path: &str) -> bool {
    if let Some((prefix, suffix)) = glob.split_once('*') {
        path.starts_with(prefix) && path.ends_with(suffix)
    } else {
        path == glob
    }
}
