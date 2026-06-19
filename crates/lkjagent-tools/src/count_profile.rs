use crate::count_guard::CountMode;
use crate::count_profile_anchor::{anchor_block, anchor_for_part};
use crate::count_profile_audit::acceptance_audit;
use crate::count_profile_body::{body_text, handoff_text, main_title, sequence_text};
use crate::count_profile_data::{EN_DESIGN_FOCUSES, JP_DESIGN_FOCUSES};
use crate::count_profile_design::design_text;
use crate::count_profile_index::{design_owner, docs_map, file_budget, main_map};
use crate::count_profile_kind::detect_kind;
use crate::count_profile_local::local_verification;
use crate::count_profile_manifest::audit_manifest;
use crate::count_profile_restart::restart_guide;
use crate::count_profile_thread::segment_brief;

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
        mode: CountMode,
        objective: &str,
    ) -> String {
        let anchors = anchor_block(self.language, objective);
        let budget = file_budget(self.language, docs, main, index_files);
        let manifest = audit_manifest(self.language, docs, main, index_files);
        let audit = acceptance_audit(self.language, self.kind, docs, main);
        let restart = restart_guide(self.language, docs, main, index_files);
        let verification = self.verification_text(mode);
        match self.language {
            Language::Japanese => format!(
                "# 構造化成果物\n\n## 目的\n\n次の依頼に対応する複数ファイル成果物です。\n\n{objective}\n\n{anchors}\n## 目次\n\n- [docs/](docs/): 設計、継続性、検証のための設計メモ {docs} 件。\n- [main/](main/): 順序付き本編ファイル {main} 件。\n\n{budget}\n{manifest}\n{audit}\n{restart}\n## 検証\n\n{verification}\n"
            ),
            Language::English => format!(
                "# Structured Output\n\n## Purpose\n\nA generated multi-file deliverable for this objective:\n\n{objective}\n\n{anchors}\n## Table of Contents\n\n- [docs/](docs/): {docs} design files for planning, continuity, and verification.\n- [main/](main/): {main} ordered main content files.\n\n{budget}\n{manifest}\n{audit}\n{restart}\n## Verification\n\n{verification}\n"
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

    pub(crate) fn main_readme(self, docs: usize, main: usize, objective: &str) -> String {
        let anchors = anchor_block(self.language, objective);
        let map = main_map(self.language, self.kind, docs, main);
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

    pub(crate) fn main_page(
        self,
        index: usize,
        docs: usize,
        total: usize,
        objective: &str,
    ) -> String {
        let arc = index.saturating_sub(1) / 10 + 1;
        let slot = index.saturating_sub(1) % 10 + 1;
        let anchors = anchor_block(self.language, objective);
        let part_anchor = anchor_for_part(self.language, objective, index);
        let brief = segment_brief(self.language, self.kind, index, total, &part_anchor);
        let local = local_verification(self.language);
        let owner = self.design_owner_line(index, docs, total);
        match self.language {
            Language::Japanese => format!(
                "# {}\n\n{brief}## 位置\n\n- 幕: {arc}\n- 節: {slot}\n{owner}\n\n## 連続性台帳\n\n{}\n\n## 依頼文\n\n{objective}\n\n{anchors}\n## 本文\n\n{}\n\n{local}## 継続メモ\n\n{}\n",
                main_title(self.language, self.kind, index),
                sequence_text(self.language, index, total),
                body_text(self.language, self.kind, index, total, &part_anchor),
                handoff_text(self.language, index, total)
            ),
            Language::English => format!(
                "# {}\n\n{brief}## Position\n\n- Arc: {arc}\n- Segment: {slot}\n{owner}\n\n## Sequence Ledger\n\n{}\n\n## Objective Context\n\n{objective}\n\n{anchors}\n## Draft Content\n\n{}\n\n{local}## Continuity Hand-Off\n\n{}\n",
                main_title(self.language, self.kind, index),
                sequence_text(self.language, index, total),
                body_text(self.language, self.kind, index, total, &part_anchor),
                handoff_text(self.language, index, total)
            ),
        }
    }

    fn design_owner_line(self, index: usize, docs: usize, main: usize) -> String {
        let value = design_owner(index, docs, main)
            .map(|owner| format!("docs/design-{owner:03}.md"))
            .unwrap_or_else(|| match self.language {
                Language::Japanese => "なし".to_string(),
                Language::English => "none".to_string(),
            });
        match self.language {
            Language::Japanese => format!("- 設計担当: {value}"),
            Language::English => format!("- Design owner: {value}"),
        }
    }

    fn design_focus(self, index: usize) -> &'static str {
        let (focuses, fallback) = match self.language {
            Language::Japanese => (&JP_DESIGN_FOCUSES, "補足設計メモ"),
            Language::English => (&EN_DESIGN_FOCUSES, "supplemental planning notes"),
        };
        let slot = index.saturating_sub(1).min(focuses.len().saturating_sub(1));
        focuses.get(slot).copied().unwrap_or(fallback)
    }

    fn verification_text(self, mode: CountMode) -> &'static str {
        match (self.language, mode) {
            (Language::Japanese, CountMode::Exact) => {
                "この成果物は指定ファイル数に合わせて生成されています。依頼が変わらない限り、合計ファイル数を保ってください。"
            }
            (Language::Japanese, CountMode::Approximate) => {
                "この成果物は目標ファイル数を中心に生成されています。依頼が変わらない限り、許容範囲から外れないようにしてください。"
            }
            (Language::English, CountMode::Exact) => {
                "The scaffold was generated as an exact counted deliverable. Keep the total file count stable unless the owner changes the target."
            }
            (Language::English, CountMode::Approximate) => {
                "The scaffold was generated at the requested target within the approximate-count guard. Keep the total inside the accepted range unless the owner changes the target."
            }
        }
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
