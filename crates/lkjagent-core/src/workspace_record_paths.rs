use crate::workspace_record::slug;

pub fn record_path(kind: &str, id: &str) -> Result<String, String> {
    record_path_at(kind, id, "", "", default_state(kind))
}

pub fn record_path_at(
    kind: &str,
    id: &str,
    now: &str,
    title: &str,
    state: &str,
) -> Result<String, String> {
    safe_segment(kind)?;
    safe_segment(id)?;
    let base = match kind {
        "today" | "journal" => dated_path("records/life/journal", now, "entry.md"),
        "todo" => format!("records/life/todo/{}/{}.md", safe_state(state), id),
        "calendar" => dated_path("records/life/calendar", now, &format!("{id}.md")),
        "finance" => month_path("records/life/finance", now, &format!("{id}.md")),
        "note" => format!("records/life/notes/{id}.md"),
        "routine" => format!("records/life/routines/{id}.md"),
        "contact" => format!("records/life/contacts/{id}.md"),
        "reference" => format!("records/knowledge/references/{id}.md"),
        "project" => format!("records/work/projects/{}/{}.md", title_slug(title), id),
        "development" => format!("records/work/development/{}/{}.md", title_slug(title), id),
        "artifact" => format!("artifacts/documents/{id}.md"),
        "proof" => format!("artifacts/proof/{id}.md"),
        other => format!("records/knowledge/notes/{other}/{id}.md"),
    };
    Ok(base)
}

pub fn archive_path(kind: &str, id: &str) -> Result<String, String> {
    safe_segment(kind)?;
    safe_segment(id)?;
    Ok(format!("archive/records/{kind}/{id}.md"))
}

pub fn date_compact(now: &str) -> Option<String> {
    date_parts(now).map(|(year, month, day)| format!("{year}{month}{day}"))
}

fn dated_path(root: &str, now: &str, leaf: &str) -> String {
    match date_parts(now) {
        Some((year, month, day)) => format!("{root}/{year}/{month}/{day}/{leaf}"),
        None => format!("{root}/undated/{leaf}"),
    }
}

fn month_path(root: &str, now: &str, leaf: &str) -> String {
    match date_parts(now) {
        Some((year, month, _)) => format!("{root}/{year}/{month}/{leaf}"),
        None => format!("{root}/undated/{leaf}"),
    }
}

fn date_parts(now: &str) -> Option<(String, String, String)> {
    let bytes = now.as_bytes();
    if bytes.len() < 10 {
        return None;
    }
    let date = &now[..10];
    let valid = date.as_bytes()[0..4].iter().all(u8::is_ascii_digit)
        && date.as_bytes()[4] == b'-'
        && date.as_bytes()[5..7].iter().all(u8::is_ascii_digit)
        && date.as_bytes()[7] == b'-'
        && date.as_bytes()[8..10].iter().all(u8::is_ascii_digit);
    if valid {
        Some((
            date[0..4].to_string(),
            date[5..7].to_string(),
            date[8..10].to_string(),
        ))
    } else {
        None
    }
}

fn safe_state(value: &str) -> String {
    if safe_segment(value).is_ok() {
        value.to_string()
    } else {
        "open".to_string()
    }
}

fn title_slug(title: &str) -> String {
    let value = slug(title);
    if value.is_empty() {
        "general".to_string()
    } else {
        value
    }
}

fn default_state(kind: &str) -> &'static str {
    match kind {
        "calendar" => "due",
        "finance" => "review",
        "project" => "active",
        _ => "open",
    }
}

fn safe_segment(value: &str) -> Result<(), String> {
    let safe = !value.is_empty()
        && value != "."
        && value != ".."
        && !value.contains('/')
        && !value.contains('\\');
    if safe {
        Ok(())
    } else {
        Err(format!("unsafe record path segment: {value}"))
    }
}
