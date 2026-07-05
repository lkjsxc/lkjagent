use crate::model::{RepoFile, Violation};

pub fn check_markdown_basics(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    for file in files
        .iter()
        .filter(|file| file.path.ends_with(".md"))
        .filter(|file| is_checked_markdown(&file.path))
        .filter(|file| !is_runtime_output(file))
    {
        violations.extend(check_shape(file));
        violations.extend(check_ascii(file));
        violations.extend(check_width_and_tables(file));
        violations.extend(check_banned_tokens(file));
        violations.extend(check_status_section(file));
    }
    violations
}

fn is_checked_markdown(path: &str) -> bool {
    path.starts_with("docs/") || matches!(path, "README.md" | "AGENTS.md")
}

fn is_runtime_output(file: &RepoFile) -> bool {
    file.path.starts_with("data/logs/")
        || file.path.starts_with("data/workspace/")
        || file.path.starts_with("tmp/")
}

fn check_shape(file: &RepoFile) -> Vec<Violation> {
    let mut violations = Vec::new();
    let mut lines = file.text.lines();
    match lines.next() {
        Some(first) if first.starts_with("# ") && !first.starts_with("## ") => {}
        _ => violations.push(Violation::new(
            &file.path,
            "doc shape",
            "first line must be an H1 beginning with '# '",
        )),
    }
    let second_nonempty = lines.find(|line| !line.trim().is_empty());
    if second_nonempty != Some("## Purpose") {
        violations.push(Violation::new(
            &file.path,
            "doc shape",
            "second nonempty line must be '## Purpose'",
        ));
    }
    let h1_count = headings_outside_fences(&file.text, "# ").len();
    if h1_count > 1 {
        violations.push(Violation::new(
            &file.path,
            "doc shape",
            format!("must contain exactly one H1, found {h1_count}"),
        ));
    }
    violations
}

fn headings_outside_fences(text: &str, prefix: &str) -> Vec<usize> {
    let mut in_fence = false;
    let mut lines = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if !in_fence && line.starts_with(prefix) {
            lines.push(index + 1);
        }
    }
    lines
}

fn check_ascii(file: &RepoFile) -> Vec<Violation> {
    if file.text.is_ascii() {
        Vec::new()
    } else {
        vec![Violation::new(
            &file.path,
            "ascii",
            "replace non-ASCII characters",
        )]
    }
}

fn check_width_and_tables(file: &RepoFile) -> Vec<Violation> {
    let mut violations = Vec::new();
    let mut in_fence = false;
    for (index, line) in file.text.lines().enumerate() {
        let line_number = index + 1;
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if !in_fence && line.len() > 100 && !trimmed.starts_with('|') {
            violations.push(Violation::new(
                &file.path,
                "prose width",
                format!("line {line_number} exceeds 100 characters"),
            ));
        }
        if trimmed.starts_with('|') {
            let columns = trimmed.trim_matches('|').split('|').count();
            if columns > 6 {
                violations.push(Violation::new(
                    &file.path,
                    "table width",
                    format!("line {line_number} has {columns} columns; split the table"),
                ));
            }
        }
    }
    violations
}

fn check_banned_tokens(file: &RepoFile) -> Vec<Violation> {
    let mut violations = Vec::new();
    for (index, line) in file.text.lines().enumerate() {
        if let Some(token) = banned_token(line) {
            violations.push(Violation::new(
                &file.path,
                "banned token",
                format!(
                    "line {} contains '{}'; state the current contract directly",
                    index + 1,
                    token
                ),
            ));
        }
    }
    violations
}

fn banned_token(line: &str) -> Option<String> {
    let lower = line.to_ascii_lowercase();
    for word in lower.split(|character: char| !character.is_ascii_alphanumeric()) {
        for token in ["version", "legacy", "backward", "compatibility", "tbd"] {
            if word == token {
                return Some(token.to_string());
            }
        }
        if word.starts_with("deprecat") {
            return Some("deprecat".to_string());
        }
        let mut chars = word.chars();
        if matches!(chars.next(), Some('v'))
            && chars.clone().next().is_some()
            && chars.all(|c| c.is_ascii_digit())
        {
            return Some(word.to_string());
        }
    }
    None
}

fn check_status_section(file: &RepoFile) -> Vec<Violation> {
    if file.path != "docs/current-state.md" && file.text.lines().any(|line| line == "## Status") {
        return vec![Violation::new(
            &file.path,
            "status section",
            "only docs/current-state.md may carry status ledger sections",
        )];
    }
    Vec::new()
}
