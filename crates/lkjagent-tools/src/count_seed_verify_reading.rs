use crate::error::{ToolError, ToolResult};

pub(crate) fn verify_reading_path(root_index: &str, main: usize) -> ToolResult<&'static str> {
    require_one(root_index, "## Reading Path", "## 読む順序", "reading path")?;
    if main == 0 {
        require_one(
            root_index,
            "No main files exist",
            "本編ファイルはありません",
            "reading path empty content",
        )?;
        return Ok("n/a");
    }
    require_one(
        root_index,
        "First main: main/part-001.md",
        "最初の本編: main/part-001.md",
        "reading path first main",
    )?;
    require_one(
        root_index,
        &format!("Last main: main/part-{main:03}.md"),
        &format!("最後の本編: main/part-{main:03}.md"),
        "reading path last main",
    )?;
    require_one(
        root_index,
        "numeric order",
        "番号順",
        "reading path order rule",
    )?;
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
