mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn manuscript_readiness_rejects_story_bible_only() -> TestResult<()> {
    let workspace = temp_workspace("manuscript-readiness-story-bible")?;
    seed_story_bible(&workspace, false)?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    lkjagent_store::state::set(
        &conn,
        "artifact requested scale stories/chronos-fracture",
        "full-draft",
    )?;
    let mut dispatch_state = state();

    let output = dispatch(
        &action(
            "artifact.audit",
            &[("root", "stories/chronos-fracture"), ("kind", "story")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    assert!(
        output.contains("readiness=missing-manuscript-content"),
        "{output}"
    );
    assert!(output.contains("manuscript_word_count: 0"), "{output}");
    assert!(output.contains("next_manuscript_path"), "{output}");
    Ok(())
}

#[test]
fn manuscript_readiness_passes_with_chapter_prose_floor() -> TestResult<()> {
    let workspace = temp_workspace("manuscript-readiness-pass")?;
    seed_story_bible(&workspace, true)?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    lkjagent_store::state::set(
        &conn,
        "artifact requested scale stories/chronos-fracture",
        "full-draft",
    )?;
    let mut dispatch_state = state();

    let output = dispatch(
        &action(
            "artifact.audit",
            &[("root", "stories/chronos-fracture"), ("kind", "story")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    assert!(output.contains("artifact audit passed"), "{output}");
    assert!(output.contains("readiness=manuscript-content"), "{output}");
    assert!(!output.contains("missing-manuscript-content"), "{output}");
    Ok(())
}

fn seed_story_bible(workspace: &Path, manuscript: bool) -> TestResult<()> {
    let root = workspace.join("stories/chronos-fracture");
    fs::create_dir_all(&root)?;
    let mut names = role_files()
        .iter()
        .map(|(path, _, _)| *path)
        .collect::<Vec<_>>();
    if manuscript {
        fs::create_dir_all(root.join("manuscript"))?;
        fs::write(root.join("manuscript/chapter-01.md"), prose(9000))?;
        names.push("manuscript/chapter-01.md");
    }
    fs::write(root.join("README.md"), root_readme(&names))?;
    fs::write(root.join("catalog.toml"), "kind = \"story\"\n")?;
    fs::write(
        root.join("objective.md"),
        "# Objective\n\n## Purpose\n\nCreate a 10,000 word novel in one chapter with reference detail, continuity note, verification note, story bible context, student stakes, rainy hallway choices, romantic tension, family pressure, and scene consequences.\n",
    )?;
    for (path, title, signals) in role_files() {
        fs::write(root.join(path), role_text(title, signals))?;
    }
    Ok(())
}

fn root_readme(files: &[&str]) -> String {
    let links = files
        .iter()
        .map(|file| format!("- [{file}]({file})"))
        .collect::<Vec<_>>()
        .join("\n");
    format!("# Chronos Fracture\n\n## Purpose\n\nNavigate the story artifact.\n\n- [catalog](catalog.toml)\n{links}\n")
}

fn role_text(title: &str, signals: &str) -> String {
    format!(
        "# {title}\n\n## {title}\n\n{signals}. This role records concrete story facts, named constraints, causal consequences, cross references, verification notes, and continuity checks that guide later scenes without acting as a placeholder.\n"
    )
}

fn prose(words: usize) -> String {
    let body = (0..words)
        .map(|index| {
            if index % 17 == 0 {
                "dialogue"
            } else {
                "school"
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    format!("# Chapter One\n\n## Scene Content\n\nScene content with continuity note and verification note. {body}\n")
}

fn role_files() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("premise.md", "Premise", "premise stakes inciting story"),
        ("timeline.md", "Timeline", "timeline sequence past future"),
        (
            "cosmology.md",
            "Cosmology",
            "cosmology universe physics rule",
        ),
        (
            "technology-rules.md",
            "Technology Rules",
            "technology rule limit cost",
        ),
        ("locations.md", "Locations", "location place district route"),
        ("society.md", "Society", "society culture law class"),
        ("factions.md", "Factions", "faction agenda rival alliance"),
        (
            "protagonist.md",
            "Protagonist",
            "protagonist goal flaw choice",
        ),
        (
            "antagonist.md",
            "Antagonist",
            "antagonist pressure motive threat",
        ),
        (
            "supporting-cast.md",
            "Supporting Cast",
            "supporting ally mentor rival",
        ),
        (
            "relationships.md",
            "Relationships",
            "relationship trust conflict bond",
        ),
        ("logline.md", "Logline", "logline must before stakes"),
        ("themes.md", "Themes", "theme cost memory identity"),
        (
            "conflict-lattice.md",
            "Conflict",
            "conflict escalation pressure choice",
        ),
        (
            "act-structure.md",
            "Act Structure",
            "act turning point reversal climax",
        ),
        (
            "chapter-spine.md",
            "Chapter Spine",
            "chapter scene reveal consequence",
        ),
        ("rules.md", "Rules", "continuity rule contradiction check"),
        (
            "readiness.md",
            "Readiness",
            "completion evidence verified audit",
        ),
    ]
}
