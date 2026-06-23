mod support;

use lkjagent_protocol::parse_completion;
use lkjagent_runtime::mode::{
    admit_tool, decide, recovery_plan_for_fault, ActiveMode, FaultClass, RecoveryClass,
    RuntimeDecision, RuntimeEvent, RuntimeFault, RuntimeSnapshot,
};
use lkjagent_tools::dispatch::{dispatch, validate_action, EffectivePolicy};
use support::{dispatch_state, store, temp_workspace, tool_runtime, TestResult};

#[test]
fn turn_budget_exhaustion_selects_blocked_handoff_plan() {
    let snapshot = recovery_snapshot();

    let decision = decide(&snapshot, RuntimeEvent::TurnBudgetExhausted);

    assert!(matches!(decision, RuntimeDecision::StartRecovery(_)));
    let RuntimeDecision::StartRecovery(plan) = decision else {
        return;
    };
    assert_eq!(plan.fault_class, FaultClass::Budget);
    assert_eq!(plan.recovery_class, RecoveryClass::TurnBudgetExhaustion);
    assert!(plan.partial_handoff);
    assert_eq!(plan.forced_tool, "runtime.handoff");
}

#[test]
fn verification_recovery_forced_tool_is_admitted() {
    let snapshot = recovery_snapshot();
    let plan = recovery_plan_for_fault(&snapshot, RuntimeFault::VerificationMismatch);

    let admission = admit_tool(&snapshot, &plan.forced_tool);

    assert_eq!(plan.forced_tool, "verify.xtask");
    assert!(admission.admitted);
}

#[test]
fn recovery_plan_examples_validate_when_model_authored() {
    let snapshot = recovery_snapshot();
    for fault in recovery_faults() {
        let plan = recovery_plan_for_fault(&snapshot, fault);
        if plan.forced_tool.starts_with("runtime.") {
            continue;
        }
        let parsed = parse_completion(&plan.exact_valid_example);
        assert!(
            parsed.is_ok(),
            "{} example failed parse: {:?}",
            plan.forced_tool,
            parsed.err()
        );
        let Ok(action) = parse_completion(&plan.exact_valid_example) else {
            return;
        };
        let validated = validate_action(&action);
        assert!(
            validated.is_ok(),
            "{} example failed validation: {:?}",
            plan.forced_tool,
            validated.err()
        );
        assert!(
            admit_tool(&snapshot, &plan.forced_tool).admitted,
            "{} not admitted by recovery policy",
            plan.forced_tool
        );
    }
}

#[test]
fn maintenance_preemption_recovery_forced_tool_is_admitted() {
    let snapshot = recovery_snapshot();
    let plan = recovery_plan_for_fault(&snapshot, RuntimeFault::MaintenanceConflict);

    let admission = admit_tool(&snapshot, &plan.forced_tool);

    assert_eq!(plan.forced_tool, "queue.list");
    assert!(admission.admitted);
}

#[test]
fn recovery_plan_examples_dispatch_for_local_routes() -> TestResult<()> {
    let snapshot = recovery_snapshot();
    for fault in recovery_faults() {
        let plan = recovery_plan_for_fault(&snapshot, fault);
        if skip_dispatch_example(&plan.forced_tool) {
            continue;
        }
        let action = parse_completion(&plan.exact_valid_example)
            .map_err(|err| format!("parse failed for {}: {err:?}", plan.forced_tool))?;
        let workspace = temp_workspace("recovery-plan-dispatch")?;
        let runtime = tool_runtime(workspace)?;
        let mut conn = store()?;
        let mut dispatch_state = dispatch_state();
        dispatch_state.effective_policy = Some(recovery_effective_policy(&plan));
        let output = dispatch(&action, &runtime, &mut conn, &mut dispatch_state);
        assert!(
            !output.content.contains("params refused")
                && !output.content.contains("unknown tool after validation")
                && !output.content.contains("effective policy refused"),
            "{} example did not reach route: {}",
            plan.forced_tool,
            output.content
        );
    }
    Ok(())
}

fn recovery_faults() -> [RuntimeFault; 18] {
    [
        RuntimeFault::Parse,
        RuntimeFault::Parameter,
        RuntimeFault::Schema,
        RuntimeFault::ToolRuntime,
        RuntimeFault::Repeat,
        RuntimeFault::PolicyContradiction,
        RuntimeFault::PayloadTooLarge,
        RuntimeFault::ArtifactAuditFailure,
        RuntimeFault::WeakArtifactContent,
        RuntimeFault::FalseCompletion,
        RuntimeFault::VerificationMismatch,
        RuntimeFault::CompletionRefused,
        RuntimeFault::CompactionPressure,
        RuntimeFault::CompactionResumeGap,
        RuntimeFault::MaintenanceConflict,
        RuntimeFault::EndpointFault,
        RuntimeFault::TurnBudgetExhausted,
        RuntimeFault::ContextInvalid,
    ]
}

fn skip_dispatch_example(tool: &str) -> bool {
    matches!(tool, "verify.xtask" | "runtime.compact" | "runtime.handoff")
}

fn recovery_effective_policy(plan: &lkjagent_runtime::mode::RecoveryPlan) -> EffectivePolicy {
    let mut allowed = plan.allowed_observation_tools.clone();
    for tool in &plan.allowed_repair_tools {
        if !allowed.iter().any(|item| item == tool) {
            allowed.push(tool.clone());
        }
    }
    if !allowed.iter().any(|item| item == &plan.forced_tool) {
        allowed.push(plan.forced_tool.clone());
    }
    EffectivePolicy {
        mode: "Recovery".to_string(),
        allowed_tools: allowed,
        blocked_tools: Vec::new(),
        shell_allowed: false,
        completion_allowed: false,
        reason: "recovery route under test".to_string(),
        preferred_next_action: plan.forced_tool.clone(),
    }
}

fn recovery_snapshot() -> RuntimeSnapshot {
    RuntimeSnapshot {
        active_mission: ActiveMode::Recovery,
        owner_work_exists: true,
        recovery_ladder_active: true,
        context_pressure_active: false,
        maintenance_eligible: false,
        required_evidence: vec!["artifact-readiness".to_string()],
        missing_evidence: vec!["artifact-readiness".to_string()],
        active_artifact: Some("dictionary/bread-terms.txt".to_string()),
        last_tool_attempt: Some("fs.write".to_string()),
        repeated_action: false,
        external_owner_input_required: false,
    }
}
