use crate::artifact_objective_parse as parse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectiveFrame {
    pub raw_text: String,
    pub normalized_title: String,
    pub artifact_kind: String,
    pub root: String,
    pub requested_paths: Vec<String>,
    pub measurement_kind: String,
    pub requested_total: usize,
    pub accepted_floor: usize,
    pub section_count: usize,
    pub audience: String,
    pub language_hint: String,
    pub forbidden_roots: Vec<String>,
    pub evidence_requirements: Vec<String>,
}

pub fn from_plan_inputs(
    root: &str,
    title: &str,
    kind: &str,
    scale: &str,
    sections: &str,
) -> ObjectiveFrame {
    let raw_text = [title, kind, scale, sections]
        .into_iter()
        .filter(|part| !part.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let mut frame = from_owner_text(&raw_text);
    frame.root = parse::clean_root(root).unwrap_or_else(|| selected_root(&frame));
    if !kind.trim().is_empty() {
        frame.artifact_kind = normalize_kind(kind, &raw_text, &frame.root);
    }
    if !title.trim().is_empty() {
        frame.normalized_title = parse::normalize_title(title);
    }
    apply_scale(&mut frame, scale);
    if let Some(count) = parse::first_number(sections) {
        frame.section_count = count;
    }
    frame.measurement_kind = measurement_for(&frame.artifact_kind, &raw_text);
    frame.accepted_floor = accepted_floor(&frame);
    frame.forbidden_roots = forbidden_roots(&frame.root, &frame.requested_paths);
    frame.evidence_requirements = parse::evidence_requirements(&frame.artifact_kind);
    frame
}

pub fn from_owner_text(text: &str) -> ObjectiveFrame {
    let requested_paths = parse::requested_paths(text);
    let root = requested_paths
        .first()
        .map(|path| parse::root_from_path(path))
        .unwrap_or_else(|| inferred_root(text));
    let artifact_kind = normalize_kind("", text, &root);
    let mut frame = ObjectiveFrame {
        raw_text: text.to_string(),
        normalized_title: parse::title_from_text(text),
        artifact_kind,
        root,
        requested_paths,
        measurement_kind: "words".to_string(),
        requested_total: parse::requested_total(text),
        accepted_floor: 0,
        section_count: parse::section_count(text),
        audience: parse::audience(text),
        language_hint: parse::language_hint(text),
        forbidden_roots: Vec::new(),
        evidence_requirements: Vec::new(),
    };
    frame.measurement_kind = measurement_for(&frame.artifact_kind, text);
    frame.accepted_floor = accepted_floor(&frame);
    frame.forbidden_roots = forbidden_roots(&frame.root, &frame.requested_paths);
    frame.evidence_requirements = parse::evidence_requirements(&frame.artifact_kind);
    frame
}

pub fn root_conflict(frame: &ObjectiveFrame) -> Option<String> {
    if !parse::generic_root(&frame.root) {
        return None;
    }
    frame.requested_paths.first().map(|path| {
        format!(
            "generic root {} conflicts with requested path {path}",
            frame.root
        )
    })
}

fn apply_scale(frame: &mut ObjectiveFrame, scale: &str) {
    if let Some(count) = parse::first_number(scale) {
        frame.requested_total = count;
    } else if frame.requested_total == 0 {
        frame.requested_total = match scale.to_ascii_lowercase().as_str() {
            "large" => 10_000,
            "medium" => 3_000,
            "small" => 800,
            _ => 0,
        };
    }
}

fn normalize_kind(kind: &str, text: &str, root: &str) -> String {
    let lower = format!("{} {} {}", kind, text, root).to_ascii_lowercase();
    if lower.contains("study") || lower.contains("flashcard") || lower.contains("問題集") {
        "study-set"
    } else if lower.contains("dictionary") || lower.contains("辞書") {
        "dictionary"
    } else if lower.contains("cookbook") || lower.contains("recipe") || lower.contains("料理") {
        "cookbook"
    } else if lower.contains("documentation") || lower.contains("guide") || lower.contains("docs/")
    {
        "documentation"
    } else if lower.contains("report") || lower.contains("レポート") {
        "report"
    } else if lower.contains("novel") || lower.contains("manuscript") || lower.contains("小説") {
        "manuscript"
    } else if lower.contains("story") || lower.starts_with("stories/") {
        "story"
    } else {
        "generic"
    }
    .to_string()
}

fn measurement_for(kind: &str, text: &str) -> String {
    let lower = text.to_ascii_lowercase();
    if lower.contains("character") || lower.contains("文字") {
        "characters"
    } else if matches!(kind, "study-set") && lower.contains("card") {
        "cards"
    } else if matches!(kind, "dictionary" | "cookbook") {
        "items"
    } else {
        "words"
    }
    .to_string()
}

fn accepted_floor(frame: &ObjectiveFrame) -> usize {
    if frame.requested_total > 0 {
        return frame.requested_total;
    }
    match frame.artifact_kind.as_str() {
        "manuscript" => 10_000,
        "report" => 1_200,
        "documentation" => 900,
        "study-set" => 60,
        "dictionary" | "cookbook" => 20,
        _ => 600,
    }
}

fn inferred_root(text: &str) -> String {
    let title = parse::slug(&parse::title_from_text(text));
    let lower = text.to_ascii_lowercase();
    if lower.contains("report") {
        format!("reports/{title}")
    } else if lower.contains("cook") {
        format!("cookbooks/{title}")
    } else if lower.contains("dictionary") {
        format!("dictionaries/{title}")
    } else if lower.contains("doc") {
        format!("docs/{title}")
    } else {
        format!("artifacts/{title}")
    }
}

fn selected_root(frame: &ObjectiveFrame) -> String {
    frame
        .requested_paths
        .first()
        .map_or_else(|| frame.root.clone(), |path| parse::root_from_path(path))
}

fn forbidden_roots(root: &str, requested_paths: &[String]) -> Vec<String> {
    let mut roots = ["structured-output", "output", "artifact", "work-product"]
        .into_iter()
        .map(str::to_string)
        .collect::<Vec<_>>();
    if parse::generic_root(root) && !requested_paths.is_empty() {
        roots.push(root.to_string());
    }
    roots
}
