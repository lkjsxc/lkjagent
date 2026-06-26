const MAX_FAILURES_SHOWN: usize = 24;
const FIRST_FAILURES_SHOWN: usize = 12;

pub(super) fn report(root: &str, failures: Vec<String>, content_requested: bool) -> String {
    let failures = applicable_failures(root, failures);
    let lanes = lanes(&failures, content_requested);
    if failures.is_empty() {
        return format!(
            "document audit passed\nroot={root}\n{lanes}\nchecks_run=topology,links,path_hygiene,content_readiness\nfailed=0\nnext_action=record document-structure evidence"
        );
    }
    let shown = shown_failures(&failures);
    format!(
        "document audit failed\nroot={root}\n{lanes}\nchecks_run=topology,links,path_hygiene,content_readiness\nfailed={}\nfailures_shown={}\nfailures_omitted={}\nfailures:\n- {}\nnext_action={}",
        failures.len(),
        shown.len(),
        failures.len().saturating_sub(shown.len()),
        shown.join("\n- "),
        next_action(&failures)
    )
}

fn applicable_failures(root: &str, failures: Vec<String>) -> Vec<String> {
    if story_artifact_root(root) {
        return failures
            .into_iter()
            .filter(|failure| !failure.starts_with("missing_readme_link"))
            .collect();
    }
    failures
}

fn story_artifact_root(root: &str) -> bool {
    root.trim_start_matches("./").starts_with("stories/")
}

fn shown_failures(failures: &[String]) -> Vec<String> {
    let mut shown = failures
        .iter()
        .take(FIRST_FAILURES_SHOWN)
        .cloned()
        .collect::<Vec<_>>();
    extend_matching(&mut shown, failures, content_failure_prefix);
    extend_matching(&mut shown, failures, |_| true);
    shown
}

fn extend_matching(shown: &mut Vec<String>, failures: &[String], predicate: fn(&str) -> bool) {
    for failure in failures {
        if shown.len() >= MAX_FAILURES_SHOWN {
            return;
        }
        if predicate(failure) && !shown.iter().any(|item| item == failure) {
            shown.push(failure.clone());
        }
    }
}

fn lanes(failures: &[String], content_requested: bool) -> String {
    format!(
        "topology={}\nlinks={}\npath_hygiene={}\ncontent_readiness={}\nartifact_readiness=not-owner",
        lane(failures, topology_failure),
        lane(failures, link_failure),
        lane(failures, path_failure),
        content_lane(failures, content_requested)
    )
}

fn lane(failures: &[String], predicate: fn(&str) -> bool) -> &'static str {
    if failures.iter().any(|failure| predicate(failure)) {
        "failed"
    } else {
        "passed"
    }
}

fn content_lane(failures: &[String], requested: bool) -> &'static str {
    if failures
        .iter()
        .any(|failure| content_failure_prefix(failure))
    {
        "failed"
    } else if requested {
        "passed"
    } else {
        "not-requested"
    }
}

fn topology_failure(failure: &str) -> bool {
    [
        "missing_root",
        "missing_readme",
        "too_few_children",
        "h1_count",
        "line_limit",
        "count_mismatch",
    ]
    .iter()
    .any(|prefix| failure.starts_with(prefix))
}

fn link_failure(failure: &str) -> bool {
    failure.starts_with("missing_readme_link")
}

fn path_failure(failure: &str) -> bool {
    [
        "markdown_suffix_directory",
        "serial_filename",
        "path_segment_too_long",
        "markdown_stem_too_long",
        "path_too_long",
        "multi_topic_slug",
        "banned_release_wording",
    ]
    .iter()
    .any(|prefix| failure.starts_with(prefix))
}

fn content_failure_prefix(failure: &str) -> bool {
    failure.contains("content") || failure.starts_with("weak_")
}

fn next_action(failures: &[String]) -> &'static str {
    if failures
        .iter()
        .any(|failure| content_failure_prefix(failure))
    {
        "fs.batch_write path-specific content or artifact.next"
    } else if failures.iter().any(|failure| path_failure(failure)) {
        "rename paths then update README and catalog links"
    } else {
        "doc.scaffold or fs.batch_write exact failed topology"
    }
}
