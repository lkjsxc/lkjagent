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
        "# {title}\n\n## Purpose\n\nThis cookbook section gives concrete Japanese kitchen guidance for the requested artifact and names what the cook should prepare, observe, and verify.\n\n## Ingredients Or Concept\n\nIngredients or concepts include rice, dashi, miso, soy sauce, mirin, sake, seasonal vegetables, seafood, tofu, noodles, and garnish. The concept explains balance, texture, temperature, and how a lookup table can compare seasoning ratios, prep ranges, and serving cues.\n\n## Method Or Procedure\n\nMethod and procedure steps name preparation, cutting, soaking or rinsing, simmering, grilling, frying, steaming, plating, and serving. Timing and yield notes describe batch size, active time, resting time, and safe holding range.\n\n## Signals, Fixes, And Verification\n\nSignals to look for include gloss, aroma, doneness, broth clarity, seasoning balance, and texture. Common mistake patterns include oversalting, soggy texture, scorched sauce, and weak garnish; avoid them with corrective action that can adjust seasoning, heat, and timing. Notes and troubleshooting record verification results.\n"
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
