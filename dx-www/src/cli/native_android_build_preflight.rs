use std::path::{Path, PathBuf};
use std::process::Command;

use crate::error::{DxError, DxResult};

use super::native_android_build_plan::slash_path;
use super::native_android_build_process::{
    discover_android_sdk, discover_java_home, discover_latest_ndk, first_executable_in,
    latest_child_dir, valid_java_home,
};
use super::native_android_build_receipt::AndroidBuildStepReceipt;

const ANDROID_RUST_TARGET: &str = "aarch64-linux-android";

#[derive(Debug, Clone)]
pub(super) struct AndroidToolchainPreflight {
    tauri: PathBuf,
    rustup: PathBuf,
    java_home: PathBuf,
    android_sdk: PathBuf,
    android_ndk: PathBuf,
    platform_tools: PathBuf,
    build_tools: PathBuf,
    platform: PathBuf,
    does_not_mutate_project: bool,
}

impl AndroidToolchainPreflight {
    pub(super) fn tauri_program(&self) -> String {
        self.tauri.to_string_lossy().to_string()
    }

    fn summary(&self) -> String {
        format!(
            "tauri={}, rustup={}, java_home={}, android_sdk={}, android_ndk={}, platform_tools={}, build_tools={}, platform={}, rust_target={ANDROID_RUST_TARGET}, does_not_mutate_project={}",
            slash_path(&self.tauri),
            slash_path(&self.rustup),
            slash_path(&self.java_home),
            slash_path(&self.android_sdk),
            slash_path(&self.android_ndk),
            slash_path(&self.platform_tools),
            slash_path(&self.build_tools),
            slash_path(&self.platform),
            self.does_not_mutate_project,
        )
    }
}

pub(super) fn check_android_toolchain_preflight() -> DxResult<AndroidToolchainPreflight> {
    let tauri = require_tool_on_path("tauri")?;
    let rustup = require_tool_on_path("rustup")?;
    require_installed_rust_target(&rustup, ANDROID_RUST_TARGET)?;

    let java_home = discover_java_home().ok_or_else(|| {
        android_toolchain_error(
            "Android build requires a valid JDK with java and jar; set JAVA_HOME or install a JDK",
        )
    })?;
    if !valid_java_home(&java_home) {
        return Err(android_toolchain_error(format!(
            "Android build found JAVA_HOME candidate but it is missing java or jar: {}",
            java_home.display()
        )));
    }

    let android_sdk = discover_android_sdk().ok_or_else(|| {
        android_toolchain_error(
            "Android build requires Android SDK platform-tools, build-tools, and platforms; set ANDROID_HOME or ANDROID_SDK_ROOT",
        )
    })?;
    let platform_tools = require_android_sdk_dir(&android_sdk, "platform-tools")?;
    let build_tools = latest_child_dir(&require_android_sdk_dir(&android_sdk, "build-tools")?)
        .ok_or_else(|| {
            android_toolchain_error(format!(
                "Android SDK build-tools has no installed revision: {}",
                android_sdk.join("build-tools").display()
            ))
        })?;
    let platform = latest_child_dir(&require_android_sdk_dir(&android_sdk, "platforms")?)
        .ok_or_else(|| {
            android_toolchain_error(format!(
                "Android SDK platforms has no installed API platform: {}",
                android_sdk.join("platforms").display()
            ))
        })?;
    let android_ndk = discover_latest_ndk(&android_sdk).ok_or_else(|| {
        android_toolchain_error(format!(
            "Android build requires an Android NDK with source.properties under {}",
            android_sdk.join("ndk").display()
        ))
    })?;

    Ok(AndroidToolchainPreflight {
        tauri,
        rustup,
        java_home,
        android_sdk,
        android_ndk,
        platform_tools,
        build_tools,
        platform,
        does_not_mutate_project: true,
    })
}

pub(super) fn android_toolchain_preflight_step(
    preflight: &AndroidToolchainPreflight,
) -> AndroidBuildStepReceipt {
    AndroidBuildStepReceipt {
        name: "android_toolchain_preflight",
        status: "passed",
        command: None,
        details: Some(preflight.summary()),
    }
}

fn require_tool_on_path(tool: &str) -> DxResult<PathBuf> {
    std::env::var_os("PATH")
        .into_iter()
        .flat_map(|paths| std::env::split_paths(&paths).collect::<Vec<_>>())
        .find_map(|dir| first_executable_in(&dir, tool))
        .ok_or_else(|| {
            android_toolchain_error(format!(
                "Android build requires `{tool}` on PATH before project files are generated or patched"
            ))
        })
}

fn require_installed_rust_target(rustup: &Path, target: &str) -> DxResult<()> {
    let output = Command::new(rustup)
        .args(["target", "list", "--installed"])
        .output()
        .map_err(|error| {
            android_toolchain_error(format!(
                "Failed to check installed Rust targets with `{}`: {error}",
                rustup.display()
            ))
        })?;

    if !output.status.success() {
        return Err(android_toolchain_error(format!(
            "`{} target list --installed` failed with status {}",
            rustup.display(),
            output.status
        )));
    }

    let installed_targets = String::from_utf8_lossy(&output.stdout);
    if installed_targets.lines().any(|line| line.trim() == target) {
        return Ok(());
    }

    Err(android_toolchain_error(format!(
        "Android build requires Rust target `{target}`; run `rustup target add {target}` before building"
    )))
}

fn require_android_sdk_dir(android_sdk: &Path, child: &'static str) -> DxResult<PathBuf> {
    let path = android_sdk.join(child);
    if path.is_dir() {
        return Ok(path);
    }
    Err(android_toolchain_error(format!(
        "Android SDK is missing `{child}` at {}",
        path.display()
    )))
}

fn android_toolchain_error(message: impl Into<String>) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some("build.target.android.toolchain".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::android_toolchain_error;
    use crate::error::DxError;

    #[test]
    fn toolchain_errors_use_android_toolchain_field() {
        match android_toolchain_error("missing") {
            DxError::ConfigValidationError { field, .. } => {
                assert_eq!(field.as_deref(), Some("build.target.android.toolchain"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
