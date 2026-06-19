use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeReleaseOperationsCommandOptions {
    pub(super) project: Option<PathBuf>,
    pub(super) release_bundle: Option<PathBuf>,
    pub(super) release_manifest: Option<PathBuf>,
    pub(super) trust_regression: Option<PathBuf>,
    pub(super) release_candidate: Option<PathBuf>,
    pub(super) ci_artifacts: Option<PathBuf>,
    pub(super) public_evidence: Option<PathBuf>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: u8,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_release_operations_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeReleaseOperationsCommandOptions> {
    let mut project: Option<PathBuf> = None;
    let mut release_bundle: Option<PathBuf> = None;
    let mut release_manifest: Option<PathBuf> = None;
    let mut trust_regression: Option<PathBuf> = None;
    let mut release_candidate: Option<PathBuf> = None;
    let mut ci_artifacts: Option<PathBuf> = None;
    let mut public_evidence: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 90u8;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_operations_options_error("--project requires a path", "project")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--release-bundle" | "--bundle" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_operations_options_error(
                        "--release-bundle requires a directory",
                        "release-bundle",
                    )
                })?;
                release_bundle = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--release-manifest" | "--signed-manifest" | "--manifest" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_operations_options_error(
                        "--release-manifest requires a JSON file",
                        "release-manifest",
                    )
                })?;
                release_manifest = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--trust-regression" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_operations_options_error(
                        "--trust-regression requires a JSON report",
                        "trust-regression",
                    )
                })?;
                trust_regression = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--release-candidate" | "--candidate" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_operations_options_error(
                        "--release-candidate requires a JSON report",
                        "release-candidate",
                    )
                })?;
                release_candidate = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--ci-artifacts" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_operations_options_error(
                        "--ci-artifacts requires a directory",
                        "ci-artifacts",
                    )
                })?;
                ci_artifacts = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--public-evidence" | "--evidence" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_operations_options_error(
                        "--public-evidence requires a directory",
                        "public-evidence",
                    )
                })?;
                public_evidence = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_operations_options_error("--output requires a path", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_operations_options_error(
                        "--format requires terminal, json, or markdown",
                        "format",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_operations_options_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(release_operations_options_error(
                    format!("Unknown forge release-operations option: {value}"),
                    "forge release-operations",
                ));
            }
            value => {
                if project.is_some() {
                    return Err(release_operations_options_error(
                        format!("Unexpected forge release-operations path: {value}"),
                        "project",
                    ));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    Ok(DxForgeReleaseOperationsCommandOptions {
        project,
        release_bundle,
        release_manifest,
        trust_regression,
        release_candidate,
        ci_artifacts,
        public_evidence,
        output,
        format,
        fail_under,
        quiet,
    })
}

fn release_operations_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_release_operations_options_accepts_all_flags() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--project".to_string(),
            "examples/template".to_string(),
            "--bundle".to_string(),
            ".dx/forge-release-bundle".to_string(),
            "--manifest".to_string(),
            ".dx/forge-release-bundle/forge-release-manifest.json".to_string(),
            "--trust-regression".to_string(),
            ".dx/forge/trust-regression.json".to_string(),
            "--candidate".to_string(),
            ".dx/ci/forge-release-candidate.json".to_string(),
            "--ci-artifacts".to_string(),
            ".dx/ci".to_string(),
            "--evidence".to_string(),
            "public".to_string(),
            "--out".to_string(),
            ".dx/ci/forge-release-operations.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--fail-under".to_string(),
            "95".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_forge_release_operations_options(&cwd, &args).expect("options");

        assert_eq!(options.project, Some(cwd.join("examples/template")));
        assert_eq!(
            options.release_bundle,
            Some(cwd.join(".dx/forge-release-bundle"))
        );
        assert_eq!(
            options.release_manifest,
            Some(cwd.join(".dx/forge-release-bundle/forge-release-manifest.json"))
        );
        assert_eq!(
            options.trust_regression,
            Some(cwd.join(".dx/forge/trust-regression.json"))
        );
        assert_eq!(
            options.release_candidate,
            Some(cwd.join(".dx/ci/forge-release-candidate.json"))
        );
        assert_eq!(options.ci_artifacts, Some(cwd.join(".dx/ci")));
        assert_eq!(options.public_evidence, Some(cwd.join("public")));
        assert_eq!(
            options.output,
            Some(cwd.join(".dx/ci/forge-release-operations.md"))
        );
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert_eq!(options.fail_under, 95);
        assert!(options.quiet);
    }

    #[test]
    fn parse_release_operations_options_accepts_positional_project() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec!["examples/template".to_string()];

        let options = parse_forge_release_operations_options(&cwd, &args).expect("project");

        assert_eq!(options.project, Some(cwd.join("examples/template")));
        assert_eq!(options.release_bundle, None);
        assert_eq!(options.release_manifest, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert_eq!(options.fail_under, 90);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_release_operations_options_rejects_unknown_option() {
        let error = parse_forge_release_operations_options(
            &std::env::current_dir().unwrap(),
            &["--pages".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown forge release-operations option: --pages");
                assert_eq!(field.as_deref(), Some("forge release-operations"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_release_operations_options_rejects_extra_positional_project() {
        let error = parse_forge_release_operations_options(
            &std::env::current_dir().unwrap(),
            &["first".to_string(), "second".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected forge release-operations path: second");
                assert_eq!(field.as_deref(), Some("project"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
