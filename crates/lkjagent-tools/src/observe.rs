use lkjagent_protocol::{render_notice, render_observation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputKind {
    Observation { status: String },
    Notice { kind: String },
    Skill { name: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputFrame {
    pub kind: OutputKind,
    pub content: String,
    pub rendered: String,
}

pub fn ok(content: impl Into<String>, cap_tokens: usize, retrieval_path: &str) -> OutputFrame {
    observation("ok", content.into(), cap_tokens, retrieval_path)
}

pub fn error(content: impl Into<String>, cap_tokens: usize) -> OutputFrame {
    observation(
        "error",
        content.into(),
        cap_tokens,
        "retry with a narrower tool action",
    )
}

pub fn notice(kind: &str, content: impl Into<String>) -> OutputFrame {
    let content = content.into();
    OutputFrame {
        kind: OutputKind::Notice {
            kind: kind.to_string(),
        },
        rendered: render_notice(kind, &content),
        content,
    }
}

pub fn skill(name: &str, rendered: impl Into<String>) -> OutputFrame {
    let content = rendered.into();
    OutputFrame {
        kind: OutputKind::Skill {
            name: name.to_string(),
        },
        rendered: content.clone(),
        content,
    }
}

pub fn estimate_tokens(text: &str) -> usize {
    text.len().saturating_add(3) / 4
}

pub fn bounded_text(text: &str, cap_tokens: usize, retrieval_path: &str) -> String {
    if estimate_tokens(text) <= cap_tokens {
        return text.to_string();
    }
    let cap_chars = cap_tokens.saturating_mul(4);
    let notice = format!("\n... truncated middle; retrieve the rest with {retrieval_path} ...\n");
    let body_budget = cap_chars.saturating_sub(notice.len());
    if body_budget == 0 {
        return notice.chars().take(cap_chars).collect();
    }
    let chars: Vec<char> = text.chars().collect();
    let head_len = body_budget / 2;
    let tail_len = body_budget.saturating_sub(head_len);
    let mut output = String::new();
    output.extend(chars.iter().take(head_len));
    output.push_str(&notice);
    let tail: Vec<char> = chars.iter().rev().take(tail_len).copied().collect();
    output.extend(tail.iter().rev());
    output
}

fn observation(
    status: &str,
    content: String,
    cap_tokens: usize,
    retrieval_path: &str,
) -> OutputFrame {
    let content = bounded_text(&content, cap_tokens, retrieval_path);
    OutputFrame {
        kind: OutputKind::Observation {
            status: status.to_string(),
        },
        rendered: render_observation(status, &content),
        content,
    }
}
