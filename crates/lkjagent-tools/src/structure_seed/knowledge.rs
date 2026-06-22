use std::fs;
use std::path::Path;

use super::model::{write_catalog, Counts};
use crate::error::ToolResult;

mod readmes;
mod topics;

const LINKS: &str = "- [Concept Network](/docs/maps/concept-network.md): map related ideas.\n- [Glossary](/docs/reference/glossary.md): define terms.\n- [Ontology](/docs/reference/ontology.md): classify concepts.\n- [Curation Workflow](/docs/curation/workflow.md): keep the network healthy.";

pub fn scaffold(workspace: &Path) -> ToolResult<String> {
    let mut counts = Counts::default();
    write_readmes(workspace, &mut counts)?;
    for (base, topics) in topics::GROUPS {
        write_topics(workspace, base, topics, &mut counts)?;
    }
    write_catalog(workspace, "knowledge", &mut counts)?;
    crate::structure_network::verify_knowledge_network(workspace)?;
    Ok(format!(
        "knowledge nucleus profile=knowledge root=docs\ncatalog=docs/catalog.toml\ncreated_files={}\nskipped_existing={}\ngrowth=incremental\nnext_step=expand one queued topic, then update maps and rebalance-plan\nverification=ok",
        counts.created, counts.skipped
    ))
}

fn write_readmes(workspace: &Path, counts: &mut Counts) -> ToolResult<()> {
    for (path, title, purpose, entries) in readmes::README_DATA {
        write_missing(
            workspace,
            path,
            &readme_content(title, purpose, entries),
            counts,
        )?;
    }
    Ok(())
}

fn write_topics(
    workspace: &Path,
    base: &str,
    topics: &[topics::Topic],
    counts: &mut Counts,
) -> ToolResult<()> {
    for (slug, title, purpose) in topics {
        let path = if base.is_empty() {
            format!("docs/{slug}.md")
        } else {
            format!("docs/{base}/{slug}.md")
        };
        write_missing(workspace, &path, &leaf_content(title, purpose), counts)?;
    }
    Ok(())
}

fn readme_content(title: &str, purpose: &str, entries: &str) -> String {
    format!("# {title}\n\n## Purpose\n\n{purpose}\n\n## Table of Contents\n\n{entries}\n")
}

fn leaf_content(title: &str, purpose: &str) -> String {
    format!("# {title}\n\n## Purpose\n\n{purpose}\n\n## Network Links\n\n{LINKS}\n\n## Growth Control\n\n- Expand one to three pages per pass.\n- Update concept-network and rebalance-plan after each pass.\n- Split or merge branches when one branch outgrows its siblings.\n\n## Status\n\ndraft.\n")
}

fn write_missing(
    workspace: &Path,
    relative: &str,
    content: &str,
    counts: &mut Counts,
) -> ToolResult<()> {
    let path = workspace.join(relative);
    if path.exists() {
        counts.skipped = counts.skipped.saturating_add(1);
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    counts.created = counts.created.saturating_add(1);
    Ok(())
}
