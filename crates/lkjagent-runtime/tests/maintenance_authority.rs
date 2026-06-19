mod support;

use lkjagent_protocol::{render_action, Action};
use lkjagent_runtime::maintenance::MaintenanceDirective as Directive;
use lkjagent_runtime::step::{step, StepInput};
use lkjagent_runtime::task::StopReason;
use lkjagent_store::events::read_events;
use lkjagent_tools::dispatch::{dispatch, DispatchState, ToolRuntime};
use lkjagent_tools::observe::OutputKind;
use rusqlite::Connection;
use support::TestResult;
use support::{action, dispatch_state, runtime_state, store, temp_workspace, tool_runtime};

#[test]
fn maintenance_state_tools_keep_normal_authority_and_budget() -> TestResult<()> {
    let workspace = temp_workspace("authority")?;
    let runtime = tool_runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut dispatch_state = dispatch_state();
    let mut state = start_cycle(5)?;

    state = run_action(
        state,
        action("queue.list", &[("status", "pending"), ("limit", "5")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )?;
    state = run_action(
        state,
        action(
            "memory.save",
            &[
                ("kind", "lesson"),
                ("title", "maintenance note"),
                ("tags", "maintenance"),
                ("content", "write durable lessons only"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )?;
    state = run_action(
        state,
        action("memory.find", &[("query", "durable"), ("limit", "5")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )?;

    assert!(matches!(
        state.maintenance.map(|cycle| cycle.turns_remaining),
        Some(2)
    ));
    assert!(!workspace.join("remote.git").exists());
    assert!(read_events(&conn)?.is_empty());
    Ok(())
}

fn start_cycle(budget: u16) -> TestResult<lkjagent_runtime::task::RuntimeState> {
    Ok(step(
        runtime_state()?,
        StepInput::StartMaintenance {
            directive: Directive::AuditSelf,
            budget,
        },
    )
    .state)
}

fn run_action(
    state: lkjagent_runtime::task::RuntimeState,
    action: Action,
    runtime: &ToolRuntime,
    conn: &mut Connection,
    dispatch_state: &mut DispatchState,
) -> TestResult<lkjagent_runtime::task::RuntimeState> {
    let completion = step(
        state,
        StepInput::Completion {
            content: render_action(&action),
            tokens: 12,
        },
    );
    assert_eq!(completion.stop_reason, Some(StopReason::Acted));
    let output = dispatch(&action, runtime, conn, dispatch_state);
    assert!(matches!(
        output.kind,
        OutputKind::Observation { ref status } if status == "ok"
    ));
    Ok(step(completion.state, StepInput::ToolOutput(output)).state)
}
