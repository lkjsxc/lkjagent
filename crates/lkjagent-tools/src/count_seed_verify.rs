use std::fs;
use std::path::Path;

use crate::count_seed_verify_text::{
    verify_acceptance_audit, verify_coverage_map, verify_design_file_sections,
    verify_main_file_sections, verify_part_ledger,
};
use crate::error::{ToolError, ToolResult};

pub(crate) struct ScaffoldCheck {
    pub(crate) files: usize,
    pub(crate) index_files: usize,
    pub(crate) docs_index: &'static str,
    pub(crate) coverage_map: &'static str,
    pub(crate) main_index: &'static str,
    pub(crate) acceptance_audit: &'static str,
    pub(crate) part_ledger: &'static str,
    pub(crate) design_sections: &'static str,
    pub(crate) main_sections: &'static str,
    pub(crate) first_main: &'static str,
    pub(crate) last_main: &'static str,
}

pub(crate) fn verify_scaffold(
    root: &Path,
    target: usize,
    docs: usize,
    main: usize,
    indexes: bool,
) -> ToolResult<ScaffoldCheck> {
    let files = count_files(root)?;
    if files != target {
        return Err(ToolError::invalid(format!(
            "counted document scaffold expected {target} files, got {files}"
        )));
    }
    let root_text = require_text(&root.join("README.md"), "root index")?;
    let acceptance_audit = verify_acceptance_audit(&root_text)?;
    let design_sections = verify_design_files(root, docs)?;
    let main_sections = verify_main_files(root, main)?;
    let first_main = status(main > 0);
    let last_main = status(main > 0);
    let docs_text = if indexes {
        Some(require_text(&root.join("docs/README.md"), "docs index")?)
    } else {
        None
    };
    let docs_index = status(docs_text.is_some());
    let coverage_map = verify_coverage_map(docs_text.as_deref(), main)?;
    let main_text = if indexes {
        Some(require_text(&root.join("main/README.md"), "main index")?)
    } else {
        None
    };
    let main_index = status(main_text.is_some());
    let part_ledger = verify_part_ledger(main_text.as_deref(), main)?;
    Ok(ScaffoldCheck {
        files,
        index_files: if indexes { 2 } else { 0 },
        docs_index,
        coverage_map,
        main_index,
        acceptance_audit,
        part_ledger,
        design_sections,
        main_sections,
        first_main,
        last_main,
    })
}

fn verify_design_files(root: &Path, docs: usize) -> ToolResult<&'static str> {
    if docs == 0 {
        return Ok("n/a");
    }
    for index in 1..=docs {
        let label = format!("design memo {index:03}");
        let text = require_text(&root.join(format!("docs/design-{index:03}.md")), &label)?;
        verify_design_file_sections(&text, &label)?;
    }
    Ok("ok")
}

fn verify_main_files(root: &Path, main: usize) -> ToolResult<&'static str> {
    if main == 0 {
        return Ok("n/a");
    }
    for index in 1..=main {
        let label = format!("main part {index:03}");
        let text = require_text(&root.join(format!("main/part-{index:03}.md")), &label)?;
        verify_main_file_sections(&text, &label)?;
    }
    Ok("ok")
}

fn require_file(path: &Path, label: &str) -> ToolResult<()> {
    if path.is_file() {
        Ok(())
    } else {
        Err(ToolError::invalid(format!(
            "counted document scaffold missing {label}"
        )))
    }
}

fn require_text(path: &Path, label: &str) -> ToolResult<String> {
    require_file(path, label)?;
    Ok(fs::read_to_string(path)?)
}

fn status(ok: bool) -> &'static str {
    if ok {
        "ok"
    } else {
        "n/a"
    }
}

fn count_files(path: &Path) -> ToolResult<usize> {
    let mut count = 0_usize;
    for entry in fs::read_dir(path)? {
        let child = entry?.path();
        if child.is_dir() {
            count = count.saturating_add(count_files(&child)?);
        } else {
            count = count.saturating_add(1);
        }
    }
    Ok(count)
}
