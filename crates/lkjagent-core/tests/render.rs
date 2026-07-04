use lkjagent_core::classify::instantiate;
use lkjagent_core::render::render_prompt;

#[test]
fn prompt_includes_task_brief() {
    let mut snapshot = instantiate(2, "What is known?");
    snapshot.task.brief = "memory_facts:\nrow memory fact".to_string();
    let prompt = render_prompt(&snapshot.task, &snapshot.steps, &snapshot.steps[0]);
    assert!(prompt.system.contains("Task brief:"));
    assert!(prompt.system.contains("row memory fact"));
}

#[test]
fn retry_prompt_fingerprint_changes() {
    let mut snapshot = instantiate(1, "answer a workspace question");
    let step = match snapshot.steps.first().cloned() {
        Some(step) => step,
        None => return assert_eq!(snapshot.steps.len(), 1),
    };
    let before = render_prompt(&snapshot.task, &snapshot.steps, &step);
    snapshot.steps[0].attempts_used = 1;
    let after = render_prompt(&snapshot.task, &snapshot.steps, &snapshot.steps[0]);
    assert_ne!(before.fingerprint, after.fingerprint);
}
