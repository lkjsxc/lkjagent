pub mod arithmetic;
pub mod automata;
pub mod bundle;
pub mod correction;
pub mod graph;
pub mod owner_docs;
pub mod owner_faults;
pub mod owner_observability;
pub mod owner_thirty;
pub mod repair;
pub mod synthesis;

use crate::model::BenchmarkTask;

pub const TINY: &[BenchmarkTask] = &[
    arithmetic::TASK,
    graph::TASK,
    automata::TASK,
    synthesis::TASK,
    repair::TASK,
    bundle::TASK,
    correction::TASK,
    owner_docs::PROJECT_TASK,
    owner_docs::RECURSIVE_TASK,
    owner_thirty::TASK,
    owner_faults::GRAPH_TASK,
    owner_faults::SCAFFOLD_TASK,
    owner_observability::STATUS_TASK,
    owner_observability::LOG_TASK,
];
