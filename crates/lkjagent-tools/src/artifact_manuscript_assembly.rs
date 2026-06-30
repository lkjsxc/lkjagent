use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::artifact_story_text::prose_words;
use crate::error::ToolResult;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AssemblyReport {
    pub target: String,
    pub sources: Vec<String>,
    pub words: usize,
}

pub(crate) fn assemble_scene_atoms(root: &Path, kind: &str) -> ToolResult<Vec<AssemblyReport>> {
    if !story_kind(kind) {
        return Ok(Vec::new());
    }
    let scenes = scene_atoms(root)?;
    let mut reports = Vec::new();
    for (chapter, atoms) in scenes {
        let Some(report) = assemble_chapter(root, &chapter, atoms)? else {
            continue;
        };
        reports.push(report);
    }
    Ok(reports)
}

pub(crate) fn render_reports(reports: &[AssemblyReport]) -> String {
    if reports.is_empty() {
        return String::new();
    }
    reports
        .iter()
        .map(|report| {
            format!(
                "manuscript_assembly=assembled\nassembled_target={}\nassembled_word_count={}\nsource_atom_paths={}",
                report.target,
                report.words,
                report.sources.join(",")
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn assemble_chapter(
    root: &Path,
    chapter: &str,
    atoms: Vec<(String, PathBuf)>,
) -> ToolResult<Option<AssemblyReport>> {
    if atoms.is_empty() {
        return Ok(None);
    }
    for (_, path) in &atoms {
        if weak_scene(path)? {
            return Ok(None);
        }
    }
    let target = format!("manuscript/{chapter}.md");
    let target_path = root.join(&target);
    if target_path.is_file() && prose_words(&fs::read_to_string(&target_path)?) > 0 {
        return Ok(None);
    }
    let mut body = format!("# {}\n\n", chapter_title(chapter));
    let mut sources = Vec::new();
    for (relative, path) in atoms {
        let text = fs::read_to_string(&path)?;
        sources.push(relative);
        body.push_str(&trim_heading(&text));
        body.push_str("\n\n");
    }
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&target_path, &body)?;
    Ok(Some(AssemblyReport {
        target,
        sources,
        words: prose_words(&body),
    }))
}

fn scene_atoms(root: &Path) -> ToolResult<BTreeMap<String, Vec<(String, PathBuf)>>> {
    let base = root.join("manuscript/scenes");
    let mut chapters = BTreeMap::new();
    if !base.is_dir() {
        return Ok(chapters);
    }
    for chapter in fs::read_dir(base)? {
        let chapter = chapter?;
        if !chapter.path().is_dir() {
            continue;
        }
        let Some(name) = chapter.file_name().to_str().map(str::to_string) else {
            continue;
        };
        let mut atoms = markdown_files(root, &chapter.path())?;
        atoms.sort_by(|left, right| left.0.cmp(&right.0));
        chapters.insert(name, atoms);
    }
    Ok(chapters)
}

fn markdown_files(root: &Path, dir: &Path) -> ToolResult<Vec<(String, PathBuf)>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            files.extend(markdown_files(root, &path)?);
        } else if path.extension().is_some_and(|ext| ext == "md") && !is_readme(&path) {
            files.push((relative(root, &path), path));
        }
    }
    Ok(files)
}

fn is_readme(path: &Path) -> bool {
    path.file_name().and_then(|name| name.to_str()) == Some("README.md")
}

fn weak_scene(path: &Path) -> ToolResult<bool> {
    let text = fs::read_to_string(path)?;
    let lower = text.to_ascii_lowercase();
    Ok(prose_words(&text) < 25
        || !text.contains("##")
        || lower.contains("content_state=structure-only")
        || lower.contains("placeholder")
        || lower.contains("todo"))
}

fn trim_heading(text: &str) -> String {
    text.lines()
        .enumerate()
        .filter(|(index, line)| *index != 0 || !line.trim_start().starts_with("# "))
        .map(|(_, line)| line)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

fn chapter_title(chapter: &str) -> String {
    chapter
        .split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn relative(root: &Path, path: &Path) -> String {
    match path.strip_prefix(root) {
        Ok(relative) => relative,
        Err(_) => path,
    }
    .to_string_lossy()
    .to_string()
}

fn story_kind(kind: &str) -> bool {
    matches!(
        kind.to_ascii_lowercase().as_str(),
        "story" | "novel" | "manuscript"
    )
}
