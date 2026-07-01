pub(super) fn contract_observation(
    plan: &lkjagent_store::artifact_graph::PlanRow,
    contract: &lkjagent_store::artifact_graph::ContractRow,
) -> String {
    format!(
        "artifact_next_result=write_contract_pending\nroot={}\nkind={}\nartifact_profile={}\ncontract_id={}\natom_ids={}\ncount_floor={}\ntarget_count={}\ncontinuity_digest={}\nnext_decision_required=true\ncandidate_action=fs.batch_write\ncandidate_contract:\ntool=fs.batch_write\npaths:\n{}\nmax_files={}\nmax_file_bytes={}\nmax_batch_bytes={}\nrequired_sections:\n{}\nforbidden_weak_phrase_classes:\n{}",
        plan.root,
        plan.artifact_kind,
        plan.profile,
        contract.contract_id,
        contract.atom_ids.replace('\n', ","),
        contract.count_floor,
        contract.target_count,
        contract.continuity_digest,
        bullets(&lkjagent_store::artifact_graph::split_lines(&contract.exact_paths)),
        contract.max_files,
        contract.max_file_bytes,
        contract.max_batch_bytes,
        bullets(&lkjagent_store::artifact_graph::split_lines(&contract.required_sections)),
        bullets(&lkjagent_store::artifact_graph::split_lines(&contract.forbidden_weak_classes)),
    )
}

pub(super) fn readiness_observation(row: &lkjagent_store::artifact_graph::ReadinessRow) -> String {
    format!(
        "artifact_progress=projected\nroot={}\nartifact_profile={}\nplan_status={}\natom_total={}\natom_ready={}\natom_missing={}\nnext_atom={}\nnext_path={}\nactive_contract={}\nmeasured_total={}\naccepted_floor={}\nassembly_pending={}\nreadiness={}\ncompletion_blockers={}\nnext_decision_required=true\ncandidate_action={}",
        row.root,
        row.profile,
        row.plan_status,
        row.atom_total,
        row.atom_ready,
        row.atom_missing,
        row.next_atom_id,
        row.next_path,
        row.active_contract_id,
        row.measured_total,
        row.accepted_floor,
        row.assembly_pending,
        row.status,
        row.completion_blockers.replace('\n', ";"),
        if row.status == "ready" { "agent.done" } else { "artifact.next" }
    )
}

pub(super) fn cursor_observation(
    cursor: &lkjagent_store::artifact_cursor::BatchCursorRow,
    kind: &str,
) -> Option<String> {
    let planned = split_paths(&cursor.planned_paths);
    if planned.is_empty() {
        return None;
    }
    let completed = split_paths(&cursor.completed_paths);
    let failed = split_paths(&cursor.failed_paths);
    let remaining = planned
        .iter()
        .filter(|path| !completed.contains(path) && !failed.contains(path))
        .cloned()
        .collect::<Vec<_>>();
    if remaining.is_empty() {
        return Some(format!(
            "artifact_next_result=ready_for_audit\nroot={}\nkind={kind}\nmissing=0\nnext_decision_required=true\ncandidate_action=artifact.audit",
            cursor.root
        ));
    }
    let contract = legacy_contract(&cursor.root, kind, &remaining[0]);
    Some(format!(
        "artifact_next_result=write_contract_pending\nroot={}\nkind={kind}\nmissing={}\nnext_decision_required=true\ncandidate_action=fs.batch_write\ncandidate_contract:\n{}",
        cursor.root,
        remaining.len(),
        contract
    ))
}

fn legacy_contract(root: &str, kind: &str, path: &str) -> String {
    if path.contains("/manuscript/") || path.starts_with("manuscript/") {
        return format!(
            "tool=fs.batch_write\nroot={root}\nkind=story\npaths:\n- {path}\nlimits:\n- max_files=1\n- max_file_bytes=1800\n- max_batch_bytes=1800\nrequired_sections:\n- finished {} prose\n- scene action and dialogue or interiority\n- continuity with prior facts\nforbidden_weak_phrase_classes:\n- scaffold-only\n- outline-only\n- story-bible-only\n- placeholder\n- owner-terms-only\n- generic-example\nmodel_instruction=author finished manuscript prose for only this path with the line protocol",
            manuscript_unit(path)
        );
    }
    format!(
        "tool=fs.batch_write\nroot={root}\nkind={kind}\npaths:\n- {path}\nlimits:\n- max_files=1\n- max_file_bytes=1800\n- max_batch_bytes=1800\nrequired_sections:\n- title\n- purpose\n- scene content or reference detail\n- continuity notes\n- verification notes\nforbidden_weak_phrase_classes:\n- scaffold-only\n- placeholder\n- owner-terms-only\n- generic-example\nmodel_instruction=author only this one listed path with 25 to 45 words and the line protocol"
    )
}

fn split_paths(value: &str) -> Vec<String> {
    value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

fn manuscript_unit(path: &str) -> &'static str {
    if path.contains("manuscript/scenes/") {
        "scene"
    } else {
        "chapter"
    }
}

fn bullets(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("- {value}"))
        .collect::<Vec<_>>()
        .join("\n")
}
