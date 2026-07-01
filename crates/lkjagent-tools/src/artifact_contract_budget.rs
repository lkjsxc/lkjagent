use std::collections::{BTreeMap, BTreeSet};

use lkjagent_store::artifact_graph::ContractRow;
use rusqlite::Connection;

use crate::error::{ToolError, ToolResult};
use crate::fs_batch::BatchFileInfo;

pub fn validate_active_contracts(conn: &Connection, files: &[BatchFileInfo]) -> ToolResult<()> {
    let contracts = lkjagent_store::artifact_graph::active_contracts(conn)?;
    let roots = artifact_roots(conn)?;
    let mut matched: BTreeMap<String, Vec<&BatchFileInfo>> = BTreeMap::new();
    for file in files {
        let contract_id = matching_contract(&contracts, &file.path);
        match contract_id {
            Some(id) => matched.entry(id).or_default().push(file),
            None if contracts.is_empty() && in_any_root(&file.path, &roots) => {
                return Err(ToolError::invalid(format!(
                    "fs.batch_write has no active artifact contract for artifact path: {}",
                    file.path
                )));
            }
            None if !contracts.is_empty() => {
                return Err(ToolError::invalid(format!(
                    "fs.batch_write path outside active artifact contract: {}",
                    file.path
                )));
            }
            None => {}
        }
    }
    for contract in contracts {
        if let Some(files) = matched.get(&contract.contract_id) {
            validate_budget(&contract, files)?;
        }
    }
    Ok(())
}

fn matching_contract(contracts: &[ContractRow], path: &str) -> Option<String> {
    contracts.iter().find_map(|contract| {
        let exact = split(&contract.exact_paths);
        if exact.contains(path) {
            Some(contract.contract_id.clone())
        } else {
            None
        }
    })
}

fn validate_budget(contract: &ContractRow, files: &[&BatchFileInfo]) -> ToolResult<()> {
    let max_files = limit(contract.max_files, "max_files")?;
    let max_file_bytes = limit(contract.max_file_bytes, "max_file_bytes")?;
    let max_batch_bytes = limit(contract.max_batch_bytes, "max_batch_bytes")?;
    if files.len() > max_files {
        return Err(ToolError::invalid(format!(
            "fs.batch_write exceeds active artifact contract max_files: contract={} count={} max={max_files}",
            contract.contract_id,
            files.len()
        )));
    }
    let mut total = 0usize;
    for file in files {
        if file.bytes > max_file_bytes {
            return Err(ToolError::invalid(format!(
                "fs.batch_write exceeds active artifact contract max_file_bytes: contract={} path={} bytes={} max={max_file_bytes}",
                contract.contract_id, file.path, file.bytes
            )));
        }
        total = total.saturating_add(file.bytes);
        if total > max_batch_bytes {
            return Err(ToolError::invalid(format!(
                "fs.batch_write exceeds active artifact contract max_batch_bytes: contract={} bytes={total} max={max_batch_bytes}",
                contract.contract_id
            )));
        }
    }
    Ok(())
}

fn artifact_roots(conn: &Connection) -> ToolResult<Vec<String>> {
    let mut statement = conn
        .prepare("SELECT DISTINCT root FROM artifact_plans ORDER BY root")
        .map_err(|error| ToolError::Store(error.to_string()))?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|error| ToolError::Store(error.to_string()))?;
    let mut roots = Vec::new();
    for row in rows {
        roots.push(row.map_err(|error| ToolError::Store(error.to_string()))?);
    }
    Ok(roots)
}

fn limit(value: i64, field: &str) -> ToolResult<usize> {
    usize::try_from(value).map_err(|_| {
        ToolError::invalid(format!(
            "active artifact contract {field} is outside the supported range"
        ))
    })
}

fn in_any_root(path: &str, roots: &[String]) -> bool {
    roots.iter().any(|root| in_root(path, root))
}

fn in_root(path: &str, root: &str) -> bool {
    let root = root.trim_end_matches('/');
    path == root || path.starts_with(&format!("{root}/"))
}

fn split(values: &str) -> BTreeSet<&str> {
    values.lines().filter(|value| !value.is_empty()).collect()
}
