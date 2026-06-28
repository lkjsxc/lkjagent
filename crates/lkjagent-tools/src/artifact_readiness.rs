use std::path::Path;

use crate::error::ToolResult;

pub fn readiness_report(kind: &str, root: &str, full: &Path, report: &str) -> ToolResult<String> {
    let converted = report.replace("document audit", "artifact audit");
    if !converted.starts_with("artifact audit passed") {
        return Ok(converted);
    }
    match kind.trim().to_ascii_lowercase().as_str() {
        "cookbook" => Ok(content_bearing(converted)),
        "story" => crate::artifact_readiness_story::story_report(root, full, converted),
        _ => Ok(converted),
    }
}

pub(crate) fn content_bearing(report: String) -> String {
    report.replace(
        "next_action=record document-structure evidence",
        "readiness=content-bearing\nnext_action=record document-structure and artifact-readiness evidence",
    )
}
