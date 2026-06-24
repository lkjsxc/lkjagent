use std::fs;
use std::path::Path;

use crate::error::ToolResult;

pub fn readiness_report(kind: &str, root: &str, full: &Path, report: &str) -> ToolResult<String> {
    let converted = report.replace("document audit", "artifact audit");
    if !converted.starts_with("artifact audit passed") {
        return Ok(converted);
    }
    match kind.trim().to_ascii_lowercase().as_str() {
        "cookbook" => Ok(content_bearing(converted)),
        "story" => story_report(root, full, converted),
        _ => Ok(converted),
    }
}

fn content_bearing(report: String) -> String {
    report.replace(
        "next_action=record document-structure evidence",
        "readiness=content-bearing\nnext_action=record document-structure and artifact-readiness evidence",
    )
}

fn story_report(root: &str, full: &Path, report: String) -> ToolResult<String> {
    let corpus = markdown_corpus(full)?.to_ascii_lowercase();
    let missing = story_missing(&corpus);
    if missing.is_empty() {
        return Ok(content_bearing(report).replace(
            "readiness=content-bearing",
            "readiness=story-semantic-content",
        ));
    }
    Ok(format!(
        "artifact audit failed\nroot={root}\nreadiness=missing-semantic-content\nfailed=1\nfailures:\n- story_semantic_missing: {}\nnext_decision_required=true\ncandidate_action=artifact.next",
        missing.join(",")
    ))
}

fn story_missing(corpus: &str) -> Vec<&'static str> {
    STORY_REQUIREMENTS
        .iter()
        .filter(|requirement| {
            !requirement
                .needles
                .iter()
                .any(|needle| corpus.contains(needle))
        })
        .map(|requirement| requirement.label)
        .collect()
}

fn markdown_corpus(root: &Path) -> ToolResult<String> {
    let mut text = String::new();
    collect_markdown(root, &mut text)?;
    Ok(text)
}

fn collect_markdown(path: &Path, text: &mut String) -> ToolResult<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            collect_markdown(&entry?.path(), text)?;
        }
    } else if path.extension().is_some_and(|ext| ext == "md") {
        text.push_str(&fs::read_to_string(path)?);
        text.push('\n');
    }
    Ok(())
}

struct Requirement {
    label: &'static str,
    needles: &'static [&'static str],
}

const STORY_REQUIREMENTS: &[Requirement] = &[
    req("premise", &["premise"]),
    req("timeline", &["timeline"]),
    req("cosmology", &["cosmology"]),
    req("technology-rules", &["technology rule", "technology rules"]),
    req("locations", &["location", "locations"]),
    req("society", &["society"]),
    req("factions", &["faction", "factions"]),
    req("protagonist", &["protagonist"]),
    req("antagonist", &["antagonist"]),
    req("supporting-cast", &["supporting cast"]),
    req("relationship-matrix", &["relationship matrix"]),
    req("logline", &["logline"]),
    req("themes", &["theme", "themes"]),
    req("conflict-lattice", &["conflict lattice"]),
    req("act-structure", &["act structure"]),
    req("chapter-spine", &["chapter spine"]),
    req("continuity-rules", &["continuity rule", "continuity rules"]),
    req("completion-evidence", &["completion evidence"]),
];

const fn req(label: &'static str, needles: &'static [&'static str]) -> Requirement {
    Requirement { label, needles }
}
