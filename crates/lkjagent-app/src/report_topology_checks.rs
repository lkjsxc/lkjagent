use rusqlite::Transaction;
use serde_json::Value;

pub(crate) fn evaluate_map(
    tx: &Transaction<'_>,
    decision: &str,
    parameters: &[u8],
    path: &str,
    bytes: &[u8],
) -> bool {
    let Ok(value) = serde_json::from_slice::<Value>(parameters) else {
        return false;
    };
    let Ok(text) = std::str::from_utf8(bytes) else {
        return false;
    };
    let Some(slug) = value["slug"].as_str() else {
        return false;
    };
    let Some(children) = value["children"]
        .as_array()
        .map(|rows| rows.iter().filter_map(Value::as_str).collect::<Vec<_>>())
    else {
        return false;
    };
    let Some(expected) = lineage(tx, decision) else {
        return false;
    };
    if value["path"].as_str() != Some(path)
        || value["unit"].as_str() != Some("index")
        || crate::journal_checks::token_units(text)
            > value["max_token_units"].as_u64().unwrap_or(0) as usize
        || crate::journal_checks::known_placeholder(text)
    {
        return false;
    }
    let lines = text.lines().collect::<Vec<_>>();
    if lines.len() < 14
        || lines[0] != "---"
        || lines[1] != "kind: report"
        || lines[2] != format!("semantic-key: {slug}")
        || lines[3] != format!("slug: {slug}")
        || lines[4] != "unit: index"
        || lines[5] != format!("minimum-words: {}", value["minimum_words"])
        || lines[6] != "children:"
    {
        return false;
    }
    let mut index = 7;
    let mut actual_children = Vec::new();
    while let Some(line) = lines.get(index) {
        if *line == "source-lineage:" {
            break;
        }
        actual_children.push(line.strip_prefix("- ").unwrap_or(""));
        index += 1;
    }
    let Some(end) = lines
        .iter()
        .enumerate()
        .skip(index + 1)
        .find_map(|(n, line)| (*line == "---").then_some(n))
    else {
        return false;
    };
    let actual = lines[index + 1..end]
        .iter()
        .filter_map(|line| line.strip_prefix("- "))
        .collect::<Vec<_>>();
    let body = lines.get(end + 3..).unwrap_or_default().join("\n");
    let Some((body, sections)) = body.rsplit_once("\n## Sections\n") else {
        return false;
    };
    actual_children == children
        && actual == expected
        && !expected.is_empty()
        && lines
            .get(end + 1)
            .is_some_and(|line| line.starts_with("# "))
        && lines.get(end + 2) == Some(&"")
        && !body.trim().is_empty()
        && sections.trim()
            == children
                .iter()
                .map(|unit| format!("- [{unit}]({unit}.md)"))
                .collect::<Vec<_>>()
                .join("\n")
}

pub(crate) fn evaluate_member(
    tx: &Transaction<'_>,
    decision: &str,
    parameters: &[u8],
    path: &str,
    bytes: &[u8],
) -> bool {
    let Ok(value) = serde_json::from_slice::<Value>(parameters) else {
        return false;
    };
    let Ok(text) = std::str::from_utf8(bytes) else {
        return false;
    };
    let Some(slug) = value["slug"].as_str() else {
        return false;
    };
    let Some(unit) = value["unit"].as_str() else {
        return false;
    };
    let Some(expected) = lineage(tx, decision) else {
        return false;
    };
    if value["path"].as_str() != Some(path)
        || crate::journal_checks::token_units(text)
            > value["max_token_units"].as_u64().unwrap_or(0) as usize
        || crate::journal_checks::known_placeholder(text)
    {
        return false;
    }
    let lines = text.lines().collect::<Vec<_>>();
    if lines.len() < 11
        || lines[0] != "---"
        || lines[1] != "kind: report"
        || lines[2] != format!("semantic-key: {slug}")
        || lines[3] != format!("slug: {slug}")
        || lines[4] != format!("unit: {unit}")
        || lines[5] != "source-lineage:"
    {
        return false;
    }
    let Some(end) = lines
        .iter()
        .enumerate()
        .skip(6)
        .find_map(|(n, line)| (*line == "---").then_some(n))
    else {
        return false;
    };
    let actual = lines[6..end]
        .iter()
        .filter_map(|line| line.strip_prefix("- "))
        .collect::<Vec<_>>();
    actual == expected
        && !expected.is_empty()
        && lines
            .get(end + 1)
            .is_some_and(|line| line.starts_with("# "))
        && lines.get(end + 2) == Some(&"")
        && !lines
            .get(end + 3..)
            .unwrap_or_default()
            .join("\n")
            .trim()
            .is_empty()
}

fn lineage(tx: &Transaction<'_>, decision: &str) -> Option<Vec<String>> {
    let mut query = tx.prepare("SELECT source_kind,CAST(source_revision AS TEXT) FROM context_items WHERE decision_id=?1 ORDER BY rowid").ok()?;
    let rows = query
        .query_map([decision], |row| {
            Ok(format!(
                "{}:{}",
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?
            ))
        })
        .ok()?
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    (!rows.is_empty()).then_some(rows)
}
