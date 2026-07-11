use std::path::Path;

use crate::node_suites::{check as check_suites, Suite};

const SUITES: &[Suite] = &[
    Suite {
        package: "lkjagent-core",
        target: "parse_contract",
        minimum_tests: 6,
    },
    Suite {
        package: "lkjagent-core",
        target: "parse_diagnosis",
        minimum_tests: 2,
    },
    Suite {
        package: "lkjagent-core",
        target: "tool_call",
        minimum_tests: 7,
    },
    Suite {
        package: "lkjagent-core",
        target: "tool_call_edges",
        minimum_tests: 2,
    },
    Suite {
        package: "lkjagent-core",
        target: "admission",
        minimum_tests: 4,
    },
    Suite {
        package: "lkjagent-app",
        target: "admission_rejection",
        minimum_tests: 1,
    },
    Suite {
        package: "lkjagent-app",
        target: "native_append_effect",
        minimum_tests: 1,
    },
    Suite {
        package: "lkjagent-app",
        target: "recovery_ladder",
        minimum_tests: 3,
    },
    Suite {
        package: "lkjagent-app",
        target: "tool_views",
        minimum_tests: 1,
    },
];

pub fn check(root: &Path) -> Result<(), Vec<String>> {
    check_suites(root, "protocol-tools", SUITES)
}
