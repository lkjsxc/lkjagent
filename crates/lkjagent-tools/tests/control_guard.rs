use lkjagent_tools::control::{CompletionGuard, ControlState};
use lkjagent_tools::count_guard::{CountGuard, CountKind, CountMode};

#[test]
fn resumed_owner_guidance_adds_exact_markdown_count_guard() {
    let mut state = ControlState::default();

    state.resume_task_with("Finish with exactly 100 markdown files in docs.");

    assert_eq!(
        state.guard,
        CompletionGuard::MarkdownCount {
            target: 100,
            mode: CountMode::Exact
        }
    );
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
                target: 100,
                mode: CountMode::Exact
            }
        }
    );
}

#[test]
fn japanese_about_file_count_request_adds_approximate_file_guard() {
    let mut state = ControlState::default();

    state.start_task("合計100ファイルぐらいの大きな成果物を作ってください。");

    assert_eq!(
        state.guard,
        CompletionGuard::FileCount {
            target: 100,
            mode: CountMode::Approximate
        }
    );
}

#[test]
fn japanese_full_width_file_count_request_adds_approximate_file_guard() {
    let mut state = ControlState::default();

    state.start_task("合計１００ファイル程度の大きな成果物を作ってください。");

    assert_eq!(
        state.guard,
        CompletionGuard::FileCount {
            target: 100,
            mode: CountMode::Approximate
        }
    );
}

#[test]
fn english_document_count_request_adds_approximate_file_guard() {
    let mut state = ControlState::default();

    state.start_task("Create around 1,000 documents total for docs and main content.");

    assert_eq!(
        state.guard,
        CompletionGuard::FileCount {
            target: 1000,
            mode: CountMode::Approximate
        }
    );
}

#[test]
fn file_count_target_uses_number_near_file_signal() {
    let mut state = ControlState::default();

    state.start_task("Create 20 files total with 100 sections of internal detail.");

    assert_eq!(
        state.guard,
        CompletionGuard::FileCount {
            target: 20,
            mode: CountMode::Exact
        }
    );
}

#[test]
fn file_count_target_ignores_model_version_numbers() {
    let mut state = ControlState::default();

    state.start_task("Use GPT-5.3-Codex-Spark style thrift and create about 100 files total.");

    assert_eq!(
        state.guard,
        CompletionGuard::FileCount {
            target: 100,
            mode: CountMode::Approximate
        }
    );
}

#[test]
fn exact_signal_overrides_about_wording() {
    let mut state = ControlState::default();

    state.start_task("Create exactly about 100 files total.");

    assert_eq!(
        state.guard,
        CompletionGuard::FileCount {
            target: 100,
            mode: CountMode::Exact
        }
    );
}
