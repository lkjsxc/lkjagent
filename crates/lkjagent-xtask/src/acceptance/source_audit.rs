use std::fs;
use std::path::Path;

pub fn canonical_authorities_are_unique(root: &Path) -> bool {
    let required = [
        (
            "crates/lkjagent-core/src/runtime_event.rs",
            "pub fn reduce(snapshot:",
        ),
        (
            "crates/lkjagent-core/src/runtime_tool_catalog.rs",
            "const DIRECT_CATALOG",
        ),
        (
            "crates/lkjagent-core/src/parse.rs",
            "pub fn parse_expected_for_decision",
        ),
        (
            "crates/lkjagent-effects/src/workspace_edit.rs",
            "pub fn prepare_exact_edit(",
        ),
        (
            "crates/lkjagent-app/src/tui_viewport.rs",
            "pub fn reconcile(",
        ),
    ];
    required.iter().all(|(path, needle)| {
        fs::read_to_string(root.join(path)).is_ok_and(|text| text.matches(needle).count() == 1)
    }) && [
        "crates/lkjagent-core/src/runtime_reducer.rs",
        "crates/lkjagent-core/src/runtime_tool_registry.rs",
        "crates/lkjagent-core/src/action_parser.rs",
        "crates/lkjagent-effects/src/file_effect.rs",
        "crates/lkjagent-app/src/viewport.rs",
    ]
    .iter()
    .all(|path| !root.join(path).exists())
}
pub fn product_surface_is_clean(root: &Path) -> bool {
    let Ok(files) = crate::facts::collect_files(root) else {
        return false;
    };
    if !crate::style::check_style(&files).is_empty() {
        return false;
    }
    let cargo = fs::read_to_string(root.join("Cargo.toml")).unwrap_or_default();
    let compose = fs::read_to_string(root.join("docker-compose.yml")).unwrap_or_default();
    let forbidden_name = files.iter().any(|file| {
        let product = file.path.starts_with("crates/lkjagent-")
            && !file.path.starts_with("crates/lkjagent-xtask/")
            && !file.path.contains("/tests/");
        product
            && file
                .path
                .split('/')
                .any(|part| part.contains("mock") || part.contains("fake"))
    });
    cargo.contains("unsafe_code = \"forbid\"")
        && cargo.matches("[profile.").count() == 1
        && cargo.contains("[profile.release]")
        && compose
            .lines()
            .filter(|line| line.contains("profiles:"))
            .all(|line| {
                ["daemon", "verify", "live", "shell", "endpoint"]
                    .iter()
                    .any(|name| line.contains(name))
            })
        && !forbidden_name
}
pub fn rejected_profiles_absent(root: &Path) -> bool {
    if root.join("evaluation/experiment_runner").exists()
        || root.join("evaluation/experiment-runner.py").exists()
    {
        return false;
    }
    let Ok(files) = crate::facts::collect_files(root) else {
        return false;
    };
    let rejected = [
        "tool-named",
        "broad-workspace",
        "unified-diff",
        "line-range",
    ];
    files
        .iter()
        .filter(|file| {
            file.path.starts_with("crates/lkjagent-")
                && !file.path.starts_with("crates/lkjagent-xtask/")
                && file.path.contains("/src/")
        })
        .all(|file| rejected.iter().all(|needle| !file.text.contains(needle)))
}
