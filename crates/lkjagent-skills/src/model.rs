use std::collections::BTreeSet;

pub const REQUIRED_HEADINGS: [&str; 6] = [
    "Purpose",
    "Trigger",
    "Context",
    "Procedure",
    "Checks",
    "Must Not",
];
pub const OPTIONAL_HANDOFF: &str = "Handoff";
pub const MAX_SKILL_LINES: usize = 120;

#[derive(Debug, Clone, Copy)]
pub struct SkillSource<'a> {
    pub path: &'a str,
    pub text: &'a str,
    pub known_paths: &'a BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
    pub name: String,
    pub display_name: String,
    pub trigger: String,
    pub text: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillViolation {
    pub rule: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SkillValidation {
    pub violations: Vec<SkillViolation>,
}

impl SkillValidation {
    pub fn push(&mut self, rule: &'static str, message: impl Into<String>) {
        self.violations.push(SkillViolation {
            rule,
            message: message.into(),
        });
    }

    pub fn is_valid(&self) -> bool {
        self.violations.is_empty()
    }

    pub fn messages(&self) -> Vec<String> {
        self.violations
            .iter()
            .map(|violation| violation.message.clone())
            .collect()
    }
}
