use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::native_android_build_disk::available_free_bytes;
use super::native_android_build_plan::{AndroidBuildPlan, relative_project_path, slash_path};
use super::native_android_build_receipt::AndroidBuildStepReceipt;

pub(super) const ANDROID_BUILD_MIN_FREE_BYTES: u64 = 6 * 1024 * 1024 * 1024;
pub(super) const GRADLE_USER_HOME_ENV: &str = "GRADLE_USER_HOME";
pub(super) const ANDROID_USER_HOME_ENV: &str = "ANDROID_USER_HOME";
pub(super) const ANDROID_AVD_HOME_ENV: &str = "ANDROID_AVD_HOME";
pub(super) const CARGO_HOME_ENV: &str = "CARGO_HOME";
pub(super) const CARGO_TARGET_DIR_ENV: &str = "CARGO_TARGET_DIR";
pub(super) const TEMP_ENV: &str = "TEMP";
pub(super) const TMP_ENV: &str = "TMP";

#[derive(Debug, Clone)]
pub(super) struct AndroidBuildEnvironment {
    pub(super) cache_root: PathBuf,
    pub(super) gradle_user_home: PathBuf,
    pub(super) android_user_home: PathBuf,
    pub(super) android_avd_home: PathBuf,
    pub(super) cargo_home: PathBuf,
    pub(super) cargo_target_dir: PathBuf,
    pub(super) temp_dir: PathBuf,
    pub(super) required_free_bytes: u64,
    pub(super) available_free_bytes: Option<u64>,
    pub(super) free_space_status: &'static str,
    pub(super) writable: bool,
}

pub(super) fn check_android_build_environment(
    project_root: &Path,
) -> DxResult<AndroidBuildEnvironment> {
    let cache_root = project_root.join(".dx/native/android-cache");
    let environment = AndroidBuildEnvironment {
        gradle_user_home: cache_root.join("gradle"),
        android_user_home: cache_root.join("android-user"),
        android_avd_home: cache_root.join("avd"),
        cargo_home: cache_root.join("cargo-home"),
        temp_dir: cache_root.join("temp"),
        cargo_target_dir: project_root.join(".dx/native/android-target"),
        required_free_bytes: ANDROID_BUILD_MIN_FREE_BYTES,
        available_free_bytes: None,
        free_space_status: "not-measured",
        writable: false,
        cache_root,
    };

    prepare_android_environment(&environment)?;
    let available_free_bytes = available_free_bytes(&environment.cargo_target_dir)
        .or_else(|| available_free_bytes(&environment.cache_root));
    validate_android_build_free_space(available_free_bytes, environment.required_free_bytes)?;

    Ok(AndroidBuildEnvironment {
        available_free_bytes,
        free_space_status: free_space_status(available_free_bytes, environment.required_free_bytes),
        writable: true,
        ..environment
    })
}

pub(super) fn android_build_environment_step(
    plan: &AndroidBuildPlan,
    environment: &AndroidBuildEnvironment,
) -> AndroidBuildStepReceipt {
    AndroidBuildStepReceipt {
        name: "android_environment_preflight",
        status: environment.free_space_status,
        command: None,
        details: Some(format!(
            "cache_root={}, cargo_target_dir={}, temp_dir={}, available_free_bytes={}, required_free_bytes={}, writable={}",
            relative_project_path(&plan.project_root, &environment.cache_root),
            relative_project_path(&plan.project_root, &environment.cargo_target_dir),
            relative_project_path(&plan.project_root, &environment.temp_dir),
            environment
                .available_free_bytes
                .map(|bytes| bytes.to_string())
                .unwrap_or_else(|| "not-measured".to_string()),
            environment.required_free_bytes,
            environment.writable,
        )),
    }
}

fn prepare_android_environment(environment: &AndroidBuildEnvironment) -> DxResult<()> {
    for dir in [
        &environment.cache_root,
        &environment.gradle_user_home,
        &environment.android_user_home,
        &environment.android_avd_home,
        &environment.cargo_home,
        &environment.temp_dir,
        &environment.cargo_target_dir,
    ] {
        ensure_android_environment_dir(dir)?;
    }

    verify_android_environment_writable(&environment.cache_root)?;
    verify_android_environment_writable(&environment.cargo_target_dir)
}

fn ensure_android_environment_dir(path: &Path) -> DxResult<()> {
    std::fs::create_dir_all(path).map_err(|error| DxError::BuildFailed {
        message: format!(
            "Failed to create Android build environment directory {}: {error}",
            path.display()
        ),
    })
}

fn verify_android_environment_writable(path: &Path) -> DxResult<()> {
    let probe = path.join(".dx-android-environment-write-probe");
    std::fs::write(&probe, b"dx android build environment write probe").map_err(|error| {
        DxError::BuildFailed {
            message: format!(
                "Android build environment directory is not writable: {} ({error})",
                path.display()
            ),
        }
    })?;
    std::fs::remove_file(&probe).map_err(|error| DxError::BuildFailed {
        message: format!(
            "Failed to clean Android build environment write probe {}: {error}",
            probe.display()
        ),
    })
}

fn validate_android_build_free_space(
    available_free_bytes: Option<u64>,
    required_free_bytes: u64,
) -> DxResult<()> {
    if let Some(available_free_bytes) = available_free_bytes {
        if available_free_bytes < required_free_bytes {
            return Err(DxError::BuildFailed {
                message: format!(
                    "Android build environment has insufficient disk space: available_free_bytes={available_free_bytes}, required_free_bytes={required_free_bytes}. Free space on the project drive or move the project to a drive with more room before rerunning the Android build command (`dx www build --target android` or `dx build --target android`)."
                ),
            });
        }
    }
    Ok(())
}

fn free_space_status(available_free_bytes: Option<u64>, required_free_bytes: u64) -> &'static str {
    match available_free_bytes {
        Some(available_free_bytes) if available_free_bytes >= required_free_bytes => "ready",
        Some(_) => "low-disk-space",
        None => "not-measured",
    }
}

pub(super) fn environment_path_for_receipt(project_root: &Path, path: &Path) -> String {
    if path.starts_with(project_root) {
        relative_project_path(project_root, path)
    } else {
        slash_path(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn environment_uses_project_local_android_build_paths() {
        let dir = tempdir().expect("tempdir");
        let environment = check_android_build_environment(dir.path()).expect("android environment");

        assert_eq!(
            environment_path_for_receipt(dir.path(), &environment.cache_root),
            ".dx/native/android-cache"
        );
        assert_eq!(
            environment_path_for_receipt(dir.path(), &environment.cargo_target_dir),
            ".dx/native/android-target"
        );
        assert!(environment.gradle_user_home.is_dir());
        assert!(environment.cargo_home.is_dir());
        assert!(environment.temp_dir.is_dir());
        assert!(environment.writable);
    }

    #[test]
    fn free_space_status_classifies_measured_and_unmeasured_space() {
        assert_eq!(free_space_status(Some(10), 5), "ready");
        assert_eq!(free_space_status(Some(4), 5), "low-disk-space");
        assert_eq!(free_space_status(None, 5), "not-measured");
    }

    #[test]
    fn low_disk_space_is_a_build_failure() {
        let error =
            validate_android_build_free_space(Some(4), 5).expect_err("low disk should fail");

        match error {
            DxError::BuildFailed { message } => {
                assert!(message.contains("insufficient disk space"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
