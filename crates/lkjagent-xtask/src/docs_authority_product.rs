use std::fs;
use std::path::{Path, PathBuf};

const EXPECTED_PRODUCT_FINGERPRINT: &str = "fnv1a64:a9994de20ceca0bd";
const PRODUCT_DIRS: &[&str] = &[
    "crates/lkjagent-app",
    "crates/lkjagent-core",
    "crates/lkjagent-effects",
    "crates/lkjagent-llm",
    "crates/lkjagent-store",
    "evaluation",
];
const PRODUCT_FILES: &[&str] = &["Cargo.toml", "Dockerfile", "docker-compose.yml"];

pub(crate) fn check(root: &Path) -> Vec<String> {
    match fingerprint(root) {
        Ok(actual) if actual == EXPECTED_PRODUCT_FINGERPRINT => Vec::new(),
        Ok(actual) => vec![format!(
            "docs-authority must be behavior-identical; product fingerprint is {actual}, expected {EXPECTED_PRODUCT_FINGERPRINT}"
        )],
        Err(error) => vec![error],
    }
}

fn fingerprint(root: &Path) -> Result<String, String> {
    let mut paths = Vec::new();
    for relative in PRODUCT_DIRS {
        collect(root, &root.join(relative), &mut paths)?;
    }
    for relative in PRODUCT_FILES {
        let path = root.join(relative);
        if !path.is_file() {
            return Err(format!("product fingerprint input is missing: {relative}"));
        }
        paths.push(path);
    }
    paths.sort();
    let mut hash = 0xcbf29ce484222325_u64;
    for path in paths {
        let relative = path
            .strip_prefix(root)
            .map_err(|error| format!("{}: {error}", path.display()))?;
        update(&mut hash, relative.to_string_lossy().as_bytes());
        update(&mut hash, &[0]);
        let bytes = fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?;
        update(&mut hash, &bytes);
        update(&mut hash, &[0xff]);
    }
    Ok(format!("fnv1a64:{hash:016x}"))
}

fn collect(root: &Path, path: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    if !path.is_dir() {
        let relative = path.strip_prefix(root).unwrap_or(path);
        return Err(format!(
            "product fingerprint directory is missing: {}",
            relative.display()
        ));
    }
    for entry in fs::read_dir(path).map_err(|error| format!("{}: {error}", path.display()))? {
        let entry = entry.map_err(|error| error.to_string())?;
        let child = entry.path();
        if child.is_dir() {
            collect(root, &child, paths)?;
        } else if child.is_file() {
            paths.push(child);
        }
    }
    Ok(())
}

fn update(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x100000001b3);
    }
}

#[cfg(test)]
mod tests {
    use super::{fingerprint, EXPECTED_PRODUCT_FINGERPRINT};
    use std::path::Path;

    #[test]
    fn product_tree_matches_bound_base() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        assert_eq!(
            fingerprint(&root).expect("product fingerprint"),
            EXPECTED_PRODUCT_FINGERPRINT
        );
    }
}
