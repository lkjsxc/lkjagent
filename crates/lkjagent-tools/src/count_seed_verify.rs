use std::fs;
use std::path::Path;

use crate::error::{ToolError, ToolResult};

pub(crate) struct ScaffoldCheck {
    pub(crate) files: usize,
    pub(crate) index_files: usize,
    pub(crate) docs_index: &'static str,
    pub(crate) main_index: &'static str,
    pub(crate) part_ledger: &'static str,
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
    require_file(&root.join("README.md"), "root index")?;
    if docs > 0 {
        require_file(
            &root.join(format!("docs/design-{docs:03}.md")),
            "last design memo",
        )?;
    }
    let first_main = verify_main_file(root, main, 1, "first main part")?;
    let last_main = verify_main_file(root, main, main, "last main part")?;
    let docs_index = require_optional_index(root, indexes, "docs/README.md")?;
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
        main_index,
        part_ledger,
        first_main,
        last_main,
    })
}

fn verify_main_file(
    root: &Path,
    main: usize,
    index: usize,
    label: &str,
) -> ToolResult<&'static str> {
    if main == 0 {
        return Ok("n/a");
    }
    require_file(&root.join(format!("main/part-{index:03}.md")), label)?;
    Ok("ok")
}

fn require_optional_index(root: &Path, indexes: bool, relative: &str) -> ToolResult<&'static str> {
    if indexes {
        require_file(&root.join(relative), relative)?;
        Ok("ok")
    } else {
        Ok("n/a")
    }
}

fn verify_part_ledger(main_index: Option<&str>, main: usize) -> ToolResult<&'static str> {
    let Some(text) = main_index else {
        return Ok("n/a");
    };
    if main == 0 {
        return Ok("n/a");
    }
    require_contains(text, "part-001.md", "main index first part")?;
    require_contains(text, &format!("part-{main:03}.md"), "main index last part")?;
    if !text.contains("## Part Ledger") && !text.contains("## 本編台帳") {
        return Err(ToolError::invalid(
            "counted document scaffold missing part ledger",
        ));
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

fn require_contains(text: &str, needle: &str, label: &str) -> ToolResult<()> {
    if text.contains(needle) {
        Ok(())
    } else {
        Err(ToolError::invalid(format!(
            "counted document scaffold missing {label}"
        )))
    }
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
