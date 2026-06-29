use crate::model::{RepoFile, Violation};

pub fn check_lines(files: &[RepoFile]) -> Vec<Violation> {
    files
        .iter()
        .filter(|file| !is_runtime_log(file))
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

fn limit_for(_file: &RepoFile) -> usize {
    200
}

fn is_runtime_log(file: &RepoFile) -> bool {
    file.path.starts_with("data/logs/") || file.path.starts_with("tmp/")
}
