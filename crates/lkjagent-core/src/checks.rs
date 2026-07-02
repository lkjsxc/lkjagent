use crate::model::{CheckResult, CheckSpec};
use crate::words::count_words;

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
            result("min_words", measured >= *n, format!("{measured} >= {n}"))
        }
        CheckSpec::MinWordsTotal { glob, n } => {
            let measured = files
                .iter()
                .filter(|fact| glob_match(glob, &fact.path))
                .map(|fact| count_words(&fact.content))
                .sum::<usize>();
            result(
                "min_words_total",
                measured >= *n,
                format!("{measured} >= {n}"),
            )
        }
        CheckSpec::MaxLines { path, n } => {
            let measured = file(files, path).map_or(0, |content| content.lines().count());
            result("max_lines", measured <= *n, format!("{measured} <= {n}"))
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
        CheckSpec::ReadmeCoverage { root } => readme_coverage(files, root),
        CheckSpec::LinksResolve { root } => links_resolve(files, root),
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

fn readme_coverage(files: &[FileFact], root: &str) -> CheckResult {
    let dirs = dirs_under(files, root);
    let passed = dirs.iter().all(|dir| {
        let readme = format!("{dir}/README.md");
        files.iter().any(|fact| fact.path == readme)
    });
    result("readme_coverage", passed, format!("dirs={}", dirs.len()))
}

fn links_resolve(files: &[FileFact], root: &str) -> CheckResult {
    let paths = files
        .iter()
        .map(|fact| fact.path.as_str())
        .collect::<Vec<_>>();
    let mut missing = 0;
    for fact in files.iter().filter(|fact| fact.path.starts_with(root)) {
        let base = fact.path.rsplit_once('/').map_or("", |(dir, _)| dir);
        for link in markdown_links(&fact.content) {
            let resolved = if base.is_empty() {
                link
            } else {
                format!("{base}/{link}")
            };
            if !paths.iter().any(|path| **path == resolved) {
                missing += 1;
            }
        }
    }
    result("links_resolve", missing == 0, format!("missing={missing}"))
}

fn dirs_under(files: &[FileFact], root: &str) -> Vec<String> {
    let mut dirs = vec![root.trim_end_matches('/').to_string()];
    for fact in files.iter().filter(|fact| fact.path.starts_with(root)) {
        let mut current = String::new();
        for part in fact
            .path
            .split('/')
            .take_while(|part| !part.ends_with(".md"))
        {
            if current.is_empty() {
                current.push_str(part);
            } else {
                current.push('/');
                current.push_str(part);
            }
            if current.starts_with(root) && !dirs.contains(&current) {
                dirs.push(current.clone());
            }
        }
    }
    dirs
}

fn markdown_links(text: &str) -> Vec<String> {
    let mut links = Vec::new();
    for line in text.lines() {
        let mut rest = line;
        while let Some(start) = rest.find("](") {
            let after = &rest[start + 2..];
            let Some(end) = after.find(')') else { break };
            let target = after[..end].split('#').next().unwrap_or("");
            if !target.is_empty() && !target.contains("://") && !target.starts_with('#') {
                links.push(target.to_string());
            }
            rest = &after[end + 1..];
        }
    }
    links
}
