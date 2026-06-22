use std::fs;
use std::path::Path;

pub fn multi_topic_docs(workspace: &Path) -> Result<(), String> {
    let root = workspace.join("docs");
    for path in required_paths() {
        require_file(&root.join(path))?;
    }
    reject_generic_only(&root)?;
    reject_mock_phrase(&root)?;
    let relation = read(&root.join("relations/project-model-domain-examples.md"))?;
    for topic in [
        "lkjagent",
        "model endpoint",
        "Asia foods",
        "Minecraft",
        "Factorio",
    ] {
        if !relation.contains(topic) {
            return Err(format!("relation page missing topic {topic}"));
        }
    }
    Ok(())
}

fn required_paths() -> &'static [&'static str] {
    &[
        "README.md",
        "catalog.toml",
        "project/README.md",
        "project/lkjagent.md",
        "model-interface/README.md",
        "model-interface/model-endpoint.md",
        "domain-examples/README.md",
        "domain-examples/asia-foods.md",
        "domain-examples/minecraft.md",
        "domain-examples/factorio.md",
        "relations/README.md",
        "relations/project-model-domain-examples.md",
    ]
}

fn reject_generic_only(root: &Path) -> Result<(), String> {
    for generic in [
        "architecture",
        "guides",
        "operations",
        "overview",
        "reference",
    ] {
        if root.join(generic).exists() && !root.join("domain-examples").exists() {
            return Err(format!("generic lkjagent-only scaffold: {generic}"));
        }
    }
    Ok(())
}

fn reject_mock_phrase(root: &Path) -> Result<(), String> {
    for file in markdown_files(root)? {
        let text = read(&file)?;
        if text
            .contains("This section contains concrete artifact content tied to the requested root")
            || text.contains("scaffold_only_content")
        {
            return Err(format!("mock content in {}", file.display()));
        }
    }
    Ok(())
}

fn markdown_files(root: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let mut files = Vec::new();
    collect(root, &mut files)?;
    Ok(files)
}

fn collect(path: &Path, files: &mut Vec<std::path::PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(path).map_err(|error| format!("read_dir failed: {error}"))? {
        let child = entry
            .map_err(|error| format!("dir entry failed: {error}"))?
            .path();
        if child.is_dir() {
            collect(&child, files)?;
        } else if child.extension().is_some_and(|ext| ext == "md") {
            files.push(child);
        }
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

fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("{} unreadable: {error}", path.display()))
}
