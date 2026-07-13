use serde_json::Value;

pub(crate) fn evaluate(parameters: &[u8], path: &str, bytes: &[u8]) -> bool {
    let Ok(value) = serde_json::from_slice::<Value>(parameters) else {
        return false;
    };
    let Ok(text) = std::str::from_utf8(bytes) else {
        return false;
    };
    if value["path"].as_str() != Some(path)
        || value["kind"].as_str() != Some("report")
        || crate::journal_checks::token_units(text)
            > value["max_token_units"].as_u64().unwrap_or(0) as usize
        || crate::journal_checks::known_placeholder(text)
    {
        return false;
    }
    let lines = text.lines().collect::<Vec<_>>();
    let Some(slug) = value["slug"].as_str() else {
        return false;
    };
    if lines.len() < 10
        || lines[0] != "---"
        || lines[1] != "kind: report"
        || lines[2] != format!("semantic-key: {slug}")
        || lines[3] != format!("slug: {slug}")
        || lines[4] != "source-lineage:"
    {
        return false;
    }
    let Some(end) = lines
        .iter()
        .enumerate()
        .skip(5)
        .find_map(|(index, line)| (*line == "---").then_some(index))
    else {
        return false;
    };
    let actual = lines[5..end]
        .iter()
        .filter_map(|line| line.strip_prefix("- "))
        .collect::<Vec<_>>();
    let expected = value["source_lineage"]
        .as_array()
        .map(|items| items.iter().filter_map(Value::as_str).collect::<Vec<_>>())
        .unwrap_or_default();
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
