use crate::error::{ToolError, ToolResult};

pub(crate) fn verify_restart_guide(
    root_index: &str,
    index_files: usize,
    main: usize,
) -> ToolResult<&'static str> {
    require_one(
        root_index,
        "## Restart Guide",
        "## 再開ガイド",
        "restart guide",
    )?;
    require_one(root_index, "README.md", "README.md", "restart guide root")?;
    require_one(
        root_index,
        "recorded scale target",
        "記録済みの規模目安",
        "restart guide count rule",
    )?;
    if index_files > 0 && main > 0 {
        require_one(
            root_index,
            "docs/README.md",
            "docs/README.md",
            "restart guide docs index",
        )?;
        require_one(
            root_index,
            "main/README.md",
            "main/README.md",
            "restart guide main index",
        )?;
        require_one(
            root_index,
            "Design owner",
            "設計担当",
            "restart guide design owner",
        )?;
        require_one(
            root_index,
            "Sequence Ledger",
            "連続性台帳",
            "restart guide sequence ledger",
        )?;
    } else {
        require_one(
            root_index,
            "No main files exist",
            "本編ファイルはありません",
            "restart guide empty content",
        )?;
    }
    Ok("ok")
}

fn require_one(text: &str, english: &str, japanese: &str, label: &str) -> ToolResult<()> {
    if text.contains(english) || text.contains(japanese) {
        Ok(())
    } else {
        Err(ToolError::invalid(format!(
            "counted document scaffold missing {label}"
        )))
    }
}
