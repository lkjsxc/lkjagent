use lkjagent_effects::workspace::OpenedWorkspace;
use lkjagent_store::transactions::NativeStore;
use std::path::Path;

type R<T> = Result<T, String>;
const TOKEN_LIMIT: usize = 512;

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
    crate::record_validation::authored("report", title, body)?;
    let slug = crate::memory_record::semantic_slug(title)
        .ok_or("report title cannot produce a semantic slug")?;
    let path = format!("artifacts/reports/{slug}.md");
    let sources = crate::journal_dispatch::source_rows(db, decision)?;
    let rendered = render(&slug, &sources, title, body);
    if crate::journal_checks::token_units(&rendered) > TOKEN_LIMIT {
        return Err("report document exceeds 512 conservative token units".into());
    }
    let root = crate::config::workspace_root(data)?;
    let root = crate::workspace_root::open(&root)?;
    let workspace = OpenedWorkspace::open(&root).map_err(err)?;
    let prepared = crate::journal_apply::prepare(db, &workspace, &path, &rendered)?;
    crate::journal_apply::apply(
        db, store, &workspace, matter, decision, entry, raw, &path, prepared,
    )
}

pub(crate) fn render(slug: &str, sources: &[(String, String)], title: &str, body: &str) -> String {
    let lineage = sources
        .iter()
        .map(|(kind, fingerprint)| format!("- {kind}:{fingerprint}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "---\nkind: report\nsemantic-key: {slug}\nslug: {slug}\nsource-lineage:\n{lineage}\n---\n# {title}\n\n{}\n",
        body.trim()
    )
}

fn err(value: impl std::fmt::Debug) -> String {
    format!("{value:?}")
}
