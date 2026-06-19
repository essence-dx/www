use std::path::Path;

use crate::config::{ConfigError, DxConfig};
use crate::error::{DxError, DxResult};

pub(super) fn load_project_config_with_diagnostics(project_root: &Path) -> DxResult<DxConfig> {
    let dx_path = project_root.join("dx");
    if dx_path.is_file() {
        let source =
            std::fs::read_to_string(&dx_path).map_err(|source| DxError::FileReadError {
                path: dx_path.clone(),
                source,
            })?;

        return DxConfig::from_dx_str(&source)
            .map_err(|error| dx_config_source_error(Path::new("dx"), &source, error.to_string()));
    }

    let legacy_toml_path = project_root.join("dx.config.toml");
    if legacy_toml_path.is_file() {
        let source = std::fs::read_to_string(&legacy_toml_path).map_err(|source| {
            DxError::FileReadError {
                path: legacy_toml_path.clone(),
                source,
            }
        })?;

        return DxConfig::from_toml_str(&source).map_err(|error| {
            legacy_toml_config_error(Path::new("dx.config.toml"), &source, error)
        });
    }

    DxConfig::load_project(project_root).map_err(DxError::from)
}

fn dx_config_source_error(file: &Path, source: &str, message: String) -> DxError {
    DxError::ConfigParseError {
        file: Some(file.to_path_buf()),
        message,
        src: Some(source.to_string()),
        span: Some(first_diagnostic_span(source)),
    }
}

fn legacy_toml_config_error(file: &Path, source: &str, error: ConfigError) -> DxError {
    match error {
        ConfigError::ParseError(error) => {
            let span = error
                .span()
                .map(|range| {
                    miette::SourceSpan::new(
                        range.start.into(),
                        range.end.saturating_sub(range.start).max(1),
                    )
                })
                .unwrap_or_else(|| first_diagnostic_span(source));
            DxError::ConfigParseError {
                file: Some(file.to_path_buf()),
                message: error.message().to_string(),
                src: Some(source.to_string()),
                span: Some(span),
            }
        }
        error => dx_config_source_error(file, source, error.to_string()),
    }
}

fn first_diagnostic_span(source: &str) -> miette::SourceSpan {
    let mut offset = 0usize;

    for line in source.lines() {
        if !line.trim().is_empty() {
            return miette::SourceSpan::new(offset.into(), line.len().max(1));
        }
        offset += line.len() + 1;
    }

    miette::SourceSpan::new(0usize.into(), 1usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_diagnostic_span_marks_first_non_empty_dx_line() {
        let source = "\n\nproject.name=\n";
        let span = first_diagnostic_span(source);

        assert_eq!(span.offset(), 2);
        assert_eq!(span.len(), "project.name=".len());
    }
}
