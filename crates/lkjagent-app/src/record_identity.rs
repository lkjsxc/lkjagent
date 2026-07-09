use lkjagent_core::workspace_record::{date_compact, render_record, slug, WorkspaceRecord};

const TARGET_TOKENS: usize = 512;
const CHARS_PER_TOKEN: usize = 4;
const PART_CHARS: usize = 1600;

pub(crate) struct PreparedRecord {
    pub main_text: String,
    pub parts: Vec<RecordPart>,
}

pub(crate) struct RecordPart {
    pub rel: String,
    pub text: String,
}

pub fn normalized_kind(kind: &str) -> &str {
    if kind == "today" {
        "journal"
    } else {
        kind
    }
}

pub fn record_id(kind: &str, now: &str, title: &str) -> String {
    if kind == "journal" {
        if let Some(date) = date_compact(now) {
            return format!("journal_{date}");
        }
    }
    let stamp = now
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect::<String>();
    let suffix = slug(title);
    format!("rec_{}_{}", stamp, suffix)
}

pub(crate) fn prepare_record_text(rel: &str, record: &WorkspaceRecord) -> PreparedRecord {
    let original = render_record(record);
    if estimated_tokens(&original) <= TARGET_TOKENS {
        return PreparedRecord {
            main_text: original,
            parts: Vec::new(),
        };
    }
    let chunks = body_chunks(&record.body);
    if chunks.len() < 2 {
        return PreparedRecord {
            main_text: original,
            parts: Vec::new(),
        };
    }
    let part_dir = part_dir(rel);
    let link_prefix = link_prefix(rel);
    let parts = chunks
        .iter()
        .enumerate()
        .map(|(index, chunk)| part(&part_dir, rel, record, chunk, index, chunks.len()))
        .collect::<Vec<_>>();
    let mut main = record.clone();
    main.body = main_body(&link_prefix, original.len(), chunks.len());
    PreparedRecord {
        main_text: render_record(&main),
        parts,
    }
}

fn estimated_tokens(text: &str) -> usize {
    text.len().div_ceil(CHARS_PER_TOKEN)
}

fn body_chunks(body: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();
    for ch in body.chars() {
        current.push(ch);
        if current.len() >= PART_CHARS && (ch.is_whitespace() || current.len() > PART_CHARS * 2) {
            chunks.push(current.trim().to_string());
            current.clear();
        }
    }
    if !current.trim().is_empty() {
        chunks.push(current.trim().to_string());
    }
    chunks
}

fn part(
    part_dir: &str,
    rel: &str,
    record: &WorkspaceRecord,
    chunk: &str,
    index: usize,
    total: usize,
) -> RecordPart {
    let number = index + 1;
    let name = format!("part-{number:03}.md");
    RecordPart {
        rel: format!("{part_dir}/{name}"),
        text: format!(
            "# {} Part {number:03}\n\nSource record: [{}](../{})\nPart: {number}/{total}\n\n{}\n",
            record.title,
            file_name(rel),
            file_name(rel),
            chunk
        ),
    }
}

fn main_body(link_prefix: &str, original_bytes: usize, total: usize) -> String {
    let mut lines = vec![
        format!(
            "Size justification: original record was about {} tokens, above target {TARGET_TOKENS}.",
            original_bytes.div_ceil(CHARS_PER_TOKEN)
        ),
        "Owner data is split into linked part files without truncation.".to_string(),
        String::new(),
        "Parts:".to_string(),
    ];
    for number in 1..=total {
        lines.push(format!(
            "- [part-{number:03}]({link_prefix}/part-{number:03}.md)"
        ));
    }
    lines.join("\n")
}

fn part_dir(rel: &str) -> String {
    rel.strip_suffix(".md").unwrap_or(rel).to_string() + ".parts"
}

fn link_prefix(rel: &str) -> String {
    format!("{}.parts", file_name(rel).trim_end_matches(".md"))
}

fn file_name(rel: &str) -> &str {
    rel.rsplit('/').next().unwrap_or(rel)
}
