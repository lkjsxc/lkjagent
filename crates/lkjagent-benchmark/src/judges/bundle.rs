use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

const EXPECTED: &[&str] = &[
    "bundle/README.md",
    "bundle/alpha.md",
    "bundle/beta.md",
    "bundle/gamma.md",
];

pub fn judge(workspace: &Path) -> Result<(), String> {
    let files = markdown_files(workspace)?;
    let expected: BTreeSet<String> = EXPECTED.iter().map(|path| path.to_string()).collect();
    if files != expected {
        return Err(format!(
            "expected markdown files {expected:?}, got {files:?}"
        ));
    }
    require_links(
        workspace,
        "bundle/README.md",
        &["alpha.md", "beta.md", "gamma.md"],
    )?;
    require_links(workspace, "bundle/alpha.md", &["README.md", "beta.md"])?;
    require_links(workspace, "bundle/beta.md", &["README.md", "gamma.md"])?;
    require_links(workspace, "bundle/gamma.md", &["README.md", "alpha.md"])?;
    Ok(())
}

fn require_links(workspace: &Path, path: &str, links: &[&str]) -> Result<(), String> {
    let text = fs::read_to_string(workspace.join(path))
        .map_err(|error| format!("{path} unreadable: {error}"))?;
    for link in links {
        let marker = format!("({link})");
        if !text.contains(&marker) {
            return Err(format!("{path} missing link {link}"));
        }
    }
    Ok(())
}

fn markdown_files(workspace: &Path) -> Result<BTreeSet<String>, String> {
    let mut files = BTreeSet::new();
    visit(workspace, workspace, &mut files)?;
    Ok(files)
}

fn visit(root: &Path, path: &Path, files: &mut BTreeSet<String>) -> Result<(), String> {
    for entry in fs::read_dir(path).map_err(|error| format!("read_dir failed: {error}"))? {
        let child = entry
            .map_err(|error| format!("dir entry failed: {error}"))?
            .path();
        if child.is_dir() {
            visit(root, &child, files)?;
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
