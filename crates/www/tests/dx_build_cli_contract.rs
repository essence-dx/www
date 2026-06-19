use std::fs;
use std::process::Command;

use tempfile::tempdir;

#[test]
fn build_help_is_read_only_and_describes_the_source_owned_contract() {
    let project = tempdir().expect("temp project");

    let output = Command::new(env!("CARGO_BIN_EXE_dx-www"))
        .current_dir(project.path())
        .args(["build", "--help"])
        .output()
        .expect("run dx-www build --help");

    assert!(output.status.success(), "help should exit successfully");
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.contains("dx build: Run the DX source-owned build engine"),
        "{stderr}"
    );
    assert!(stderr.contains("does not install node_modules"), "{stderr}");
    assert!(
        !project.path().join(".dx").exists(),
        "dx build --help must not create build output"
    );
}

#[test]
fn build_rejects_unknown_arguments_before_writing_output() {
    let project = tempdir().expect("temp project");
    fs::create_dir_all(project.path().join("app")).expect("app dir");
    fs::write(
        project.path().join("app/page.tsx"),
        "export default function Page() { return <main>Launch</main>; }\n",
    )
    .expect("page");

    let output = Command::new(env!("CARGO_BIN_EXE_dx-www"))
        .current_dir(project.path())
        .args(["build", "--definitely-not-supported"])
        .output()
        .expect("run dx-www build with bad arg");

    assert!(
        !output.status.success(),
        "unknown build args should fail before build execution"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.contains("Unknown option for dx build: --definitely-not-supported"),
        "{stderr}"
    );
    assert!(
        stderr.contains("dx build: Run the DX source-owned build engine"),
        "{stderr}"
    );
    assert!(
        !project.path().join(".dx").exists(),
        "unknown build args must not create build output"
    );
}
