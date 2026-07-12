use std::path::Path;

use crate::docs::check_docs;
use crate::docs_authority_contract;
use crate::facts::collect_files;
use crate::gate::check_lines;
use crate::model::RepoFile;

pub fn check(root: &Path) -> Result<(), Vec<String>> {
    let files = collect_files(root).map_err(|error| vec![error])?;
    let failures = check_contract(&files);
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures)
    }
}

pub fn check_contract(files: &[RepoFile]) -> Vec<String> {
    let mut failures = check_docs(files)
        .into_iter()
        .chain(check_lines(files))
        .map(|violation| violation.message())
        .collect::<Vec<_>>();
    docs_authority_contract::check(files, &mut failures);
    failures
}
