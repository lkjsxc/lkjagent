use super::body::{leaf_body, readme_body};
use super::model::{PlannedFile, ScaffoldInput, ScaffoldProfile};

pub(super) fn catalog_file(input: &ScaffoldInput, profile: ScaffoldProfile) -> PlannedFile {
    PlannedFile {
        path: "catalog.toml".to_string(),
        title: "Catalog".to_string(),
        role: "scaffold metadata".to_string(),
        body: format!(
            "title = \"{}\"\nkind = \"{}\"\nprofile = \"{:?}\"\n",
            input.title, input.kind, profile
        ),
    }
}

pub(super) fn readme(path: &str, title: &str, role: &str, entries: String) -> PlannedFile {
    PlannedFile {
        path: path.to_string(),
        title: title.to_string(),
        role: role.to_string(),
        body: readme_body(title, role, &entries),
    }
}

pub(super) fn leaf(path: &str, title: &str, role: &str) -> PlannedFile {
    PlannedFile {
        path: path.to_string(),
        title: title.to_string(),
        role: role.to_string(),
        body: leaf_body(title, role),
    }
}
