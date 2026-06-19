use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgePublisherKeyGenerateCommandOptions {
    pub(super) out: Option<PathBuf>,
    pub(super) signer: Option<String>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) force: bool,
    pub(super) quiet: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgePublisherKeySignCommandOptions {
    pub(super) key: Option<PathBuf>,
    pub(super) manifest: Option<PathBuf>,
    pub(super) manifest_output: Option<PathBuf>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_publisher_key_generate_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgePublisherKeyGenerateCommandOptions> {
    let mut out: Option<PathBuf> = None;
    let mut signer: Option<String> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut force = false;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--out" | "--dir" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publisher_key_options_error(
                        "--out requires a directory",
                        "forge publisher-key generate",
                    )
                })?;
                out = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--signer" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publisher_key_options_error(
                        "--signer requires a name",
                        "forge publisher-key generate",
                    )
                })?;
                signer = Some(value.clone());
                index += 2;
            }
            "--output" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publisher_key_options_error(
                        "--output requires a report path",
                        "forge publisher-key generate",
                    )
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publisher_key_options_error(
                        "--format requires terminal, json, or markdown",
                        "forge publisher-key generate",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--force" => {
                force = true;
                index += 1;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(publisher_key_options_error(
                    format!("Unknown forge publisher-key generate option: {value}"),
                    "forge publisher-key generate",
                ));
            }
            value => {
                if out.is_some() {
                    return Err(publisher_key_options_error(
                        format!("Unexpected forge publisher-key generate path: {value}"),
                        "forge publisher-key generate",
                    ));
                }
                out = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    Ok(DxForgePublisherKeyGenerateCommandOptions {
        out,
        signer,
        output,
        format,
        force,
        quiet,
    })
}

pub(super) fn parse_forge_publisher_key_sign_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgePublisherKeySignCommandOptions> {
    let mut key: Option<PathBuf> = None;
    let mut manifest: Option<PathBuf> = None;
    let mut manifest_output: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--key" | "--private-key" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publisher_key_options_error(
                        "--key requires a private key file",
                        "forge publisher-key sign",
                    )
                })?;
                key = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--manifest" | "--release-manifest" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publisher_key_options_error(
                        "--manifest requires a release manifest JSON file",
                        "forge publisher-key sign",
                    )
                })?;
                manifest = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--manifest-output" | "--signed-manifest" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publisher_key_options_error(
                        "--manifest-output requires a JSON path",
                        "forge publisher-key sign",
                    )
                })?;
                manifest_output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publisher_key_options_error(
                        "--output requires a report path",
                        "forge publisher-key sign",
                    )
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publisher_key_options_error(
                        "--format requires terminal, json, or markdown",
                        "forge publisher-key sign",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(publisher_key_options_error(
                    format!("Unknown forge publisher-key sign option: {value}"),
                    "forge publisher-key sign",
                ));
            }
            value => {
                if manifest.is_some() {
                    return Err(publisher_key_options_error(
                        format!("Unexpected forge publisher-key sign path: {value}"),
                        "forge publisher-key sign",
                    ));
                }
                manifest = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    Ok(DxForgePublisherKeySignCommandOptions {
        key,
        manifest,
        manifest_output,
        output,
        format,
        quiet,
    })
}

fn publisher_key_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_generate_options_accepts_all_flags() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--out".to_string(),
            ".dx/forge/publisher".to_string(),
            "--signer".to_string(),
            "dx-forge-test-publisher".to_string(),
            "--output".to_string(),
            "publisher-key.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--force".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_forge_publisher_key_generate_options(&cwd, &args).expect("options");

        assert_eq!(options.out, Some(cwd.join(".dx/forge/publisher")));
        assert_eq!(options.signer.as_deref(), Some("dx-forge-test-publisher"));
        assert_eq!(options.output, Some(cwd.join("publisher-key.md")));
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert!(options.force);
        assert!(options.quiet);
    }

    #[test]
    fn parse_generate_options_accepts_positional_out_dir() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![".dx/forge/publisher".to_string()];

        let options =
            parse_forge_publisher_key_generate_options(&cwd, &args).expect("positional out");

        assert_eq!(options.out, Some(cwd.join(".dx/forge/publisher")));
        assert_eq!(options.signer, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
    }

    #[test]
    fn parse_generate_options_rejects_unknown_option() {
        let error = parse_forge_publisher_key_generate_options(
            &std::env::current_dir().unwrap(),
            &["--private-key".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(
                    message,
                    "Unknown forge publisher-key generate option: --private-key"
                );
                assert_eq!(field.as_deref(), Some("forge publisher-key generate"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_generate_options_rejects_extra_positional_out_dir() {
        let error = parse_forge_publisher_key_generate_options(
            &std::env::current_dir().unwrap(),
            &["first".to_string(), "second".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(
                    message,
                    "Unexpected forge publisher-key generate path: second"
                );
                assert_eq!(field.as_deref(), Some("forge publisher-key generate"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_sign_options_accepts_all_flags() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--key".to_string(),
            ".dx/forge/publisher/publisher-key.private.json".to_string(),
            "--manifest".to_string(),
            "release-bundle/forge-release-bundle.json".to_string(),
            "--manifest-output".to_string(),
            "release-bundle/signed.json".to_string(),
            "--output".to_string(),
            "publisher-sign.json".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_forge_publisher_key_sign_options(&cwd, &args).expect("options");

        assert_eq!(
            options.key,
            Some(cwd.join(".dx/forge/publisher/publisher-key.private.json"))
        );
        assert_eq!(
            options.manifest,
            Some(cwd.join("release-bundle/forge-release-bundle.json"))
        );
        assert_eq!(
            options.manifest_output,
            Some(cwd.join("release-bundle/signed.json"))
        );
        assert_eq!(options.output, Some(cwd.join("publisher-sign.json")));
        assert_eq!(options.format, DxOutputFormat::Json);
        assert!(options.quiet);
    }

    #[test]
    fn parse_sign_options_accepts_positional_manifest() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec!["release-bundle/forge-release-bundle.json".to_string()];

        let options = parse_forge_publisher_key_sign_options(&cwd, &args).expect("manifest");

        assert_eq!(
            options.manifest,
            Some(cwd.join("release-bundle/forge-release-bundle.json"))
        );
        assert_eq!(options.key, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
    }

    #[test]
    fn parse_sign_options_rejects_unknown_option() {
        let error = parse_forge_publisher_key_sign_options(
            &std::env::current_dir().unwrap(),
            &["--out".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown forge publisher-key sign option: --out");
                assert_eq!(field.as_deref(), Some("forge publisher-key sign"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_sign_options_rejects_extra_positional_manifest() {
        let error = parse_forge_publisher_key_sign_options(
            &std::env::current_dir().unwrap(),
            &["first".to_string(), "second".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected forge publisher-key sign path: second");
                assert_eq!(field.as_deref(), Some("forge publisher-key sign"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
