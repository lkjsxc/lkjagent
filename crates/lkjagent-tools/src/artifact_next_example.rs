pub fn batch_write(root: &str, kind: &str, paths: &[String]) -> String {
    let files = paths
        .iter()
        .map(|path| {
            format!(
                "path: {root}/{path}\ncontent:\n{}",
                content_for(root, kind, path)
            )
        })
        .collect::<Vec<_>>()
        .join("\n-- lkjagent-next-file --\n");
    format!("<act>\n<tool>fs.batch_write</tool>\n<files>\n{files}\n</files>\n</act>")
}

fn content_for(root: &str, kind: &str, path: &str) -> String {
    let title = title_from_path(path);
    match kind.to_ascii_lowercase().as_str() {
        "cookbook" => cookbook_content(&title),
        "story" => story_content(&title),
        _ => documentation_content(root, kind, path, &title),
    }
}

fn cookbook_content(title: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\nThis cookbook page gives concrete Japanese kitchen guidance for the requested artifact and names what the cook prepares, observes, and verifies.\n\n## Ingredients Or Concept\n\nIngredients or concepts include rice, dashi, miso, soy sauce, mirin, sake, seasonal vegetables, seafood, tofu, noodles, and garnish. The concept explains balance, texture, temperature, and how lookup tables compare seasoning ratios, prep ranges, and serving cues.\n\n## Method Or Procedure\n\nMethod and procedure steps name preparation, cutting, soaking or rinsing, simmering, grilling, frying, steaming, plating, and serving. Timing and yield notes describe batch size, active time, resting time, and safe holding range.\n\n## Signals, Fixes, And Verification\n\nSignals to look for include gloss, aroma, doneness, broth clarity, seasoning balance, and texture. Common mistake patterns include oversalting, soggy texture, scorched sauce, and weak garnish; avoid them with corrective action that can adjust seasoning, heat, and timing. Notes and troubleshooting record verification results.\n"
    )
}

fn story_content(title: &str) -> String {
    format!(
        "# {title}\n\n## Scene Content\n\nA named protagonist enters {title} with a concrete want, an obstacle, sensory pressure, and a visible consequence. The scene names the location, the object under dispute, the decision made on page, and the cost that carries into the following beat.\n\n## Continuity Notes\n\nContinuity notes track intent, unresolved tension, changed relationships, setting details, and revision evidence for the manuscript.\n"
    )
}

fn documentation_content(root: &str, kind: &str, path: &str, title: &str) -> String {
    format!(
        "# {title}\n\n## Purpose\n\nThis page is the repair target for `{root}/{path}` in the active `{kind}` artifact.\n\n## Source Boundary\n\n- Artifact root: `{root}`.\n- Target path: `{path}`.\n- Current example supplies only local artifact metadata and must be replaced with owner terms or sourced facts when those are available.\n\n## Required Evidence\n\n- Link this page to the local README or relation page that owns its role.\n- Name the source path, command result, owner term, observed fact, or decision that supports the content.\n- Run `artifact.audit` after writing so readiness comes from the audit ledger.\n"
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
