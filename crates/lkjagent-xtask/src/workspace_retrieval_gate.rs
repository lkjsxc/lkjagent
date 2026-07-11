use std::path::Path;

use crate::node_suites::{check as check_suites, Suite};

const SUITES: &[Suite] = &[
    Suite {
        package: "lkjagent-app",
        target: "explore",
        minimum_tests: 3,
    },
    Suite {
        package: "lkjagent-app",
        target: "workspace_evidence",
        minimum_tests: 3,
    },
    Suite {
        package: "lkjagent-app",
        target: "workspace_rebalance",
        minimum_tests: 2,
    },
    Suite {
        package: "lkjagent-app",
        target: "record_wrappers",
        minimum_tests: 2,
    },
    Suite {
        package: "lkjagent-app",
        target: "cli_rows",
        minimum_tests: 4,
    },
    Suite {
        package: "lkjagent-app",
        target: "archive_compensation",
        minimum_tests: 1,
    },
    Suite {
        package: "lkjagent-store",
        target: "workspace_rows",
        minimum_tests: 1,
    },
    Suite {
        package: "lkjagent-store",
        target: "record_rows",
        minimum_tests: 1,
    },
];

pub fn check(root: &Path) -> Result<(), Vec<String>> {
    check_suites(root, "workspace-retrieval-maintenance", SUITES)
}
