use std::path::Path;

use crate::error::{DxError, DxResult};

use super::native_android_build_environment::{
    AndroidBuildEnvironment, android_build_environment_step, check_android_build_environment,
};
use super::native_android_build_gradle::patch_android_gradle_helper;
use super::native_android_build_plan::{
    AndroidBuildPlan, build_android_build_plan, relative_project_path,
};
use super::native_android_build_preflight::{
    AndroidToolchainPreflight, android_toolchain_preflight_step, check_android_toolchain_preflight,
};
use super::native_android_build_process::{
    android_build_command, run_android_command, tauri_android_init_command, tauri_icon_command,
};
use super::native_android_build_receipt::{
    AndroidBuildStepReceipt, build_android_receipt, inspect_android_artifact, write_android_receipt,
};
use super::native_shell_materializer::ensure_www_native_shell_for_build;

pub(super) fn cmd_www_build_android(project_root: &Path, invoked_as: &'static str) -> DxResult<()> {
    let plan = build_android_build_plan(project_root)?;
    let mut steps = Vec::new();

    let preflight = check_android_toolchain_preflight()?;
    steps.push(android_toolchain_preflight_step(&preflight));
    let environment = check_android_build_environment(&plan.project_root)?;
    steps.push(android_build_environment_step(&plan, &environment));

    eprintln!("Building DX WWW Android debug APK...");

    ensure_www_native_shell_for_build(&plan.project_root)?;
    steps.push(AndroidBuildStepReceipt {
        name: "native_shell",
        status: "ready",
        command: Some("dx www native-shell --target tauri --write".to_string()),
        details: Some("mobile-compatible Tauri shell is present".to_string()),
    });

    steps.push(ensure_android_icons(&plan, &preflight, &environment)?);
    steps.push(ensure_android_project(&plan, &preflight, &environment)?);
    steps.push(patch_android_gradle_helper(&plan.project_root)?);

    let build_command = android_build_command(preflight.tauri_program());
    run_android_command(&plan.project_root, &build_command, &environment)?;
    steps.push(AndroidBuildStepReceipt {
        name: "apk_build",
        status: "passed",
        command: Some(build_command.display()),
        details: Some("arm64 debug APK built through DX Native/Tauri".to_string()),
    });

    let artifact = inspect_android_artifact(&plan)?;
    let receipt = build_android_receipt(&plan, invoked_as, steps, artifact, &environment)?;
    write_android_receipt(&plan.receipt_path, &receipt)?;

    eprintln!(
        "Android APK built: {}",
        relative_project_path(&plan.project_root, &plan.apk_path)
    );
    eprintln!(
        "Android receipt: {}",
        relative_project_path(&plan.project_root, &plan.receipt_path)
    );

    Ok(())
}

fn ensure_android_icons(
    plan: &AndroidBuildPlan,
    preflight: &AndroidToolchainPreflight,
    environment: &AndroidBuildEnvironment,
) -> DxResult<AndroidBuildStepReceipt> {
    if plan.icons_dir.join("icon.png").is_file() {
        return Ok(AndroidBuildStepReceipt {
            name: "android_icons",
            status: "present",
            command: None,
            details: Some("src-tauri/icons/icon.png already exists".to_string()),
        });
    }

    let icon_source = plan
        .icon_source
        .as_ref()
        .ok_or_else(|| DxError::ConfigValidationError {
            message: "Android build requires src-tauri/icons/icon.png or a source icon at public/icon.svg, public/logo.svg, public/icon.png, or public/logo.png".to_string(),
            field: Some("build.target.android.icons".to_string()),
        })?;
    let command = tauri_icon_command(
        preflight.tauri_program(),
        relative_project_path(&plan.project_root, icon_source),
    );

    run_android_command(&plan.project_root, &command, environment)?;

    Ok(AndroidBuildStepReceipt {
        name: "android_icons",
        status: "generated",
        command: Some(command.display()),
        details: Some("mobile icons generated from project source icon".to_string()),
    })
}

fn ensure_android_project(
    plan: &AndroidBuildPlan,
    preflight: &AndroidToolchainPreflight,
    environment: &AndroidBuildEnvironment,
) -> DxResult<AndroidBuildStepReceipt> {
    if plan.android_app_gradle.is_file() {
        return Ok(AndroidBuildStepReceipt {
            name: "android_project",
            status: "present",
            command: None,
            details: Some(relative_project_path(
                &plan.project_root,
                &plan.android_project_root,
            )),
        });
    }

    let command = tauri_android_init_command(preflight.tauri_program());
    run_android_command(&plan.project_root, &command, environment)?;

    Ok(AndroidBuildStepReceipt {
        name: "android_project",
        status: "generated",
        command: Some(command.display()),
        details: Some(relative_project_path(
            &plan.project_root,
            &plan.android_project_root,
        )),
    })
}
