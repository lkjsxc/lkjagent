mod support;

use std::fs;

use lkjagent_context::budget::LOG_LOADED_SKILLS;
use lkjagent_protocol::{Action, Param};
use lkjagent_tools::control::{CompletionGuard, ControlState};
use lkjagent_tools::dispatch::{dispatch, dispatch_with_text};
use lkjagent_tools::observe::OutputKind;
use support::{action, runtime, state, store, temp_workspace, valid_skill, TestResult};

#[test]
fn skill_tools_use_and_refuse_documented_faults() -> TestResult<()> {
    let workspace = temp_workspace("skill")?;
    let runtime = runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut state = state();

    fs::write(
        runtime.skill_library.join("demo-skill.md"),
        valid_skill("Demo Skill"),
    )?;

    let used = dispatch(
        &action("skill.use", &[("name", "demo-skill")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(matches!(used.kind, OutputKind::Skill { .. }));
    assert!(used.rendered.contains("<skill>"));

    let again = dispatch_with_text(
        &action("skill.use", &[("name", "demo-skill")]),
        "load-demo-again",
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(matches!(again.kind, OutputKind::Notice { .. }));
    assert!(again.content.contains("skill already loaded"));

    let unknown = dispatch(
        &action("skill.use", &[("name", "missing-skill")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(is_error(&unknown));

    fs::write(
        runtime.skill_library.join("other-skill.md"),
        valid_skill("Other Skill"),
    )?;
    state.loaded_skill_tokens = LOG_LOADED_SKILLS;
    let over_budget = dispatch(
        &action("skill.use", &[("name", "other-skill")]),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(matches!(over_budget.kind, OutputKind::Notice { .. }));
    assert!(over_budget.content.contains("loaded skills exceed"));
    Ok(())
}

#[test]
fn control_tools_close_wait_and_report_errors() -> TestResult<()> {
    let workspace = temp_workspace("control")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut done_state = state();

    let done = dispatch(
        &action("agent.done", &[("summary", "finished")]),
        &runtime,
        &mut conn,
        &mut done_state,
    );
    assert!(done.content.contains("summary=finished"));
    let second_done = dispatch_with_text(
        &action("agent.done", &[("summary", "again")]),
        "done-again",
        &runtime,
        &mut conn,
        &mut done_state,
    );
    assert!(is_error(&second_done));

    let mut ask_state = state();
    let ask = dispatch(
        &action("agent.ask", &[("question", "Need input?")]),
        &runtime,
        &mut conn,
        &mut ask_state,
    );
    assert!(ask.content.contains("waiting"));
    let second_ask = dispatch(
        &action("agent.ask", &[("question", "Again?")]),
        &runtime,
        &mut conn,
        &mut ask_state,
    );
    assert!(is_error(&second_ask));
    assert!(second_ask.content.contains("already outstanding"));
    Ok(())
}

#[test]
fn control_classifies_recursive_knowledge_requests() {
    let mut state = ControlState::default();

    state.start_task("百科事典を作ってください");

    assert_eq!(state.guard, CompletionGuard::RecursiveKnowledge);
}

#[test]
fn dispatcher_reports_validation_and_repeat_notices() -> TestResult<()> {
    let workspace = temp_workspace("dispatch")?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;
    let mut state = state();
    let bad = Action::new(
        "fs.read",
        vec![Param::new("bogus", "x"), Param::new("bogus", "y")],
    );
    let validation = dispatch(&bad, &runtime, &mut conn, &mut state);
    assert!(matches!(validation.kind, OutputKind::Notice { .. }));
    assert!(validation.content.contains("duplicate params: bogus"));
    assert!(validation.content.contains("missing params: path"));
    assert!(validation.content.contains("unknown params: bogus"));

    let unknown = dispatch(
        &Action::new("think", Vec::new()),
        &runtime,
        &mut conn,
        &mut state,
    );
    assert!(matches!(unknown.kind, OutputKind::Notice { .. }));
    assert!(unknown.content.contains("valid tools"));

    let ask = action("agent.ask", &[("question", "Continue?")]);
    let first = dispatch_with_text(&ask, "same-act", &runtime, &mut conn, &mut state);
    let second = dispatch_with_text(&ask, "same-act", &runtime, &mut conn, &mut state);
    assert!(first.content.contains("waiting"));
    assert!(matches!(second.kind, OutputKind::Notice { .. }));
    assert!(second.content.contains("repeat action refused"));
    Ok(())
}

fn is_error(output: &lkjagent_tools::dispatch::DispatchOutput) -> bool {
    matches!(
        &output.kind,
        OutputKind::Observation { status } if status == "error"
    )
}
