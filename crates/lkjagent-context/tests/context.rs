use lkjagent_context::admission::{admit, AdmissionDecision, PendingFrame};
use lkjagent_context::assemble::{append_frame, assemble_messages, serialize_request};
use lkjagent_context::budget::{self, BUDGET_ROWS};
use lkjagent_context::compaction::{needs_compaction, rebuild_plan, CompactionDecision};
use lkjagent_context::model::{ContextState, Frame, FrameKind, NoticeKind, PrefixSection, Role};

const BUDGETS_DOC: &str = include_str!("../../../docs/architecture/context/budgets.md");

#[test]
fn budget_constants_match_the_doc_table() {
    for row in BUDGET_ROWS {
        let expected = format!("| {} | {}", row.region, comma_number(row.cap));
        assert!(
            BUDGETS_DOC.contains(&expected),
            "missing budget row: {}",
            row.region
        );
    }
    assert_eq!(budget::prefix_cap_total(), 5_376);
    assert_eq!(budget::initial_log_space(), 26_368);
    assert!(budget::config_preserves_log_floor(
        budget::WINDOW_TOKENS,
        budget::prefix_cap_total(),
        budget::GENERATION_RESERVE
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
        vec![Frame::new(FrameKind::Observation, "old log", 24_000)],
    );
    assert!(needs_compaction(&current));

    let prefix = vec![
        Frame::new(FrameKind::Prefix(PrefixSection::Identity), "identity", 768),
        Frame::new(
            FrameKind::Prefix(PrefixSection::GrammarRegistry),
            "grammar",
            1_024,
        ),
        Frame::new(FrameKind::Prefix(PrefixSection::SkillIndex), "skills", 512),
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

    match rebuild_plan(&current, prefix, summary) {
        CompactionDecision::Rebuild(plan) => {
            assert!(plan.after_tokens <= budget::POST_COMPACTION_TARGET);
            assert!(plan.before_tokens >= budget::WHOLE_WINDOW_TRIGGER);
            assert!(plan
                .next
                .log
                .first()
                .is_some_and(|frame| frame.content.contains("task summary")));
        }
        other => assert_eq!(format!("{other:?}"), "rebuild decision"),
    }
}
