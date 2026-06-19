use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::error::{DxError, DxResult};

pub(super) const ANDROID_BUILD_RECEIPT_PATH: &str = ".dx/native/android-build-receipt.json";
pub(super) const ARM64_DEBUG_APK_PATH: &str =
    "src-tauri/gen/android/app/build/outputs/apk/arm64/debug/app-arm64-debug.apk";
pub(super) const ANDROID_PROJECT_PATH: &str = "src-tauri/gen/android";
pub(super) const ANDROID_APP_GRADLE_PATH: &str = "src-tauri/gen/android/app/build.gradle.kts";

#[derive(Debug, Clone)]
pub(super) struct AndroidBuildPlan {
    pub(super) project_root: PathBuf,
    pub(super) src_tauri_root: PathBuf,
    pub(super) android_project_root: PathBuf,
    pub(super) android_app_gradle: PathBuf,
    pub(super) icons_dir: PathBuf,
    pub(super) icon_source: Option<PathBuf>,
    pub(super) apk_path: PathBuf,
    pub(super) receipt_path: PathBuf,
}

pub(super) fn build_android_build_plan(project_root: &Path) -> DxResult<AndroidBuildPlan> {
    let project_root = normalize_project_root(project_root)?;
    let src_tauri_root = project_root.join("src-tauri");

    Ok(AndroidBuildPlan {
        android_project_root: project_root.join(ANDROID_PROJECT_PATH),
        android_app_gradle: project_root.join(ANDROID_APP_GRADLE_PATH),
        icons_dir: src_tauri_root.join("icons"),
        icon_source: discover_icon_source(&project_root),
        apk_path: project_root.join(ARM64_DEBUG_APK_PATH),
        receipt_path: project_root.join(ANDROID_BUILD_RECEIPT_PATH),
        src_tauri_root,
        project_root,
    })
}

pub(super) fn discover_icon_source(project_root: &Path) -> Option<PathBuf> {
    [
        "public/icon.svg",
        "public/logo.svg",
        "public/icon.png",
        "public/logo.png",
    ]
    .iter()
    .map(|relative| project_root.join(relative))
    .find(|path| path.is_file())
}

pub(super) fn read_project_name(project_root: &Path) -> DxResult<String> {
    let dx_path = project_root.join("dx");
    let dx = std::fs::read_to_string(&dx_path).map_err(|error| DxError::IoError {
        path: Some(dx_path),
        message: error.to_string(),
    })?;
    if let Some(name) = dx_project_name(&dx) {
        return Ok(name);
    }

    Ok(project_root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("dx-www-app")
        .to_string())
}

pub(super) fn read_tauri_config_value(src_tauri_root: &Path, key: &str) -> Option<String> {
    let config = std::fs::read_to_string(src_tauri_root.join("tauri.conf.json")).ok()?;
    let value: Value = serde_json::from_str(&config).ok()?;
    value.get(key)?.as_str().map(ToString::to_string)
}

pub(super) fn native_lib_name(project_name: &str) -> String {
    format!("{}_native_lib", kebab_case(project_name).replace('-', "_"))
}

pub(super) fn relative_project_path(project_root: &Path, path: &Path) -> String {
    path.strip_prefix(project_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

pub(super) fn slash_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn normalize_project_root(project_root: &Path) -> DxResult<PathBuf> {
    if !project_root.is_dir() {
        return Err(DxError::ProjectNotFound {
            path: project_root.to_path_buf(),
        });
    }

    if project_root.is_absolute() {
        Ok(project_root.to_path_buf())
    } else {
        let cwd = std::env::current_dir().map_err(|error| DxError::IoError {
            path: None,
            message: error.to_string(),
        })?;
        Ok(cwd.join(project_root))
    }
}

fn dx_project_name(dx: &str) -> Option<String> {
    if let Some(project) = dx.find("project(") {
        let after_project = &dx[project..];
        let name = after_project.find("name=\"")? + "name=\"".len();
        let after_name = &after_project[name..];
        let end = after_name.find('"')?;
        return Some(after_name[..end].to_string());
    }

    let setting = dx.find("project.name=\"")? + "project.name=\"".len();
    let after_name = &dx[setting..];
    let end = after_name.find('"')?;
    Some(after_name[..end].to_string())
}

fn kebab_case(value: &str) -> String {
    let mut output = String::new();
    let mut previous_dash = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            output.push(character.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash {
            output.push('-');
            previous_dash = true;
        }
    }
    output.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn android_build_plan_uses_dx_native_android_artifact_paths() {
        let dir = tempdir().expect("tempdir");
        std::fs::write(dir.path().join("dx"), r#"project(name="mobile-proof")"#)
            .expect("dx config");

        let plan = build_android_build_plan(dir.path()).expect("android build plan");

        assert_eq!(
            relative_project_path(&plan.project_root, &plan.android_project_root),
            "src-tauri/gen/android"
        );
        assert_eq!(
            relative_project_path(&plan.project_root, &plan.apk_path),
            ARM64_DEBUG_APK_PATH
        );
        assert_eq!(
            relative_project_path(&plan.project_root, &plan.receipt_path),
            ANDROID_BUILD_RECEIPT_PATH
        );
    }

    #[test]
    fn android_build_plan_keeps_toolchain_paths_non_verbatim() {
        let dir = tempdir().expect("tempdir");
        let plan = build_android_build_plan(dir.path()).expect("android build plan");
        let project_root = slash_path(&plan.project_root);

        assert!(
            !project_root.starts_with("//?/"),
            "Android NDK linkers do not reliably accept Windows verbatim paths: {project_root}"
        );
    }

    #[test]
    fn native_lib_name_uses_tauri_android_library_convention() {
        assert_eq!(native_lib_name("DX Mobile App"), "dx_mobile_app_native_lib");
    }

    #[test]
    fn project_name_supports_function_and_property_syntax() {
        assert_eq!(
            dx_project_name(r#"project(name="function-style")"#).as_deref(),
            Some("function-style")
        );
        assert_eq!(
            dx_project_name(r#"project.name="property-style""#).as_deref(),
            Some("property-style")
        );
    }
}
