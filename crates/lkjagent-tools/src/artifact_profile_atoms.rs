use crate::artifact_objective::ObjectiveFrame;
use crate::artifact_profile::{AtomTemplate, ProfileSpec};

pub(crate) fn static_profile(
    name: &str,
    atoms: Vec<AtomTemplate>,
    measurement: &str,
) -> ProfileSpec {
    ProfileSpec {
        name: name.to_string(),
        measurement_kind: measurement.to_string(),
        weak_classes: weak_classes(),
        atoms,
    }
}

pub(crate) fn atom(role: &str, path: &str, floor: usize, sections: &[&str]) -> AtomTemplate {
    AtomTemplate {
        role: role.to_string(),
        path: path.to_string(),
        measurement_kind: "words".to_string(),
        target_count: floor.saturating_mul(2),
        count_floor: floor,
        byte_budget: 1_800,
        required_sections: sections.iter().copied().map(str::to_string).collect(),
        assembly_target: None,
        depends_on: Vec::new(),
    }
}

pub(crate) fn story_atoms() -> Vec<AtomTemplate> {
    vec![
        atom("premise", "premise.md", 80, &["purpose", "premise"]),
        atom("setting", "setting.md", 100, &["purpose", "setting"]),
        atom("cast", "characters.md", 100, &["purpose", "characters"]),
        atom("plot", "plot.md", 120, &["purpose", "plot"]),
        atom(
            "continuity",
            "continuity.md",
            80,
            &["purpose", "continuity"],
        ),
        atom(
            "completion",
            "completion-evidence.md",
            60,
            &["purpose", "evidence"],
        ),
    ]
}

pub(crate) fn report_atoms(frame: &ObjectiveFrame) -> Vec<AtomTemplate> {
    let floor = per_atom_floor(frame, 6, 160);
    [
        ("summary", "executive-summary.md"),
        ("evidence", "evidence.md"),
        ("analysis", "analysis.md"),
        ("recommendations", "recommendations.md"),
        ("risks", "risks.md"),
        ("appendices", "appendices.md"),
    ]
    .into_iter()
    .map(|(role, path)| atom(role, path, floor, &["purpose", role]))
    .collect()
}

pub(crate) fn doc_atoms(frame: &ObjectiveFrame) -> Vec<AtomTemplate> {
    let floor = per_atom_floor(frame, 6, 120);
    [
        "overview",
        "usage",
        "architecture",
        "operations",
        "verification",
        "examples",
    ]
    .into_iter()
    .map(|role| atom(role, &format!("{role}.md"), floor, &["purpose", role]))
    .collect()
}

pub(crate) fn study_atoms(frame: &ObjectiveFrame) -> Vec<AtomTemplate> {
    let lessons = frame.section_count.clamp(3, 12);
    let mut atoms = vec![
        atom(
            "objectives",
            "objectives.md",
            60,
            &["purpose", "objectives"],
        ),
        item_atom("flashcards", "flashcards.md", 20),
        item_atom("drills", "drills.md", 10),
        item_atom("quizzes", "quizzes.md", 10),
        atom("review", "review-plan.md", 60, &["purpose", "review"]),
    ];
    for index in 1..=lessons {
        atoms.push(atom(
            "lesson",
            &format!("lessons/lesson-{index:02}.md"),
            100,
            &["purpose", "lesson"],
        ));
    }
    atoms
}

pub(crate) fn item_atom(role: &str, path: &str, floor: usize) -> AtomTemplate {
    let mut out = atom(role, path, floor, &["purpose", role]);
    out.measurement_kind = "items".to_string();
    out.target_count = floor;
    out.byte_budget = 1_800;
    out
}

fn per_atom_floor(frame: &ObjectiveFrame, count: usize, fallback: usize) -> usize {
    frame
        .accepted_floor
        .checked_div(count)
        .unwrap_or(fallback)
        .max(fallback)
}

fn weak_classes() -> Vec<String> {
    [
        "missing-file",
        "below-count-floor",
        "missing-required-section",
        "scaffold-only",
        "outline-only",
        "placeholder",
        "story-bible-only",
        "owner-terms-only",
        "generic-example",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}
