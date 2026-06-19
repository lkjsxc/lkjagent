use crate::count_profile_anchor::{anchor_block, anchor_for_part};
use crate::count_profile_body::{body_text, handoff_text, main_title, sequence_text};
use crate::count_profile_data::{EN_DESIGN_FOCUSES, JP_DESIGN_FOCUSES};
use crate::count_profile_design::design_text;
use crate::count_profile_index::{docs_map, file_budget, main_map};

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

    pub(crate) fn root_readme(
        self,
        docs: usize,
        main: usize,
        index_files: usize,
        objective: &str,
    ) -> String {
        let anchors = anchor_block(self.language, objective);
        let budget = file_budget(self.language, docs, main, index_files);
        match self.language {
            Language::Japanese => format!(
                "# 構造化成果物\n\n## 目的\n\n次の依頼に対応する複数ファイル成果物です。\n\n{objective}\n\n{anchors}\n## 目次\n\n- [docs/](docs/): 設計、継続性、検証のための設計メモ {docs} 件。\n- [main/](main/): 順序付き本編ファイル {main} 件。\n\n{budget}\n## 検証\n\nこの成果物は指定ファイル数に合わせて生成されています。依頼が変わらない限り、合計ファイル数を保ってください。\n"
            ),
            Language::English => format!(
                "# Structured Output\n\n## Purpose\n\nA generated multi-file deliverable for this objective:\n\n{objective}\n\n{anchors}\n## Table of Contents\n\n- [docs/](docs/): {docs} design files for planning, continuity, and verification.\n- [main/](main/): {main} ordered main content files.\n\n{budget}\n## Verification\n\nThe scaffold was generated as an exact counted deliverable. Keep the total file count stable unless the owner changes the target.\n"
            ),
        }
    }

    pub(crate) fn docs_readme(self, docs: usize, main: usize, objective: &str) -> String {
        let anchors = anchor_block(self.language, objective);
        let map = docs_map(self.language, docs, main);
        match self.language {
            Language::Japanese => format!(
                "# docs\n\n## 目的\n\n設計、継続性、検証条件を整理する索引です。\n\n{anchors}\n## 構成\n\n- 設計メモ数: {docs}\n- 対象本編数: {main}\n- 各設計メモは担当範囲、設計タスク、検証観点を持ちます。\n\n{map}\n## 依頼文\n\n{objective}\n"
            ),
            Language::English => format!(
                "# docs\n\n## Purpose\n\nIndex for design, continuity, and verification conditions.\n\n{anchors}\n## Structure\n\n- Design memo count: {docs}\n- Covered main files: {main}\n- Each design memo carries coverage, design work, and verification checks.\n\n{map}\n## Objective Context\n\n{objective}\n"
            ),
        }
    }

    pub(crate) fn main_readme(self, main: usize, objective: &str) -> String {
        let anchors = anchor_block(self.language, objective);
        let map = main_map(self.language, main);
        match self.language {
            Language::Japanese => format!(
                "# main\n\n## 目的\n\n順序付き本編ファイルの索引です。\n\n{anchors}\n## 構成\n\n- 本編ファイル数: {main}\n- 各本編は位置、連続性台帳、本文、継続メモを持ちます。\n- 読む順序は part-001.md から番号順です。\n\n{map}\n## 依頼文\n\n{objective}\n"
            ),
            Language::English => format!(
                "# main\n\n## Purpose\n\nIndex for ordered main content files.\n\n{anchors}\n## Structure\n\n- Main file count: {main}\n- Each main file carries position, sequence ledger, body, and handoff notes.\n- Read in numeric order from part-001.md onward.\n\n{map}\n## Objective Context\n\n{objective}\n"
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
        let anchors = anchor_block(self.language, objective);
        match self.language {
            Language::Japanese => format!(
                "# 設計メモ {index:03}\n\n## 焦点\n\n{}\n\n## 依頼文\n\n{objective}\n\n{anchors}\n{design}",
                self.design_focus(index),
            ),
            Language::English => format!(
                "# Design Memo {index:03}\n\n## Focus\n\n{}\n\n## Objective Context\n\n{objective}\n\n{anchors}\n{design}",
                self.design_focus(index),
            ),
        }
    }

    pub(crate) fn main_page(self, index: usize, total: usize, objective: &str) -> String {
        let arc = index.saturating_sub(1) / 10 + 1;
        let slot = index.saturating_sub(1) % 10 + 1;
        let anchors = anchor_block(self.language, objective);
        let part_anchor = anchor_for_part(self.language, objective, index);
        match self.language {
            Language::Japanese => format!(
                "# {}\n\n## 位置\n\n- 幕: {arc}\n- 節: {slot}\n\n## 連続性台帳\n\n{}\n\n## 依頼文\n\n{objective}\n\n{anchors}\n## 本文\n\n{}\n\n## 継続メモ\n\n{}\n",
                main_title(self.language, self.kind, index),
                sequence_text(self.language, index, total),
                body_text(self.language, self.kind, index, total, &part_anchor),
                handoff_text(self.language, index, total)
            ),
            Language::English => format!(
                "# {}\n\n## Position\n\n- Arc: {arc}\n- Segment: {slot}\n\n## Sequence Ledger\n\n{}\n\n## Objective Context\n\n{objective}\n\n{anchors}\n## Draft Content\n\n{}\n\n## Continuity Hand-Off\n\n{}\n",
                main_title(self.language, self.kind, index),
                sequence_text(self.language, index, total),
                body_text(self.language, self.kind, index, total, &part_anchor),
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
