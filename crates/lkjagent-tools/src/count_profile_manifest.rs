use crate::count_guard::CountMode;
use crate::count_profile::Language;

pub(crate) fn audit_manifest(
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
    let has_content = docs > 0 || main > 0;
    let index_scope = if index_files > 0 && has_content {
        "all"
    } else {
        "n/a"
    };
    let content_blocks = if has_content { "required" } else { "n/a" };
    let design_owner_links = if docs > 0 && main > 0 {
        "required"
    } else {
        "n/a"
    };
    let local_verification = if main > 0 { "required" } else { "n/a" };
    let reading_path = if main > 0 { "required" } else { "n/a" };
    let sequence_paths = if main > 0 { "required" } else { "n/a" };
    let heading = match language {
        Language::Japanese => "## 監査マニフェスト",
        Language::English => "## Audit Manifest",
    };
    let scale_line = match mode {
        CountMode::Exact => format!("- files: {total}"),
        CountMode::Approximate => format!("- scale_files: about {total}"),
    };
    format!(
        "{heading}\n\n- root: structured-output\n{scale_line}\n- index_files: {index_files}\n- design_memos: {docs}\n- main_files: {main}\n- index_scope: {index_scope}\n- section_scope: all\n- content_blocks: {content_blocks}\n- restart_guide: required\n- design_owner_links: {design_owner_links}\n- local_verification: {local_verification}\n- reading_path: {reading_path}\n- sequence_paths: {sequence_paths}\n- closure_reason: deterministic_scaffold\n- completion: ready\n"
    )
}
