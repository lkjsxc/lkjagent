use crate::runtime_fingerprint::{stable_fingerprint, FingerprintError};

pub use crate::workspace_record_paths::{archive_path, date_compact, record_path, record_path_at};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceRecord {
    pub id: String,
    pub kind: String,
    pub title: String,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>,
    pub links: Vec<String>,
    pub state_keys: Vec<String>,
    pub body: String,
}

impl WorkspaceRecord {
    pub fn new(
        id: impl Into<String>,
        kind: impl Into<String>,
        title: impl Into<String>,
        now: impl Into<String>,
    ) -> Self {
        let now = now.into();
        Self {
            id: id.into(),
            kind: kind.into(),
            title: title.into(),
            state: "open".to_string(),
            created_at: now.clone(),
            updated_at: now,
            tags: Vec::new(),
            links: Vec::new(),
            state_keys: Vec::new(),
            body: String::new(),
        }
    }
}

pub fn default_state_for_kind(kind: &str) -> &'static str {
    match kind {
        "calendar" => "due",
        "finance" => "review",
        "routine" => "ready",
        "project" => "active",
        "proof" => "collect",
        _ => "open",
    }
}

pub fn state_keys_for_record(kind: &str, id: &str, state: &str) -> Vec<String> {
    let mut keys = vec!["index:stale/records".to_string()];
    match kind {
        "todo" => keys.push(format!("todo:{state}/{id}")),
        "calendar" => keys.push(format!("calendar:{state}/{id}")),
        "finance" => keys.push(format!("finance:{state}/{id}")),
        "routine" => keys.push(format!("routine:{state}/{id}")),
        "project" => keys.push(format!("project:{state}/{id}")),
        "development" => keys.push(format!("dev:repo-work/{id}")),
        "proof" => keys.push(format!("proof:collect/{id}")),
        _ => {}
    }
    keys
}

pub fn render_record(record: &WorkspaceRecord) -> String {
    format!(
        "---\nid: {}\nkind: {}\ntitle: {}\nstate: {}\ncreated_at: {}\nupdated_at: {}\ntags: {}\nlinks: {}\nstate_keys: {}\n---\n\n# {}\n\n## Body\n\n{}\n",
        record.id,
        record.kind,
        record.title,
        record.state,
        record.created_at,
        record.updated_at,
        list(&record.tags),
        list(&record.links),
        list(&record.state_keys),
        record.title,
        record.body.trim()
    )
}

pub fn parse_record(text: &str) -> Result<WorkspaceRecord, String> {
    let rest = text
        .strip_prefix("---\n")
        .ok_or_else(|| "record frontmatter open missing".to_string())?;
    let (head, body) = rest
        .split_once("\n---\n")
        .ok_or_else(|| "record frontmatter close missing".to_string())?;
    let value = |key: &str| field(head, key).ok_or_else(|| format!("record {key} missing"));
    Ok(WorkspaceRecord {
        id: value("id")?,
        kind: value("kind")?,
        title: value("title")?,
        state: value("state")?,
        created_at: value("created_at")?,
        updated_at: value("updated_at")?,
        tags: parse_list(&value("tags")?),
        links: parse_list(&value("links")?),
        state_keys: parse_list(&value("state_keys")?),
        body: body.trim_start().to_string(),
    })
}

pub fn record_fingerprint(text: &str) -> Result<String, FingerprintError> {
    stable_fingerprint(&text)
}

pub fn slug(text: &str) -> String {
    let slug = text
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>();
    slug.split('-')
        .filter(|part| !part.is_empty())
        .take(8)
        .collect::<Vec<_>>()
        .join("-")
}

fn field(head: &str, key: &str) -> Option<String> {
    let prefix = format!("{key}: ");
    head.lines()
        .find_map(|line| line.strip_prefix(&prefix).map(str::to_string))
}

fn list(values: &[String]) -> String {
    if values.is_empty() {
        return "[]".to_string();
    }
    format!("[{}]", values.join(", "))
}

fn parse_list(value: &str) -> Vec<String> {
    value
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(str::to_string)
        .collect()
}
