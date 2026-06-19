use crate::count_profile::Language;

pub(crate) fn reading_path(language: Language, main: usize) -> String {
    if main == 0 {
        return match language {
            Language::Japanese => "## 読む順序\n\n本編ファイルはありません。\n".to_string(),
            Language::English => "## Reading Path\n\nNo main files exist.\n".to_string(),
        };
    }
    match language {
        Language::Japanese => format!(
            "## 読む順序\n\n- 最初の本編: main/part-001.md\n- 最後の本編: main/part-{main:03}.md\n- 読む順序: main/README.md の本編台帳に沿って番号順に進みます。\n"
        ),
        Language::English => format!(
            "## Reading Path\n\n- First main: main/part-001.md\n- Last main: main/part-{main:03}.md\n- Read order: follow main/README.md Part Ledger in numeric order.\n"
        ),
    }
}
