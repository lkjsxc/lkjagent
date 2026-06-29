use std::path::Path;

pub(crate) struct IdentityContract {
    pub selected: Vec<String>,
    pub valid_example: String,
    pub response: String,
}

pub(crate) fn identity_contract_for(root: &str, kind: &str, full: &Path) -> IdentityContract {
    let selected = identity_paths(kind)
        .into_iter()
        .filter(|path| !full.join(path).is_file())
        .take(1)
        .map(str::to_string)
        .collect::<Vec<_>>();
    let selected = if selected.is_empty() {
        vec!["objective.md".to_string()]
    } else {
        selected
    };
    let valid_example = crate::artifact_next_example::batch_write_contract(root, kind, &selected);
    let response = format!(
        "artifact_next_result=root_needs_identity\nroot={root}\nkind={kind}\nmissing=catalog,readme,semantic-leaf\nruntime_event=ArtifactRootIncomplete\nnext_decision_required=true\ncandidate_action=fs.batch_write\ncandidate_contract:\n{valid_example}"
    );
    IdentityContract {
        selected,
        valid_example,
        response,
    }
}

fn identity_paths(kind: &str) -> Vec<&'static str> {
    if kind.eq_ignore_ascii_case("story") {
        vec![
            "catalog.toml",
            "README.md",
            "objective.md",
            "setting-overview.md",
            "cast.md",
        ]
    } else {
        vec![
            "catalog.toml",
            "README.md",
            "objective.md",
            "overview.md",
            "verification-notes.md",
        ]
    }
}
