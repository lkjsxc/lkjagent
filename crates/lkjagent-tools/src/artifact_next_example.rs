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
        "# {title}\n\n## Scene Content\n\nA named protagonist enters {title} with a concrete want, an obstacle, sensory pressure, and a visible consequence. The scene names the location, the object under dispute, the decision made on page, and the cost that carries into the following beat.\n\n## Continuity Notes\n\nContinuity notes track intent, unresolved tension, changed relationships, setting details, and revision evidence for the manuscript.\n"
    )
}

fn generic_content(title: &str) -> String {
    format!(
        "# {title}\n\n## Concrete Record\n\n{title} defines the artifact role, the observed constraints, the file paths it governs, and the decision that makes the page useful for lookup. It gives named examples, operational checks, and verification evidence tied to this exact path.\n\n## Examples And Checks\n\nExample one names a path, an invariant, and the command or audit that proves it. Example two names a failure mode, the repair owner, and the evidence needed before completion.\n"
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
