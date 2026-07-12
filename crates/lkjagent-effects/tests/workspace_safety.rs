#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use lkjagent_effects::workspace::{
    OpenedWorkspace, LIST_ENTRIES, READ_LINES, SEARCH_HITS, WORKSPACE_BYTES,
};

type TestResult = Result<(), Box<dyn std::error::Error>>;
static NEXT: AtomicU64 = AtomicU64::new(1);

#[test]
fn workspace_safety_root_listing_pages_and_search_are_deterministic() -> TestResult {
    let root = fixture("bounds")?;
    fs::create_dir(root.join("many"))?;
    for index in (0..LIST_ENTRIES + 7).rev() {
        fs::write(root.join(format!("many/file-{index:03}")), "needle\n")?;
    }
    let workspace = OpenedWorkspace::open(&root)?;
    let root_page = workspace.list_directory(".", 0, 20)?;
    assert_eq!(root_page.entries[0].name, "many");

    let first = workspace.list_directory("many", 0, LIST_ENTRIES)?;
    assert_eq!(first.entries.len(), LIST_ENTRIES);
    assert_eq!(first.next_offset, Some(LIST_ENTRIES));
    assert!(first.truncated);
    assert_eq!(first.entries[0].name, "file-000");
    assert_eq!(first.entries[LIST_ENTRIES - 1].name, "file-199");
    let second = workspace.list_directory("many", LIST_ENTRIES, LIST_ENTRIES)?;
    assert_eq!(second.entries.len(), 7);
    assert_eq!(second.entries[0].name, "file-200");
    assert_eq!(second.next_offset, None);
    assert!(!second.truncated);

    let search = workspace.search_text(".", "needle")?;
    assert_eq!(search.hits.len(), SEARCH_HITS);
    assert!(search.truncated);
    assert!(search
        .hits
        .windows(2)
        .all(|pair| pair[0].path <= pair[1].path));
    assert!(search.hits[0].path.starts_with("many/"));
    Ok(())
}

#[test]
fn workspace_safety_read_pages_preserve_bytes_and_revision() -> TestResult {
    let root = fixture("read")?;
    fs::write(root.join("abc"), "abc")?;
    fs::write(root.join("empty"), "")?;
    fs::write(root.join("lines"), "alpha\nβeta\n\n")?;
    fs::write(root.join("invalid"), [0xff, 0xfe])?;
    let workspace = OpenedWorkspace::open(&root)?;

    let abc = workspace.read_file("abc", 1, READ_LINES)?;
    assert_eq!(
        abc.revision,
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(abc.revision.len(), 64);
    assert!(abc
        .revision
        .bytes()
        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)));
    let empty = workspace.read_file("empty", 1, 1)?;
    assert_eq!(empty.total_lines, 0);
    assert!(empty.lines.is_empty());

    let first = workspace.read_file("lines", 1, 2)?;
    let actual = first
        .lines
        .iter()
        .map(|line| (line.number, line.text.as_str()))
        .collect::<Vec<_>>();
    assert_eq!(actual, [(1, "alpha"), (2, "βeta")]);
    assert_eq!(first.total_lines, 3);
    assert_eq!(first.next_line, Some(3));
    assert!(first.truncated && first.final_newline);
    let last = workspace.read_file("lines", 3, 1)?;
    assert_eq!(last.lines[0].text, "");
    assert_eq!(last.next_line, None);
    assert!(!last.truncated);
    assert!(workspace.read_file("invalid", 1, 1).is_err());
    assert!(workspace.read_file("abc", 0, 1).is_err());
    assert!(workspace.read_file("abc", 1, READ_LINES + 1).is_err());
    Ok(())
}

#[test]
fn workspace_safety_rejects_content_and_directory_bounds() -> TestResult {
    let root = fixture("content-bounds")?;
    fs::write(root.join("large"), vec![b'x'; WORKSPACE_BYTES + 1])?;
    fs::create_dir(root.join("too-many"))?;
    for index in 0..=500 {
        fs::write(root.join(format!("too-many/{index:03}")), "x")?;
    }
    let workspace = OpenedWorkspace::open(&root)?;
    assert!(workspace.read_file("large", 1, 1).is_err());
    assert!(workspace.search_text("large", "x")?.truncated);
    assert!(workspace.list_directory("too-many", 0, 20).is_err());
    Ok(())
}

#[test]
fn workspace_safety_invalid_utf8_names_are_rejected() -> TestResult {
    let root = fixture("utf8")?;
    fs::create_dir(root.join("sub"))?;
    fs::write(root.join("sub").join(OsString::from_vec(vec![0xff])), "x")?;
    let workspace = OpenedWorkspace::open(&root)?;
    assert!(workspace.list_directory("sub", 0, 20).is_err());
    assert!(workspace.search_text("sub", "x").is_err());
    Ok(())
}

fn fixture(name: &str) -> Result<PathBuf, std::io::Error> {
    let id = NEXT.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "lkjagent-workspace-{name}-{}-{id}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}
