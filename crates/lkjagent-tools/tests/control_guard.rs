use lkjagent_tools::control::{CompletionGuard, ControlState};
use lkjagent_tools::count_guard::{CountGuard, CountKind};

#[test]
fn resumed_owner_guidance_adds_exact_markdown_count_guard() {
    let mut state = ControlState::default();

    state.resume_task_with("Finish with exactly 100 markdown files in docs.");

    assert_eq!(state.guard, CompletionGuard::MarkdownCount { target: 100 });
}

#[test]
fn resumed_owner_guidance_preserves_stronger_recursive_guard() {
    let mut state = ControlState::default();
    state.start_task("Build a recursive knowledge base.");

    state.resume_task_with("Also keep exactly 100 markdown files.");

    assert_eq!(
        state.guard,
        CompletionGuard::RecursiveKnowledgeCount {
            count: CountGuard {
                kind: CountKind::Markdown,
                target: 100
            }
        }
    );
}

#[test]
fn japanese_file_count_request_adds_general_file_guard() {
    let mut state = ControlState::default();

    state.start_task("合計100ファイルぐらいの大きな成果物を作ってください。");

    assert_eq!(state.guard, CompletionGuard::FileCount { target: 100 });
}
