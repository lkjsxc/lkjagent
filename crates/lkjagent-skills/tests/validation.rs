use std::collections::BTreeSet;

use lkjagent_skills::load::load_text;
use lkjagent_skills::model::SkillSource;
use lkjagent_skills::validate::{parse, validate};

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn valid_skill_parses_name_display_and_trigger() -> TestResult<()> {
    let known = BTreeSet::new();
    let source = SkillSource {
        path: "docs/agent/skills/test-skill.md",
        text: &valid_skill(),
        known_paths: &known,
    };
    let skill = parse(&source).map_err(|report| report.messages().join("; "))?;
    assert_eq!(skill.name, "test-skill");
    assert_eq!(skill.display_name, "Test Skill");
    assert_eq!(skill.trigger, "A repeatable task needs this procedure.");
    Ok(())
}

#[test]
fn validation_reports_all_detected_violations() {
    let known = BTreeSet::new();
    let text = "Bad\n---\n# Skill: Bad\n\n## Purpose\né\n\n## Trigger\none. two.\n\n## Context\n[missing](missing.md)\n";
    let source = SkillSource {
        path: "docs/agent/skills/bad.md",
        text,
        known_paths: &known,
    };
    assert_eq!(
        validate(&source).messages(),
        vec![
            "filename must be the kebab-case skill name plus .md",
            "first line must be '# Skill: <Name>'",
            "skill text must be ASCII",
            "headings must be Purpose, Trigger, Context, Procedure, Checks, Must Not, optional Handoff",
            "Trigger section must contain exactly one sentence",
            "link target does not exist: missing.md",
        ]
    );
}

#[test]
fn validation_reports_line_count_and_width() {
    let known = BTreeSet::new();
    let mut text = valid_skill();
    for _ in 0..100 {
        text.push_str("\nextra");
    }
    text.push('\n');
    text.push_str(
        "this line is intentionally longer than one hundred and twenty characters so the width rule has concrete evidence in the validator table",
    );
    let source = SkillSource {
        path: "docs/agent/skills/test-skill.md",
        text: &text,
        known_paths: &known,
    };
    assert_eq!(
        validate(&source).messages(),
        vec![
            "skill must be at most 120 lines",
            "line 126 exceeds 120 characters",
        ]
    );
}

#[test]
fn load_wraps_valid_skill_as_skill_frame() -> TestResult<()> {
    let known = BTreeSet::new();
    let frame = load_text("docs/agent/skills/test-skill.md", &valid_skill(), &known)
        .map_err(|report| report.messages().join("; "))?;
    assert_eq!(frame.name, "test-skill");
    assert!(frame.content.starts_with("<skill>\n# Skill: Test Skill"));
    assert!(frame.content.ends_with("\n</skill>"));
    Ok(())
}

fn valid_skill() -> String {
    [
        "# Skill: Test Skill",
        "",
        "## Purpose",
        "",
        "Do one repeatable thing.",
        "",
        "## Trigger",
        "",
        "A repeatable task needs this procedure.",
        "",
        "## Context",
        "",
        "- Local files provide the needed context.",
        "",
        "## Procedure",
        "",
        "1. Run `pwd`.",
        "",
        "## Checks",
        "",
        "- `pwd` exits 0.",
        "",
        "## Must Not",
        "",
        "- Do not guess.",
    ]
    .join("\n")
}
