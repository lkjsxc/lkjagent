use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use crate::error::CliError;
use crate::store::skill_dir;

pub fn skills(data_dir: &Path) -> Result<String, CliError> {
    let root = skill_dir(data_dir);
    let known = known_paths(&root)?;
    let mut lines = Vec::new();
    for path in &known {
        let text = fs::read_to_string(path)?;
        let source = lkjagent_skills::model::SkillSource {
            path,
            text: &text,
            known_paths: &known,
        };
        if let Ok(skill) = lkjagent_skills::validate::parse(&source) {
            lines.push(format!(
                "name={} trigger={} last_refined={}",
                skill.name,
                skill.trigger,
                last_refined(Path::new(path))
            ));
        }
    }
    if lines.is_empty() {
        Ok("skills=0".to_string())
    } else {
        Ok(lines.join("\n"))
    }
}

fn known_paths(root: &Path) -> Result<BTreeSet<String>, CliError> {
    let mut paths = BTreeSet::new();
    if !root.exists() {
        return Ok(paths);
    }
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|extension| extension == "md") {
            paths.insert(path.to_string_lossy().replace('\\', "/"));
        }
    }
    Ok(paths)
}

fn last_refined(path: &Path) -> String {
    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
        .map_or_else(
            || "unknown".to_string(),
            |duration| duration.as_secs().to_string(),
        )
}
