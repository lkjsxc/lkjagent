# Shell

## Position

Shell remains available for software development tasks that dedicated tools
cannot express. It is not a normal personal-record or retrieval tool.

## Admission

Require:

- a project or repository operation state;
- explicit shell-capable persisted view;
- workspace-bound working directory;
- bounded command length, timeout, and output;
- denied secret paths and environment exposure;
- no interactive process unless the state expects PTY;
- idempotency or an explicit non-idempotent effect class.

## Output

Store full bounded output outside normal prompt context. Return exit status,
duration, truncated safe excerpt, and artifact ref. Failed output is
recovery-only.

## Preference

If a native workspace, Git, or verification tool can express the action, hide
shell and persist that reason in the tool view.
