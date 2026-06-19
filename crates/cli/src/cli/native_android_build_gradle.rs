use std::path::Path;

use crate::error::{DxError, DxResult};

use super::native_android_build_plan::relative_project_path;
use super::native_android_build_receipt::AndroidBuildStepReceipt;

const GRADLE_BUILD_TASK_ROOT: &str = "src-tauri/gen/android/buildSrc/src/main/java";

pub(super) fn patch_android_gradle_helper(
    project_root: &Path,
) -> DxResult<AndroidBuildStepReceipt> {
    let helper_root = project_root.join(GRADLE_BUILD_TASK_ROOT);
    if !helper_root.is_dir() {
        return Ok(AndroidBuildStepReceipt {
            name: "windows_gradle_helper",
            status: "not_found",
            command: None,
            details: Some("Tauri Android helper was not generated".to_string()),
        });
    }

    let mut inspected = 0usize;
    let mut patched = Vec::new();
    for entry in walkdir::WalkDir::new(&helper_root) {
        let entry = entry.map_err(|error| DxError::IoError {
            path: None,
            message: error.to_string(),
        })?;
        if !entry.file_type().is_file() || entry.file_name().to_string_lossy() != "BuildTask.kt" {
            continue;
        }
        inspected += 1;
        let path = entry.path();
        let source = std::fs::read_to_string(path).map_err(|error| DxError::IoError {
            path: Some(path.to_path_buf()),
            message: error.to_string(),
        })?;
        let checked_source = if let Some(updated) = patch_gradle_helper_source(&source) {
            std::fs::write(path, updated).map_err(|error| DxError::IoError {
                path: Some(path.to_path_buf()),
                message: error.to_string(),
            })?;
            patched.push(relative_project_path(project_root, path));
            std::fs::read_to_string(path).map_err(|error| DxError::IoError {
                path: Some(path.to_path_buf()),
                message: error.to_string(),
            })?
        } else {
            source
        };
        if gradle_helper_has_forbidden_node_wrapper(&checked_source) {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "Tauri Android BuildTask still invokes the node wrapper after patching: {}",
                    relative_project_path(project_root, path)
                ),
                field: Some("build.target.android.gradle_helper".to_string()),
            });
        }
    }

    let (status, details) = if !patched.is_empty() {
        ("patched", patched.join(", "))
    } else if inspected > 0 {
        (
            "ready",
            format!("inspected {inspected} BuildTask.kt file(s)"),
        )
    } else {
        ("not_found", "BuildTask.kt was not present".to_string())
    };

    Ok(AndroidBuildStepReceipt {
        name: "windows_gradle_helper",
        status,
        command: None,
        details: Some(details),
    })
}

fn patch_gradle_helper_source(source: &str) -> Option<String> {
    let mut updated = source.to_string();
    updated = updated.replace(
        r#"val executable = """node""""#,
        r#"val executable = """tauri""""#,
    );
    updated = updated.replace(
        r#"val args = listOf("tauri", "android", "android-studio-script")"#,
        r#"val args = listOf("android", "android-studio-script")"#,
    );

    if updated == source {
        None
    } else {
        Some(updated)
    }
}

fn gradle_helper_has_forbidden_node_wrapper(source: &str) -> bool {
    source.contains(r#""""node""""#)
        || source.contains(r#""tauri", "android", "android-studio-script""#)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patch_gradle_helper_source_removes_node_wrapper() {
        let source = r#"
open class BuildTask : DefaultTask() {
    fun runTauriCli() {
        val executable = """node""";
        val args = listOf("tauri", "android", "android-studio-script");
    }
}
"#;

        let patched = patch_gradle_helper_source(source).expect("patched helper");

        assert!(patched.contains(r#"val executable = """tauri""""#));
        assert!(patched.contains(r#"val args = listOf("android", "android-studio-script")"#));
        assert!(!patched.contains(r#"listOf("tauri", "android", "android-studio-script")"#));
        assert!(!gradle_helper_has_forbidden_node_wrapper(&patched));
    }

    #[test]
    fn detects_unpatched_node_wrapper_markers() {
        assert!(gradle_helper_has_forbidden_node_wrapper(
            r#"val executable = """node""""#
        ));
        assert!(gradle_helper_has_forbidden_node_wrapper(
            r#"val args = listOf("tauri", "android", "android-studio-script")"#
        ));
    }
}
