use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgePublishPlanCommandOptions {
    pub(super) project: Option<PathBuf>,
    pub(super) release_bundle: Option<PathBuf>,
    pub(super) pages: Option<PathBuf>,
    pub(super) registry_smoke: Option<PathBuf>,
    pub(super) release_operations: Option<PathBuf>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: u8,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_publish_plan_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgePublishPlanCommandOptions> {
    let mut project: Option<PathBuf> = None;
    let mut release_bundle: Option<PathBuf> = None;
    let mut pages: Option<PathBuf> = None;
    let mut registry_smoke: Option<PathBuf> = None;
    let mut release_operations: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 90u8;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publish_plan_options_error("--project requires a path", "project")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--release-bundle" | "--bundle" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publish_plan_options_error(
                        "--release-bundle requires a directory",
                        "release-bundle",
                    )
                })?;
                release_bundle = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--pages" | "--pages-bundle" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publish_plan_options_error("--pages requires a directory", "pages")
                })?;
                pages = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--registry-smoke" | "--r2-smoke" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publish_plan_options_error(
                        "--registry-smoke requires a JSON report",
                        "registry-smoke",
                    )
                })?;
                registry_smoke = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--release-operations" | "--operations" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publish_plan_options_error(
                        "--release-operations requires a JSON report",
                        "release-operations",
                    )
                })?;
                release_operations = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publish_plan_options_error("--output requires a path", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publish_plan_options_error(
                        "--format requires terminal, json, or markdown",
                        "format",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publish_plan_options_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(publish_plan_options_error(
                    format!("Unknown forge publish-plan option: {value}"),
                    "forge publish-plan",
                ));
            }
            value => {
                if project.is_some() {
                    return Err(publish_plan_options_error(
                        format!("Unexpected forge publish-plan path: {value}"),
                        "project",
                    ));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    Ok(DxForgePublishPlanCommandOptions {
        project,
        release_bundle,
        pages,
        registry_smoke,
        release_operations,
        output,
        format,
        fail_under,
        quiet,
    })
}

fn publish_plan_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_publish_plan_options_accepts_all_flags() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--project".to_string(),
            "examples/template".to_string(),
            "--bundle".to_string(),
            ".dx/forge-release-bundle".to_string(),
            "--pages-bundle".to_string(),
            ".dx/forge-pages".to_string(),
            "--r2-smoke".to_string(),
            ".dx/ci/forge-registry-smoke.json".to_string(),
            "--operations".to_string(),
            ".dx/ci/forge-release-operations.json".to_string(),
            "--out".to_string(),
            ".dx/ci/forge-publish-plan.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--fail-under".to_string(),
            "96".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_forge_publish_plan_options(&cwd, &args).expect("options");

        assert_eq!(options.project, Some(cwd.join("examples/template")));
        assert_eq!(
            options.release_bundle,
            Some(cwd.join(".dx/forge-release-bundle"))
        );
        assert_eq!(options.pages, Some(cwd.join(".dx/forge-pages")));
        assert_eq!(
            options.registry_smoke,
            Some(cwd.join(".dx/ci/forge-registry-smoke.json"))
        );
        assert_eq!(
            options.release_operations,
            Some(cwd.join(".dx/ci/forge-release-operations.json"))
        );
        assert_eq!(
            options.output,
            Some(cwd.join(".dx/ci/forge-publish-plan.md"))
        );
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert_eq!(options.fail_under, 96);
        assert!(options.quiet);
    }

    #[test]
    fn parse_publish_plan_options_accepts_positional_project() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec!["examples/template".to_string()];

        let options = parse_forge_publish_plan_options(&cwd, &args).expect("project");

        assert_eq!(options.project, Some(cwd.join("examples/template")));
        assert_eq!(options.release_bundle, None);
        assert_eq!(options.pages, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert_eq!(options.fail_under, 90);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_publish_plan_options_rejects_unknown_option() {
        let error =
            parse_forge_publish_plan_options(&std::env::current_dir().unwrap(), &["--ci".into()])
                .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown forge publish-plan option: --ci");
                assert_eq!(field.as_deref(), Some("forge publish-plan"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_publish_plan_options_rejects_extra_positional_project() {
        let error = parse_forge_publish_plan_options(
            &std::env::current_dir().unwrap(),
            &["first".into(), "second".into()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected forge publish-plan path: second");
                assert_eq!(field.as_deref(), Some("project"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
