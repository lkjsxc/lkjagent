use super::CompletionGuard;
use crate::count_guard::count_target;

pub(super) fn classify(content: &str) -> CompletionGuard {
    let lower = content.to_ascii_lowercase();
    let count = count_target(&lower, content);
    let recursive =
        lower.contains("recursive") || content.contains("再帰") || content.contains("再起");
    let structure = lower.contains("structure")
        || lower.contains("structured")
        || lower.contains("organize")
        || content.contains("構造");
    let base = if knowledge_request(&lower, content)
        && (recursive || structure || creation_request(&lower, content))
    {
        CompletionGuard::RecursiveKnowledge
    } else if recursive && structure {
        CompletionGuard::RecursiveStructure
    } else {
        CompletionGuard::None
    };
    if let Some(count) = count {
        base.with_count(count)
    } else {
        base
    }
}

pub(super) fn merge_guard(current: CompletionGuard, next: CompletionGuard) -> CompletionGuard {
    let count = next.count_guard().or_else(|| current.count_guard());
    let current_base = current.structural_base();
    let next_base = next.structural_base();
    let base = if guard_rank(next_base) > guard_rank(current_base) {
        next_base
    } else {
        current_base
    };
    if let Some(count) = count {
        base.with_count(count)
    } else if base == CompletionGuard::None {
        next
    } else {
        base
    }
}

fn guard_rank(guard: CompletionGuard) -> u8 {
    match guard {
        CompletionGuard::None => 0,
        CompletionGuard::FileCount { .. } | CompletionGuard::MarkdownCount { .. } => 1,
        CompletionGuard::RecursiveStructure | CompletionGuard::RecursiveStructureCount { .. } => 2,
        CompletionGuard::RecursiveKnowledge | CompletionGuard::RecursiveKnowledgeCount { .. } => 3,
    }
}

fn knowledge_request(lower: &str, content: &str) -> bool {
    lower.contains("encyclopedia")
        || lower.contains("knowledge base")
        || lower.contains("knowledge")
        || lower.contains("wiki")
        || content.contains("百科事典")
        || content.contains("知識")
}

fn creation_request(lower: &str, content: &str) -> bool {
    lower.contains("build")
        || lower.contains("create")
        || lower.contains("make")
        || lower.contains("write")
        || lower.contains("generate")
        || lower.contains("docs")
        || lower.contains("documentation")
        || content.contains("作")
        || content.contains("生成")
        || content.contains("構築")
}
