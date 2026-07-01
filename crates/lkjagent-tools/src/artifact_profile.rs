use crate::artifact_objective::ObjectiveFrame;
use crate::artifact_profile_atoms::{
    atom, doc_atoms, report_atoms, static_profile, story_atoms, study_atoms,
};
use crate::artifact_profile_more::{cookbook_atoms, dictionary_atoms, generic_atoms};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileSpec {
    pub name: String,
    pub measurement_kind: String,
    pub weak_classes: Vec<String>,
    pub atoms: Vec<AtomTemplate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomTemplate {
    pub role: String,
    pub path: String,
    pub measurement_kind: String,
    pub target_count: usize,
    pub count_floor: usize,
    pub byte_budget: usize,
    pub required_sections: Vec<String>,
    pub assembly_target: Option<String>,
    pub depends_on: Vec<String>,
}

pub fn profile_for(frame: &ObjectiveFrame) -> ProfileSpec {
    match frame.artifact_kind.as_str() {
        "manuscript" => manuscript(frame),
        "story" => static_profile("story", story_atoms(), "words"),
        "report" => static_profile("report", report_atoms(frame), "words"),
        "documentation" => static_profile("documentation", doc_atoms(frame), "words"),
        "study-set" => static_profile("study-set", study_atoms(frame), "items"),
        "dictionary" => static_profile("dictionary", dictionary_atoms(frame), "items"),
        "cookbook" => static_profile("cookbook", cookbook_atoms(frame), "items"),
        _ => static_profile("generic", generic_atoms(frame), "words"),
    }
}

fn manuscript(frame: &ObjectiveFrame) -> ProfileSpec {
    let units = manuscript_units(frame);
    let scene_floor = frame.accepted_floor.saturating_div(units).clamp(120, 250);
    let mut atoms = story_atoms();
    for chapter in 1..=units {
        let scene_path = format!("manuscript/scenes/chapter-{chapter:02}/scene-01.md");
        let target = format!("manuscript/chapter-{chapter:02}.md");
        let mut scene = atom("scene", &scene_path, scene_floor, &["purpose", "scene"]);
        scene.assembly_target = Some(target.clone());
        atoms.push(scene);
        let mut chapter_atom = atom("chapter", &target, scene_floor, &["purpose", "chapter"]);
        chapter_atom.depends_on = vec![scene_path];
        atoms.push(chapter_atom);
    }
    static_profile("manuscript", atoms, "words")
}

fn manuscript_units(frame: &ObjectiveFrame) -> usize {
    if frame.section_count > 0 {
        frame.section_count
    } else {
        frame.accepted_floor.saturating_add(249) / 250
    }
    .clamp(5, 80)
}
