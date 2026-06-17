use crate::model::{RepoFile, Violation};

pub fn check_markdown_basics(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    for file in files.iter().filter(|file| file.path.ends_with(".md")) {
        violations.extend(check_shape(file));
        violations.extend(check_ascii(file));
        violations.extend(check_width_and_tables(file));
        violations.extend(check_banned_tokens(file));
    }
    violations
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
    violations
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
        if !in_fence && line.len() > 120 && !trimmed.starts_with('|') {
            violations.push(Violation::new(
                &file.path,
                "prose width",
                format!("line {line_number} exceeds 120 characters"),
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
        for token in ["version", "legacy", "backward"] {
            if word == token {
                return Some(token.to_string());
            }
        }
        if word.starts_with("deprecat") {
            return Some("deprecat".to_string());
        }
    }
    lower
        .split(|character: char| !character.is_ascii_alphanumeric())
        .find(|word| {
            let mut chars = word.chars();
            matches!(chars.next(), Some('v'))
                && chars.clone().next().is_some()
                && chars.all(|c| c.is_ascii_digit())
        })
        .map(str::to_string)
}
