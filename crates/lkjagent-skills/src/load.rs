use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use crate::model::{SkillSource, SkillValidation};
use crate::validate::parse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillFrame {
    pub name: String,
    pub content: String,
}

pub fn load_file(
    path: &Path,
    known_paths: &BTreeSet<String>,
) -> Result<SkillFrame, SkillValidation> {
    let text = fs::read_to_string(path).map_err(|error| io_validation(error.to_string()))?;
    let path_text = path.to_string_lossy().replace('\\', "/");
    load_text(&path_text, &text, known_paths)
}

pub fn load_text(
    path: &str,
    text: &str,
    known_paths: &BTreeSet<String>,
) -> Result<SkillFrame, SkillValidation> {
    let source = SkillSource {
        path,
        text,
        known_paths,
    };
    let skill = parse(&source)?;
    Ok(SkillFrame {
        name: skill.name,
        content: format!("<skill>\n{}\n</skill>", skill.text),
    })
}

fn io_validation(message: String) -> SkillValidation {
    let mut validation = SkillValidation::default();
    validation.push("io", format!("could not read skill file: {message}"));
    validation
}
