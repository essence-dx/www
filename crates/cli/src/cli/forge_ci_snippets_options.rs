use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeCiSnippetsCommandOptions {
    pub(super) out: Option<PathBuf>,
    pub(super) publisher_key: Option<PathBuf>,
    pub(super) artifact_dir: String,
    pub(super) pages_dir: String,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: u8,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_ci_snippets_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeCiSnippetsCommandOptions> {
    let mut out: Option<PathBuf> = None;
    let mut publisher_key: Option<PathBuf> = None;
    let mut artifact_dir = ".dx/ci".to_string();
    let mut pages_dir = ".dx/forge-pages".to_string();
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 90u8;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--out" | "--dir" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    ci_snippets_options_error("--out requires a directory", "out")
                })?;
                out = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--artifact-dir" | "--artifacts" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    ci_snippets_options_error("--artifact-dir requires a path", "artifact-dir")
                })?;
                artifact_dir = value.clone();
                index += 2;
            }
            "--publisher-key" | "--private-key" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    ci_snippets_options_error(
                        "--publisher-key requires a private key file",
                        "publisher-key",
                    )
                })?;
                publisher_key = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--pages-dir" | "--pages" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    ci_snippets_options_error("--pages-dir requires a path", "pages-dir")
                })?;
                pages_dir = value.clone();
                index += 2;
            }
            "--output" | "--report" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    ci_snippets_options_error("--output requires a path", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    ci_snippets_options_error(
                        "--format requires terminal, json, or markdown",
                        "format",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    ci_snippets_options_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(ci_snippets_options_error(
                    format!("Unknown forge ci-snippets option: {value}"),
                    "forge ci-snippets",
                ));
            }
            value => {
                return Err(ci_snippets_options_error(
                    format!("Unexpected forge ci-snippets path: {value}"),
                    "forge ci-snippets",
                ));
            }
        }
    }

    Ok(DxForgeCiSnippetsCommandOptions {
        out,
        publisher_key,
        artifact_dir,
        pages_dir,
        output,
        format,
        fail_under,
        quiet,
    })
}

fn ci_snippets_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    fn assert_config_error(error: DxError, expected_message: &str, expected_field: &str) {
        match error {
            DxError::ConfigValidationError { message, field } => {
                assert!(message.contains(expected_message), "{message}");
                assert_eq!(field.as_deref(), Some(expected_field));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn ci_snippets_options_accept_flags_and_aliases() {
        let cwd = PathBuf::from("G:/workspace");
        let args = strings(&[
            "--dir",
            ".dx/snippets",
            "--artifacts",
            ".dx/artifacts",
            "--private-key",
            ".dx/key.json",
            "--pages",
            ".dx/pages",
            "--report",
            ".dx/report.md",
            "--format",
            "markdown",
            "--fail-under",
            "93",
            "--quiet",
        ]);

        let options = parse_forge_ci_snippets_options(&cwd, &args).expect("options");

        assert!(
            options
                .out
                .as_ref()
                .is_some_and(|path| path.ends_with(".dx/snippets"))
        );
        assert_eq!(options.artifact_dir, ".dx/artifacts");
        assert!(
            options
                .publisher_key
                .as_ref()
                .is_some_and(|path| path.ends_with(".dx/key.json"))
        );
        assert_eq!(options.pages_dir, ".dx/pages");
        assert!(
            options
                .output
                .as_ref()
                .is_some_and(|path| path.ends_with(".dx/report.md"))
        );
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert_eq!(options.fail_under, 93);
        assert!(options.quiet);
    }

    #[test]
    fn ci_snippets_options_keep_defaults() {
        let cwd = PathBuf::from("G:/workspace");
        let options = parse_forge_ci_snippets_options(&cwd, &[]).expect("options");

        assert!(options.out.is_none());
        assert!(options.publisher_key.is_none());
        assert_eq!(options.artifact_dir, ".dx/ci");
        assert_eq!(options.pages_dir, ".dx/forge-pages");
        assert!(options.output.is_none());
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert_eq!(options.fail_under, 90);
        assert!(!options.quiet);
    }

    #[test]
    fn ci_snippets_options_reject_unknown_options_and_positional_paths() {
        let cwd = PathBuf::from("G:/workspace");

        let unknown = parse_forge_ci_snippets_options(&cwd, &strings(&["--wat"]))
            .expect_err("unknown option");
        assert_config_error(
            unknown,
            "Unknown forge ci-snippets option: --wat",
            "forge ci-snippets",
        );

        let positional = parse_forge_ci_snippets_options(&cwd, &strings(&["template"]))
            .expect_err("positional path");
        assert_config_error(
            positional,
            "Unexpected forge ci-snippets path: template",
            "forge ci-snippets",
        );
    }

    #[test]
    fn ci_snippets_options_reject_missing_values() {
        let cwd = PathBuf::from("G:/workspace");
        let error = parse_forge_ci_snippets_options(&cwd, &strings(&["--publisher-key"]))
            .expect_err("missing publisher key");

        assert_config_error(
            error,
            "--publisher-key requires a private key file",
            "publisher-key",
        );
    }
}
