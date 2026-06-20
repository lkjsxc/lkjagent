mod support;

use lkjagent_tools::dispatch::dispatch;
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn memory_find_accepts_graph_dot_note() -> TestResult<()> {
    let found = save_and_find("graph.note", "graph note recovery detail")?;
    assert!(found.content.contains("query_normalized=graph note"));
    assert!(found.content.contains("graph note recovery"));
    Ok(())
}

#[test]
fn memory_find_accepts_parameter_hyphen_fault() -> TestResult<()> {
    let found = save_and_find("parameter-fault", "parameter fault recovery detail")?;
    assert!(found.content.contains("query_normalized=parameter fault"));
    assert!(found.content.contains("parameter fault recovery"));
    Ok(())
}

#[test]
fn memory_find_accepts_brackets_in_tags() -> TestResult<()> {
    let found = save_and_find("[graph][tags]", "graph tags recovery detail")?;
    assert!(found.content.contains("query_normalized=graph tags"));
    assert!(found.content.contains("graph tags recovery"));
    Ok(())
}

#[test]
fn memory_save_duplicate_returns_existing_id_or_skip_notice() -> TestResult<()> {
    let workspace = temp_workspace("memory-duplicate")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    let params = [
        ("kind", "lesson"),
        ("title", "Graph note recovery"),
        ("tags", "graph,recovery"),
        ("content", "Normalize safe graph note aliases."),
    ];

    let first = dispatch(
        &action("memory.save", &params),
        &runtime,
        &mut conn,
        &mut state,
    );
    state.reset_repeat_tracking();
    let second = dispatch(
        &action("memory.save", &params),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(first.content.contains("memory_id=1"));
    assert!(second.content.contains("memory_id=1"));
    assert!(second.content.contains("duplicate=skipped"));
    Ok(())
}

#[test]
fn memory_prune_deletes_exact_duplicate_rows() -> TestResult<()> {
    let workspace = temp_workspace("memory-prune")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    conn.execute(
        "INSERT INTO memory (kind, title, tags, content, tokens, created_at, updated_at)
         VALUES ('lesson', 'duplicate lesson', 'a,b', 'same content', 10, '2026', '2026')",
        [],
    )?;
    let first = conn.last_insert_rowid();
    conn.execute(
        "INSERT INTO memory (kind, title, tags, content, tokens, created_at, updated_at)
         VALUES ('lesson', 'duplicate lesson', 'b,a', 'same content', 10, '2026', '2026')",
        [],
    )?;
    let second = conn.last_insert_rowid();
    for id in [first, second] {
        conn.execute(
            "INSERT INTO memory_fts (rowid, title, tags, content) VALUES (?1, ?2, ?3, ?4)",
            (id, "duplicate lesson", "a,b", "same content"),
        )?;
    }

    let output = dispatch(
        &action("memory.prune", &[]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(output.content.contains("memory prune completed"));
    assert!(output.content.contains("deleted_rows=1"));
    Ok(())
}

#[test]
fn memory_find_rejects_empty_after_normalization() -> TestResult<()> {
    let workspace = temp_workspace("memory-empty-normalized")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();

    let output = dispatch(
        &action("memory.find", &[("query", "..."), ("limit", "5")]),
        &runtime,
        &mut conn,
        &mut state,
    );

    assert!(is_error(&output));
    assert!(output.content.contains("query has no searchable tokens"));
    Ok(())
}

fn save_and_find(
    query: &str,
    content: &str,
) -> TestResult<lkjagent_tools::dispatch::DispatchOutput> {
    let workspace = temp_workspace("memory-punctuation")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    dispatch(
        &action(
            "memory.save",
            &[
                ("kind", "lesson"),
                ("title", "Recovery detail"),
                ("tags", "graph,recovery"),
                ("content", content),
            ],
        ),
        &runtime,
        &mut conn,
        &mut state,
    );
    Ok(dispatch(
        &action("memory.find", &[("query", query), ("limit", "5")]),
        &runtime,
        &mut conn,
        &mut state,
    ))
}

fn is_error(output: &lkjagent_tools::dispatch::DispatchOutput) -> bool {
    matches!(
        &output.kind,
        OutputKind::Observation { status } if status == "error"
    )
}
