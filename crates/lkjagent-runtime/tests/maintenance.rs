mod support;

use lkjagent_protocol::render_action;
use lkjagent_runtime::maintenance::{
    choose_directive, idle_boundary, load_directive_stamps, prepare_idle_cycle, stamp_directive,
    BoundaryDecision, DirectiveStamp, MaintenanceDirective as Directive,
    DEFAULT_MAINTENANCE_TURN_BUDGET,
};
use lkjagent_runtime::step::{step, StepInput};
use lkjagent_runtime::task::{StopReason, TaskState};
use support::{action, ok_output, runtime_state, store, TestResult};

#[test]
fn maintenance_starts_scripted_cycle_for_each_directive() -> TestResult<()> {
    for directive in Directive::all() {
        let started = step(
            runtime_state()?,
            StepInput::StartMaintenance {
                directive: *directive,
                budget: DEFAULT_MAINTENANCE_TURN_BUDGET,
            },
        );
        assert_eq!(started.stop_reason, Some(StopReason::Maintenance));
        assert!(started.state.context.log.iter().any(|frame| {
            frame.content.contains(directive.as_str()) && frame.content.contains("turn_budget=8")
        }));
    }
    Ok(())
}

#[test]
fn directive_rotation_uses_stalest_state_stamp() -> TestResult<()> {
    let stamps = vec![
        stamp(Directive::Distill, Some("2026-01-03T00:00:00Z")),
        stamp(Directive::RefineSkills, Some("2026-01-01T00:00:00Z")),
        stamp(Directive::PruneMemory, None),
        stamp(Directive::AuditSelf, Some("2026-01-02T00:00:00Z")),
    ];
    assert_eq!(choose_directive(&stamps), Directive::PruneMemory);

    let conn = store()?;
    stamp_directive(&conn, Directive::Distill, "2026-01-03T00:00:00Z")?;
    stamp_directive(&conn, Directive::RefineSkills, "2026-01-01T00:00:00Z")?;
    stamp_directive(&conn, Directive::AuditSelf, "2026-01-02T00:00:00Z")?;
    let decision = prepare_idle_cycle(&conn, &runtime_state()?, "2026-01-04T00:00:00Z")?;
    assert!(matches!(
        decision,
        BoundaryDecision::StartCycle {
            directive: Directive::PruneMemory,
            ..
        }
    ));
    let loaded = load_directive_stamps(&conn)?;
    assert!(loaded.iter().any(|stamp| {
        stamp.directive == Directive::PruneMemory
            && stamp.last_run.as_deref() == Some("2026-01-04T00:00:00Z")
    }));
    Ok(())
}

#[test]
fn queue_preempts_at_boundary_not_mid_turn() -> TestResult<()> {
    let started = start_cycle(Directive::Distill, 3)?;
    let acting = step(
        started,
        StepInput::Completion {
            content: render_action(&action("fs.read", &[("path", "notes.md")])),
            tokens: 12,
        },
    );
    assert!(matches!(
        idle_boundary(&acting.state, 1, &[]),
        BoundaryDecision::ContinueCycle { .. }
    ));

    let observed = step(acting.state, StepInput::ToolOutput(ok_output("read ok")));
    assert!(matches!(
        idle_boundary(&observed.state, 1, &[]),
        BoundaryDecision::PreemptForQueue { pending: 1 }
    ));
    Ok(())
}

#[test]
fn empty_maintenance_cycle_closes_without_task_distillation() -> TestResult<()> {
    let started = start_cycle(Directive::AuditSelf, 2)?;
    let acting = step(
        started,
        StepInput::Completion {
            content: render_action(&action("agent.done", &[("summary", "nothing useful")])),
            tokens: 10,
        },
    );
    let done = step(acting.state, StepInput::ToolOutput(ok_output("done")));
    assert_eq!(done.stop_reason, Some(StopReason::Done));
    assert!(done.state.maintenance.is_none());
    assert!(matches!(done.state.task, TaskState::Idle));
    assert!(done
        .effects
        .iter()
        .all(|effect| !matches!(effect, lkjagent_runtime::step::Effect::DistillTask { .. })));
    Ok(())
}

#[test]
fn maintenance_budget_exhaustion_ends_cycle_before_action() -> TestResult<()> {
    let started = start_cycle(Directive::Distill, 1)?;
    let result = step(
        started,
        StepInput::Completion {
            content: render_action(&action("queue.list", &[("status", "all")])),
            tokens: 10,
        },
    );
    assert_eq!(result.stop_reason, Some(StopReason::BudgetNotice));
    assert!(result.state.maintenance.is_none());
    assert!(result
        .effects
        .iter()
        .all(|effect| !matches!(effect, lkjagent_runtime::step::Effect::ExecuteTool { .. })));
    Ok(())
}

fn start_cycle(
    directive: Directive,
    budget: u16,
) -> TestResult<lkjagent_runtime::task::RuntimeState> {
    Ok(step(
        runtime_state()?,
        StepInput::StartMaintenance { directive, budget },
    )
    .state)
}

fn stamp(directive: Directive, last_run: Option<&str>) -> DirectiveStamp {
    DirectiveStamp {
        directive,
        last_run: last_run.map(str::to_string),
    }
}
