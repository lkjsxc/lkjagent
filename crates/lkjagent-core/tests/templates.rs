use lkjagent_core::classify::{classify, instantiate};
use lkjagent_core::model::{CheckSpec, StepKind, TemplateId};

#[test]
fn classifier_fixture_table_routes_specific_and_ambiguous_work() {
    let cases = vec![
        (
            "Write a manuscript chapter about Aurora.",
            TemplateId::Manuscript,
        ),
        ("Draft chapter 3 of the novel.", TemplateId::Manuscript),
        ("Refresh the documentation pages.", TemplateId::DocsTree),
        ("Repair docs/README.md links.", TemplateId::DocsTree),
        ("What changed in the workspace?", TemplateId::Question),
        ("Why did the build fail?", TemplateId::Question),
        ("How should I run the daemon?", TemplateId::Question),
        ("Edit README.md with install notes.", TemplateId::FileWork),
        (
            "Create notes/today.md for the meeting.",
            TemplateId::FileWork,
        ),
        ("Rewrite crates/core.rs summary.", TemplateId::FileWork),
        ("Append a journal note for today.", TemplateId::Journal),
        ("Record my schedule for Friday.", TemplateId::Journal),
        ("Make a todo list for releases.", TemplateId::Journal),
        ("Survey the repository and report.", TemplateId::Generic),
        ("Explore options for a name.", TemplateId::Generic),
        ("Think about navigation structure.", TemplateId::Generic),
        ("Summarize the plan when ready.", TemplateId::Generic),
        ("Look around and tell me the risks.", TemplateId::Generic),
        ("Organize thoughts for later.", TemplateId::Generic),
        ("Map the current situation.", TemplateId::Generic),
    ];
    for (objective, expected) in cases {
        assert_eq!(classify(objective), expected, "{objective}");
    }
}

#[test]
fn simple_template_snapshots_have_expected_shapes() {
    let generic = instantiate(1, "Survey the workspace and report.");
    assert_eq!(generic.steps[0].kind, StepKind::Explore);
    assert_eq!(generic.steps[0].action_budget, 20);
    assert_eq!(generic.steps[1].kind, StepKind::Respond);

    let question = instantiate(2, "What is in README.md?");
    assert_eq!(question.steps[0].kind, StepKind::Explore);
    assert_eq!(question.steps[0].action_budget, 6);

    let direct_question = instantiate(3, "What is an agent?");
    assert_eq!(direct_question.steps.len(), 1);
    assert_eq!(direct_question.steps[0].kind, StepKind::Respond);

    let file_work = instantiate(4, "Write notes/new-page.md with setup notes.");
    assert_eq!(file_work.steps[0].kind, StepKind::Plan);
    assert!(file_work.task.checks.contains(&CheckSpec::FileExists {
        path: "notes/new-page.md".to_string()
    }));

    let journal = instantiate(5, "Add a journal note about the release.");
    assert_eq!(journal.steps[0].kind, StepKind::Write);
    assert_eq!(
        journal.steps[0].output_path.as_deref(),
        Some("journal/today.md")
    );
    assert_eq!(journal.steps[1].kind, StepKind::Respond);
}
