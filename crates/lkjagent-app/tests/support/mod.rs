#![allow(dead_code)]

pub mod automatic_checks_fixture;
pub mod report_long;

use std::path::{Path, PathBuf};

fn automatic_checks_root(name: &str) -> Result<PathBuf, std::io::Error> {
    let path = std::env::temp_dir().join(format!("lkjagent-auto-{name}-{}", std::process::id()));
    if path.exists() {
        std::fs::remove_dir_all(&path)?;
    }
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

pub fn isolate_workspace(data: &Path) -> Result<PathBuf, std::io::Error> {
    let workspace = workspace(data);
    if workspace.exists() {
        std::fs::remove_dir_all(&workspace)?;
    }
    std::fs::create_dir_all(&workspace)?;
    std::fs::create_dir_all(data)?;
    let alias = data.join("workspace");
    if alias.is_symlink() || alias.is_file() {
        std::fs::remove_file(&alias)?;
    } else if alias.is_dir() {
        std::fs::remove_dir_all(&alias)?;
    }
    std::os::unix::fs::symlink(&workspace, alias)?;
    persist_workspace_config(data, &workspace)?;
    Ok(workspace)
}

pub fn retain_workspace_config(data: &Path) -> Result<(), std::io::Error> {
    persist_workspace_config(data, &workspace(data))
}

fn persist_workspace_config(data: &Path, workspace: &Path) -> Result<(), std::io::Error> {
    let path = data.join("lkjagent.json");
    let mut config = if path.exists() {
        serde_json::from_slice::<serde_json::Map<String, serde_json::Value>>(&std::fs::read(&path)?)
            .map_err(std::io::Error::other)?
    } else {
        serde_json::Map::new()
    };
    config.insert(
        "workspace_root".to_string(),
        serde_json::Value::String(workspace.to_string_lossy().into_owned()),
    );
    std::fs::write(
        path,
        serde_json::to_vec(&config).map_err(std::io::Error::other)?,
    )?;
    Ok(())
}

pub fn workspace(data: &Path) -> PathBuf {
    data.with_extension("workspace")
}

pub fn action_chars(tool: &str, params: &[(char, &str)]) -> String {
    let pairs = params
        .iter()
        .map(|(kind, value)| (field_name(*kind), *value))
        .collect::<Vec<_>>();
    action_pairs(tool, &pairs)
}

pub fn shell_action(command: &str) -> String {
    action_pairs("shell.run", &[("command", command)])
}

pub fn memory_save(topic: &str, content: &str) -> String {
    action_pairs("memory.save", &[("topic", topic), ("content", content)])
}

pub fn action_pairs(tool: &str, params: &[(&str, &str)]) -> String {
    action_for(
        "__DECISION_ID__",
        "__CONTEXT_FRAME_FINGERPRINT__",
        tool,
        params,
    )
}

pub fn action_for(_decision: &str, _context: &str, tool: &str, params: &[(&str, &str)]) -> String {
    let mut out = format!("<tool_call><tool>{}</tool><input>", xml(tool));
    for (name, value) in params {
        out.push_str(&format!("<{}>{}</{}>", xml(name), xml(value), xml(name)));
    }
    out.push_str("</input></tool_call>");
    out
}

fn field_name(kind: char) -> &'static str {
    match kind {
        'p' => "path",
        'q' => "query",
        _ => "content",
    }
}

fn xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&apos;")
        .replace('"', "&quot;")
}
