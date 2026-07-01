mod support;

use std::fs;
use std::io::Error;

use lkjagent_store::artifact_graph::{
    replace_atoms, replace_edges, upsert_plan, AtomInput, EdgeInput, PlanInput,
};
use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn audit_assembles_scene_and_marks_projection_ready() -> TestResult<()> {
    let workspace = temp_workspace("artifact-assembly-readiness")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let root = "stories/assembly";
    let plan_id = seed_plan(&conn, root)?;
    seed_atoms(&conn, plan_id)?;
    write_scene(&runtime.workspace, root)?;

    let mut dispatch_state = state();
    let output = dispatch(
        &action(
            "artifact.audit",
            &[
                ("root", root),
                ("kind", "manuscript"),
                ("count", ""),
                ("mode", ""),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;

    let Some(readiness) = lkjagent_store::artifact_graph::readiness_for_plan(&conn, plan_id)?
    else {
        return Err(Box::new(Error::other("missing readiness")));
    };
    let assembly_runs: i64 = conn.query_row(
        "SELECT COUNT(*) FROM artifact_assembly_runs WHERE plan_id = ?1 AND status = 'assembled'",
        [plan_id],
        |row| row.get(0),
    )?;

    assert!(output.contains("artifact audit passed"), "{output}");
    assert!(runtime
        .workspace
        .join(root)
        .join("manuscript/chapter-01.md")
        .is_file());
    assert_eq!(assembly_runs, 1);
    assert_eq!(readiness.status, "ready");
    assert_eq!(readiness.atom_ready, 2);
    assert_eq!(readiness.atom_missing, 0);
    assert_eq!(readiness.assembly_pending, "false");
    Ok(())
}

fn seed_plan(conn: &rusqlite::Connection, root: &str) -> TestResult<i64> {
    let empty = Vec::<String>::new();
    Ok(upsert_plan(
        conn,
        &PlanInput {
            case_id: 1,
            artifact_id: "artifact-assembly",
            owner_objective: "Assemble one manuscript chapter",
            artifact_kind: "manuscript",
            root,
            profile: "manuscript",
            normalized_title: "Assembly",
            measurement_kind: "words",
            requested_total: 50,
            accepted_floor: 50,
            section_count: 1,
            language_hint: "en",
            forbidden_roots: &empty,
            status: "active",
        },
        "now",
    )?)
}

fn seed_atoms(conn: &rusqlite::Connection, plan_id: i64) -> TestResult<()> {
    let scene_sections = vec!["purpose".to_string(), "scene".to_string()];
    let chapter_sections = vec!["purpose".to_string(), "chapter".to_string()];
    let empty = Vec::<String>::new();
    let atoms = vec![
        AtomInput {
            plan_id,
            atom_id: "scene-1".to_string(),
            sequence: 1,
            role: "scene",
            path: "manuscript/scenes/chapter-01/scene-01.md",
            status: "planned",
            measurement_kind: "words",
            target_count: 30,
            count_floor: 25,
            measured_count: 0,
            byte_budget: 1800,
            required_sections: &scene_sections,
            weak_classes: &empty,
            assembly_target: "manuscript/chapter-01.md",
        },
        AtomInput {
            plan_id,
            atom_id: "chapter-1".to_string(),
            sequence: 2,
            role: "chapter",
            path: "manuscript/chapter-01.md",
            status: "planned",
            measurement_kind: "words",
            target_count: 30,
            count_floor: 25,
            measured_count: 0,
            byte_budget: 1800,
            required_sections: &chapter_sections,
            weak_classes: &empty,
            assembly_target: "",
        },
    ];
    replace_atoms(conn, plan_id, &atoms)?;
    replace_edges(
        conn,
        plan_id,
        &[EdgeInput {
            plan_id,
            from_atom_id: "scene-1".to_string(),
            to_atom_id: "chapter-1".to_string(),
            relation: "assembles",
        }],
    )?;
    Ok(())
}

fn write_scene(workspace: &std::path::Path, root: &str) -> TestResult<()> {
    let root_path = workspace.join(root);
    fs::create_dir_all(root_path.join("manuscript/scenes/chapter-01"))?;
    fs::write(
        root_path.join("objective.md"),
        "30 word manuscript, 1 chapter.",
    )?;
    fs::write(
        root_path.join("README.md"),
        "# Assembly\n\n## Purpose\n\nNavigate the manuscript.\n",
    )?;
    fs::write(root_path.join("catalog.toml"), "kind = \"manuscript\"\n")?;
    fs::write(
        root_path.join("manuscript/scenes/chapter-01/scene-01.md"),
        "# Scene One\n\n## Purpose\n\n## Scene\n\nMira walked into the archive and said the ledger was awake. Tomas looked at the silver pages and thought about the promise they had made before sunrise. The room answered with a low bell while rain pressed against the glass.",
    )?;
    Ok(())
}
