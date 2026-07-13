use super::{shared, Context, Measured};
use crate::evaluation_harness::validate_cast;

pub fn measure(ctx: &Context<'_>) -> Result<Measured, String> {
    let cast = validate_cast(ctx.cast).ok();
    let facts = cast.as_ref();
    let messages = shared::count(ctx.db, "SELECT count(*) FROM conversation_messages")?;
    let identities = shared::count(
        ctx.db,
        "SELECT count(DISTINCT id) FROM conversation_messages",
    )?;
    let restart_resume = shared::restart_survivors(&ctx.capture.raw, ctx.db, "message")?;
    let fields = vec![
        (
            "fact_cast_sha256".into(),
            facts
                .map(|value| value.cast_fingerprint.clone())
                .unwrap_or_default(),
        ),
        (
            "fact_input_frame_count".into(),
            facts
                .map(|value| value.input_frames)
                .unwrap_or(0)
                .to_string(),
        ),
        (
            "fact_output_frame_count".into(),
            facts
                .map(|value| value.output_frames)
                .unwrap_or(0)
                .to_string(),
        ),
        (
            "fact_resize_count".into(),
            facts
                .map(|value| value.resize_frames)
                .unwrap_or(0)
                .to_string(),
        ),
        (
            "fact_japanese_input_count".into(),
            facts
                .map(|value| value.japanese_inputs)
                .unwrap_or(0)
                .to_string(),
        ),
        (
            "fact_search_input_count".into(),
            facts
                .map(|value| value.search_inputs)
                .unwrap_or(0)
                .to_string(),
        ),
        (
            "fact_alternate_screen_enter_count".into(),
            facts
                .map(|value| value.alt_screen_enter)
                .unwrap_or(0)
                .to_string(),
        ),
        (
            "fact_alternate_screen_exit_count".into(),
            facts
                .map(|value| value.alt_screen_exit)
                .unwrap_or(0)
                .to_string(),
        ),
        (
            "fact_slow_call_interval_ms".into(),
            facts
                .map(|value| value.slow_interval_ms)
                .unwrap_or(0)
                .to_string(),
        ),
        ("fact_message_identity_count".into(), identities.to_string()),
        (
            "fact_duplicate_message_identity_count".into(),
            messages.saturating_sub(identities).to_string(),
        ),
        (
            "fact_restart_resume_count".into(),
            restart_resume.to_string(),
        ),
        (
            "fact_activity_responsive_count".into(),
            facts
                .map(|value| value.activity_frames)
                .unwrap_or(0)
                .to_string(),
        ),
    ];
    let passed = facts.is_some_and(|value| {
        value.input_frames > 0
            && value.output_frames > 0
            && value.resize_frames > 0
            && value.japanese_inputs > 0
            && value.search_inputs > 0
            && value.slow_interval_ms >= 1_000
            && value.alt_screen_enter > 0
            && value.alt_screen_exit > 0
            && value.activity_frames > 0
    }) && messages >= 5
        && identities == messages;
    Ok(Measured::new(passed, fields))
}
