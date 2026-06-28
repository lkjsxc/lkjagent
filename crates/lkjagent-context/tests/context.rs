use lkjagent_context::admission::{admit, AdmissionDecision, PendingFrame};
use lkjagent_context::assemble::{append_frame, assemble_messages, serialize_request};
use lkjagent_context::budget::{self, ContextBudgetPolicy, ContextPressure};
use lkjagent_context::compaction::{needs_compaction, rebuild_plan, CompactionDecision};
use lkjagent_context::model::{ContextState, Frame, FrameKind, NoticeKind, PrefixSection, Role};

const BUDGETS_DOC: &str = include_str!("../../../docs/architecture/context/budgets.md");

#[test]
fn budget_constants_match_the_doc_table() {
    let policy = ContextBudgetPolicy::default();
    for row in budget::budget_rows_for(policy) {
        let expected = format!("| {} | {}", row.region, comma_number(row.cap));
        assert!(
            BUDGETS_DOC.contains(&expected),
            "missing budget row: {}",
            row.region
        );
    }
    assert_eq!(budget::prefix_cap_total(), 5_376);
    assert_eq!(budget::initial_log_space(), 18_688);
    assert_eq!(policy.window, 24_576);
    assert_eq!(policy.soft_trigger, 18_432);
    assert_eq!(policy.hard_trigger, 21_504);
    assert_eq!(policy.post_compaction_target, 8_192);
    assert!(budget::config_preserves_log_floor(
        policy.window,
        budget::prefix_cap_total(),
        policy.reserve
    ));
}

fn comma_number(value: usize) -> String {
    let text = value.to_string();
    let mut output = String::new();
    for (index, character) in text.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            output.push(',');
        }
        output.push(character);
    }
    output.chars().rev().collect()
}

#[test]
fn sixteen_k_policy_derives_earlier_compaction() {
    let policy = match ContextBudgetPolicy::derive(16_384, 2_048, None) {
        Ok(policy) => policy,
        Err(error) => return assert_eq!(error.to_string(), "16k policy"),
    };
    assert_eq!(policy.available_log_space(), 8_960);
    assert_eq!(policy.soft_trigger, 12_288);
    assert_eq!(policy.hard_trigger, 13_312);
    assert_eq!(policy.post_compaction_target, 5_888);
    assert!(policy.hard_trigger < policy.window - policy.reserve);
    assert!(policy.post_compaction_target < policy.hard_trigger);
}

#[test]
fn invalid_or_stale_context_trigger_is_safe() {
    let error = match ContextBudgetPolicy::derive(16_383, 2_048, None) {
        Ok(_) => "unexpected success".to_string(),
        Err(error) => error.to_string(),
    };
    assert!(error.contains("at least 16384"));

    let policy = match ContextBudgetPolicy::derive(16_384, 2_048, Some(28_672)) {
        Ok(policy) => policy,
        Err(error) => return assert_eq!(error.to_string(), "derived trigger"),
    };
    assert_eq!(policy.hard_trigger, 13_312);
}

#[test]
fn pressure_model_predicts_before_hard_limit() {
    let policy = ContextBudgetPolicy::default();
    assert_eq!(policy.pressure(8_000, 1_000), ContextPressure::Green);
    assert_eq!(policy.pressure(17_800, 800), ContextPressure::Yellow);
    assert_eq!(policy.pressure(18_500, 1_000), ContextPressure::Orange);
    assert_eq!(policy.pressure(20_000, 2_000), ContextPressure::Red);
    assert_eq!(policy.pressure(24_100, 0), ContextPressure::BlackInvalid);
}

#[test]
fn admission_truncation_carries_retrieval_notice() {
    let frame = PendingFrame::new(FrameKind::Observation, "large output", 2_400)
        .with_retrieval_path("shell.run with a narrower filter");

    match admit(frame) {
        AdmissionDecision::Truncate { frame, notice } => {
            assert_eq!(frame.tokens.0, budget::LOG_OBSERVATION);
            assert_eq!(notice.kind, FrameKind::Notice(NoticeKind::Truncation));
            assert!(notice.content.contains("shell.run with a narrower filter"));
        }
        other => assert_eq!(format!("{other:?}"), "truncation decision"),
    }

    match admit(PendingFrame::new(
        FrameKind::Owner,
        "large owner message",
        4_500,
    )) {
        AdmissionDecision::Refuse { notice } => {
            assert!(notice.content.contains("retrieval path"));
        }
        other => assert_eq!(format!("{other:?}"), "refusal decision"),
    }
}

#[test]
fn serialized_requests_are_byte_monotonic_between_compactions() {
    let state = ContextState::new(
        vec![Frame::new(
            FrameKind::Prefix(PrefixSection::Identity),
            "identity",
            10,
        )],
        vec![Frame::new(FrameKind::Owner, "<owner>\nhello\n</owner>", 8)],
    );
    let before = serialize_request(&state);
    let next = append_frame(
        &state,
        Frame::new(FrameKind::Observation, "<observation>ok</observation>", 8),
    );
    let after = serialize_request(&next);

    assert!(after.starts_with(&before));
    assert!(after.len() > before.len());

    let messages = assemble_messages(&next);
    assert_eq!(
        messages.first().map(|message| message.role),
        Some(Role::System)
    );
}

#[test]
fn over_budget_state_rebuilds_under_target_with_summary_at_log_head() {
    let current = ContextState::new(
        vec![Frame::new(
            FrameKind::Prefix(PrefixSection::Identity),
            "old prefix",
            5_000,
        )],
        vec![Frame::new(FrameKind::Observation, "old log", 17_000)],
    );
    let policy = ContextBudgetPolicy::default();
    assert!(needs_compaction(&current, policy, 0));

    let prefix = vec![
        Frame::new(FrameKind::Prefix(PrefixSection::Identity), "identity", 768),
        Frame::new(
            FrameKind::Prefix(PrefixSection::GrammarRegistry),
            "grammar",
            1_024,
        ),
        Frame::new(FrameKind::Prefix(PrefixSection::GraphState), "graph", 512),
        Frame::new(
            FrameKind::Prefix(PrefixSection::WorkspaceBrief),
            "workspace",
            1_024,
        ),
        Frame::new(
            FrameKind::Prefix(PrefixSection::MemoryDigest),
            "task summary first",
            2_048,
        ),
    ];
    let summary = Frame::new(
        FrameKind::Notice(NoticeKind::Compaction),
        "task summary: continue context engine",
        256,
    );

    match rebuild_plan(&current, prefix, summary, policy) {
        CompactionDecision::Rebuild(plan) => {
            assert!(plan.after_tokens <= policy.post_compaction_target);
            assert!(plan.before_tokens >= policy.hard_trigger);
            assert!(plan
                .next
                .log
                .first()
                .is_some_and(|frame| frame.content.contains("task summary")));
        }
        other => assert_eq!(format!("{other:?}"), "rebuild decision"),
    }
}
