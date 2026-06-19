use lkjagent_tools::control::{CompletionGuard, ControlState};

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

    assert_eq!(state.guard, CompletionGuard::RecursiveKnowledge);
}
