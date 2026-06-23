use super::model::{ScaffoldInput, ScaffoldPlan, ScaffoldProfile};
use super::semantic_workspace_body as body;
use super::semantic_workspace_terms::requested_terms;

pub fn plan(input: &ScaffoldInput) -> ScaffoldPlan {
    let terms = requested_terms(input);
    let mut files = vec![body::root_file(input, &terms)];
    files.extend(body::request_files(input, &terms));
    files.extend(body::project_files());
    files.extend(body::relation_files(input, &terms));
    files.extend(body::topic_files(&terms));
    files.push(body::catalog(input));
    ScaffoldPlan {
        root: input.root.clone(),
        profile: ScaffoldProfile::GenericStructuredDocs,
        files,
    }
}
