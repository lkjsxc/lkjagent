use std::path::Path;

pub fn check(root: &Path, identifier: &str) -> Result<(), Vec<String>> {
    match identifier {
        "docs-authority" => crate::docs_authority_gate::check(root),
        "evaluation-harness" => crate::evaluation_harness::check(root),
        _ => Err(vec![format!("unknown node gate: {identifier}")]),
    }
}
