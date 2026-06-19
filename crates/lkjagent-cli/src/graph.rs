use std::path::Path;

use crate::error::CliError;
use crate::store::open_store;

pub fn graph(data_dir: &Path) -> Result<String, CliError> {
    let conn = open_store(data_dir)?;
    let graph = lkjagent_graph::source_graph();
    let report = lkjagent_graph::validate_graph(graph);
    let active = match lkjagent_store::graph::active_case(&conn)? {
        Some(case) => format!(
            "active_case={}\nfamily={}\nphase={}\nnode={}\nstatus={}",
            case.id, case.family, case.phase, case.active_node, case.status
        ),
        None => "active_case=none".to_string(),
    };
    Ok(format!(
        "source_nodes={}\nsource_edges={}\nsource_valid={}\n{}",
        graph.nodes.len(),
        graph.edges.len(),
        report.is_ok(),
        active
    ))
}
