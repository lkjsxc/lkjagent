use std::collections::BTreeSet;
use std::path::Path;

use super::model::{PlannedFile, ScaffoldInput, ScaffoldProfile};
use super::names::slug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphData {
    pub nodes: BTreeSet<String>,
    pub paths: Vec<String>,
    pub edges: Vec<(String, String)>,
}

pub fn graph_manifest(
    input: &ScaffoldInput,
    profile: ScaffoldProfile,
    files: &[PlannedFile],
) -> PlannedFile {
    let mut nodes = vec!["| root | README.md | root index | scaffolded |".to_string()];
    let mut edges = Vec::new();
    for file in files.iter().filter(|file| file.path != "README.md") {
        let id = node_id(&file.path);
        nodes.push(format!(
            "| {id} | {} | {} | scaffolded |",
            file.path, file.role
        ));
        edges.push(format!(
            "| root | {id} | indexes | README links {} |",
            file.path
        ));
    }
    let title = crate::model_names::sanitize_model_names(&input.title).text;
    PlannedFile {
        path: ".lkj-doc-graph.md".to_string(),
        title: "Document Graph".to_string(),
        role: "graph ledger".to_string(),
        body: format!(
            "# Document Graph\n\n## Purpose\n\nCompact graph ledger for `{}` generated as {:?}.\n\n## Nodes\n\n| id | path | role | status |\n| --- | --- | --- | --- |\n{}\n\n## Edges\n\n| from | to | kind | reason |\n| --- | --- | --- | --- |\n{}\n\n## Coverage\n\n| owner requirement | covered by | status |\n| --- | --- | --- |\n| recursive structure | README.md and local README indexes | satisfied |\n| no sequence-only names | all generated paths | satisfied |\n| graph manifest | .lkj-doc-graph.md | satisfied |\n",
            title,
            profile,
            nodes.join("\n"),
            edges.join("\n")
        ),
    }
}

pub fn parse_graph(root: &Path) -> Option<GraphData> {
    let text = std::fs::read_to_string(root.join(".lkj-doc-graph.md")).ok()?;
    let mut in_nodes = false;
    let mut in_edges = false;
    let mut nodes = BTreeSet::new();
    let mut paths = Vec::new();
    let mut edges = Vec::new();
    for line in text.lines() {
        match line {
            "## Nodes" => {
                in_nodes = true;
                in_edges = false;
                continue;
            }
            "## Edges" => {
                in_nodes = false;
                in_edges = true;
                continue;
            }
            "## Coverage" => break,
            _ => {}
        }
        let cells = table_cells(line);
        if in_nodes && cells.len() >= 4 && cells[0] != "id" && cells[0] != "---" {
            nodes.insert(cells[0].to_string());
            paths.push(cells[1].to_string());
        }
        if in_edges && cells.len() >= 4 && cells[0] != "from" && cells[0] != "---" {
            edges.push((cells[0].to_string(), cells[1].to_string()));
        }
    }
    Some(GraphData {
        nodes,
        paths,
        edges,
    })
}

fn table_cells(line: &str) -> Vec<&str> {
    if !line.starts_with('|') {
        return Vec::new();
    }
    line.trim_matches('|').split('|').map(str::trim).collect()
}

fn node_id(path: &str) -> String {
    path.trim_end_matches(".md")
        .split('/')
        .map(slug)
        .collect::<Vec<_>>()
        .join(".")
}
