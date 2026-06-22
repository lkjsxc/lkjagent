use crate::doc_catalog::check_doc_catalog;
use crate::doc_common::check_markdown_basics;
use crate::doc_links::check_doc_links;
use crate::doc_special::check_special_docs;
use crate::doc_topology::check_doc_topology;
use crate::model::{RepoFile, Violation};

pub fn check_docs(files: &[RepoFile]) -> Vec<Violation> {
    let mut violations = Vec::new();
    violations.extend(check_markdown_basics(files));
    violations.extend(check_doc_topology(files));
    violations.extend(check_doc_links(files));
    violations.extend(check_doc_catalog(files));
    violations.extend(check_special_docs(files));
    violations
}
