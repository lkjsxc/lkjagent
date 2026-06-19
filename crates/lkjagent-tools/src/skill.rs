use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{ToolError, ToolResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedSkill {
    pub name: String,
    pub frame: String,
}

pub fn use_skill(library: &Path, name: &str) -> ToolResult<LoadedSkill> {
    let path = skill_path(library, name)?;
    if !path.exists() {
        return Err(ToolError::Skill(format!("unknown skill: {name}")));
    }
    let known_paths = collect_known_paths(library)?;
    let frame = lkjagent_skills::load::load_file(&path, &known_paths)
        .map_err(|report| ToolError::Skill(report.messages().join("; ")))?;
    Ok(LoadedSkill {
        name: frame.name,
        frame: frame.content,
    })
}

fn skill_path(library: &Path, name: &str) -> ToolResult<PathBuf> {
    if !valid_name(name) {
        return Err(ToolError::invalid(
            "skill name must be a kebab-case file stem",
        ));
    }
    Ok(library.join(format!("{name}.md")))
}

fn valid_name(name: &str) -> bool {
    !name.is_empty()
        && name.contains('-')
        && !name.starts_with('-')
        && !name.ends_with('-')
        && !name.contains("--")
        && name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

fn collect_known_paths(root: &Path) -> ToolResult<BTreeSet<String>> {
    let mut paths = BTreeSet::new();
    collect_dir(root, &mut paths)?;
    Ok(paths)
}

fn collect_dir(path: &Path, paths: &mut BTreeSet<String>) -> ToolResult<()> {
    if !path.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let next = entry.path();
        if next.is_dir() {
            collect_dir(&next, paths)?;
        } else if next.extension().is_some_and(|extension| extension == "md") {
            paths.insert(next.to_string_lossy().replace('\\', "/"));
        }
    }
    Ok(())
}
