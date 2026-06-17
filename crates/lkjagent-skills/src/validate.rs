mod links;

use crate::model::{
    Skill, SkillSource, SkillValidation, MAX_SKILL_LINES, OPTIONAL_HANDOFF, REQUIRED_HEADINGS,
};
use links::local_link_paths;

pub fn validate(source: &SkillSource<'_>) -> SkillValidation {
    let mut report = SkillValidation::default();
    check_filename(source, &mut report);
    check_h1(source, &mut report);
    check_frontmatter(source, &mut report);
    check_line_contract(source, &mut report);
    check_headings(source, &mut report);
    check_trigger(source, &mut report);
    check_links(source, &mut report);
    report
}

pub fn parse(source: &SkillSource<'_>) -> Result<Skill, SkillValidation> {
    let report = validate(source);
    if !report.is_valid() {
        return Err(report);
    }
    Ok(Skill {
        name: skill_name(source.path),
        display_name: display_name(source.text),
        trigger: section_body(source.text, "Trigger").trim().to_string(),
        text: source.text.to_string(),
        path: source.path.to_string(),
    })
}

fn check_filename(source: &SkillSource<'_>, report: &mut SkillValidation) {
    let name = skill_name(source.path);
    if !source.path.ends_with(".md") || !is_kebab(&name) {
        report.push(
            "filename",
            "filename must be the kebab-case skill name plus .md",
        );
    }
}

fn check_h1(source: &SkillSource<'_>, report: &mut SkillValidation) {
    let Some(first) = source.text.lines().next() else {
        report.push("h1", "first line must be '# Skill: <Name>'");
        return;
    };
    let Some(name) = first.strip_prefix("# Skill: ") else {
        report.push("h1", "first line must be '# Skill: <Name>'");
        return;
    };
    if name.trim().is_empty() {
        report.push("h1", "skill display name must not be empty");
    }
}

fn check_frontmatter(source: &SkillSource<'_>, report: &mut SkillValidation) {
    if source.text.starts_with("---") {
        report.push("frontmatter", "YAML frontmatter is not allowed");
    }
}

fn check_line_contract(source: &SkillSource<'_>, report: &mut SkillValidation) {
    if !source.text.is_ascii() {
        report.push("ascii", "skill text must be ASCII");
    }
    if source.text.lines().count() > MAX_SKILL_LINES {
        report.push("line-count", "skill must be at most 120 lines");
    }
    for (index, line) in source.text.lines().enumerate() {
        if line.len() > 120 {
            report.push(
                "line-width",
                format!("line {} exceeds 120 characters", index + 1),
            );
        }
    }
}

fn check_headings(source: &SkillSource<'_>, report: &mut SkillValidation) {
    let headings = headings(source.text);
    let has_handoff = headings
        .last()
        .is_some_and(|heading| *heading == OPTIONAL_HANDOFF);
    let body = if has_handoff {
        &headings[..headings.len().saturating_sub(1)]
    } else {
        headings.as_slice()
    };
    if body != REQUIRED_HEADINGS {
        report.push(
            "headings",
            "headings must be Purpose, Trigger, Context, Procedure, Checks, Must Not, optional Handoff",
        );
    }
}

fn check_trigger(source: &SkillSource<'_>, report: &mut SkillValidation) {
    let body = section_body(source.text, "Trigger");
    let trimmed = body.trim();
    if trimmed.is_empty() {
        report.push("trigger", "Trigger section must contain one sentence");
        return;
    }
    let ends_like_sentence =
        trimmed.ends_with('.') || trimmed.ends_with('!') || trimmed.ends_with('?');
    let has_second_sentence =
        trimmed.contains(". ") || trimmed.contains("! ") || trimmed.contains("? ");
    if !ends_like_sentence || has_second_sentence {
        report.push(
            "trigger",
            "Trigger section must contain exactly one sentence",
        );
    }
}

fn check_links(source: &SkillSource<'_>, report: &mut SkillValidation) {
    for (target, path) in local_link_paths(source.path, source.text) {
        if !source.known_paths.contains(&path) {
            report.push("link", format!("link target does not exist: {target}"));
        }
    }
}

fn headings(text: &str) -> Vec<&str> {
    text.lines()
        .filter_map(|line| line.strip_prefix("## "))
        .collect()
}

fn section_body(text: &str, heading: &str) -> String {
    let marker = format!("## {heading}");
    let mut lines = text.lines().skip_while(|line| *line != marker).skip(1);
    let mut body = Vec::new();
    for line in &mut lines {
        if line.starts_with("## ") {
            break;
        }
        body.push(line);
    }
    body.join("\n")
}

fn display_name(text: &str) -> String {
    text.lines()
        .next()
        .and_then(|line| line.strip_prefix("# Skill: "))
        .map(str::trim)
        .unwrap_or("")
        .to_string()
}

fn skill_name(path: &str) -> String {
    path.rsplit('/')
        .next()
        .and_then(|name| name.strip_suffix(".md"))
        .unwrap_or("")
        .to_string()
}

fn is_kebab(name: &str) -> bool {
    !name.is_empty()
        && name.contains('-')
        && !name.starts_with('-')
        && !name.ends_with('-')
        && !name.contains("--")
        && name.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
}
