use std::fs;
use std::path::Path;

use crate::error::ToolResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScaffoldProfile {
    Generic,
    Knowledge,
}

pub struct ReadmeSeed {
    pub path: &'static str,
    pub title: &'static str,
    pub purpose: &'static str,
    pub entries: &'static str,
}

pub struct LeafSeed {
    pub path: &'static str,
    pub title: &'static str,
    pub purpose: &'static str,
    pub links: &'static str,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Counts {
    pub created: usize,
    pub skipped: usize,
}

pub const fn readme(
    path: &'static str,
    title: &'static str,
    purpose: &'static str,
    entries: &'static str,
) -> ReadmeSeed {
    ReadmeSeed {
        path,
        title,
        purpose,
        entries,
    }
}

pub const fn leaf(
    path: &'static str,
    title: &'static str,
    purpose: &'static str,
    links: &'static str,
) -> LeafSeed {
    LeafSeed {
        path,
        title,
        purpose,
        links,
    }
}

pub fn write_readmes(
    workspace: &Path,
    seeds: &[ReadmeSeed],
    counts: &mut Counts,
) -> ToolResult<()> {
    for seed in seeds {
        write_missing(workspace, seed.path, &readme_content(seed), counts)?;
    }
    Ok(())
}

pub fn write_leaves(workspace: &Path, seeds: &[LeafSeed], counts: &mut Counts) -> ToolResult<()> {
    for seed in seeds {
        write_missing(workspace, seed.path, &leaf_content(seed), counts)?;
    }
    Ok(())
}

pub fn write_doc_graph(workspace: &Path, profile: &str, counts: &mut Counts) -> ToolResult<()> {
    let content = format!(
        "# Document Graph\n\n## Purpose\n\nCompact graph ledger for the `{profile}` structure seed.\n\n## Nodes\n\n| id | path | role | status |\n| --- | --- | --- | --- |\n| root | README.md | root index | scaffolded |\n| architecture | architecture/README.md | architecture index | scaffolded |\n| guides | guides/README.md | guides index | scaffolded |\n| operations | operations/README.md | operations index | scaffolded |\n\n## Edges\n\n| from | to | kind | reason |\n| --- | --- | --- | --- |\n| root | architecture | indexes | local table of contents |\n| root | guides | indexes | local table of contents |\n| root | operations | indexes | verification path |\n\n## Coverage\n\n| owner requirement | covered by | status |\n| --- | --- | --- |\n| recursive structure | README.md and child README indexes | satisfied |\n| no part files | all generated paths | satisfied |\n| graph manifest | .lkj-doc-graph.md | satisfied |\n"
    );
    write_missing(workspace, "docs/.lkj-doc-graph.md", &content, counts)
}

fn readme_content(seed: &ReadmeSeed) -> String {
    format!(
        "# {}\n\n## Purpose\n\n{}\n\n## Table of Contents\n\n{}\n",
        seed.title, seed.purpose, seed.entries
    )
}

fn leaf_content(seed: &LeafSeed) -> String {
    format!(
        "# {}\n\n## Purpose\n\n{}\n\n## Network Links\n\n{}\n\n## Status\n\ndraft.\n",
        seed.title, seed.purpose, seed.links
    )
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
