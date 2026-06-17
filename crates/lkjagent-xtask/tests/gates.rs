use lkjagent_xtask::doc_common::check_markdown_basics;
use lkjagent_xtask::doc_special::check_special_docs;
use lkjagent_xtask::doc_topology::check_doc_topology;
use lkjagent_xtask::lines::check_lines;
use lkjagent_xtask::model::{RepoFile, Violation};
use lkjagent_xtask::style::check_style;

fn messages(violations: Vec<Violation>) -> Vec<String> {
    violations
        .into_iter()
        .map(|violation| violation.message())
        .collect()
}

#[test]
fn markdown_basics_report_exact_messages() {
    let long_line = "a".repeat(121);
    let files = vec![
        RepoFile::new("docs/h1.md", "Title\n\n## Purpose\n"),
        RepoFile::new("docs/purpose.md", "# Title\n\ntext\n"),
        RepoFile::new("docs/ascii.md", "# Title\n\n## Purpose\n\ncafe\u{301}\n"),
        RepoFile::new(
            "docs/width.md",
            format!("# Title\n\n## Purpose\n\n{long_line}\n"),
        ),
        RepoFile::new(
            "docs/table.md",
            "# Title\n\n## Purpose\n\n| a | b | c | d | e | f | g |\n",
        ),
        RepoFile::new(
            "docs/banned.md",
            "# Title\n\n## Purpose\n\nThis version is stale.\n",
        ),
        RepoFile::new(
            "docs/release.md",
            "# Title\n\n## Purpose\n\nThis v2 name is stale.\n",
        ),
    ];

    assert_eq!(
        messages(check_markdown_basics(&files)),
        vec![
            "docs/h1.md: doc shape: first line must be an H1 beginning with '# '",
            "docs/purpose.md: doc shape: second nonempty line must be '## Purpose'",
            "docs/ascii.md: ascii: replace non-ASCII characters",
            "docs/width.md: prose width: line 5 exceeds 120 characters",
            "docs/table.md: table width: line 5 has 7 columns; split the table",
            "docs/banned.md: banned token: line 5 contains 'version'; state the current contract directly",
            "docs/release.md: banned token: line 5 contains 'v2'; state the current contract directly",
        ]
    );
}

#[test]
fn doc_topology_reports_missing_readme_thin_toc_and_manifest() {
    let files = vec![
        RepoFile::new(
            "docs/README.md",
            "# Docs\n\n## Purpose\n\nmap\n\n## Table of Contents\n\n- [a.md](a.md): a.\n\n## All Files\n\n- `a.md`\n",
        ),
        RepoFile::new("docs/a.md", "# A\n\n## Purpose\n\na\n"),
        RepoFile::new("docs/b.md", "# B\n\n## Purpose\n\nb\n"),
        RepoFile::new("docs/thin/README.md", "# Thin\n\n## Purpose\n\nthin\n\n## Table of Contents\n"),
        RepoFile::new("docs/thin/only.md", "# Only\n\n## Purpose\n\nonly\n"),
        RepoFile::new("docs/missing/file.md", "# File\n\n## Purpose\n\nfile\n"),
    ];

    assert_eq!(
        messages(check_doc_topology(&files)),
        vec![
            "docs/README.md: readme topology: link child 'b.md' from the table of contents",
            "docs/README.md: readme topology: link child 'missing' from the table of contents",
            "docs/README.md: readme topology: link child 'thin' from the table of contents",
            "docs/missing: readme topology: directory must contain README.md",
            "docs/thin: readme topology: directory must contain at least two children beside README.md",
            "docs/thin/README.md: readme topology: link child 'only.md' from the table of contents",
            "docs/README.md: all files: list 'b.md' in the All Files manifest",
            "docs/README.md: all files: list 'missing/file.md' in the All Files manifest",
            "docs/README.md: all files: list 'thin/README.md' in the All Files manifest",
            "docs/README.md: all files: list 'thin/only.md' in the All Files manifest",
        ]
    );
}

#[test]
fn special_docs_report_skill_task_and_crate_readme_violations() {
    let files = vec![
        RepoFile::new(
            "docs/agent/skills/foo.md",
            "# Skill: Foo\n\n## Purpose\n\nx\n",
        ),
        RepoFile::new("docs/execution/tasks/foo.md", "# Foo\n\n## Purpose\n\nx\n"),
        RepoFile::new("crates/lkjagent-demo/src/lib.rs", "//! demo\n"),
        RepoFile::new(
            "crates/lkjagent-bad/README.md",
            "# Bad\n\n## Purpose\n\nbad\n",
        ),
        RepoFile::new(
            "crates/lkjagent-bad/src/README.md",
            "# Src\n\n## Purpose\n\nsrc\n",
        ),
    ];

    assert_eq!(
        messages(check_special_docs(&files)),
        vec![
            "docs/agent/skills/foo.md: skill shape: headings must be Purpose, Trigger, Context, Procedure, Checks, Must Not, optional Handoff",
            "docs/agent/skills/foo.md: skill shape: filename must be a kebab-case skill name",
            "docs/execution/tasks/foo.md: task shape: headings must match the task template",
            "crates/lkjagent-bad/README.md: crate readme: name the Doc contract",
            "crates/lkjagent-bad/README.md: crate readme: add a Table of Contents",
            "crates/lkjagent-bad/src/README.md: crate readme: add a Table of Contents",
            "crates/lkjagent-demo: crate readme: add README.md for this crate directory",
            "crates/lkjagent-demo/src: crate readme: add README.md for this crate directory",
        ]
    );
}

#[test]
fn line_check_reports_normal_and_skill_limits() {
    let files = vec![
        RepoFile::new("README.md", "x\n".repeat(201)),
        RepoFile::new(
            "docs/agent/skills/demo.md",
            format!("# Skill: Demo\n{}", "x\n".repeat(120)),
        ),
    ];

    assert_eq!(
        messages(check_lines(&files)),
        vec![
            "README.md: line limit: has 201 lines, limit is 200; split by ownership",
            "docs/agent/skills/demo.md: line limit: has 121 lines, limit is 120; split by ownership",
        ]
    );
}

#[test]
fn style_check_reports_panic_paths_and_unapproved_dependencies() {
    let files = vec![
        RepoFile::new(
            "crates/lkjagent-cli/src/main.rs",
            "fn main() { panic!(\"x\"); }\n",
        ),
        RepoFile::new(
            "crates/lkjagent-cli/Cargo.toml",
            "[dependencies]\nanyhow = \"1\"\n",
        ),
        RepoFile::new(
            "crates/lkjagent-xtask/src/main.rs",
            "fn main() { panic!(\"x\"); }\n",
        ),
    ];

    assert_eq!(
        messages(check_style(&files)),
        vec![
            "crates/lkjagent-cli/src/main.rs: panic path: line 1 contains 'panic!'; return an error value instead",
            "crates/lkjagent-cli/Cargo.toml: dependency allowlist: dependency 'anyhow' is not documented as allowed",
        ]
    );
}
