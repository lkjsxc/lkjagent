use std::fs;
use std::path::Path;

use crate::error::ToolResult;
use crate::fs::workspace_path;

pub fn write(workspace: &Path, root: &str, label: &str, kind: &str) -> ToolResult<()> {
    let full = workspace_path(workspace, root)?;
    fs::create_dir_all(&full)?;
    fs::write(full.join("artifact-card.txt"), body(root, label, kind))?;
    link_from_readme(&full)?;
    Ok(())
}

fn link_from_readme(root: &Path) -> ToolResult<()> {
    let readme = root.join("README.md");
    if !readme.is_file() {
        return Ok(());
    }
    let mut text = fs::read_to_string(&readme)?;
    if !text.contains("artifact-card.txt") {
        text.push_str("\n- [artifact-card.txt](artifact-card.txt): artifact identity card.\n");
        fs::write(readme, text)?;
    }
    Ok(())
}

fn body(root: &str, label: &str, kind: &str) -> String {
    let semantic_id = semantic_id(root, kind);
    format!(
        "# Artifact Card\n\n<artifact-card>\n<root>{root}</root>\n<label>{}</label>\n<kind>{kind}</kind>\n<semantic-id>{semantic_id}</semantic-id>\n</artifact-card>\n",
        label.trim()
    )
}

fn semantic_id(root: &str, kind: &str) -> String {
    let leaf = root.rsplit('/').next().map_or("artifact", |value| value);
    format!("{}:{}", kind.trim(), leaf)
}
