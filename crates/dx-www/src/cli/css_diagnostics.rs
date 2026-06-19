use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

pub(super) fn validate_style_sources(project_root: &Path) -> DxResult<()> {
    for style_path in style_diagnostic_paths(project_root) {
        validate_css_source_file(project_root, &style_path)?;
    }

    Ok(())
}

fn validate_css_source_file(project_root: &Path, source_file: &Path) -> DxResult<()> {
    let source = std::fs::read_to_string(source_file).map_err(|source| DxError::FileReadError {
        path: source_file.to_path_buf(),
        source,
    })?;
    let source_path = project_relative_slash_path(project_root, source_file);

    if let Some(error) = css_source_error(&source_path, &source) {
        return Err(error);
    }

    Ok(())
}

fn style_diagnostic_paths(project_root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for root in ["app", "components", "styles"] {
        collect_style_diagnostic_paths(&project_root.join(root), &mut paths);
    }
    paths.sort();
    paths.dedup();
    paths
}

fn collect_style_diagnostic_paths(dir: &Path, paths: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            if !is_skipped_source_dir(&path) {
                collect_style_diagnostic_paths(&path, paths);
            }
            continue;
        }

        if path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension == "css")
        {
            paths.push(path);
        }
    }
}

fn is_skipped_source_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| matches!(name, ".dx" | ".git" | "node_modules" | "target"))
}

fn css_source_error(source_path: &str, source: &str) -> Option<DxError> {
    let mut open_blocks = Vec::new();
    let mut quote = None::<(char, usize)>;
    let mut escaped = false;
    let mut comment_start = None::<usize>;
    let mut chars = source.char_indices().peekable();

    while let Some((index, ch)) = chars.next() {
        if comment_start.is_some() {
            if ch == '*' && chars.peek().is_some_and(|(_, next)| *next == '/') {
                chars.next();
                comment_start = None;
            }
            continue;
        }

        if let Some((active_quote, _start)) = quote {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == active_quote {
                quote = None;
            }
            continue;
        }

        if ch == '/' && chars.peek().is_some_and(|(_, next)| *next == '*') {
            chars.next();
            comment_start = Some(index);
            continue;
        }

        match ch {
            '"' | '\'' => quote = Some((ch, index)),
            '{' => open_blocks.push(index),
            '}' => {
                if open_blocks.pop().is_none() {
                    return Some(css_parse_error(
                        "CSS block has a closing `}` without an opening `{`.",
                        source_path,
                        source,
                        index,
                    ));
                }
            }
            _ => {}
        }
    }

    if let Some(start) = comment_start {
        return Some(css_parse_error(
            "CSS comment is missing a closing `*/`.",
            source_path,
            source,
            start,
        ));
    }

    if let Some((_quote, start)) = quote {
        return Some(css_parse_error(
            "CSS string is missing a closing quote.",
            source_path,
            source,
            start,
        ));
    }

    open_blocks.pop().map(|start| {
        css_parse_error(
            "CSS block is missing a closing `}`.",
            source_path,
            source,
            start,
        )
    })
}

fn css_parse_error(message: &str, source_path: &str, source: &str, offset: usize) -> DxError {
    let (line, column) = source_line_column(source, offset);
    DxError::ParseError {
        message: message.to_string(),
        file: PathBuf::from(source_path),
        line: Some(line as u32),
        column: Some(column as u32),
        src: Some(source.to_string()),
        span: Some(miette::SourceSpan::new(offset.into(), 1usize)),
    }
}

fn source_line_column(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut line_start = 0usize;

    for (index, ch) in source.char_indices() {
        if index >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            line_start = index + ch.len_utf8();
        }
    }

    (line, offset.saturating_sub(line_start) + 1)
}

fn project_relative_slash_path(project_root: &Path, path: &Path) -> String {
    path.strip_prefix(project_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_diagnostics_reports_unclosed_blocks_with_source_span() {
        let source = ".shell {\n  color: var(--dx-foreground);\n";

        let error = css_source_error("styles/app.css", source).expect("css diagnostic");

        match error {
            DxError::ParseError {
                message,
                file,
                line,
                column,
                src,
                span,
            } => {
                assert_eq!(message, "CSS block is missing a closing `}`.");
                assert_eq!(file, PathBuf::from("styles/app.css"));
                assert_eq!(line, Some(1));
                assert_eq!(column, Some(8));
                assert_eq!(src.as_deref(), Some(source));
                assert_eq!(span.map(|span| (span.offset(), span.len())), Some((7, 1)));
            }
            _ => panic!("expected CSS parse error"),
        }
    }

    #[test]
    fn css_diagnostics_ignores_braces_inside_strings_and_comments() {
        let source = concat!(
            ".icon {\n",
            "  content: \"}\";\n",
            "  background-image: url(\"data:image/svg+xml,{shape}\");\n",
            "  /* { ignored } */\n",
            "}\n",
        );

        assert!(css_source_error("styles/app.css", source).is_none());
    }
}
