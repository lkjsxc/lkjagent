pub(super) fn report(root: &str, failures: Vec<String>, content_requested: bool) -> String {
    let lanes = lanes(&failures, content_requested);
    if failures.is_empty() {
        return format!(
            "document audit passed\nroot={root}\n{lanes}\nchecks_run=topology,links,path_hygiene,content_readiness\nfailed=0\nnext_action=record document-structure evidence"
        );
    }
    format!(
        "document audit failed\nroot={root}\n{lanes}\nchecks_run=topology,links,path_hygiene,content_readiness\nfailed={}\nfailures:\n- {}\nnext_action={}",
        failures.len(),
        failures.join("\n- "),
        next_action(&failures)
    )
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
