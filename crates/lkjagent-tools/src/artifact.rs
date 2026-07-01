use std::path::Path;

use crate::error::{ToolError, ToolResult};
use crate::fs::workspace_path;
use rusqlite::Connection;

pub fn plan(
    conn: &Connection,
    now: &str,
    root: &str,
    title: &str,
    kind: &str,
    scale: &str,
    sections: &str,
) -> ToolResult<String> {
    let frame = crate::artifact_objective::from_plan_inputs(root, title, kind, scale, sections);
    if let Some(conflict) = crate::artifact_objective::root_conflict(&frame) {
        return Err(ToolError::invalid(conflict));
    }
    let profile = crate::artifact_profile::profile_for(&frame);
    let case_id = crate::artifact_ledger_state::case_id(conn)?;
    let compiled = crate::artifact_plan_compile::compile(conn, &frame, &profile, case_id, now)?;
    let kind = frame.artifact_kind.as_str();
    let output = crate::doc::plan(
        &frame.root,
        kind,
        scale_count(scale),
        "approx",
        title,
        sections,
    )?;
    crate::artifact_ledger_support::record_plan(conn, &frame.root, kind, scale, now)?;
    let row = lkjagent_store::artifact_graph::plan_for_root(conn, &frame.root)?;
    let graph = row.map_or_else(String::new, |plan| {
        crate::artifact_contract_render::render_plan(
            &plan,
            compiled.atom_count,
            &compiled.required_atoms,
        )
    });
    Ok(append_if_present(output, &graph))
}

pub fn audit(
    workspace: &Path,
    conn: &Connection,
    now: &str,
    root: &str,
    kind: &str,
    count: &str,
    mode: &str,
) -> ToolResult<String> {
    if let Some(report) = crate::artifact_address_support::audit_refusal(workspace, root, kind)? {
        return crate::artifact_ledger_support::record_audit(
            workspace, conn, root, kind, &report, now,
        );
    }
    let kind = crate::artifact_kind::audit_kind(workspace, conn, root, kind)?;
    if kind.eq_ignore_ascii_case("dictionary") {
        let report = crate::dictionary_audit::audit(workspace, root)?;
        let report = if let Some(atom_report) =
            crate::artifact_atom_audit::audit_plan(workspace, conn, now, root)?
        {
            append_if_present(report, &atom_report)
        } else {
            report
        };
        let report = crate::artifact_ledger_support::record_audit(
            workspace, conn, root, &kind, &report, now,
        )?;
        return Ok(report);
    }
    let full = workspace_path(workspace, root)?;
    let assembly = crate::artifact_manuscript_assembly::assemble_scene_atoms(&full, &kind)?;
    let assembly_report = crate::artifact_manuscript_assembly::render_reports(&assembly);
    let report = append_if_present(
        crate::doc::audit(workspace, root, count, mode)?,
        &assembly_report,
    );
    let catalog = crate::artifact_kind::optional_catalog(&full);
    if !catalog.is_empty() && crate::artifact_kind::kind_mismatch(root, &kind, &catalog) {
        let report = format!(
            "document audit failed\nroot={root}\nchecks=15\npassed=14\nfailed=1\nfailures:\n- artifact_kind_mismatch: expected={kind}\nnext_action=artifact.next identity contract for matching artifact kind"
        );
        let report = crate::artifact_ledger_support::record_audit(
            workspace, conn, root, &kind, &report, now,
        )?;
        return Ok(report);
    }
    if let Some(atom_report) = crate::artifact_atom_audit::audit_plan(workspace, conn, now, root)? {
        let report = append_if_present(report, &atom_report);
        return crate::artifact_ledger_support::record_audit(
            workspace, conn, root, &kind, &report, now,
        );
    }
    if !report.starts_with("document audit passed") {
        let report = crate::artifact_ledger_support::record_audit(
            workspace, conn, root, &kind, &report, now,
        )?;
        return Ok(report);
    }
    if let Some(drift) = crate::artifact_drift::japanese_cookbook(&full)? {
        if !drift.is_empty() {
            let report = drift.observation(root);
            let report = crate::artifact_ledger_support::record_audit(
                workspace, conn, root, &kind, &report, now,
            )?;
            return Ok(report);
        }
    }
    let scale = crate::artifact_ledger_state::stored_scale(conn, root)?;
    let report = crate::artifact_readiness::readiness_report(&kind, root, &full, &report, &scale)?;
    crate::artifact_ledger_support::record_audit(workspace, conn, root, &kind, &report, now)
}

pub fn next(workspace: &Path, root: &str, path: &str, kind: &str) -> ToolResult<String> {
    crate::artifact_next::next(workspace, root, path, kind)
}

pub fn next_with_cursor(
    workspace: &Path,
    conn: &Connection,
    now: &str,
    root: &str,
    path: &str,
    kind: &str,
) -> ToolResult<String> {
    crate::artifact_next::next_with_cursor(workspace, conn, now, root, path, kind)
}

fn scale_count(scale: &str) -> &str {
    let trimmed = scale.trim();
    let valid_doc_count = trimmed
        .parse::<usize>()
        .ok()
        .is_some_and(|count| (3..=100).contains(&count));
    if valid_doc_count {
        trimmed
    } else {
        ""
    }
}

fn append_if_present(report: String, appendix: &str) -> String {
    if appendix.is_empty() {
        report
    } else {
        format!("{report}\n{appendix}")
    }
}

pub fn reject_empty_root(root: &str) -> ToolResult<()> {
    if root.trim().is_empty() {
        return Err(ToolError::invalid("artifact root must not be empty"));
    }
    Ok(())
}
