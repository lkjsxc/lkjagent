#[allow(dead_code)]
mod obligation_network_support;

use lkjagent_runtime::kernel::RuntimeEvent;
use obligation_network_support::*;

#[test]
fn store_projected_active_contract_routes_to_batch_write() -> Result<(), String> {
    let mut input = owner_input();
    input.artifact_root = Some("reports/market-map".to_string());
    input.artifact_kind = Some("report".to_string());
    input.artifact_plan_status = Some("contracted".to_string());
    input.artifact_atom_total = 6;
    input.artifact_atom_missing = 6;
    input.artifact_active_contract = Some("contract-1".to_string());
    input.artifact_readiness = Some("contracted".to_string());
    input.latest_observation = Some(contract_observation());

    let decision = decision(input, RuntimeEvent::OwnerMessageReceived)?;

    assert_eq!(next_tool(&decision)?, "fs.batch_write");
    let contract = decision
        .content_write_contract
        .as_ref()
        .ok_or("missing content contract")?;
    assert_eq!(contract.contract_id.as_deref(), Some("contract-1"));
    assert_eq!(contract.count_floor, 120);
    Ok(())
}

fn contract_observation() -> String {
    "artifact_next_result=write_contract_pending
root=reports/market-map
kind=report
artifact_profile=report
contract_id=contract-1
atom_ids=reports/market-map:analysis.md
count_floor=120
target_count=200
continuity_digest=root=reports/market-map atom=analysis
next_decision_required=true
candidate_action=fs.batch_write
candidate_contract:
tool=fs.batch_write
paths:
- reports/market-map/analysis.md"
        .to_string()
}
