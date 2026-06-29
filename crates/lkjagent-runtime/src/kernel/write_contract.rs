use crate::kernel::decision::ContentWriteContract;
use crate::kernel::obligation_facts::write_contract_facts_for_snapshot;
use crate::kernel::snapshot::RuntimeSnapshot;

pub(crate) fn content_contract_for(snapshot: &RuntimeSnapshot) -> Option<ContentWriteContract> {
    write_contract_facts_for_snapshot(snapshot).map(|facts| facts.to_content_contract())
}
