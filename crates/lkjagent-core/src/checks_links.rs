use crate::checks::FileFact;
use crate::model::CheckResult;

pub(crate) fn readme_coverage(files: &[FileFact], root: &str) -> CheckResult {
    let dirs = dirs_under(files, root);
    let passed = dirs.iter().all(|dir| {
        let readme = format!("{dir}/README.md");
        let Some(fact) = files.iter().find(|fact| fact.path == readme) else {
            return false;
        };
        let links = links_from(&fact.content, dir, &dirs);
        child_paths(files, dir)
            .iter()
            .all(|child| links.contains(child))
    });
    result("readme_coverage", passed, dirs.len().to_string())
}

pub(crate) fn links_resolve(files: &[FileFact], root: &str) -> CheckResult {
    let paths = path_list(files);
    let dirs = dirs_under(files, root);
    let mut missing = 0;
    for fact in files.iter().filter(|fact| fact.path.starts_with(root)) {
        let base = fact.path.rsplit_once('/').map_or("", |(dir, _)| dir);
        for link in raw_links(&fact.content) {
            let Some(resolved) = normalize(base, &link, &dirs) else {
                continue;
            };
            if !paths.contains(&resolved) {
                missing += 1;
            }
        }
    }
    result("links_resolve", missing == 0, missing.to_string())
}

fn child_paths(files: &[FileFact], dir: &str) -> Vec<String> {
    let prefix = format!("{}/", dir.trim_end_matches('/'));
    let mut children = Vec::new();
    for fact in files.iter().filter(|fact| fact.path.starts_with(&prefix)) {
        let rest = &fact.path[prefix.len()..];
        let child = if let Some((first, _)) = rest.split_once('/') {
            format!("{prefix}{first}/README.md")
        } else if rest == "README.md" {
            continue;
        } else {
            format!("{prefix}{rest}")
        };
        if !children.contains(&child) {
            children.push(child);
        }
    }
    children
}

fn links_from(content: &str, base: &str, dirs: &[String]) -> Vec<String> {
    raw_links(content)
        .into_iter()
        .filter_map(|link| normalize(base, &link, dirs))
        .collect()
}

fn normalize(base: &str, link: &str, dirs: &[String]) -> Option<String> {
    let target = link.split('#').next().unwrap_or("").trim();
    if target.is_empty()
        || target.starts_with('#')
        || target.starts_with('/')
        || target.contains("://")
    {
        return None;
    }
    let mut parts = base
        .split('/')
        .filter(|part| !part.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    for part in target.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            value => parts.push(value.to_string()),
        }
    }
    let mut path = parts.join("/");
    if target.ends_with('/') || dirs.contains(&path) {
        path = format!("{}/README.md", path.trim_end_matches('/'));
    }
    Some(path)
}

fn dirs_under(files: &[FileFact], root: &str) -> Vec<String> {
    let mut dirs = vec![root.trim_end_matches('/').to_string()];
    for fact in files.iter().filter(|fact| fact.path.starts_with(root)) {
        let mut current = String::new();
        for part in fact
            .path
            .split('/')
            .take_while(|part| !part.ends_with(".md"))
        {
            if current.is_empty() {
                current.push_str(part);
            } else {
                current.push('/');
                current.push_str(part);
            }
            if current.starts_with(root) && !dirs.contains(&current) {
                dirs.push(current.clone());
            }
        }
    }
    dirs
}

fn raw_links(text: &str) -> Vec<String> {
    let mut links = Vec::new();
    for line in text.lines() {
        let mut rest = line;
        while let Some(start) = rest.find("](") {
            let after = &rest[start + 2..];
            let Some(end) = after.find(')') else { break };
            links.push(after[..end].to_string());
            rest = &after[end + 1..];
        }
    }
    links
}

fn path_list(files: &[FileFact]) -> Vec<String> {
    files.iter().map(|fact| fact.path.clone()).collect()
}

fn result(name: &str, passed: bool, measured: String) -> CheckResult {
    CheckResult {
        name: name.to_string(),
        params: None,
        decision_id: None,
        evidence_fingerprint: None,
        passed,
        measured,
    }
}
