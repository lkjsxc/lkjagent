use lkjagent_context::budget::ContextBudgetPolicy;
use rusqlite::Connection;

use super::ledger;
use super::text::{
    bullets, cell, context_line, line, section, state_u64, state_value, token_line,
    trim_to_char_budget, LOG_TAIL_RESERVE_CHARS, MAX_LOG_CHARS,
};
use crate::error::RuntimeResult;

type CaseRow = lkjagent_store::graph::GraphCaseRow;
type UsageRow = lkjagent_store::token_usage::TokenUsageEvent;

struct SnapshotContext<'a> {
    conn: &'a Connection,
    now: &'a str,
    queue_depth: usize,
    used: u64,
    budget: ContextBudgetPolicy,
    usage: Option<&'a UsageRow>,
    case: Option<&'a CaseRow>,
}

pub fn render(conn: &Connection, now: &str, budget: ContextBudgetPolicy) -> RuntimeResult<String> {
    let case = lkjagent_store::graph::active_case(conn)?;
    let events = lkjagent_store::events::read_events(conn)?;
    let usage = lkjagent_store::token_usage::latest(conn)?;
    let queue_depth = ledger::queue_depth(conn)?;
    let used = state_u64(conn, "context used tokens")?;
    let mut out = String::from("# lkjagent GPT-5.5-Pro Run Log\n\n");
    snapshot(
        &mut out,
        SnapshotContext {
            conn,
            now,
            queue_depth,
            used,
            budget,
            usage: usage.as_ref(),
            case: case.as_ref(),
        },
    )?;
    owner_objective(&mut out, case.as_ref());
    constraints(&mut out, case.as_ref());
    state_tracks(&mut out, conn, case.as_ref())?;
    plan(&mut out, case.as_ref());
    ledger::touched_paths(&mut out, conn, case.as_ref())?;
    ledger::evidence(&mut out, conn, case.as_ref())?;
    ledger::faults(&mut out, case.as_ref(), &events);
    let transcript_budget = MAX_LOG_CHARS
        .saturating_sub(out.chars().count())
        .saturating_sub(LOG_TAIL_RESERVE_CHARS);
    ledger::transcript(&mut out, &events, transcript_budget);
    ledger::verification(&mut out, case.as_ref());
    trim_to_char_budget(&mut out, MAX_LOG_CHARS);
    Ok(out)
}

fn snapshot(out: &mut String, ctx: SnapshotContext<'_>) -> RuntimeResult<()> {
    section(out, "Snapshot");
    line(out, "created_at", ctx.now);
    line(
        out,
        "daemon_state",
        &state_value(ctx.conn, "daemon state", "stopped")?,
    );
    line(out, "queue_depth", &ctx.queue_depth.to_string());
    line(
        out,
        "active_case",
        &ctx.case
            .map_or("none".to_string(), |case| case.id.to_string()),
    );
    line(
        out,
        "active_node",
        ctx.case.map_or("none", |case| &case.active_node),
    );
    line(
        out,
        "active_phase",
        ctx.case.map_or("none", |case| &case.phase),
    );
    line(out, "context", &context_line(ctx.used, ctx.budget));
    line(out, "token_usage", &token_line(ctx.usage));
    out.push('\n');
    Ok(())
}

fn owner_objective(out: &mut String, case: Option<&CaseRow>) {
    section(out, "Owner Objective");
    out.push_str("Raw:\n\n```text\n");
    out.push_str(case.map_or("none", |case| case.raw_owner_text.as_str()));
    out.push_str("\n```\n\nNormalized:\n\n```text\n");
    out.push_str(case.map_or("none", |case| case.objective.as_str()));
    out.push_str("\n```\n\n");
}

fn constraints(out: &mut String, case: Option<&CaseRow>) {
    section(out, "Constraints And Preferences");
    let Some(case) = case else {
        out.push_str("* none\n\n");
        return;
    };
    bullets(out, "evidence", &case.evidence_requirements);
    bullets(out, "checks", &case.pending_checks);
    bullets(out, "packages", &case.selected_packages);
    out.push('\n');
}

fn state_tracks(out: &mut String, conn: &Connection, case: Option<&CaseRow>) -> RuntimeResult<()> {
    section(out, "Active State Tracks");
    out.push_str("| rank | posture | label | intensity | confidence | phase | evidence gap |\n");
    out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
    let rows = match case {
        Some(case) => lkjagent_store::graph::state_tracks::state_tracks_for_case(conn, case.id)?,
        None => Vec::new(),
    };
    if rows.is_empty() {
        out.push_str("| 0 | none | none | 0 | 0 | none | none |\n\n");
        return Ok(());
    }
    for (index, row) in rows.iter().take(6).enumerate() {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} |\n",
            index + 1,
            cell(&row.posture),
            cell(&row.label),
            row.intensity,
            row.confidence,
            cell(&row.phase),
            cell(&row.evidence_gap.join("; "))
        ));
    }
    out.push('\n');
    Ok(())
}

fn plan(out: &mut String, case: Option<&CaseRow>) {
    section(out, "Plan");
    out.push_str("| step | status | target paths | evidence | checks |\n");
    out.push_str("| --- | --- | --- | --- | --- |\n");
    let Some(case) = case else {
        out.push_str("| none | none | none | none | none |\n\n");
        return;
    };
    out.push_str(&format!(
        "| active | {} | {} | pending | {} |\n\n",
        cell(&case.status),
        cell(&case.active_node),
        cell(&case.pending_checks.join("; "))
    ));
}
