type R<T> = Result<T, String>;
const WORD_LIMIT: u32 = 20_000;

pub(crate) enum Shape {
    Short {
        slug: String,
    },
    Map {
        slug: String,
        children: Vec<String>,
        minimum_words: u32,
    },
    Child {
        slug: String,
        unit: String,
    },
}

pub(crate) fn parse(args: &[(String, String)]) -> R<Shape> {
    let get = |name: &str| {
        args.iter()
            .find(|item| item.0 == name)
            .map(|item| item.1.trim())
    };
    match (
        get("slug"),
        get("unit"),
        get("children"),
        get("minimum_words"),
    ) {
        (None, None, None, None) => Ok(Shape::Short {
            slug: crate::memory_record::semantic_slug(get("title").ok_or("record title missing")?)
                .ok_or("report title cannot produce a semantic slug")?,
        }),
        (Some(slug), Some("index"), Some(children), Some(words)) => Ok(Shape::Map {
            slug: part(slug).ok_or("report slug is not canonical")?,
            children: parse_children(children)?,
            minimum_words: parse_words(words)?,
        }),
        (Some(slug), Some(unit), None, None) if unit != "index" => Ok(Shape::Child {
            slug: part(slug).ok_or("report slug is not canonical")?,
            unit: unit_part(unit).ok_or("report unit is not canonical")?,
        }),
        _ => Err("report shape is not admitted".into()),
    }
}

pub(crate) fn short(slug: &str, sources: &[(String, String)], title: &str, body: &str) -> String {
    format!(
        "---\nkind: report\nsemantic-key: {slug}\nslug: {slug}\nsource-lineage:\n{}\n---\n# {title}\n\n{}\n",
        sources.iter().map(line).collect::<Vec<_>>().join("\n"),
        body.trim()
    )
}

pub(crate) fn map(
    slug: &str,
    sources: &[(String, String)],
    title: &str,
    body: &str,
    children: &[String],
    minimum_words: u32,
) -> String {
    let rows = children
        .iter()
        .map(|unit| format!("- {unit}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "---\nkind: report\nsemantic-key: {slug}\nslug: {slug}\nunit: index\nminimum-words: {minimum_words}\nchildren:\n{rows}\nsource-lineage:\n{}\n---\n# {title}\n\n{}\n\n## Sections\n{}\n",
        sources.iter().map(line).collect::<Vec<_>>().join("\n"),
        body.trim(),
        children.iter().map(|unit| format!("- [{unit}]({unit}.md)")).collect::<Vec<_>>().join("\n")
    )
}

pub(crate) fn child(
    slug: &str,
    unit: &str,
    sources: &[(String, String)],
    title: &str,
    body: &str,
) -> String {
    format!(
        "---\nkind: report\nsemantic-key: {slug}\nslug: {slug}\nunit: {unit}\nsource-lineage:\n{}\n---\n# {title}\n\n{}\n",
        sources.iter().map(line).collect::<Vec<_>>().join("\n"),
        body.trim()
    )
}

fn parse_children(value: &str) -> R<Vec<String>> {
    let mut out = Vec::new();
    for child in value.split(',').map(str::trim) {
        let child = unit_part(child).ok_or("report children are not canonical")?;
        if out.contains(&child) {
            return Err("report children must be unique".into());
        }
        out.push(child);
    }
    (out.len() >= 2)
        .then_some(out)
        .ok_or("report map requires at least two children".into())
}

fn parse_words(value: &str) -> R<u32> {
    let words = value
        .parse::<u32>()
        .map_err(|_| "minimum_words is invalid")?;
    ((1..=WORD_LIMIT).contains(&words))
        .then_some(words)
        .ok_or("minimum_words is out of bounds".into())
}

fn line((kind, fingerprint): &(String, String)) -> String {
    format!("- {kind}:{fingerprint}")
}

fn part(value: &str) -> Option<String> {
    canonical(value).filter(|value| value != "index")
}

fn unit_part(value: &str) -> Option<String> {
    canonical(value).filter(|value| value != "index")
}

fn canonical(value: &str) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()
        && value.len() <= 80
        && value.bytes().any(|byte| byte.is_ascii_lowercase())
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.as_bytes().windows(2).any(|pair| pair == b"--")
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value
            .strip_prefix("part-")
            .is_some_and(|tail| !tail.is_empty() && tail.bytes().all(|byte| byte.is_ascii_digit())))
    .then(|| value.to_string())
}
