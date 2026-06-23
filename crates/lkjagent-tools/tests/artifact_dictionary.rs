mod support;

use std::fs;

use lkjagent_tools::dispatch::dispatch;
use support::{action, runtime, state, store, temp_workspace, TestResult};

#[test]
fn artifact_audit_rejects_shallow_dictionary_term_list() -> TestResult<()> {
    let workspace = temp_workspace("artifact-dictionary-shallow")?;
    fs::create_dir_all(workspace.join("dictionary"))?;
    fs::write(
        workspace.join("dictionary/bread-terms.txt"),
        shallow_terms(),
    )?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;

    let output = dispatch(
        &action(
            "artifact.audit",
            &[("root", "dictionary"), ("kind", "dictionary")],
        ),
        &runtime,
        &mut conn,
        &mut state(),
    );

    assert!(output.content.contains("artifact audit failed"));
    assert!(output.content.contains("kind=dictionary"));
    assert!(output.content.contains("entries=32"));
    assert!(output.content.contains("definition_coverage"));
    assert!(output.content.contains("example_coverage"));
    Ok(())
}

#[test]
fn artifact_audit_accepts_detailed_dictionary_entries() -> TestResult<()> {
    let workspace = temp_workspace("artifact-dictionary-detailed")?;
    fs::create_dir_all(workspace.join("dictionary"))?;
    fs::write(
        workspace.join("dictionary/bread-dictionary.md"),
        detailed_dictionary(),
    )?;
    let runtime = runtime(workspace)?;
    let mut conn = store()?;

    let output = dispatch(
        &action(
            "artifact.audit",
            &[("root", "dictionary"), ("kind", "dictionary")],
        ),
        &runtime,
        &mut conn,
        &mut state(),
    );

    assert!(output.content.contains("artifact audit passed"));
    assert!(output.content.contains("readiness=content-bearing"));
    assert!(output.content.contains("entries=20"));
    Ok(())
}

fn shallow_terms() -> String {
    (1..=32)
        .map(|index| format!("- bread term {index}: short note"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn detailed_dictionary() -> String {
    let mut text = "# Bread Dictionary\n\n".to_string();
    for index in 1..=20 {
        text.push_str(&format!(
            "## Bread Term {index}\n\nPronunciation: bread term {index}\n\nPart of speech: noun phrase\n\nDefinition: Bread term {index} names a specific baking idea used by bread makers during mixing and baking.\n\nEtymology: Origin: bakery usage adapted from workshop language for repeated bread practice.\n\nExample: The baker used bread term {index} while explaining the loaf to apprentices.\n\n"
        ));
    }
    text
}
