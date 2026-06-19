use std::fs;
use std::path::Path;

use crate::count_guard::CountGuard;
use crate::count_profile::DeliverableProfile;
use crate::count_seed_allocation::allocation_for;
use crate::count_seed_verify::verify_scaffold;
use crate::error::{ToolError, ToolResult};

pub fn scaffold_counted_documents(
    workspace: &Path,
    guard: CountGuard,
    objective: &str,
) -> ToolResult<String> {
    if guard.target == 0 {
        return Err(ToolError::invalid(
            "counted document scaffold needs at least 1 file",
        ));
    }
    let root = workspace.join("structured-output");
    if root.exists() {
        fs::remove_dir_all(&root)?;
    }
    fs::create_dir_all(root.join("docs"))?;
    fs::create_dir_all(root.join("main"))?;
    let objective = objective_summary(objective);
    let allocation = allocation_for(guard.target, &objective);
    let profile = DeliverableProfile::from_objective(&objective);
    write_file(
        &root.join("README.md"),
        &profile.root_readme(
            allocation.docs,
            allocation.main,
            allocation.index_files(),
            guard.mode,
            &objective,
        ),
    )?;
    if allocation.indexes {
        write_file(
            &root.join("docs/README.md"),
            &profile.docs_readme(allocation.docs, allocation.main, &objective),
        )?;
        write_file(
            &root.join("main/README.md"),
            &profile.main_readme(allocation.docs, allocation.main, &objective),
        )?;
    }
    for index in 1..=allocation.docs {
        write_file(
            &root.join(format!("docs/design-{index:03}.md")),
            &profile.doc_page(index, allocation.docs, allocation.main, &objective),
        )?;
    }
    for index in 1..=allocation.main {
        write_file(
            &root.join(format!("main/part-{index:03}.md")),
            &profile.main_page(index, allocation.docs, allocation.main, &objective),
        )?;
    }
    let check = verify_scaffold(
        &root,
        guard.target,
        allocation.docs,
        allocation.main,
        allocation.indexes,
    )?;
    Ok(format!(
        "counted document scaffold root=structured-output\nfiles={}\nindex_files={}\ndesign_memos={}\nmain_files={}\nroot_index=ok\nfile_budget={}\naudit_manifest={}\ndocs_index={}\ncoverage_map={}\nmain_index={}\nacceptance_audit={}\nrestart_guide={}\npart_ledger={}\nindex_scope={}\nsection_scope=all\ncontent_blocks={}\ndesign_sections={}\nmain_sections={}\ndesign_owner_links={}\nlocal_verification={}\nreading_path={}\nsequence_paths={}\nfirst_main={}\nlast_main={}\nverification=ok\nclosure_reason=deterministic_scaffold\ncompletion=ready",
        check.files,
        check.index_files,
        allocation.docs,
        allocation.main,
        check.file_budget,
        check.audit_manifest,
        check.docs_index,
        check.coverage_map,
        check.main_index,
        check.acceptance_audit,
        check.restart_guide,
        check.part_ledger,
        check.index_scope,
        check.content_blocks,
        check.design_sections,
        check.main_sections,
        check.design_owner_links,
        check.local_verification,
        check.reading_path,
        check.sequence_paths,
        check.first_main,
        check.last_main
    ))
}

fn objective_summary(objective: &str) -> String {
    let trimmed = objective.trim();
    if trimmed.is_empty() {
        return "No explicit objective was provided.".to_string();
    }
    trimmed.chars().take(400).collect()
}

fn write_file(path: &Path, content: &str) -> ToolResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}
