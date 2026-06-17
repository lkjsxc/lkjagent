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
fn maintenance_uses_normal_tool_authority_and_budget() -> TestResult<()> {
    let workspace = temp_workspace("authority")?;
    let runtime = tool_runtime(workspace.clone())?;
    let mut conn = store()?;
    let mut dispatch_state = dispatch_state();
    let mut state = start_cycle(5)?;

    state = run_action(
        state,
        action("fs.write", &[("path", "note.md"), ("content", "hi")]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )?;
    state = run_action(
        state,
        action(
            "queue.enqueue",
            &[("content", "follow up"), ("reason", "maintenance")],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )?;
    state = run_action(
        state,
        action(
            "queue.edit",
            &[
                ("id", "1"),
                ("content", "edited"),
                ("reason", "maintenance"),
            ],
        ),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )?;
    state = run_action(
        state,
        action("shell.run", &[("command", git_remote_command())]),
        &runtime,
        &mut conn,
        &mut dispatch_state,
    )?;

    assert_eq!(std::fs::read_to_string(workspace.join("note.md"))?, "hi");
    assert!(workspace.join("remote.git/refs/heads/main").exists());
    assert!(matches!(
        state.maintenance.map(|cycle| cycle.turns_remaining),
        Some(1)
    ));
    let events = read_events(&conn)?;
    assert!(events.iter().any(|event| {
        event.kind == "queue_mutation" && event.content.contains("operation=enqueue")
    }));
    assert!(events.iter().any(|event| {
        event.kind == "queue_mutation" && event.content.contains("operation=edit")
    }));
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

fn git_remote_command() -> &'static str {
    "git init --bare remote.git && git init work && git -C work config user.email test@example.com && git -C work config user.name Test && printf hi > work/README.md && git -C work add README.md && git -C work commit -m init && git -C work remote add origin ../remote.git && git -C work push origin HEAD:main"
}
