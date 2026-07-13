use rusqlite::Connection;
use std::path::Path;

pub(crate) fn allowed(objective: &[u8]) -> bool {
    let text = String::from_utf8_lossy(objective).to_ascii_lowercase();
    if text.contains("no changes") || text.contains("make no changes") {
        return true;
    }
    ![
        "write ",
        "remember ",
        "correct ",
        "edit ",
        "replace ",
        "create ",
        "update ",
        "journal",
    ]
    .iter()
    .any(|word| text.contains(word))
}

pub(crate) fn managed_path(db: &Path, objective: &[u8]) -> Option<String> {
    let connection = Connection::open(db).ok()?;
    let mut query = connection
        .prepare(
            "SELECT CAST(current_path AS TEXT) FROM workspace_documents
             WHERE status='active' AND current_revision_id IS NOT NULL ORDER BY current_path",
        )
        .ok()?;
    let paths = query
        .query_map([], |row| row.get::<_, String>(0))
        .ok()?
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    let objective = String::from_utf8_lossy(objective).to_ascii_lowercase();
    paths
        .iter()
        .find(|path| relevant(path, &objective))
        .cloned()
        .or_else(|| paths.into_iter().next())
}

fn relevant(path: &str, objective: &str) -> bool {
    let path = path.to_ascii_lowercase();
    let slug = path
        .rsplit('/')
        .next()
        .and_then(|name| name.strip_suffix(".md"))
        .unwrap_or("");
    objective.contains(&path) || (!slug.is_empty() && objective.contains(slug))
}

#[cfg(test)]
mod tests {
    use super::{allowed, relevant};

    #[test]
    fn effectful_objectives_cannot_reuse_unrelated_checks() {
        assert!(!allowed(
            b"Remember this owner fact as Transit Card Location"
        ));
        assert!(!allowed(b"Write today's grounded journal"));
        assert!(allowed(
            b"Re-read the checked file, make no changes, and report it"
        ));
        assert!(relevant(
            "knowledge/notes/transit-card-location.md",
            "use saved transit-card-location memory"
        ));
        assert!(!relevant(
            "life/journal/2026/07/13/entry.md",
            "use saved transit-card-location memory"
        ));
    }
}
