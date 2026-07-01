mod support;

use std::fs;
use std::path::Path;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn manuscript_audit_assembles_scene_atoms_to_exact_chapter() -> TestResult<()> {
    let workspace = temp_workspace("manuscript-assembly")?;
    seed_scene_atoms(&workspace)?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();

    let output = dispatch(
        &action(
            "artifact.audit",
            &[("root", "stories/assembled"), ("kind", "story")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;
    let chapter = runtime
        .workspace
        .join("stories/assembled/manuscript/chapter-01.md");
    let text = fs::read_to_string(chapter)?;

    assert!(output.contains("artifact audit passed"), "{output}");
    assert!(output.contains("readiness=manuscript-content"), "{output}");
    assert!(output.contains("manuscript_assembly=assembled"), "{output}");
    assert!(
        output.contains("assembled_target=manuscript/chapter-01.md"),
        "{output}"
    );
    assert!(text.contains("rainy hallway"), "{text}");
    assert!(text.contains("library window"), "{text}");
    Ok(())
}

#[test]
fn manuscript_audit_waits_for_scene_word_floor() -> TestResult<()> {
    let workspace = temp_workspace("manuscript-assembly-floor")?;
    seed_short_scene_atom(&workspace)?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut dispatch_state = state();

    let output = dispatch(
        &action(
            "artifact.audit",
            &[("root", "stories/assembled"), ("kind", "story")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;
    let chapter = runtime
        .workspace
        .join("stories/assembled/manuscript/chapter-01.md");

    assert!(output.contains("missing-manuscript-content"), "{output}");
    assert!(!chapter.exists());
    Ok(())
}

fn seed_scene_atoms(workspace: &Path) -> TestResult<()> {
    let root = workspace.join("stories/assembled");
    let scenes = root.join("manuscript/scenes/chapter-01");
    fs::create_dir_all(&scenes)?;
    fs::write(root.join("catalog.toml"), "kind = \"story\"\n")?;
    fs::write(root.join("README.md"), readme())?;
    fs::create_dir_all(root.join("manuscript/scenes"))?;
    fs::write(root.join("manuscript/scenes/README.md"), scenes_readme())?;
    fs::write(
        root.join("manuscript/scenes/notes.md"),
        scene("story bible"),
    )?;
    fs::write(scenes.join("README.md"), chapter_readme())?;
    fs::write(root.join("objective.md"), objective())?;
    fs::write(scenes.join("scene-01.md"), scene("rainy hallway"))?;
    fs::write(scenes.join("scene-02.md"), scene("library window"))?;
    Ok(())
}

fn readme() -> &'static str {
    "# Assembled\n\n## Purpose\n\nNavigate the assembled manuscript.\n\n- [objective](objective.md)\n- [scene 1](manuscript/scenes/chapter-01/scene-01.md)\n- [scene 2](manuscript/scenes/chapter-01/scene-02.md)\n"
}

fn scenes_readme() -> &'static str {
    "# Scenes\n\n## Purpose\n\nNavigate scene atoms.\n\n- [Chapter](chapter-01/)\n- [Notes](notes.md)\n"
}

fn chapter_readme() -> &'static str {
    "# Chapter Scenes\n\n## Purpose\n\nNavigate chapter scene atoms.\n\n- [Scene 1](scene-01.md)\n- [Scene 2](scene-02.md)\n"
}

fn objective() -> &'static str {
    "# Objective\n\n## Purpose\n\nCreate a 100 word manuscript chapter at stories/assembled/manuscript/chapter-01.md with scene content, continuity note, verification note, story bible context, rainy hallway choices, library window tension, student stakes, family pressure, and chapter consequences.\n"
}

fn seed_short_scene_atom(workspace: &Path) -> TestResult<()> {
    let root = workspace.join("stories/assembled");
    let scenes = root.join("manuscript/scenes/chapter-01");
    fs::create_dir_all(&scenes)?;
    fs::write(root.join("catalog.toml"), "kind = \"story\"\n")?;
    fs::write(root.join("README.md"), readme())?;
    fs::create_dir_all(root.join("manuscript/scenes"))?;
    fs::write(root.join("manuscript/scenes/README.md"), scenes_readme())?;
    fs::write(
        root.join("manuscript/scenes/notes.md"),
        scene("story bible"),
    )?;
    fs::write(scenes.join("README.md"), chapter_readme())?;
    fs::write(root.join("objective.md"), long_objective())?;
    fs::write(scenes.join("scene-01.md"), scene("rainy hallway"))?;
    fs::write(scenes.join("scene-02.md"), scene("library window"))?;
    Ok(())
}

fn long_objective() -> &'static str {
    "# Objective\n\n## Purpose\n\nCreate a 900 word manuscript chapter at stories/assembled/manuscript/chapter-01.md with scene content, reference detail, rainy hallway choices, student stakes, romantic tension, family pressure, and chapter consequences.\n"
}

fn scene(anchor: &str) -> String {
    let body = (0..60)
        .map(|index| if index % 9 == 0 { anchor } else { "school" })
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "# Scene\n\n## Scene Content\n\nScene content with continuity note and verification note. {body}\n"
    )
}
