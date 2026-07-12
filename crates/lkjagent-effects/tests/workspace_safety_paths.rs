#![cfg(unix)]

use std::fs;
use std::os::unix::{fs::symlink, net::UnixListener};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

use lkjagent_effects::workspace::OpenedWorkspace;

type TestResult = Result<(), Box<dyn std::error::Error>>;
static NEXT: AtomicU64 = AtomicU64::new(1);

#[test]
fn workspace_safety_rejects_traversal_reserved_and_symlinks() -> TestResult {
    let root = fixture("paths")?;
    fs::create_dir(root.join("dir"))?;
    fs::write(root.join("dir/file"), "safe")?;
    let outside = fixture("outside")?;
    fs::write(outside.join("file"), "secret")?;
    symlink(&outside, root.join("link-dir"))?;
    symlink(outside.join("file"), root.join("link-file"))?;
    let workspace = OpenedWorkspace::open(&root)?;

    for path in [
        "",
        "..",
        "/tmp",
        "dir/../file",
        "dir/./file",
        "dir//file",
        "dir/",
        ".lkjagent",
        ".lkjagent-state/x",
        "link-dir/file",
        "link-file",
    ] {
        assert!(workspace.read_file(path, 1, 1).is_err(), "accepted {path}");
    }
    assert!(workspace.read_file(".", 1, 1).is_err());
    assert!(workspace.list_directory("link-dir", 0, 20).is_err());
    assert!(workspace.search_text("link-dir", "secret").is_err());
    Ok(())
}

#[test]
fn workspace_safety_holds_root_and_rejects_symlink_races() -> TestResult {
    let root = fixture("root")?;
    fs::write(root.join("value"), "original")?;
    let workspace = OpenedWorkspace::open(&root)?;
    let moved = root.with_extension("moved");
    fs::rename(&root, &moved)?;
    fs::create_dir(&root)?;
    fs::write(root.join("value"), "replacement")?;
    assert_eq!(
        workspace.read_file("value", 1, 1)?.lines[0].text,
        "original"
    );
    let link = root.with_extension("link");
    symlink(&root, &link)?;
    assert!(OpenedWorkspace::open(&link).is_err());

    let race = fixture("race")?;
    let outside = fixture("race-outside")?;
    fs::write(outside.join("file"), "outside")?;
    fs::create_dir(race.join("live"))?;
    fs::write(race.join("live/file"), "inside")?;
    let workspace = OpenedWorkspace::open(&race)?;
    let stop = Arc::new(AtomicBool::new(false));
    let writer_stop = Arc::clone(&stop);
    let writer = std::thread::spawn(move || {
        while !writer_stop.load(Ordering::Relaxed) {
            let _ = fs::remove_file(race.join("live/file"));
            let _ = fs::remove_dir(race.join("live"));
            let _ = symlink(&outside, race.join("live"));
            let _ = fs::remove_file(race.join("live"));
            let _ = fs::create_dir(race.join("live"));
            let _ = fs::write(race.join("live/file"), "inside");
        }
    });
    for _ in 0..500 {
        if let Ok(page) = workspace.read_file("live/file", 1, 1) {
            if let Some(line) = page.lines.first() {
                assert_eq!(line.text, "inside");
            }
        }
    }
    stop.store(true, Ordering::Relaxed);
    writer.join().map_err(|_| "race writer panicked")?;
    Ok(())
}

#[test]
fn workspace_safety_special_files_return_promptly() -> TestResult {
    let root = fixture("special")?;
    assert!(Command::new("mkfifo")
        .arg(root.join("pipe"))
        .status()?
        .success());
    let _socket = UnixListener::bind(root.join("socket"))?;
    let workspace = OpenedWorkspace::open(&root)?;
    let start = Instant::now();
    assert!(workspace.read_file("pipe", 1, 1).is_err());
    assert!(workspace.read_file("socket", 1, 1).is_err());
    assert!(start.elapsed() < Duration::from_secs(1));
    let devices = OpenedWorkspace::open(std::path::Path::new("/dev"))?;
    assert!(devices.read_file("null", 1, 1).is_err());
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
