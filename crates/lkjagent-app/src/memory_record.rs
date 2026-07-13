use lkjagent_effects::workspace::OpenedWorkspace;
use lkjagent_store::transactions::NativeStore;
use std::path::Path;

type R<T> = Result<T, String>;
const TOKEN_LIMIT: usize = 512;
const SLUG_LIMIT: usize = 80;

#[allow(clippy::too_many_arguments)]
pub(crate) fn dispatch(
    data: &Path,
    db: &Path,
    store: &mut NativeStore,
    matter: &str,
    decision: &str,
    entry: &lkjagent_core::runtime_decision::ToolViewEntry,
    raw: &str,
    title: &str,
    body: &str,
) -> R<String> {
    crate::record_validation::authored("memory", title, body)?;
    let slug = semantic_slug(title).ok_or("memory title cannot produce a semantic slug")?;
    let path = format!("knowledge/notes/{slug}.md");
    let sources = crate::journal_dispatch::source_rows(db, decision)?
        .into_iter()
        .filter_map(|(kind, fingerprint)| (kind == "owner").then_some(fingerprint))
        .collect::<Vec<_>>();
    if sources.is_empty() {
        return Err("memory source lineage has no current owner context".into());
    }
    let rendered = render(&slug, &sources, title, body);
    if crate::journal_checks::token_units(&rendered) > TOKEN_LIMIT {
        return Err("memory document exceeds 512 conservative token units".into());
    }
    let root = crate::config::workspace_root(data)?;
    let root = crate::workspace_root::open(&root)?;
    let workspace = OpenedWorkspace::open(&root).map_err(err)?;
    let prepared = crate::journal_apply::prepare(db, &workspace, &path, &rendered)?;
    crate::journal_apply::apply(
        db, store, &workspace, matter, decision, entry, raw, &path, prepared,
    )
}

pub fn semantic_slug(title: &str) -> Option<String> {
    let mut slug = String::new();
    let mut separator = false;
    for value in title.chars() {
        if value.is_ascii_alphanumeric() {
            if separator && !slug.is_empty() && slug.len() < SLUG_LIMIT {
                slug.push('-');
            }
            separator = false;
            if slug.len() < SLUG_LIMIT {
                slug.push(value.to_ascii_lowercase());
            }
        } else {
            separator = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    (!slug.is_empty()).then_some(slug)
}

pub(crate) fn render(slug: &str, sources: &[String], title: &str, body: &str) -> String {
    let lineage = sources
        .iter()
        .map(|value| format!("- {value}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "---\nkind: memory\nsemantic-key: {slug}\nslug: {slug}\nsource-fingerprints:\n{lineage}\n---\n# {title}\n\n{}\n",
        body.trim()
    )
}

fn err(value: impl std::fmt::Debug) -> String {
    format!("{value:?}")
}
