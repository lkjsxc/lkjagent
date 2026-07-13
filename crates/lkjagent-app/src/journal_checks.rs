use serde_json::Value;

pub(crate) fn evaluate(parameters: &[u8], path: &str, bytes: &[u8]) -> bool {
    let Ok(value) = serde_json::from_slice::<Value>(parameters) else {
        return false;
    };
    let Ok(text) = std::str::from_utf8(bytes) else {
        return false;
    };
    if value["path"].as_str() != Some(path)
        || value["kind"].as_str() != Some("journal")
        || token_units(text) > value["max_token_units"].as_u64().unwrap_or(0) as usize
        || text.trim().is_empty()
        || known_placeholder(text)
    {
        return false;
    }
    structure(&value, text)
}

fn structure(value: &Value, text: &str) -> bool {
    let lines = text.lines().collect::<Vec<_>>();
    if lines.len() < 9
        || lines[0] != "---"
        || lines[1] != "kind: journal"
        || lines[2] != format!("date: {}", value["date"].as_str().unwrap_or(""))
        || lines[3] != "source-fingerprints:"
    {
        return false;
    }
    let Some(end) = lines
        .iter()
        .enumerate()
        .skip(4)
        .find_map(|(index, line)| (*line == "---").then_some(index))
    else {
        return false;
    };
    let actual = lines[4..end]
        .iter()
        .filter_map(|line| line.strip_prefix("- "))
        .collect::<Vec<_>>();
    let expected = value["source_fingerprints"]
        .as_array()
        .map(|items| items.iter().filter_map(Value::as_str).collect::<Vec<_>>())
        .unwrap_or_default();
    let title = lines.get(end + 1).and_then(|line| line.strip_prefix("# "));
    let body = lines.get(end + 3..).unwrap_or_default().join("\n");
    actual.len() == end.saturating_sub(4)
        && actual == expected
        && title.is_some_and(|value| !value.trim().is_empty())
        && lines.get(end + 2) == Some(&"")
        && !body.trim().is_empty()
}

pub(crate) fn token_units(text: &str) -> usize {
    text.len().div_ceil(4).max(text.chars().count())
}

pub(crate) fn known_placeholder(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    [
        "replace_me",
        "field_value",
        "lorem ipsum",
        "missing details",
        "no details provided",
        "[placeholder]",
        "<placeholder>",
        "tbd",
    ]
    .iter()
    .any(|value| lower.contains(value))
}
