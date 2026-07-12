use std::path::Path;

pub fn source_errors(path: &Path, text: &str, source: &str) -> Vec<String> {
    let label = path.to_string_lossy();
    let names = ["source", "source_commit", "git_head"];
    let lines = text.lines().collect::<Vec<_>>();
    let mut errors = Vec::new();
    if marker_file(path) && text.trim() != source {
        errors.push(format!("{label}: stale source marker"));
    }
    for line in &lines {
        let columns = line.split('\t').collect::<Vec<_>>();
        if columns.len() == 2
            && names.contains(&columns[0].trim().to_ascii_lowercase().as_str())
            && columns[1].trim() != source
        {
            errors.push(format!("{label}: stale source marker"));
        }
    }
    if let Some(header) = lines.first() {
        let columns = header.split('\t').collect::<Vec<_>>();
        for (index, name) in columns
            .iter()
            .enumerate()
            .filter(|_| columns.len() > 2)
            .filter(|(_, name)| names.contains(&name.trim().to_ascii_lowercase().as_str()))
        {
            for row in lines.iter().skip(1) {
                if row
                    .split('\t')
                    .nth(index)
                    .is_some_and(|value| value.trim() != source)
                {
                    errors.push(format!("{label}: stale {name} marker"));
                }
            }
        }
    }
    errors.sort();
    errors.dedup();
    errors
}

fn marker_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| matches!(name, "source.txt" | "source-commit.txt" | "git-head.txt"))
}
