use lkjagent_effects::workspace::OpenedWorkspace;
use lkjagent_effects::workspace_edit::ObservedTarget;

pub(crate) struct AbsentObservation {
    pub(crate) body: String,
    pub(crate) context: String,
}

pub(crate) fn observe(
    workspace: &OpenedWorkspace,
    objective: &[u8],
    path: &str,
) -> Option<AbsentObservation> {
    if !String::from_utf8_lossy(objective).contains(path) {
        return None;
    }
    let absent = match workspace.observe_edit_target(path) {
        Ok(ObservedTarget::Absent) => true,
        Ok(ObservedTarget::Present(_)) => false,
        Err(_) => workspace
            .prepare_absent_edit(path.into(), "absence-observation", 0o644)
            .is_ok(),
    };
    absent.then(|| AbsentObservation {
        body: serde_json::json!({"path":path,"revision":"absent","absent":true}).to_string(),
        context: format!("observed absent target: {path}"),
    })
}

#[cfg(test)]
mod tests {
    use super::observe;
    use lkjagent_effects::workspace::OpenedWorkspace;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn only_safe_absent_targets_become_observations() -> Result<(), Box<dyn std::error::Error>> {
        let root =
            std::env::temp_dir().join(format!("lkjagent-absent-read-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("notes"))?;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))?;
        let workspace = OpenedWorkspace::open(&root)?;
        let absent = observe(&workspace, b"create notes/new.txt", "notes/new.txt")
            .ok_or("absent target missing")?;
        assert!(absent.body.contains("\"absent\":true"));
        assert_eq!(absent.context, "observed absent target: notes/new.txt");
        assert!(observe(
            &workspace,
            b"create nested/deeper/new.txt",
            "nested/deeper/new.txt"
        )
        .is_some());
        fs::write(root.join("notes/current.txt"), "current")?;
        assert!(observe(&workspace, b"read notes/current.txt", "notes/current.txt").is_none());
        assert!(observe(&workspace, b"create elsewhere", "notes/new.txt").is_none());
        assert!(observe(&workspace, b"create ../escape", "../escape").is_none());
        fs::remove_dir_all(root)?;
        Ok(())
    }
}
