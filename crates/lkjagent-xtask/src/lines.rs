use crate::model::{RepoFile, Violation};

pub fn check_lines(files: &[RepoFile]) -> Vec<Violation> {
    files
        .iter()
        .filter_map(|file| {
            let limit = limit_for(file);
            let count = file.line_count();
            if count > limit {
                Some(Violation::new(
                    &file.path,
                    "line limit",
                    format!("has {count} lines, limit is {limit}; split by ownership"),
                ))
            } else {
                None
            }
        })
        .collect()
}

fn limit_for(file: &RepoFile) -> usize {
    if file.path.ends_with(".md") && file.text.starts_with("# Skill:") {
        120
    } else {
        200
    }
}
