pub mod arithmetic;
pub mod automata;
pub mod bundle;
pub mod correction;
pub mod graph;
pub mod large_artifact;
pub mod long_novel;
pub mod owner_address;
pub mod owner_continuation;
pub mod owner_doc_topics;
pub mod owner_docs;
pub mod owner_loop_ops;
pub mod owner_ops;
pub mod owner_uploaded;
pub mod program;
pub mod story_manuscript;

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
        JudgeKind::MultiTopicDocumentation => owner_doc_topics::multi_topic_docs(workspace),
        JudgeKind::GraphStateParamRecovery => owner_ops::graph_state_recovery(workspace),
        JudgeKind::BatchWriteProtocolRecovery => owner_ops::batch_write_recovery(workspace),
        JudgeKind::RecoveryLoopLongStory => owner_ops::recovery_loop_long_story(workspace),
        JudgeKind::GraphPlanExample => owner_loop_ops::graph_plan_example(workspace),
        JudgeKind::GraphTransitionTarget => owner_loop_ops::graph_transition_target(workspace),
        JudgeKind::MemoryFtsQuery => owner_loop_ops::memory_fts_query(workspace),
        JudgeKind::MaintenanceMemoryDuplicate => {
            owner_loop_ops::maintenance_memory_duplicate(workspace)
        }
        JudgeKind::PolicyContradiction => owner_loop_ops::policy_contradiction(workspace),
        JudgeKind::GraphNoteKindRecovery => owner_loop_ops::graph_note_kind_recovery(workspace),
        JudgeKind::BreadCookbookArtifact => owner_loop_ops::bread_cookbook_artifact(workspace),
        JudgeKind::TurnBudgetContinuation => owner_continuation::turn_budget_checkpoint(workspace),
        JudgeKind::UploadedRunFixtures => owner_uploaded::uploaded_run_fixtures(workspace),
        JudgeKind::LongNovelFailure => long_novel::long_novel_failure(workspace),
        JudgeKind::StoryManuscript => story_manuscript::story_manuscript(workspace),
        JudgeKind::ArtifactAddressController => {
            owner_address::artifact_address_controller(workspace)
        }
        JudgeKind::StatusAccounting => owner_ops::status_accounting(workspace),
        JudgeKind::ModelHandoffLog => owner_ops::model_handoff_log(workspace),
        JudgeKind::LargeArtifactManuscript => large_artifact::manuscript(workspace),
        JudgeKind::LargeArtifactReport => large_artifact::report(workspace),
        JudgeKind::LargeArtifactStudySet => large_artifact::study_set(workspace),
        JudgeKind::LargeArtifactDocumentation => large_artifact::documentation(workspace),
        JudgeKind::LargeArtifactGenericRoot => large_artifact::generic_root(workspace),
        JudgeKind::LargeArtifactAtomRetry => large_artifact::atom_retry(workspace),
        JudgeKind::LargeArtifactAssembly => large_artifact::assembly(workspace),
    };
    Ok(match result {
        Ok(()) => JudgeOutcome::pass(task.points),
        Err(reason) => JudgeOutcome::fail(task.points, reason),
    })
}
