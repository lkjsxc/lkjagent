use crate::error::{ToolError, ToolResult};

use super::body::{leaf_body, readme_body};
use super::fit::{exact_group_count, max_tree_count};
use super::graph::graph_manifest;
use super::model::{PlannedFile, ScaffoldInput, ScaffoldPlan, ScaffoldProfile, ShapeGroup};
use super::names::{slug, title_from_path};
use super::roles::EXTRA_ROLES;
use super::semantic_seed;
use super::shapes::{select_profile, shape};

pub fn semantic_doc_plan(input: &ScaffoldInput) -> ToolResult<ScaffoldPlan> {
    let profile = select_profile(input);
    if profile == ScaffoldProfile::LkjagentSemanticSeed && input.count.is_none() {
        return Ok(semantic_seed::plan(input));
    }
    let mut files = if let Some(target) = input.count {
        counted_files(input, shape(profile), target)?
    } else {
        tree_files(input, shape(profile))
    };
    files.push(graph_manifest(input, profile, &files));
    Ok(ScaffoldPlan {
        root: input.root.clone(),
        profile,
        files,
    })
}

fn tree_files(input: &ScaffoldInput, groups: &[ShapeGroup]) -> Vec<PlannedFile> {
    let mut files = vec![readme(
        "README.md",
        &input.title,
        "root index",
        root_entries(groups),
    )];
    for group in groups {
        files.push(readme(
            &format!("{}/README.md", group.dir),
            group.title,
            group.role,
            leaf_entries(group.leaves),
        ));
        files.extend(group.leaves.iter().map(|role| {
            leaf(
                &format!("{}/{}.md", group.dir, slug(role)),
                &title_from_path(role),
                role,
            )
        }));
    }
    files
}

fn counted_files(
    input: &ScaffoldInput,
    groups: &[ShapeGroup],
    target: usize,
) -> ToolResult<Vec<PlannedFile>> {
    if target < 3 {
        return Err(ToolError::invalid(
            "doc count must allow README, graph, and one semantic file",
        ));
    }
    let without_graph = target.saturating_sub(1);
    if target < 8 {
        return Ok(flat_files(input, without_graph));
    }
    if target > max_tree_count(groups) {
        return Ok(flat_files(input, without_graph));
    }
    let group_count = exact_group_count(groups, without_graph);
    Ok(counted_tree(input, &groups[..group_count], without_graph))
}

fn flat_files(input: &ScaffoldInput, target: usize) -> Vec<PlannedFile> {
    let roles = roles(input, target.saturating_sub(1));
    let entries = with_graph_entry(role_entries(&roles));
    let mut files = vec![readme("README.md", &input.title, "root index", entries)];
    files.extend(
        roles
            .iter()
            .map(|role| leaf(&format!("{}.md", slug(role)), &title_from_path(role), role)),
    );
    files
}

fn counted_tree(input: &ScaffoldInput, groups: &[ShapeGroup], target: usize) -> Vec<PlannedFile> {
    let mut files = vec![readme(
        "README.md",
        &input.title,
        "root index",
        root_entries(groups),
    )];
    let base = 1usize
        .saturating_add(groups.len())
        .saturating_add(groups.len() * 2);
    let mut extra = target.saturating_sub(base);
    for group in groups {
        let add = extra.min(group.leaves.len().saturating_sub(2));
        let take = 2usize.saturating_add(add).min(group.leaves.len());
        extra = extra.saturating_sub(add);
        let leaves = &group.leaves[..take];
        files.push(readme(
            &format!("{}/README.md", group.dir),
            group.title,
            group.role,
            leaf_entries(leaves),
        ));
        files.extend(leaves.iter().map(|role| {
            leaf(
                &format!("{}/{}.md", group.dir, slug(role)),
                &title_from_path(role),
                role,
            )
        }));
    }
    files
}

fn root_entries(groups: &[ShapeGroup]) -> String {
    let entries = groups
        .iter()
        .map(|group| {
            format!(
                "- [{}/]({}/README.md): {}.",
                group.dir, group.dir, group.role
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    with_graph_entry(entries)
}

fn leaf_entries(leaves: &[&str]) -> String {
    let roles = leaves
        .iter()
        .map(|role| role.to_string())
        .collect::<Vec<_>>();
    role_entries(&roles)
}

fn role_entries(roles: &[String]) -> String {
    roles
        .iter()
        .map(|role| format!("- [{}.md]({}.md): {}.", slug(role), slug(role), role))
        .collect::<Vec<_>>()
        .join("\n")
}

fn with_graph_entry(entries: String) -> String {
    if entries.is_empty() {
        "- [.lkj-doc-graph.md](.lkj-doc-graph.md): graph manifest.".to_string()
    } else {
        format!("{entries}\n- [.lkj-doc-graph.md](.lkj-doc-graph.md): graph manifest.")
    }
}

fn readme(path: &str, title: &str, role: &str, entries: String) -> PlannedFile {
    PlannedFile {
        path: path.to_string(),
        title: title.to_string(),
        role: role.to_string(),
        body: readme_body(title, role, &entries),
    }
}

fn leaf(path: &str, title: &str, role: &str) -> PlannedFile {
    PlannedFile {
        path: path.to_string(),
        title: title.to_string(),
        role: role.to_string(),
        body: leaf_body(title, role),
    }
}

fn roles(input: &ScaffoldInput, count: usize) -> Vec<String> {
    let mut roles = input.sections.clone();
    for role in EXTRA_ROLES {
        if roles.len() >= count {
            break;
        }
        roles.push((*role).to_string());
    }
    roles.truncate(count);
    roles
}
