pub mod arithmetic;
pub mod automata;
pub mod bundle;
pub mod correction;
pub mod graph;
pub mod owner_docs;
pub mod owner_ops;
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
        JudgeKind::SemanticProjectDocs => owner_docs::project_docs(workspace),
        JudgeKind::RecursiveDocTree => owner_docs::recursive_tree(workspace),
        JudgeKind::ThirtySemanticDocs => owner_docs::thirty_docs(workspace),
        JudgeKind::GraphStateParamRecovery => owner_ops::graph_state_recovery(workspace),
        JudgeKind::DocScaffoldParamRecovery => owner_ops::doc_scaffold_recovery(workspace),
        JudgeKind::StatusAccounting => owner_ops::status_accounting(workspace),
        JudgeKind::GptHandoffLog => owner_ops::gpt_handoff_log(workspace),
    };
    Ok(match result {
        Ok(()) => JudgeOutcome::pass(task.points),
        Err(reason) => JudgeOutcome::fail(task.points, reason),
    })
}
