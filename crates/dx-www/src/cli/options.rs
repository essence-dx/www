use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DxOutputFormat {
    Terminal,
    Json,
    Markdown,
}

impl DxOutputFormat {
    pub(super) fn parse(value: &str) -> DxResult<Self> {
        match value {
            "terminal" | "term" => Ok(Self::Terminal),
            "json" => Ok(Self::Json),
            "markdown" | "md" => Ok(Self::Markdown),
            other => Err(DxError::ConfigValidationError {
                message: format!("Unknown output format: {other}"),
                field: Some("format".to_string()),
            }),
        }
    }

    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Terminal => "terminal",
            Self::Json => "json",
            Self::Markdown => "markdown",
        }
    }
}

pub(super) fn resolve_cli_path(cwd: &Path, value: &str) -> PathBuf {
    let path = PathBuf::from(value);
    if path.is_absolute() {
        path
    } else {
        cwd.join(path)
    }
}

pub(super) fn parse_score_threshold(value: &str) -> DxResult<u8> {
    value
        .parse::<u8>()
        .map_err(|_| DxError::ConfigValidationError {
            message: format!("Invalid fail-under score: {value}"),
            field: Some("fail-under".to_string()),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_format_parse_accepts_aliases_and_reports_canonical_labels() {
        assert_eq!(
            DxOutputFormat::parse("terminal").unwrap(),
            DxOutputFormat::Terminal
        );
        assert_eq!(
            DxOutputFormat::parse("term").unwrap(),
            DxOutputFormat::Terminal
        );
        assert_eq!(DxOutputFormat::parse("json").unwrap(), DxOutputFormat::Json);
        assert_eq!(
            DxOutputFormat::parse("markdown").unwrap(),
            DxOutputFormat::Markdown
        );
        assert_eq!(
            DxOutputFormat::parse("md").unwrap(),
            DxOutputFormat::Markdown
        );

        assert_eq!(DxOutputFormat::Terminal.as_str(), "terminal");
        assert_eq!(DxOutputFormat::Json.as_str(), "json");
        assert_eq!(DxOutputFormat::Markdown.as_str(), "markdown");
    }

    #[test]
    fn output_format_parse_rejects_unknown_format() {
        let error = DxOutputFormat::parse("xml").unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert!(message.contains("Unknown output format: xml"));
                assert_eq!(field.as_deref(), Some("format"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn resolve_cli_path_preserves_absolute_paths_and_joins_relative_paths() {
        let cwd = std::env::current_dir().unwrap();
        let relative = resolve_cli_path(&cwd, "nested/file.txt");
        assert_eq!(relative, cwd.join("nested/file.txt"));

        let absolute = cwd.join("already-absolute.txt");
        assert_eq!(resolve_cli_path(&cwd, absolute.to_str().unwrap()), absolute);
    }

    #[test]
    fn parse_score_threshold_rejects_non_numeric_scores() {
        assert_eq!(parse_score_threshold("95").unwrap(), 95);

        let error = parse_score_threshold("high").unwrap_err();
        match error {
            DxError::ConfigValidationError { message, field } => {
                assert!(message.contains("Invalid fail-under score: high"));
                assert_eq!(field.as_deref(), Some("fail-under"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
