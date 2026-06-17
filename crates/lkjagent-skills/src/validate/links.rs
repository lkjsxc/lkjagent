pub(super) fn local_link_paths(source_path: &str, text: &str) -> Vec<(String, String)> {
    markdown_links(text)
        .into_iter()
        .filter_map(|target| resolve_link(source_path, &target).map(|path| (target, path)))
        .collect()
}

fn markdown_links(text: &str) -> Vec<String> {
    let mut links = Vec::new();
    for line in text.lines() {
        collect_links_from_line(line, &mut links);
    }
    links
}

fn collect_links_from_line(line: &str, links: &mut Vec<String>) {
    let mut rest = line;
    while let Some(start) = rest.find("](") {
        let after_start = &rest[start + 2..];
        let Some(end) = after_start.find(')') else {
            break;
        };
        links.push(after_start[..end].to_string());
        rest = &after_start[end + 1..];
    }
}

fn resolve_link(source_path: &str, target: &str) -> Option<String> {
    if target.starts_with("http://") || target.starts_with("https://") || target.starts_with('#') {
        return None;
    }
    let path = target.split('#').next().unwrap_or("").trim();
    if path.is_empty() {
        return None;
    }
    let base = source_path.rsplit_once('/').map_or("", |(base, _)| base);
    Some(normalize_path(base, path))
}

fn normalize_path(base: &str, target: &str) -> String {
    let joined = if target.starts_with('/') {
        target.trim_start_matches('/').to_string()
    } else if base.is_empty() {
        target.to_string()
    } else {
        format!("{base}/{target}")
    };
    let mut parts = Vec::new();
    for part in joined.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                let _ = parts.pop();
            }
            value => parts.push(value),
        }
    }
    parts.join("/")
}
