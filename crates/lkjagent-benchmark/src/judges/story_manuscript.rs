use std::fs;
use std::path::Path;

pub fn story_manuscript(workspace: &Path) -> Result<(), String> {
    let transcript = read(workspace.join("transcript.md"))?;
    require_all(
        &transcript,
        &[
            "task=story-manuscript",
            "root=stories/the-bell-rings-twice",
            "manuscript_target_words=10000",
            "chapter_count=10",
            "next_manuscript_path=stories/the-bell-rings-twice/manuscript/chapter-01.md",
        ],
    )?;
    forbid_any(
        &transcript,
        &[
            "structured-output",
            "story_bible_only=true",
            "generic_root=stories/novel-named",
        ],
    )?;
    let chapter = workspace.join("stories/the-bell-rings-twice/manuscript/chapter-01.md");
    let chapter_text = read(chapter)?;
    if word_count(&chapter_text) < 80 {
        return Err("chapter prose too short".to_string());
    }
    if workspace.join("structured-output").exists() {
        return Err("structured-output scaffold exists".to_string());
    }
    Ok(())
}

fn read(path: impl AsRef<Path>) -> Result<String, String> {
    fs::read_to_string(path.as_ref())
        .map_err(|error| format!("{}: {error}", path.as_ref().display()))
}

fn require_all(text: &str, needles: &[&str]) -> Result<(), String> {
    for needle in needles {
        if !text.contains(needle) {
            return Err(format!("missing {needle}"));
        }
    }
    Ok(())
}

fn forbid_any(text: &str, needles: &[&str]) -> Result<(), String> {
    for needle in needles {
        if text.contains(needle) {
            return Err(format!("forbidden {needle}"));
        }
    }
    Ok(())
}

fn word_count(text: &str) -> usize {
    text.split_whitespace()
        .filter(|word| word.chars().any(|ch| ch.is_ascii_alphabetic()))
        .count()
}
