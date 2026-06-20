use crate::count_guard::CountMode;
use crate::count_profile::{DeliverableKind, Language};
use crate::count_profile_paths::{design_path, main_path};
use crate::count_profile_stage::{stage_label, stage_range};
use crate::count_profile_thread::segment_role;

pub(crate) fn file_budget(
    language: Language,
    docs: usize,
    main: usize,
    index_files: usize,
    mode: CountMode,
) -> String {
    let total = 1_usize
        .saturating_add(index_files)
        .saturating_add(docs)
        .saturating_add(main);
    match (language, mode) {
        (Language::Japanese, CountMode::Approximate) => format!(
            "## 規模目安\n\n- ルート索引: 1\n- ディレクトリ索引: {index_files}\n- 設計メモ: {docs}\n- 本編ファイル: {main}\n- 目安規模: 約 {total} ファイル\n"
        ),
        (Language::English, CountMode::Approximate) => format!(
            "## Scale Plan\n\n- Root index: 1\n- Directory indexes: {index_files}\n- Design memos: {docs}\n- Main files: {main}\n- Approximate scale: about {total} files\n"
        ),
        (Language::Japanese, CountMode::Exact) => format!(
            "## ファイル内訳\n\n- ルート索引: 1\n- ディレクトリ索引: {index_files}\n- 設計メモ: {docs}\n- 本編ファイル: {main}\n- 合計ファイル数: {total}\n"
        ),
        (Language::English, CountMode::Exact) => format!(
            "## File Budget\n\n- Root index: 1\n- Directory indexes: {index_files}\n- Design memos: {docs}\n- Main files: {main}\n- Total files: {total}\n"
        ),
    }
}

pub(crate) fn main_map(
    language: Language,
    kind: DeliverableKind,
    docs: usize,
    total: usize,
) -> String {
    if total == 0 {
        return match language {
            Language::Japanese => "## 進行地図\n\n本編ファイルはありません。\n".to_string(),
            Language::English => "## Progress Map\n\nNo main files exist.\n".to_string(),
        };
    }
    let lines = (0..6)
        .filter_map(|slot| map_line(language, total, slot))
        .collect::<Vec<_>>()
        .join("\n");
    let map = match language {
        Language::Japanese => format!("## 進行地図\n\n{lines}\n"),
        Language::English => format!("## Progress Map\n\n{lines}\n"),
    };
    format!("{map}{}", part_ledger(language, kind, docs, total))
}

pub(crate) fn docs_map(language: Language, docs: usize, main: usize) -> String {
    if docs == 0 {
        return match language {
            Language::Japanese => "## 設計対応表\n\n設計メモはありません。\n".to_string(),
            Language::English => "## Coverage Map\n\nNo design memos exist.\n".to_string(),
        };
    }
    let lines = (1..=docs)
        .map(|index| docs_map_line(language, index, docs, main))
        .collect::<Vec<_>>()
        .join("\n");
    match language {
        Language::Japanese => format!("## 設計対応表\n\n{lines}\n"),
        Language::English => format!("## Coverage Map\n\n{lines}\n"),
    }
}

fn map_line(language: Language, total: usize, slot: usize) -> Option<String> {
    let (start, end) = stage_range(total, slot)?;
    let label = stage_label(language, slot);
    Some(match language {
        Language::Japanese if start == end => {
            format!("- {label}: {}", main_path(start))
        }
        Language::Japanese => {
            format!("- {label}: {} から {}", main_path(start), main_path(end))
        }
        Language::English if start == end => {
            format!("- {label}: {}", main_path(start))
        }
        Language::English => {
            format!("- {label}: {} through {}", main_path(start), main_path(end))
        }
    })
}

fn docs_map_line(language: Language, index: usize, docs: usize, main: usize) -> String {
    let file = design_path(index);
    let Some((start, end)) = coverage_range(index, docs, main) else {
        return match language {
            Language::Japanese => format!("- {file}: 全体構成のみ"),
            Language::English => format!("- {file}: overall structure only"),
        };
    };
    match language {
        Language::Japanese if start == end => {
            format!("- {file}: {}", main_path(start))
        }
        Language::Japanese => {
            format!("- {file}: {} から {}", main_path(start), main_path(end))
        }
        Language::English if start == end => {
            format!("- {file}: {}", main_path(start))
        }
        Language::English => {
            format!("- {file}: {} through {}", main_path(start), main_path(end))
        }
    }
}

fn part_ledger(language: Language, kind: DeliverableKind, docs: usize, total: usize) -> String {
    let lines = (1..=total)
        .map(|index| part_ledger_line(language, kind, docs, index, total))
        .collect::<Vec<_>>()
        .join("\n");
    match language {
        Language::Japanese => format!("\n## 本編台帳\n\n{lines}\n"),
        Language::English => format!("\n## Part Ledger\n\n{lines}\n"),
    }
}

fn part_ledger_line(
    language: Language,
    kind: DeliverableKind,
    docs: usize,
    index: usize,
    total: usize,
) -> String {
    let role = segment_role(language, kind, index, total);
    let design = match (language, design_owner(index, docs, total)) {
        (Language::Japanese, Some(owner)) => format!("設計: {}", design_path(owner)),
        (Language::Japanese, None) => "設計: なし".to_string(),
        (Language::English, Some(owner)) => format!("design: {}", design_path(owner)),
        (Language::English, None) => "design: none".to_string(),
    };
    format!("- {}: {role}; {design}", main_path(index))
}

pub(crate) fn design_owner(index: usize, docs: usize, main: usize) -> Option<usize> {
    (1..=docs).find(|doc| {
        coverage_range(*doc, docs, main)
            .map(|(start, end)| index >= start && index <= end)
            .unwrap_or(false)
    })
}

fn coverage_range(index: usize, docs: usize, main: usize) -> Option<(usize, usize)> {
    if docs == 0 || main == 0 {
        return None;
    }
    let slot = index.saturating_sub(1).min(docs.saturating_sub(1));
    let start = slot.saturating_mul(main) / docs + 1;
    let mut end = (slot.saturating_add(1)).saturating_mul(main) / docs;
    if end < start {
        end = start;
    }
    Some((start.min(main), end.min(main)))
}
