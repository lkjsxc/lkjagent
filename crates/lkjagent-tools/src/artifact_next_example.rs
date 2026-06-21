pub fn batch_write(root: &str, kind: &str, paths: &[String]) -> String {
    let files = paths
        .iter()
        .map(|path| format!("path: {root}/{path}\ncontent:\n{}", content_for(kind, path)))
        .collect::<Vec<_>>()
        .join("\n-- lkjagent-next-file --\n");
    format!("<act>\n<tool>fs.batch_write</tool>\n<files>\n{files}\n</files>\n</act>")
}

fn content_for(kind: &str, path: &str) -> String {
    let title = title_from_path(path);
    match kind.to_ascii_lowercase().as_str() {
        "cookbook" => cookbook_content(&title),
        "story" => story_content(&title),
        _ => generic_content(&title),
    }
}

fn cookbook_content(title: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\nThis bread cookbook section gives concrete kitchen guidance for the requested artifact and names what the cook should prepare, observe, and verify.\n\n## Ingredients Or Concept\n\nIngredients include flour, water, salt, yeast, and optional oil. The concept explains hydration, gluten development, fermentation range, dough temperature, and how a lookup table can compare flour strength, water range, and bake temperature.\n\n## Method Or Procedure\n\nMethod and procedure steps: mix, rest, knead or fold, proof, shape, score, and bake. Timing and yield notes describe batch size, rise time, oven range, and cooling time.\n\n## Signals, Fixes, And Verification\n\nSignals to look for include a domed dough surface, bubbles, elastic stretch, and a hollow crust sound. Common mistake patterns include underproofing, weak shaping, and excess flour; avoid them with corrective action that can correct texture, fix temperature, and adjust bake range. Notes and troubleshooting record verification results.\n"
    )
}

fn story_content(title: &str) -> String {
    format!(
        "# {title}\n\n## Scene Content\n\nThis section contains concrete scene material with named characters, setting texture, conflict, consequence, and sensory detail. It records what changes in the scene and why the next section follows from that decision.\n\n## Continuity Notes\n\nContinuity notes track intent, unresolved tension, and revision evidence for the manuscript.\n"
    )
}

fn generic_content(title: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\nThis section contains concrete artifact content tied to the requested root.\n\n## Details\n\nThe body names facts, decisions, examples, and verification notes so audit can inspect more than headings or status prose.\n"
    )
}

fn title_from_path(path: &str) -> String {
    let stem = match path
        .rsplit('/')
        .next()
        .and_then(|name| name.strip_suffix(".md"))
    {
        Some(stem) => stem,
        None => path,
    };
    stem.split('-')
        .filter(|part| !part.is_empty())
        .map(capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}

fn capitalize(part: &str) -> String {
    let mut chars = part.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
}
