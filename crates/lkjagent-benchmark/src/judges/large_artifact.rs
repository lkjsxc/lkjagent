use std::fs;
use std::path::Path;

pub fn manuscript(workspace: &Path) -> Result<(), String> {
    let words = read_words(workspace, "stories/aurora/manuscript/chapter-01.md")?
        + read_words(workspace, "stories/aurora/manuscript/chapter-02.md")?;
    require(words >= 10_000, format!("manuscript words {words}/10000"))
}

pub fn report(workspace: &Path) -> Result<(), String> {
    for path in [
        "executive-summary.md",
        "evidence.md",
        "analysis.md",
        "recommendations.md",
        "risks.md",
    ] {
        require(
            read_words(workspace, &format!("reports/market/{path}"))? >= 80,
            path,
        )?;
    }
    Ok(())
}

pub fn study_set(workspace: &Path) -> Result<(), String> {
    for path in [
        "objectives.md",
        "lessons/lesson-01.md",
        "flashcards.md",
        "drills.md",
        "quizzes.md",
    ] {
        require(
            read_words(workspace, &format!("study/rust-cert/{path}"))? >= 20,
            path,
        )?;
    }
    Ok(())
}

pub fn documentation(workspace: &Path) -> Result<(), String> {
    for path in [
        "overview.md",
        "usage.md",
        "architecture.md",
        "operations.md",
        "verification.md",
    ] {
        require(
            read_words(workspace, &format!("docs/product-kit/{path}"))? >= 30,
            path,
        )?;
    }
    Ok(())
}

pub fn generic_root(workspace: &Path) -> Result<(), String> {
    let text = fs::read_to_string(workspace.join("transcript.md")).map_err(|e| e.to_string())?;
    require(
        text.contains("generic_root_refused=true"),
        "generic refusal missing",
    )?;
    require(
        !workspace.join("structured-output").exists(),
        "generic root exists",
    )
}

pub fn atom_retry(workspace: &Path) -> Result<(), String> {
    let weak = fs::read_to_string(workspace.join("events.log")).map_err(|e| e.to_string())?;
    require(weak.contains("atom_status=weak"), "weak atom event missing")?;
    require(
        weak.contains("atom_status=ready"),
        "ready retry event missing",
    )
}

pub fn assembly(workspace: &Path) -> Result<(), String> {
    let source = read_words(
        workspace,
        "stories/assembly/manuscript/scenes/chapter-01/scene-01.md",
    )?;
    let target = read_words(workspace, "stories/assembly/manuscript/chapter-01.md")?;
    require(source >= 200, "source scene too short")?;
    require(target >= source, "target did not assemble source")
}

fn read_words(workspace: &Path, path: &str) -> Result<usize, String> {
    let text =
        fs::read_to_string(workspace.join(path)).map_err(|error| format!("{path}: {error}"))?;
    Ok(text
        .split_whitespace()
        .filter(|word| word.chars().any(|ch| ch.is_alphabetic()))
        .count())
}

fn require(ok: bool, reason: impl Into<String>) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(reason.into())
    }
}
