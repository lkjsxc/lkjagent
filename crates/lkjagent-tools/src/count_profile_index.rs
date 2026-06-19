use crate::count_profile::Language;

pub(crate) fn file_budget(
    language: Language,
    docs: usize,
    main: usize,
    index_files: usize,
) -> String {
    let total = 1_usize
        .saturating_add(index_files)
        .saturating_add(docs)
        .saturating_add(main);
    match language {
        Language::Japanese => format!(
            "## ファイル内訳\n\n- ルート索引: 1\n- ディレクトリ索引: {index_files}\n- 設計メモ: {docs}\n- 本編ファイル: {main}\n- 合計ファイル数: {total}\n"
        ),
        Language::English => format!(
            "## File Budget\n\n- Root index: 1\n- Directory indexes: {index_files}\n- Design memos: {docs}\n- Main files: {main}\n- Total files: {total}\n"
        ),
    }
}

pub(crate) fn main_map(language: Language, total: usize) -> String {
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
    match language {
        Language::Japanese => format!("## 進行地図\n\n{lines}\n"),
        Language::English => format!("## Progress Map\n\n{lines}\n"),
    }
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
    let label = match language {
        Language::Japanese => JP_STAGES[slot],
        Language::English => EN_STAGES[slot],
    };
    Some(match language {
        Language::Japanese if start == end => {
            format!("- {label}: main/part-{start:03}.md")
        }
        Language::Japanese => {
            format!("- {label}: main/part-{start:03}.md から main/part-{end:03}.md")
        }
        Language::English if start == end => {
            format!("- {label}: main/part-{start:03}.md")
        }
        Language::English => {
            format!("- {label}: main/part-{start:03}.md through main/part-{end:03}.md")
        }
    })
}

fn docs_map_line(language: Language, index: usize, docs: usize, main: usize) -> String {
    let file = format!("design-{index:03}.md");
    let Some((start, end)) = coverage_range(index, docs, main) else {
        return match language {
            Language::Japanese => format!("- {file}: 全体構成のみ"),
            Language::English => format!("- {file}: overall structure only"),
        };
    };
    match language {
        Language::Japanese if start == end => {
            format!("- {file}: main/part-{start:03}.md")
        }
        Language::Japanese => {
            format!("- {file}: main/part-{start:03}.md から main/part-{end:03}.md")
        }
        Language::English if start == end => {
            format!("- {file}: main/part-{start:03}.md")
        }
        Language::English => {
            format!("- {file}: main/part-{start:03}.md through main/part-{end:03}.md")
        }
    }
}

fn stage_range(total: usize, slot: usize) -> Option<(usize, usize)> {
    let start = slot.saturating_mul(total) / 6 + 1;
    let end = (slot.saturating_add(1)).saturating_mul(total) / 6;
    (start <= end).then_some((start, end))
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

const JP_STAGES: [&str; 6] = ["導入", "探索", "対立拡大", "中盤反転", "危機", "収束"];
const EN_STAGES: [&str; 6] = [
    "opening",
    "exploration",
    "rising conflict",
    "midpoint reversal",
    "crisis",
    "resolution",
];
