use crate::control::CompletionGuard;
use crate::error::ToolError;

pub fn guard_write_path(guard: CompletionGuard, path: &str) -> Result<(), ToolError> {
    if !guard.is_knowledge() || guard.has_count() {
        return Ok(());
    }
    let parts = path_parts(path);
    if parts.first().is_none_or(|part| *part != "docs") {
        return Err(ToolError::invalid(
            "recursive knowledge writes must stay under docs/",
        ));
    }
    if parts.iter().skip(1).any(|part| *part == "docs") {
        return Err(ToolError::invalid(
            "recursive knowledge paths must not contain nested docs segments",
        ));
    }
    if let Some(top) = parts.get(1) {
        if !top_level_allowed(top) {
            return Err(ToolError::invalid(format!(
                "recursive knowledge writes must use the seeded top-level docs map, got docs/{top}"
            )));
        }
    }
    Ok(())
}

pub fn guard_shell_command(guard: CompletionGuard, command: &str) -> Result<(), ToolError> {
    if guard.is_knowledge() && !guard.has_count() && shell_writes(command) {
        return Err(ToolError::invalid(
            "recursive knowledge shell.run is read-only after the nucleus; use fs.write for one mapped page at a time",
        ));
    }
    Ok(())
}

fn path_parts(path: &str) -> Vec<&str> {
    path.trim_start_matches("./")
        .split('/')
        .filter(|part| !part.is_empty() && *part != ".")
        .collect()
}

fn top_level_allowed(top: &str) -> bool {
    matches!(
        top,
        "README.md"
            | "current-state.md"
            | "maps"
            | "domains"
            | "reference"
            | "curation"
            | "execution"
    )
}

fn shell_writes(command: &str) -> bool {
    let lower = command
        .to_ascii_lowercase()
        .replace(" >/dev/null", "")
        .replace(">/dev/null", "")
        .replace("2> /dev/null", "")
        .replace("2>/dev/null", "");
    lower.contains("mkdir")
        || lower.contains(" >")
        || lower.contains(">>")
        || lower.contains("tee ")
        || lower.contains("rm ")
        || lower.contains("mv ")
        || lower.contains("cp ")
}
