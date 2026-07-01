mod events;
mod model;
mod projection;
mod read;
mod write;

pub use events::{record_assembly_run, record_event, upsert_readiness};
pub use model::{AtomInput, AtomRow, ContractInput, ContractRow, EdgeInput, PlanInput, PlanRow};
pub use projection::{AssemblyRunInput, EventInput, ReadinessInput, ReadinessRow};
pub use read::{
    active_contract_for_plan, active_contracts, atoms_for_plan, latest_plan_for_case,
    plan_for_root, readiness_for_case, readiness_for_plan,
};
pub use write::{
    create_contract, replace_atoms, replace_edges, set_contract_status, update_atom_status,
    upsert_plan,
};

pub fn split_lines(value: &str) -> Vec<String> {
    value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}
