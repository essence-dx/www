use crate::error::{DxError, DxResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DxBuildTarget {
    Web,
    Android,
}

impl DxBuildTarget {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Web => "web",
            Self::Android => "android",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DxBuildCommandOptions {
    pub(super) target: DxBuildTarget,
}

impl Default for DxBuildCommandOptions {
    fn default() -> Self {
        Self {
            target: DxBuildTarget::Web,
        }
    }
}

pub(super) fn parse_build_options(
    args: &[String],
    command_name: &'static str,
) -> DxResult<DxBuildCommandOptions> {
    let mut options = DxBuildCommandOptions::default();
    let mut target_seen = false;
    let mut index = 0usize;

    while index < args.len() {
        let arg = &args[index];
        match arg.as_str() {
            "--target" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    build_options_error("--target requires a value", "build.target")
                })?;
                if value.starts_with('-') {
                    return Err(build_options_error(
                        "--target requires a value",
                        "build.target",
                    ));
                }
                set_build_target(&mut options, &mut target_seen, value, command_name)?;
                index += 2;
            }
            value if value.starts_with("--target=") => {
                let target = value.trim_start_matches("--target=");
                set_build_target(&mut options, &mut target_seen, target, command_name)?;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(build_options_error(
                    format!("Unknown option for {command_name}: {value}"),
                    "build",
                ));
            }
            value => {
                return Err(build_options_error(
                    format!("Unexpected argument for {command_name}: {value}"),
                    "build",
                ));
            }
        }
    }

    Ok(options)
}

fn set_build_target(
    options: &mut DxBuildCommandOptions,
    target_seen: &mut bool,
    value: &str,
    command_name: &'static str,
) -> DxResult<()> {
    if *target_seen {
        return Err(build_options_error(
            format!("{command_name} accepts --target only once"),
            "build.target",
        ));
    }
    let value = value.trim();
    if value.is_empty() {
        return Err(build_options_error(
            "--target requires a non-empty value",
            "build.target",
        ));
    }
    options.target = parse_build_target(value, command_name)?;
    *target_seen = true;
    Ok(())
}

fn parse_build_target(value: &str, command_name: &'static str) -> DxResult<DxBuildTarget> {
    match value {
        "web" => Ok(DxBuildTarget::Web),
        "android" => Ok(DxBuildTarget::Android),
        _ => Err(build_options_error(
            format!("Unknown target for {command_name}: {value}. Supported targets: web, android"),
            "build.target",
        )),
    }
}

fn build_options_error(message: impl Into<String>, field: impl Into<String>) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn defaults_to_web_build_target() {
        let options = parse_build_options(&[], "dx build").expect("empty dx build args");

        assert_eq!(options.target, DxBuildTarget::Web);
    }

    #[test]
    fn parses_android_build_target() {
        let options = parse_build_options(&strings(&["--target", "android"]), "dx build")
            .expect("android build target");
        let equals_options = parse_build_options(&strings(&["--target=android"]), "dx build")
            .expect("android build target");

        assert_eq!(options.target, DxBuildTarget::Android);
        assert_eq!(equals_options.target, DxBuildTarget::Android);
    }

    #[test]
    fn rejects_unknown_build_args() {
        let error = parse_build_options(&["--bad".to_string()], "dx build")
            .expect_err("unknown build args should fail");

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown option for dx build: --bad");
                assert_eq!(field.as_deref(), Some("build"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn rejects_unknown_build_target() {
        let error = parse_build_options(&strings(&["--target", "desktop"]), "dx build")
            .expect_err("unknown build target should fail");

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(
                    message,
                    "Unknown target for dx build: desktop. Supported targets: web, android"
                );
                assert_eq!(field.as_deref(), Some("build.target"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn rejects_missing_empty_and_duplicate_build_target() {
        for (args, expected) in [
            (vec!["--target"], "--target requires a value"),
            (vec!["--target", "--bad"], "--target requires a value"),
            (vec!["--target="], "--target requires a non-empty value"),
            (
                vec!["--target", "web", "--target", "android"],
                "dx build accepts --target only once",
            ),
        ] {
            let error = parse_build_options(&strings(&args), "dx build").expect_err(expected);

            match error {
                DxError::ConfigValidationError { message, field } => {
                    assert_eq!(message, expected);
                    assert_eq!(field.as_deref(), Some("build.target"));
                }
                other => panic!("unexpected error: {other:?}"),
            }
        }
    }

    #[test]
    fn error_messages_preserve_www_build_command_name() {
        let error = parse_build_options(&strings(&["--target", "ios"]), "dx www build")
            .expect_err("unsupported www build target should fail");

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(
                    message,
                    "Unknown target for dx www build: ios. Supported targets: web, android"
                );
                assert_eq!(field.as_deref(), Some("build.target"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
