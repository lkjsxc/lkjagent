use crate::artifact_objective::ObjectiveFrame;
use crate::artifact_profile::AtomTemplate;
use crate::artifact_profile_atoms::{atom, item_atom};

pub(crate) fn dictionary_atoms(frame: &ObjectiveFrame) -> Vec<AtomTemplate> {
    let entries = frame.section_count.clamp(5, 30);
    let mut atoms = vec![
        item_atom("index", "index.md", entries),
        atom(
            "cross-references",
            "cross-references.md",
            80,
            &["purpose", "references"],
        ),
        atom(
            "completion",
            "completion-evidence.md",
            60,
            &["purpose", "evidence"],
        ),
    ];
    for index in 1..=entries {
        atoms.push(item_atom(
            "entry",
            &format!("entries/entry-{index:02}.md"),
            1,
        ));
    }
    atoms
}

pub(crate) fn cookbook_atoms(frame: &ObjectiveFrame) -> Vec<AtomTemplate> {
    let recipes = frame.section_count.clamp(5, 20);
    let mut atoms = vec![
        atom(
            "ingredients",
            "ingredients.md",
            80,
            &["purpose", "ingredients"],
        ),
        atom(
            "techniques",
            "techniques.md",
            80,
            &["purpose", "techniques"],
        ),
        atom("index", "index.md", 40, &["purpose", "index"]),
    ];
    for index in 1..=recipes {
        atoms.push(item_atom(
            "recipe",
            &format!("recipes/recipe-{index:02}.md"),
            1,
        ));
    }
    atoms
}

pub(crate) fn generic_atoms(frame: &ObjectiveFrame) -> Vec<AtomTemplate> {
    let parts = frame.section_count.clamp(2, 8);
    let floor = per_atom_floor(frame, parts + 4, 120);
    let mut atoms = vec![
        atom("objective", "objective.md", 80, &["purpose", "objective"]),
        atom("structure", "structure.md", 80, &["purpose", "structure"]),
        atom("evidence", "evidence.md", 80, &["purpose", "evidence"]),
        atom(
            "completion",
            "completion-evidence.md",
            60,
            &["purpose", "evidence"],
        ),
    ];
    for index in 1..=parts {
        atoms.push(atom(
            "content",
            &format!("content/part-{index:02}.md"),
            floor,
            &["purpose", "content"],
        ));
    }
    atoms
}

fn per_atom_floor(frame: &ObjectiveFrame, count: usize, fallback: usize) -> usize {
    frame
        .accepted_floor
        .checked_div(count)
        .unwrap_or(fallback)
        .max(fallback)
}
