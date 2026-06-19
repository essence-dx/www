use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

fn run_dx(project: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_dx-www"))
        .current_dir(project)
        .args(args)
        .output()
        .expect("run dx-www")
}

fn assert_dx_success(project: &Path, args: &[&str]) {
    let output = run_dx(project, args);
    assert!(
        output.status.success(),
        "dx-www {:?} failed\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
        args,
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn forge_launch_smoke_materializes_packages_and_passes_strict_checks() {
    let project = tempdir().expect("temp project");
    let root = project.path();

    assert_dx_success(root, &["add", "ui/button", "--write"]);
    assert_dx_success(root, &["add", "icon", "search", "--write"]);
    assert_dx_success(
        root,
        &[
            "check",
            ".",
            "--strict-forge",
            "--format",
            "json",
            "--fail-under",
            "80",
        ],
    );
    assert_dx_success(
        root,
        &[
            "forge",
            "doctor",
            "--project",
            ".",
            "--format",
            "json",
            "--fail-under",
            "80",
        ],
    );

    assert!(root.join("components/ui/button.tsx").exists());
    assert!(root.join("components/icons/search.tsx").exists());
    assert!(root.join(".dx/forge/source-manifest.json").exists());
    assert!(root.join(".dx/forge/docs/shadcn-ui-button.md").exists());
    assert!(root.join(".dx/forge/docs/dx-icon-search.md").exists());
    assert!(!root.join("node_modules").exists());
}
