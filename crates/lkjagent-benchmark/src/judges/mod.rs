pub mod arithmetic;
pub mod automata;
pub mod bundle;
pub mod correction;
pub mod graph;
pub mod program;

use std::path::Path;

use crate::error::BenchResult;
use crate::model::{BenchmarkTask, JudgeKind, JudgeOutcome};

pub fn judge_task(task: &BenchmarkTask, workspace: &Path) -> BenchResult<JudgeOutcome> {
    let result = match task.judge {
        JudgeKind::Crt => arithmetic::judge(workspace),
        JudgeKind::ShortestPath => graph::judge(workspace),
        JudgeKind::EvenOnesDfa => automata::judge(workspace),
        JudgeKind::FibonacciShell => program::judge_fibonacci(workspace),
        JudgeKind::RepairRankShell => program::judge_rank(workspace),
        JudgeKind::ReadmeBundle => bundle::judge(workspace),
        JudgeKind::CorrectedComposites => correction::judge(workspace),
    };
    Ok(match result {
        Ok(()) => JudgeOutcome::pass(task.points),
        Err(reason) => JudgeOutcome::fail(task.points, reason),
    })
}
