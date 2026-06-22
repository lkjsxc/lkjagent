use super::graph::graph_manifest;
use super::model::{PlannedFile, ScaffoldInput, ScaffoldPlan, ScaffoldProfile};
use super::semantic_seed_body as body;

pub fn plan(input: &ScaffoldInput) -> ScaffoldPlan {
    let mut files = vec![
        file(
            "README.md",
            "lkjagent Documentation",
            "semantic root",
            body::root(),
        ),
        file(
            "project/README.md",
            "Project",
            "project runtime",
            body::project_readme(),
        ),
        file(
            "project/purpose.md",
            "Purpose",
            "project purpose",
            body::project_purpose(),
        ),
        file(
            "project/operating-model.md",
            "Operating Model",
            "project operating model",
            body::project_operating_model(),
        ),
        file(
            "model-interface/README.md",
            "Model Interface",
            "provider-neutral model boundary",
            body::model_readme(),
        ),
        file(
            "model-interface/contract.md",
            "Contract",
            "model proposal contract",
            body::model_contract(),
        ),
        file(
            "model-interface/action-protocol.md",
            "Action Protocol",
            "action language boundary",
            body::action_protocol(),
        ),
        file(
            "implementation/README.md",
            "Implementation",
            "Rust implementation substrate",
            body::implementation_readme(),
        ),
        file(
            "implementation/rust.md",
            "Rust",
            "Rust substrate",
            body::rust_page(),
        ),
        file(
            "implementation/functional-core.md",
            "Functional Core",
            "pure core boundary",
            body::functional_core(),
        ),
        file(
            "relations/README.md",
            "Relations",
            "relation index",
            body::relations_readme(),
        ),
        file(
            "relations/project-model-implementation.md",
            "Project Model Implementation",
            "project relation",
            body::relation_main(),
        ),
        file(
            "relations/state-to-prompt.md",
            "State To Prompt",
            "state prompt relation",
            body::relation_state_prompt(),
        ),
    ];
    files.push(graph_manifest(
        input,
        ScaffoldProfile::LkjagentSemanticSeed,
        &files,
    ));
    ScaffoldPlan {
        root: input.root.clone(),
        profile: ScaffoldProfile::LkjagentSemanticSeed,
        files,
    }
}

fn file(path: &str, title: &str, role: &str, body: String) -> PlannedFile {
    PlannedFile {
        path: path.to_string(),
        title: title.to_string(),
        role: role.to_string(),
        body,
    }
}
