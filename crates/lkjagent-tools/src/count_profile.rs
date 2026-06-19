use crate::count_profile_body::{body_text, handoff_text, main_title, sequence_text};
use crate::count_profile_data::{EN_DESIGN_FOCUSES, JP_DESIGN_FOCUSES};
use crate::count_profile_design::design_text;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DeliverableProfile {
    language: Language,
    kind: DeliverableKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Language {
    English,
    Japanese,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DeliverableKind {
    Narrative,
    Guide,
    Report,
    General,
}

impl DeliverableProfile {
    pub(crate) fn from_objective(objective: &str) -> Self {
        Self {
            language: detect_language(objective),
            kind: detect_kind(objective),
        }
    }

    pub(crate) fn root_readme(self, docs: usize, main: usize, objective: &str) -> String {
        match self.language {
            Language::Japanese => format!(
                "# 構造化成果物\n\n## 目的\n\n次の依頼に対応する複数ファイル成果物です。\n\n{objective}\n\n## 目次\n\n- [docs/](docs/): 設計、継続性、検証のための設計メモ {docs} 件。\n- [main/](main/): 順序付き本編ファイル {main} 件。\n\n## 検証\n\nこの成果物は指定ファイル数に合わせて生成されています。依頼が変わらない限り、合計ファイル数を保ってください。\n"
            ),
            Language::English => format!(
                "# Structured Output\n\n## Purpose\n\nA generated multi-file deliverable for this objective:\n\n{objective}\n\n## Table of Contents\n\n- [docs/](docs/): {docs} design files for planning, continuity, and verification.\n- [main/](main/): {main} ordered main content files.\n\n## Verification\n\nThe scaffold was generated as an exact counted deliverable. Keep the total file count stable unless the owner changes the target.\n"
            ),
        }
    }

    pub(crate) fn doc_page(
        self,
        index: usize,
        docs: usize,
        main: usize,
        objective: &str,
    ) -> String {
        let design = design_text(self.language, self.kind, index, docs, main);
        match self.language {
            Language::Japanese => format!(
                "# 設計メモ {index:03}\n\n## 焦点\n\n{}\n\n## 依頼文\n\n{objective}\n\n{design}",
                self.design_focus(index),
            ),
            Language::English => format!(
                "# Design Memo {index:03}\n\n## Focus\n\n{}\n\n## Objective Context\n\n{objective}\n\n{design}",
                self.design_focus(index),
            ),
        }
    }

    pub(crate) fn main_page(self, index: usize, total: usize, objective: &str) -> String {
        let arc = index.saturating_sub(1) / 10 + 1;
        let slot = index.saturating_sub(1) % 10 + 1;
        match self.language {
            Language::Japanese => format!(
                "# {}\n\n## 位置\n\n- 幕: {arc}\n- 節: {slot}\n\n## 連続性台帳\n\n{}\n\n## 依頼文\n\n{objective}\n\n## 本文\n\n{}\n\n## 継続メモ\n\n{}\n",
                main_title(self.language, self.kind, index),
                sequence_text(self.language, index, total),
                body_text(self.language, self.kind, index, total),
                handoff_text(self.language, index, total)
            ),
            Language::English => format!(
                "# {}\n\n## Position\n\n- Arc: {arc}\n- Segment: {slot}\n\n## Sequence Ledger\n\n{}\n\n## Objective Context\n\n{objective}\n\n## Draft Content\n\n{}\n\n## Continuity Hand-Off\n\n{}\n",
                main_title(self.language, self.kind, index),
                sequence_text(self.language, index, total),
                body_text(self.language, self.kind, index, total),
                handoff_text(self.language, index, total)
            ),
        }
    }

    fn design_focus(self, index: usize) -> &'static str {
        let slot = index.saturating_sub(1).min(11);
        let (focuses, fallback) = match self.language {
            Language::Japanese => (&JP_DESIGN_FOCUSES, "補足設計メモ"),
            Language::English => (&EN_DESIGN_FOCUSES, "supplemental planning notes"),
        };
        focuses.get(slot).copied().unwrap_or(fallback)
    }
}

fn detect_language(objective: &str) -> Language {
    if objective.chars().any(|ch| {
        ('\u{3040}'..='\u{30ff}').contains(&ch) || ('\u{4e00}'..='\u{9fff}').contains(&ch)
    }) {
        Language::Japanese
    } else {
        Language::English
    }
}

fn detect_kind(objective: &str) -> DeliverableKind {
    let lower = objective.to_lowercase();
    if contains_any(
        &lower,
        &["story", "novel", "fiction", "narrative", "物語", "小説"],
    ) {
        DeliverableKind::Narrative
    } else if contains_any(
        &lower,
        &["guide", "manual", "tutorial", "procedure", "手順", "説明書"],
    ) {
        DeliverableKind::Guide
    } else if contains_any(
        &lower,
        &["report", "analysis", "research", "調査", "分析", "報告"],
    ) {
        DeliverableKind::Report
    } else {
        DeliverableKind::General
    }
}

fn contains_any(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| haystack.contains(needle))
}
