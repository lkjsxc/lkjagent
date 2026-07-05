use crate::runtime_candidate::SelectorCandidate;
use crate::runtime_state::RuntimeSnapshot;

pub(crate) fn apply_edge_blocks(
    snapshot: &RuntimeSnapshot,
    mut item: SelectorCandidate,
) -> SelectorCandidate {
    let Some(key) = &item.state_key else {
        return item;
    };
    let label = key.as_label();
    item.blocked_by = snapshot
        .active_edges()
        .into_iter()
        .filter(|edge| edge.relation.0 == "blocks")
        .filter(|edge| edge.to_ref.kind == "state" && edge.to_ref.id == label)
        .map(|edge| edge.id)
        .collect();
    item
}
