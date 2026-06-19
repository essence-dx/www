use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::thread::{self, JoinHandle};

use crate::error::{DxError, DxResult};

use super::native_android_build_environment::{
    ANDROID_AVD_HOME_ENV, ANDROID_USER_HOME_ENV, AndroidBuildEnvironment, CARGO_HOME_ENV,
    CARGO_TARGET_DIR_ENV, GRADLE_USER_HOME_ENV, TEMP_ENV, TMP_ENV,
};

const COMMAND_OUTPUT_TAIL_CHARS: usize = 4_000;
const ANDROID_DISK_SPACE_FAILURE_MARKERS: &[&str] = &[
    "ENOSPC",
    "os error 112",
    "not enough space",
    "No space left on device",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AndroidCommand {
    program: String,
    args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AndroidCommandProbe {
    pub(super) status: &'static str,
    pub(super) command: Option<String>,
    pub(super) stdout: String,
    pub(super) stderr: String,
}

#[derive(Debug)]
struct AndroidCommandResult {
    status: ExitStatus,
    stdout: String,
    stderr: String,
}

impl AndroidCommand {
    pub(super) fn new(program: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            program: program.into(),
            args,
        }
    }

    pub(super) fn display(&self) -> String {
        std::iter::once(self.program.as_str())
            .chain(self.args.iter().map(String::as_str))
            .map(display_command_part)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn display_command_part(part: &str) -> String {
    if part.is_empty() || part.chars().any(char::is_whitespace) {
        format!("\"{}\"", part.replace('"', "\\\""))
    } else {
        part.to_string()
    }
}

pub(super) fn android_build_command(tauri_program: String) -> AndroidCommand {
    AndroidCommand::new(
        tauri_program,
        vec![
            "android".to_string(),
            "build".to_string(),
            "--debug".to_string(),
            "--apk".to_string(),
            "--split-per-abi".to_string(),
            "--target".to_string(),
            "aarch64".to_string(),
            "--ci".to_string(),
            "--ignore-version-mismatches".to_string(),
        ],
    )
}

pub(super) fn tauri_icon_command(tauri_program: String, icon_source: String) -> AndroidCommand {
    AndroidCommand::new(
        tauri_program,
        vec![
            "icon".to_string(),
            icon_source,
            "--output".to_string(),
            "src-tauri/icons".to_string(),
        ],
    )
}

pub(super) fn tauri_android_init_command(tauri_program: String) -> AndroidCommand {
    AndroidCommand::new(
        tauri_program,
        vec![
            "android".to_string(),
            "init".to_string(),
            "--ci".to_string(),
            "--skip-targets-install".to_string(),
        ],
    )
}

pub(super) fn apksigner_verify_command(apk_relative_path: &str) -> Option<AndroidCommand> {
    Some(AndroidCommand::new(
        find_android_sdk_tool("apksigner")?,
        vec![
            "verify".to_string(),
            "--verbose".to_string(),
            apk_relative_path.to_string(),
        ],
    ))
}

pub(super) fn jar_list_command(
    apk_relative_path: &str,
    entries: &[&str],
) -> Option<AndroidCommand> {
    let mut args = vec!["tf".to_string(), apk_relative_path.to_string()];
    args.extend(entries.iter().map(|entry| (*entry).to_string()));
    Some(AndroidCommand::new(discover_jar_executable()?, args))
}

pub(super) fn adb_devices_command() -> Option<AndroidCommand> {
    Some(AndroidCommand::new(
        find_android_sdk_tool("adb").unwrap_or_else(|| "adb".to_string()),
        vec!["devices".to_string()],
    ))
}

pub(super) fn run_android_command(
    project_root: &Path,
    spec: &AndroidCommand,
    environment: &AndroidBuildEnvironment,
) -> DxResult<()> {
    let result = run_android_command_capture(project_root, spec, environment)?;

    if result.status.success() {
        return Ok(());
    }

    let combined_output = format!("{}\n{}", result.stdout, result.stderr);
    if android_disk_space_failure(&combined_output) {
        return Err(DxError::BuildFailed {
            message: format!(
                "`{}` failed because the Android toolchain ran out of disk space while building native artifacts. Free space on the build drive or move the project/cache to a drive with more room, then rerun the same Android build command (`dx www build --target android` or `dx build --target android`).\nstdout:\n{}\nstderr:\n{}",
                spec.display(),
                result.stdout,
                result.stderr
            ),
        });
    }

    Err(DxError::BuildFailed {
        message: format!(
            "`{}` failed with status {}\nstdout:\n{}\nstderr:\n{}",
            spec.display(),
            result.status,
            result.stdout,
            result.stderr
        ),
    })
}

fn run_android_command_capture(
    project_root: &Path,
    spec: &AndroidCommand,
    environment: &AndroidBuildEnvironment,
) -> DxResult<AndroidCommandResult> {
    let mut command = Command::new(&spec.program);
    command
        .args(&spec.args)
        .current_dir(project_root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    apply_android_environment(&mut command, environment);

    let mut child = command.spawn().map_err(|error| DxError::BuildFailed {
        message: format!("Failed to start `{}`: {error}", spec.display()),
    })?;
    let stdout_reader = spawn_output_tail_reader(child.stdout.take());
    let stderr_reader = spawn_output_tail_reader(child.stderr.take());

    let status = match child.wait() {
        Ok(status) => status,
        Err(error) => {
            let stdout = join_output_tail_reader(stdout_reader);
            let stderr = join_output_tail_reader(stderr_reader);
            return Err(DxError::BuildFailed {
                message: format!(
                    "Failed while waiting for `{}`: {error}\nstdout:\n{}\nstderr:\n{}",
                    spec.display(),
                    stdout,
                    stderr
                ),
            });
        }
    };
    let stdout = join_output_tail_reader(stdout_reader);
    let stderr = join_output_tail_reader(stderr_reader);

    Ok(AndroidCommandResult {
        status,
        stdout,
        stderr,
    })
}

fn spawn_output_tail_reader<R>(reader: Option<R>) -> Option<JoinHandle<String>>
where
    R: Read + Send + 'static,
{
    reader.map(|reader| {
        thread::spawn(move || {
            let mut reader = reader;
            let mut tail = BoundedOutputTail::new(COMMAND_OUTPUT_TAIL_CHARS);
            let mut buffer = [0_u8; 8_192];

            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(bytes_read) => tail.push_lossy(&buffer[..bytes_read]),
                    Err(error) => {
                        tail.push_text(&format!("\n<failed to read command output: {error}>"));
                        break;
                    }
                }
            }

            tail.finish()
        })
    })
}

fn join_output_tail_reader(reader: Option<JoinHandle<String>>) -> String {
    match reader {
        Some(reader) => match reader.join() {
            Ok(output) => output,
            Err(_) => "<command output reader failed>".to_string(),
        },
        None => String::new(),
    }
}

struct BoundedOutputTail {
    max_chars: usize,
    text: String,
    truncated: bool,
}

impl BoundedOutputTail {
    fn new(max_chars: usize) -> Self {
        Self {
            max_chars,
            text: String::new(),
            truncated: false,
        }
    }

    fn push_lossy(&mut self, bytes: &[u8]) {
        let text = String::from_utf8_lossy(bytes);
        self.push_text(text.as_ref());
    }

    fn push_text(&mut self, text: &str) {
        self.text.push_str(text);
        self.trim();
    }

    fn trim(&mut self) {
        let char_count = self.text.chars().count();
        if char_count <= self.max_chars {
            return;
        }
        self.text = self
            .text
            .chars()
            .skip(char_count - self.max_chars)
            .collect();
        self.truncated = true;
    }

    fn finish(self) -> String {
        let text = self.text.trim().to_string();
        if self.truncated && !text.is_empty() {
            format!("...{text}")
        } else {
            text
        }
    }
}

pub(super) fn run_optional_android_command(
    project_root: &Path,
    spec: Option<AndroidCommand>,
    environment: &AndroidBuildEnvironment,
) -> AndroidCommandProbe {
    let Some(spec) = spec else {
        return AndroidCommandProbe {
            status: "tool-unavailable",
            command: None,
            stdout: String::new(),
            stderr: String::new(),
        };
    };

    match run_android_command_capture(project_root, &spec, environment) {
        Ok(result) if result.status.success() => AndroidCommandProbe {
            status: "passed",
            command: Some(spec.display()),
            stdout: result.stdout,
            stderr: result.stderr,
        },
        Ok(result) => AndroidCommandProbe {
            status: "failed",
            command: Some(spec.display()),
            stdout: result.stdout,
            stderr: result.stderr,
        },
        Err(error) => AndroidCommandProbe {
            status: "start-failed",
            command: Some(spec.display()),
            stdout: String::new(),
            stderr: error.to_string(),
        },
    }
}

fn apply_android_environment(command: &mut Command, environment: &AndroidBuildEnvironment) {
    command.env("CARGO_BUILD_JOBS", "1");
    command.env("GRADLE_OPTS", bounded_gradle_opts());

    command.env(GRADLE_USER_HOME_ENV, &environment.gradle_user_home);
    command.env(ANDROID_USER_HOME_ENV, &environment.android_user_home);
    command.env(ANDROID_AVD_HOME_ENV, &environment.android_avd_home);
    command.env(CARGO_HOME_ENV, &environment.cargo_home);
    command.env(CARGO_TARGET_DIR_ENV, &environment.cargo_target_dir);
    command.env(TEMP_ENV, &environment.temp_dir);
    command.env(TMP_ENV, &environment.temp_dir);

    let android_sdk = discover_android_sdk();
    if let Some(sdk) = &android_sdk {
        if std::env::var_os("ANDROID_HOME").is_none() {
            command.env("ANDROID_HOME", sdk);
        }
        if std::env::var_os("ANDROID_SDK_ROOT").is_none() {
            command.env("ANDROID_SDK_ROOT", sdk);
        }
        if let Some(ndk) = discover_latest_ndk(sdk) {
            if std::env::var_os("ANDROID_NDK_HOME").is_none() {
                command.env("ANDROID_NDK_HOME", &ndk);
            }
            if std::env::var_os("NDK_HOME").is_none() {
                command.env("NDK_HOME", &ndk);
            }
        }
    }

    if env_path("JAVA_HOME")
        .filter(|path| valid_java_home(path))
        .is_none()
    {
        if let Some(java_home) = discover_java_home() {
            command.env("JAVA_HOME", java_home);
        }
    }
}

fn android_disk_space_failure(output: &str) -> bool {
    let output = output.to_ascii_lowercase();
    ANDROID_DISK_SPACE_FAILURE_MARKERS
        .iter()
        .any(|marker| output.contains(&marker.to_ascii_lowercase()))
}

pub(super) fn discover_android_sdk() -> Option<PathBuf> {
    env_path("ANDROID_HOME")
        .filter(|path| valid_android_sdk(path))
        .or_else(|| env_path("ANDROID_SDK_ROOT"))
        .filter(|path| valid_android_sdk(path))
        .or_else(|| first_existing_dir(android_sdk_candidates()))
}

pub(super) fn discover_latest_ndk(android_sdk: &Path) -> Option<PathBuf> {
    latest_child_dir_matching(&android_sdk.join("ndk"), |path| {
        path.join("source.properties").is_file()
    })
}

fn find_android_sdk_tool(tool: &str) -> Option<String> {
    let sdk = discover_android_sdk()?;
    if tool == "adb" {
        if let Some(adb) = first_executable_in(&sdk.join("platform-tools"), "adb") {
            return Some(adb.to_string_lossy().to_string());
        }
    }

    let build_tools = latest_child_dir(&sdk.join("build-tools"))?;
    first_executable_in(&build_tools, tool).map(|tool_path| tool_path.to_string_lossy().to_string())
}

fn discover_jar_executable() -> Option<String> {
    if let Some(java_home) = env_path("JAVA_HOME")
        .filter(|path| valid_java_home(path))
        .or_else(discover_java_home)
    {
        if let Some(jar) = first_executable_in(&java_home.join("bin"), "jar") {
            return Some(jar.to_string_lossy().to_string());
        }
    }
    Some("jar".to_string())
}

pub(super) fn discover_java_home() -> Option<PathBuf> {
    java_home_candidates()
        .into_iter()
        .find(|path| valid_java_home(path))
}

fn android_sdk_candidates() -> Vec<PathBuf> {
    let mut candidates = vec![
        PathBuf::from("G:/Dev/SDKs/Android"),
        PathBuf::from("C:/Android/Sdk"),
    ];
    if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
        candidates.push(PathBuf::from(local_app_data).join("Android/Sdk"));
    }
    if let Some(user_profile) = std::env::var_os("USERPROFILE") {
        candidates.push(PathBuf::from(user_profile).join("AppData/Local/Android/Sdk"));
    }
    candidates
}

fn java_home_candidates() -> Vec<PathBuf> {
    let mut roots = vec![
        PathBuf::from("G:/Dev/Java"),
        PathBuf::from("C:/Program Files/Java"),
        PathBuf::from("C:/Program Files/Eclipse Adoptium"),
    ];
    roots.extend(
        ["JAVA_HOME"]
            .iter()
            .filter_map(std::env::var_os)
            .map(PathBuf::from),
    );
    let mut candidates = Vec::new();
    for root in roots {
        if root.is_dir() {
            candidates.push(latest_child_dir(&root).unwrap_or(root));
        } else {
            candidates.push(root);
        }
    }
    candidates
}

fn env_path(key: &str) -> Option<PathBuf> {
    std::env::var_os(key)
        .map(PathBuf::from)
        .filter(|path| path.is_dir())
}

fn first_existing_dir(candidates: Vec<PathBuf>) -> Option<PathBuf> {
    candidates
        .into_iter()
        .find(|path| path.is_dir() && valid_android_sdk(path))
}

pub(super) fn latest_child_dir(parent: &Path) -> Option<PathBuf> {
    latest_child_dir_matching(parent, |_| true)
}

fn latest_child_dir_matching(parent: &Path, keep: impl Fn(&Path) -> bool) -> Option<PathBuf> {
    let mut children = std::fs::read_dir(parent)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .filter(|path| keep(path))
        .collect::<Vec<_>>();
    children.sort_by(|left, right| {
        numeric_revision_key(right)
            .cmp(&numeric_revision_key(left))
            .then_with(|| child_name(right).cmp(&child_name(left)))
            .then_with(|| right.cmp(left))
    });
    children.into_iter().next()
}

fn numeric_revision_key(path: &Path) -> Vec<u32> {
    let mut numbers = Vec::new();
    let mut current = String::new();
    for character in child_name(path).chars() {
        if character.is_ascii_digit() {
            current.push(character);
        } else if !current.is_empty() {
            numbers.push(current.parse::<u32>().unwrap_or(0));
            current.clear();
        }
    }
    if !current.is_empty() {
        numbers.push(current.parse::<u32>().unwrap_or(0));
    }
    if numbers.is_empty() {
        numbers.push(0);
    }
    numbers
}

fn child_name(path: &Path) -> String {
    path.file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}

fn bounded_gradle_opts() -> String {
    let base = std::env::var("GRADLE_OPTS").unwrap_or_default();
    let mut options = base
        .split_whitespace()
        .filter(|option| !option.starts_with("-Dorg.gradle.workers.max="))
        .filter(|option| !option.starts_with("-Dorg.gradle.daemon="))
        .filter(|option| !option.starts_with("-Dorg.gradle.jvmargs="))
        .filter(|option| !option.starts_with("-Xmx"))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    options.push("-Dorg.gradle.workers.max=2".to_string());
    options.push("-Dorg.gradle.daemon=false".to_string());
    options.push("-Xmx1536m".to_string());
    options.join(" ")
}

fn valid_android_sdk(path: &Path) -> bool {
    path.join("platform-tools").is_dir() && path.join("build-tools").is_dir()
}

pub(super) fn valid_java_home(path: &Path) -> bool {
    first_executable_in(&path.join("bin"), "java").is_some()
        && first_executable_in(&path.join("bin"), "jar").is_some()
}

pub(super) fn first_executable_in(dir: &Path, name: &str) -> Option<PathBuf> {
    executable_names(name)
        .into_iter()
        .map(|candidate| dir.join(candidate))
        .find(|candidate| candidate.is_file())
}

fn executable_names(name: &str) -> Vec<String> {
    if cfg!(windows) {
        vec![
            format!("{name}.exe"),
            format!("{name}.bat"),
            format!("{name}.cmd"),
            name.to_string(),
        ]
    } else {
        vec![name.to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn android_build_command_uses_tauri_arm64_debug_apk_lane() {
        let command = android_build_command("tauri".to_string());

        assert_eq!(
            command.display(),
            "tauri android build --debug --apk --split-per-abi --target aarch64 --ci --ignore-version-mismatches"
        );
    }

    #[test]
    fn command_display_quotes_parts_with_spaces() {
        let command = AndroidCommand::new(
            "C:/Program Files/Java/bin/jar.exe",
            vec!["tf".to_string(), "target/app debug.apk".to_string()],
        );

        assert_eq!(
            command.display(),
            "\"C:/Program Files/Java/bin/jar.exe\" tf \"target/app debug.apk\""
        );
    }

    #[test]
    fn latest_child_dir_prefers_highest_numeric_revision() -> std::io::Result<()> {
        let dir = tempdir()?;
        std::fs::create_dir(dir.path().join("9.0.0"))?;
        std::fs::create_dir(dir.path().join("26.0.0"))?;
        std::fs::create_dir(dir.path().join("35.0.0"))?;

        let latest = latest_child_dir(dir.path()).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "missing latest child")
        })?;
        let latest_name = latest
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "missing latest child name")
            })?;

        assert_eq!(latest_name, "35.0.0");
        Ok(())
    }

    #[test]
    fn latest_child_dir_matching_skips_invalid_newer_revisions() -> std::io::Result<()> {
        let dir = tempdir()?;
        std::fs::create_dir_all(dir.path().join("35.0.0"))?;
        std::fs::create_dir_all(dir.path().join("34.0.0"))?;
        std::fs::write(
            dir.path().join("34.0.0/source.properties"),
            "Pkg.Revision=34.0.0",
        )?;

        let latest =
            latest_child_dir_matching(dir.path(), |path| path.join("source.properties").is_file())
                .ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::NotFound, "missing latest valid child")
                })?;
        let latest_name = latest
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "missing latest valid child name",
                )
            })?;

        assert_eq!(latest_name, "34.0.0");
        Ok(())
    }

    #[test]
    fn android_disk_space_failures_are_classified_from_process_output() {
        assert!(android_disk_space_failure("failed with ENOSPC"));
        assert!(android_disk_space_failure(
            "There is not enough space on the disk. (os error 112)"
        ));
        assert!(android_disk_space_failure("No space left on device"));
        assert!(!android_disk_space_failure("compilation failed"));
    }

    #[test]
    fn windows_sdk_tools_allow_batch_launchers() -> std::io::Result<()> {
        let dir = tempdir()?;
        let tool_name = if cfg!(windows) {
            "apksigner.bat"
        } else {
            "apksigner"
        };
        std::fs::write(dir.path().join(tool_name), "")?;

        let tool = first_executable_in(dir.path(), "apksigner").ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "missing tool path")
        })?;
        let tool_file_name = tool
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "missing tool file name")
            })?;

        assert_eq!(tool_file_name, tool_name);
        Ok(())
    }
}
