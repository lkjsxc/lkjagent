use std::fs;
use std::path::Path;

use crate::error::{ToolError, ToolResult};

struct Area {
    slug: &'static str,
    title: &'static str,
    purpose: &'static str,
}

const AREAS: &[Area] = &[
    Area {
        slug: "api",
        title: "API",
        purpose: "interface contracts and integration behavior",
    },
    Area {
        slug: "architecture",
        title: "Architecture",
        purpose: "system structure and design tradeoffs",
    },
    Area {
        slug: "operations",
        title: "Operations",
        purpose: "deployment, maintenance, and verification routines",
    },
    Area {
        slug: "security",
        title: "Security",
        purpose: "access control and data protection practices",
    },
    Area {
        slug: "data",
        title: "Data",
        purpose: "data modeling, quality, and lifecycle expectations",
    },
    Area {
        slug: "quality",
        title: "Quality",
        purpose: "test strategy and release confidence",
    },
    Area {
        slug: "product",
        title: "Product",
        purpose: "user workflows and product decisions",
    },
    Area {
        slug: "support",
        title: "Support",
        purpose: "incident response and customer-facing repair paths",
    },
    Area {
        slug: "platform",
        title: "Platform",
        purpose: "shared runtime and infrastructure concerns",
    },
    Area {
        slug: "reference",
        title: "Reference",
        purpose: "glossary and stable reference material",
    },
];

pub fn scaffold_markdown_corpus(workspace: &Path, target: usize) -> ToolResult<String> {
    let minimum = AREAS.len().saturating_add(1);
    if target < minimum {
        return Err(ToolError::invalid(format!(
            "benchmark corpus needs at least {minimum} markdown files"
        )));
    }
    let root = workspace.join("docs/benchmark-corpus");
    if root.exists() {
        fs::remove_dir_all(&root)?;
    }
    fs::create_dir_all(&root)?;
    let leaves = target.saturating_sub(minimum);
    let plan = leaf_plan(leaves);
    write_file(&root.join("README.md"), &root_readme(&plan))?;
    let mut ordinal = 1_usize;
    for (area, count) in AREAS.iter().zip(plan.iter().copied()) {
        let dir = root.join(area.slug);
        fs::create_dir_all(&dir)?;
        write_file(&dir.join("README.md"), &area_readme(area, count))?;
        for local in 1..=count {
            let name = format!("topic-{ordinal:03}.md");
            write_file(&dir.join(&name), &leaf(area, ordinal, local))?;
            ordinal = ordinal.saturating_add(1);
        }
    }
    verify(&root, target)?;
    Ok(format!(
        "benchmark corpus scaffold root=docs/benchmark-corpus\nmarkdown_files={target}\nnon_markdown_files=0\nverification=ok\nnext_action=agent.done"
    ))
}

fn leaf_plan(leaves: usize) -> Vec<usize> {
    let base = leaves / AREAS.len();
    let extra = leaves % AREAS.len();
    (0..AREAS.len())
        .map(|index| base + usize::from(index < extra))
        .collect()
}

fn root_readme(plan: &[usize]) -> String {
    let mut entries = String::new();
    for (area, count) in AREAS.iter().zip(plan.iter()) {
        entries.push_str(&format!(
            "- [{}/]({}/README.md): {} topic pages.\n",
            area.slug, area.slug, count
        ));
    }
    format!(
        "# Benchmark Corpus\n\n## Purpose\n\nThis corpus provides a deterministic set of documentation pages for Docker and local-agent smoke testing.\n\n## Table of Contents\n\n{}",
        entries
    )
}

fn area_readme(area: &Area, count: usize) -> String {
    let mut entries = String::new();
    for index in 1..=count {
        entries.push_str(&format!(
            "- [topic-{index:03}.md](topic-{index:03}.md): {} scenario {index}.\n",
            area.purpose
        ));
    }
    format!(
        "# {}\n\n## Purpose\n\nThis section documents {}.\n\n## Table of Contents\n\n{}",
        area.title, area.purpose, entries
    )
}

fn leaf(area: &Area, ordinal: usize, local: usize) -> String {
    format!(
        "# {} Topic {local}\n\n## Purpose\n\nThis page documents {} for benchmark scenario {ordinal}.\n\n## Guidance\n\nUse this page as stable English documentation content when validating file creation, indexing, and markdown traversal behavior.\n\n## Verification Notes\n\nA passing corpus keeps this page under its section README and the corpus root README.\n\n## Links\n\n- [Section index](README.md)\n- [Corpus index](../README.md)\n",
        area.title, area.purpose
    )
}

fn write_file(path: &Path, content: &str) -> ToolResult<()> {
    fs::write(path, content)?;
    Ok(())
}

fn verify(root: &Path, target: usize) -> ToolResult<()> {
    let mut markdown = 0_usize;
    let mut other = 0_usize;
    count_files(root, &mut markdown, &mut other)?;
    if markdown != target || other != 0 {
        return Err(ToolError::invalid(format!(
            "benchmark corpus verification failed: markdown_files={markdown} non_markdown_files={other}"
        )));
    }
    Ok(())
}

fn count_files(path: &Path, markdown: &mut usize, other: &mut usize) -> ToolResult<()> {
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        if child.is_dir() {
            count_files(&child, markdown, other)?;
        } else if child.extension().is_some_and(|extension| extension == "md") {
            *markdown = markdown.saturating_add(1);
        } else {
            *other = other.saturating_add(1);
        }
    }
    Ok(())
}
