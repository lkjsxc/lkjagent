use lkjagent_store::artifact_graph::{split_lines, ContractRow, PlanRow};

pub fn render_contract(plan: &PlanRow, contract: &ContractRow, missing: usize) -> String {
    let paths = split_lines(&contract.exact_paths);
    format!(
        "artifact_next_result=write_contract_pending\nroot={}\nkind={}\nartifact_profile={}\nplan_status={}\nmissing={missing}\nnext_atom={}\nnext_path={}\nactive_contract={}\nnext_decision_required=true\ncandidate_action=fs.batch_write\ncandidate_contract:\n{}\nline_protocol_example:\n{}",
        plan.root,
        plan.artifact_kind,
        plan.profile,
        plan.status,
        contract.atom_ids.replace('\n', ","),
        paths.first().map(String::as_str).unwrap_or("none"),
        contract.contract_id,
        contract_block(plan, contract),
        line_example(paths.first().map(String::as_str).unwrap_or("none")),
    )
}

pub fn render_plan(plan: &PlanRow, atom_count: usize, atoms: &[String]) -> String {
    format!(
        "artifact_plan=ready\nartifact_id={}\nroot={}\nkind={}\nartifact_profile={}\nplan_status={}\natom_total={}\naccepted_floor={}\nrequired_atoms={}\nnext_decision_required=true\ncandidate_action=artifact.next",
        plan.artifact_id,
        plan.root,
        plan.artifact_kind,
        plan.profile,
        plan.status,
        atom_count,
        plan.accepted_floor,
        atoms.join(","),
    )
}

pub fn contract_block(plan: &PlanRow, contract: &ContractRow) -> String {
    format!(
        "tool=fs.batch_write\ncontract_id={}\nroot={}\nkind={}\natom_ids={}\npaths:\n{}\nmax_files={}\nmax_file_bytes={}\nmax_batch_bytes={}\ntarget_count={}\ncount_floor={}\nrequired_sections:\n{}\ncontinuity_digest={}\nforbidden_weak_classes:\n{}\nmodel_instruction=author only the listed path with line protocol",
        contract.contract_id,
        plan.root,
        plan.artifact_kind,
        contract.atom_ids.replace('\n', ","),
        bullets(&split_lines(&contract.exact_paths)),
        contract.max_files,
        contract.max_file_bytes,
        contract.max_batch_bytes,
        contract.target_count,
        contract.count_floor,
        bullets(&split_lines(&contract.required_sections)),
        contract.continuity_digest,
        bullets(&split_lines(&contract.forbidden_weak_classes)),
    )
}

fn line_example(path: &str) -> String {
    format!("<action>\n<tool>fs.batch_write</tool>\n<files>\npath: {path}\ncontent:\n</files>\n</action>")
}

fn bullets(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("- {value}"))
        .collect::<Vec<_>>()
        .join("\n")
}
