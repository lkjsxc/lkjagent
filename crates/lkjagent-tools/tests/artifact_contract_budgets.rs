mod support;

use lkjagent_store::artifact_graph::{create_contract, upsert_plan, ContractInput, PlanInput};
use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn active_contract_refuses_file_count_above_limit() -> TestResult<()> {
    let (runtime, mut conn, mut dispatch_state) =
        seeded(&["reports/budget/a.md", "reports/budget/b.md"], 1, 100, 200)?;
    let output = dispatch(
        &action(
            "fs.batch_write",
            &[(
                "files",
                &batch(&[
                    ("reports/budget/a.md", "alpha"),
                    ("reports/budget/b.md", "bravo"),
                ]),
            )],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;
    assert!(output.contains("max_files"), "{output}");
    Ok(())
}

#[test]
fn active_contract_refuses_per_file_byte_overflow() -> TestResult<()> {
    let (runtime, mut conn, mut dispatch_state) = seeded(&["reports/budget/a.md"], 1, 10, 50)?;
    let output = dispatch(
        &action(
            "fs.batch_write",
            &[("files", &batch(&[("reports/budget/a.md", "abcdefghijk")]))],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;
    assert!(output.contains("max_file_bytes"), "{output}");
    Ok(())
}

#[test]
fn active_contract_refuses_batch_byte_overflow() -> TestResult<()> {
    let (runtime, mut conn, mut dispatch_state) =
        seeded(&["reports/budget/a.md", "reports/budget/b.md"], 2, 20, 15)?;
    let output = dispatch(
        &action(
            "fs.batch_write",
            &[(
                "files",
                &batch(&[
                    ("reports/budget/a.md", "abcdefghij"),
                    ("reports/budget/b.md", "klmnopqrst"),
                ]),
            )],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;
    assert!(output.contains("max_batch_bytes"), "{output}");
    Ok(())
}

#[test]
fn artifact_root_write_requires_active_contract() -> TestResult<()> {
    let workspace = temp_workspace("artifact-contract-no-active")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    seed_plan(&mut conn, "reports/budget")?;
    let mut dispatch_state = state();
    let output = dispatch(
        &action(
            "fs.batch_write",
            &[("files", &batch(&[("reports/budget/a.md", "alpha")]))],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )
    .content;
    assert!(output.contains("no active artifact contract"), "{output}");
    Ok(())
}

fn seeded(
    paths: &[&str],
    max_files: i64,
    max_file_bytes: i64,
    max_batch_bytes: i64,
) -> TestResult<(
    lkjagent_tools::dispatch::ToolRuntime,
    rusqlite::Connection,
    lkjagent_tools::dispatch::DispatchState,
)> {
    let workspace = temp_workspace("artifact-contract-budget")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let plan_id = seed_plan(&mut conn, "reports/budget")?;
    seed_contract(
        &mut conn,
        plan_id,
        paths,
        max_files,
        max_file_bytes,
        max_batch_bytes,
    )?;
    Ok((runtime, conn, state()))
}

fn seed_plan(conn: &mut rusqlite::Connection, root: &str) -> TestResult<i64> {
    let empty = Vec::<String>::new();
    Ok(upsert_plan(
        conn,
        &PlanInput {
            case_id: 1,
            artifact_id: "artifact-budget",
            owner_objective: "Budget report",
            artifact_kind: "report",
            root,
            profile: "report",
            normalized_title: "Budget Report",
            measurement_kind: "words",
            requested_total: 100,
            accepted_floor: 80,
            section_count: 1,
            language_hint: "en",
            forbidden_roots: &empty,
            status: "active",
        },
        "now",
    )?)
}

fn seed_contract(
    conn: &mut rusqlite::Connection,
    plan_id: i64,
    paths: &[&str],
    max_files: i64,
    max_file_bytes: i64,
    max_batch_bytes: i64,
) -> TestResult<()> {
    let atom_ids = vec!["atom-budget".to_string()];
    let exact = paths
        .iter()
        .map(|path| (*path).to_string())
        .collect::<Vec<_>>();
    let empty = Vec::<String>::new();
    create_contract(
        conn,
        &ContractInput {
            contract_id: "contract-budget",
            plan_id,
            atom_ids: &atom_ids,
            exact_paths: &exact,
            max_files,
            max_file_bytes,
            max_batch_bytes,
            target_count: 100,
            count_floor: 80,
            required_sections: &empty,
            continuity_digest: "root=reports/budget",
            forbidden_weak_classes: &empty,
            status: "active",
        },
        "now",
    )?;
    Ok(())
}

fn batch(files: &[(&str, &str)]) -> String {
    files
        .iter()
        .map(|(path, content)| format!("path: {path}\ncontent:\n{content}"))
        .collect::<Vec<_>>()
        .join("\n-- lkjagent-next-file --\n")
}
