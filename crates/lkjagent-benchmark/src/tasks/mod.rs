pub mod arithmetic;
pub mod automata;
pub mod bundle;
pub mod correction;
pub mod graph;
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
];
