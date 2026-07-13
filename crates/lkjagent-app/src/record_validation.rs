type R<T> = Result<T, String>;

pub(crate) fn authored(family: &str, title: &str, body: &str) -> R<()> {
    if title.trim().is_empty() || body.trim().is_empty() {
        return Err(format!("{family} title and body must be nonempty"));
    }
    if title
        .chars()
        .any(|value| matches!(value, '\r' | '\n') || value.is_control())
    {
        return Err(format!("{family} title must be one safe line"));
    }
    if body
        .chars()
        .any(|value| value.is_control() && !matches!(value, '\n' | '\t'))
    {
        return Err(format!("{family} body contains unsafe controls"));
    }
    if crate::journal_checks::known_placeholder(title)
        || crate::journal_checks::known_placeholder(body)
    {
        return Err(format!("{family} contains a known placeholder"));
    }
    if unsafe_record_text(title, body) {
        return Err(format!(
            "{family} contains prohibited sensitive or raw output content"
        ));
    }
    Ok(())
}

fn unsafe_record_text(title: &str, body: &str) -> bool {
    let text = format!("{title}\n{body}").to_ascii_lowercase();
    let sensitive = [
        "password:",
        "password=",
        "secret:",
        "secret=",
        "token:",
        "token=",
        "api_key:",
        "api_key=",
        "authorization: bearer",
    ];
    let raw = body.trim();
    sensitive.iter().any(|value| text.contains(value))
        || ((raw.starts_with('{') || raw.starts_with('['))
            && serde_json::from_str::<serde_json::Value>(raw).is_ok())
        || [
            "model-output-rejected",
            "failed model output",
            "harness fault",
            "raw harness json",
        ]
        .iter()
        .any(|value| text.contains(value))
}
