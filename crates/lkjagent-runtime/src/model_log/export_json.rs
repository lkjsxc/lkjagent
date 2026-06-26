use std::path::Path;

pub(super) fn existing_files(dir: &Path, candidates: &[&str]) -> String {
    candidates
        .iter()
        .filter(|file| dir.join(file).is_file())
        .map(|file| format!("\"{}\"", json_escape(file)))
        .collect::<Vec<_>>()
        .join(",")
}

pub(super) fn refresh_content(dir: &Path, content: &str, candidates: &[&str]) -> Option<String> {
    let mut previously_known = listed_files(content);
    previously_known.extend(listed_missing_paths(content));
    let stripped = remove_array_field(content, "missing_files");
    let (open, close) = array_bounds(&stripped, "files")?;
    let next = format!(
        "{}[{}]{}",
        &stripped[..open],
        existing_files(dir, candidates),
        &stripped[close + 1..]
    );
    Some(insert_missing_files(
        &next,
        &missing_files(dir, &previously_known),
    ))
}

pub(super) fn json_escape(value: &str) -> String {
    value
        .chars()
        .flat_map(|ch| match ch {
            '\\' => "\\\\".chars().collect::<Vec<_>>(),
            '"' => "\\\"".chars().collect::<Vec<_>>(),
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            other => vec![other],
        })
        .collect()
}

fn missing_files(dir: &Path, previous_files: &[String]) -> String {
    previous_files
        .iter()
        .filter(|file| !dir.join(file).is_file())
        .map(|file| {
            format!(
                "{{\"path\":\"{}\",\"reason\":\"listed_file_absent\"}}",
                json_escape(file)
            )
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn listed_files(content: &str) -> Vec<String> {
    let Some((open, close)) = array_bounds(content, "files") else {
        return Vec::new();
    };
    quoted_values(&content[open + 1..close])
}

fn listed_missing_paths(content: &str) -> Vec<String> {
    let Some((open, close)) = array_bounds(content, "missing_files") else {
        return Vec::new();
    };
    content[open + 1..close]
        .split("\"path\":\"")
        .skip(1)
        .filter_map(|part| part.split('"').next().map(ToString::to_string))
        .collect()
}

fn quoted_values(content: &str) -> Vec<String> {
    content
        .split('"')
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, value)| value.to_string())
        .collect()
}

fn array_bounds(content: &str, field: &str) -> Option<(usize, usize)> {
    let marker = format!("\"{field}\":");
    let start = content.find(&marker)?;
    let open = content[start..].find('[')? + start;
    let close = content[open..].find(']')? + open;
    Some((open, close))
}

fn remove_array_field(content: &str, field: &str) -> String {
    let marker = format!(",\"{field}\":");
    let Some(start) = content.find(&marker) else {
        return content.to_string();
    };
    let Some((_, close)) = array_bounds(&content[start + 1..], field) else {
        return content.to_string();
    };
    format!("{}{}", &content[..start], &content[start + close + 2..])
}

fn insert_missing_files(content: &str, missing_files: &str) -> String {
    let newline = content.ends_with('\n');
    let trimmed = content.trim_end_matches('\n');
    let Some(end) = trimmed.rfind('}') else {
        return content.to_string();
    };
    let inserted = format!(
        "{},\"missing_files\":[{}]{}",
        &trimmed[..end],
        missing_files,
        &trimmed[end..]
    );
    if newline {
        format!("{inserted}\n")
    } else {
        inserted
    }
}
