use lkjagent_protocol::{Action, Param};
use lkjagent_tools::dispatch::validate_action;

type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

#[test]
fn graph_plan_requires_checks_or_paths_before_dispatch() -> TestResult<()> {
    let action = Action::new(
        "graph.plan",
        vec![
            Param::new("objective", "Plan"),
            Param::new("steps", "Record"),
            Param::new("reason", "Owner"),
        ],
    );
    let error = match validate_action(&action) {
        Ok(_) => return Err("missing conditional should refuse".into()),
        Err(error) => error,
    };

    assert!(error.contains("tool=graph.plan"));
    assert!(error.contains("missing_any=checks|paths"));
    assert!(error.contains("<paths>.</paths>"));
    Ok(())
}

#[test]
fn graph_plan_with_paths_passes_registry_validation() -> TestResult<()> {
    let action = Action::new(
        "graph.plan",
        vec![
            Param::new("objective", "Plan"),
            Param::new("steps", "Record"),
            Param::new("paths", "stories/chronos-fracture"),
            Param::new("reason", "Owner"),
        ],
    );
    let validated = validate_action(&action).map_err(|error| format!("validation: {error}"))?;

    assert_eq!(validated.tool, "graph.plan");
    assert_eq!(
        validated.params.get("paths").map(String::as_str),
        Some("stories/chronos-fracture")
    );
    Ok(())
}
