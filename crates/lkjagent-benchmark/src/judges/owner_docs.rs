use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

pub fn project_docs(workspace: &Path) -> Result<(), String> {
    let root = workspace.join("docs");
    require_file(&root.join("README.md"))?;
    require_file(&root.join(".lkj-doc-graph.md"))?;
    no_serial_names(&root)?;
    for dir in "overview architecture guides operations reference".split_whitespace() {
        let local = root.join(dir);
        require_file(&local.join("README.md"))?;
        require_link(&root.join("README.md"), &format!("{dir}/README.md"))?;
    }
    require_link(&root.join("architecture/README.md"), "runtime.md")?;
    require_graph_sections(&root.join(".lkj-doc-graph.md"))
}

pub fn recursive_tree(workspace: &Path) -> Result<(), String> {
    let root = workspace.join("docs");
    require_file(&root.join(".lkj-doc-graph.md"))?;
    no_serial_names(&root)?;
    let dirs = directories(&root)?;
    if !dirs.iter().any(|dir| depth_after(&root, dir) >= 2) {
        return Err("documentation tree is not recursive".to_string());
    }
    for dir in dirs {
        let readme = dir.join("README.md");
        require_file(&readme)?;
        require_text(&readme, "## Purpose")?;
        require_local_links(&dir, &readme)?;
    }
    Ok(())
}

pub fn thirty_docs(workspace: &Path) -> Result<(), String> {
    let root = workspace.join("docs");
    let files = markdown_files(&root)?;
    let docs = files
        .iter()
        .filter(|path| path.as_str() != ".lkj-doc-graph.md")
        .count();
    if docs != 30 {
        return Err(format!("expected 30 documentation files, got {docs}"));
    }
    require_file(&root.join(".lkj-doc-graph.md"))?;
    no_serial_names(&root)?;
    for dir in directories(&root)? {
        require_file(&dir.join("README.md"))?;
    }
    Ok(())
}

fn require_local_links(dir: &Path, readme: &Path) -> Result<(), String> {
    let text = read(readme)?;
    for entry in fs::read_dir(dir).map_err(|error| format!("read_dir failed: {error}"))? {
        let path = entry
            .map_err(|error| format!("dir entry failed: {error}"))?
            .path();
        let name = file_name(&path)?;
        if path.is_dir() {
            if !text.contains(&format!("({name}/README.md)"))
                && !text.contains(&format!("({name})"))
            {
                return Err(format!(
                    "{} missing child dir link {name}",
                    readme.display()
                ));
            }
        } else if path.extension().is_some_and(|ext| ext == "md")
            && name != "README.md"
            && !name.starts_with('.')
        {
            require_link(readme, &name)?;
        }
    }
    Ok(())
}

fn no_serial_names(root: &Path) -> Result<(), String> {
    for file in markdown_files(root)? {
        if is_serial(&file) {
            return Err(format!("serial placeholder filename: {file}"));
        }
    }
    Ok(())
}

fn is_serial(path: &str) -> bool {
    let stem = path
        .rsplit('/')
        .next()
        .unwrap_or(path)
        .trim_end_matches(".md")
        .to_ascii_lowercase();
    ["part", "section", "chapter", "file", "doc"]
        .iter()
        .any(|prefix| {
            stem.strip_prefix(prefix).is_some_and(|rest| {
                let rest = rest.trim_start_matches(['-', '_']);
                !rest.is_empty() && rest.chars().all(|ch| ch.is_ascii_digit())
            })
        })
}

fn require_graph_sections(path: &Path) -> Result<(), String> {
    for section in ["## Nodes", "## Edges", "## Coverage"] {
        require_text(path, section)?;
    }
    Ok(())
}

fn require_file(path: &Path) -> Result<(), String> {
    if path.is_file() {
        Ok(())
    } else {
        Err(format!("missing file {}", path.display()))
    }
}

fn require_link(path: &Path, target: &str) -> Result<(), String> {
    require_text(path, &format!("({target})"))
}

fn require_text(path: &Path, needle: &str) -> Result<(), String> {
    let text = read(path)?;
    if text.contains(needle) {
        Ok(())
    } else {
        Err(format!("{} missing {needle}", path.display()))
    }
}

fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("{} unreadable: {error}", path.display()))
}

fn directories(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut dirs = vec![root.to_path_buf()];
    visit_dirs(root, &mut dirs)?;
    Ok(dirs)
}

fn visit_dirs(path: &Path, dirs: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(path).map_err(|error| format!("read_dir failed: {error}"))? {
        let child = entry
            .map_err(|error| format!("dir entry failed: {error}"))?
            .path();
        if child.is_dir() {
            dirs.push(child.clone());
            visit_dirs(&child, dirs)?;
        }
    }
    Ok(())
}

fn markdown_files(root: &Path) -> Result<BTreeSet<String>, String> {
    let mut files = BTreeSet::new();
    visit_files(root, root, &mut files)?;
    Ok(files)
}

fn visit_files(root: &Path, path: &Path, files: &mut BTreeSet<String>) -> Result<(), String> {
    for entry in fs::read_dir(path).map_err(|error| format!("read_dir failed: {error}"))? {
        let child = entry
            .map_err(|error| format!("dir entry failed: {error}"))?
            .path();
        if child.is_dir() {
            visit_files(root, &child, files)?;
        } else if child.extension().is_some_and(|extension| extension == "md") {
            files.insert(relative(root, &child)?);
        }
    }
    Ok(())
}

fn relative(root: &Path, child: &Path) -> Result<String, String> {
    child
        .strip_prefix(root)
        .map_err(|error| format!("relative path failed: {error}"))
        .map(|path| path.to_string_lossy().replace('\\', "/"))
}

fn file_name(path: &Path) -> Result<String, String> {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .ok_or_else(|| format!("missing file name: {}", path.display()))
}

fn depth_after(root: &Path, dir: &Path) -> usize {
    dir.strip_prefix(root)
        .ok()
        .map(|path| path.components().count())
        .unwrap_or(0)
}
