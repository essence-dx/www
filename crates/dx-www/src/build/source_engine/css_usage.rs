use std::collections::BTreeSet;
use std::path::Path;

use crate::error::{DxError, DxResult};
use crate::parser::style::extract_class_attribute_tokens;

#[derive(Debug, Clone, Default)]
pub struct CssUsage {
    classes: BTreeSet<String>,
    has_unknown_class_usage: bool,
}

impl CssUsage {
    pub fn contains_class(&self, class_name: &str) -> bool {
        self.classes.contains(class_name)
    }

    pub fn is_empty(&self) -> bool {
        self.classes.is_empty()
    }

    pub fn has_unknown_class_usage(&self) -> bool {
        self.has_unknown_class_usage
    }
}

pub fn collect_css_usage(project_root: &Path) -> DxResult<CssUsage> {
    let mut usage = CssUsage::default();
    for directory in ["app", "components", "pages", "src"] {
        collect_directory(project_root, &project_root.join(directory), &mut usage)?;
    }
    Ok(usage)
}

fn collect_directory(project_root: &Path, directory: &Path, usage: &mut CssUsage) -> DxResult<()> {
    if !directory.is_dir() {
        return Ok(());
    }

    for entry in std::fs::read_dir(directory).map_err(|error| DxError::IoError {
        path: Some(directory.to_path_buf()),
        message: error.to_string(),
    })? {
        let entry = entry.map_err(|error| DxError::IoError {
            path: Some(directory.to_path_buf()),
            message: error.to_string(),
        })?;
        let path = entry.path();
        if skip_path(project_root, &path) {
            continue;
        }
        if path.is_dir() {
            collect_directory(project_root, &path, usage)?;
        } else if is_class_source(&path) {
            collect_file(&path, usage)?;
        }
    }

    Ok(())
}

fn collect_file(path: &Path, usage: &mut CssUsage) -> DxResult<()> {
    let source = std::fs::read_to_string(path).map_err(|error| DxError::IoError {
        path: Some(path.to_path_buf()),
        message: error.to_string(),
    })?;
    if contains_unknown_class_usage(&source) {
        usage.has_unknown_class_usage = true;
    }
    usage
        .classes
        .extend(extract_class_attribute_tokens(&source));
    Ok(())
}

fn is_class_source(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| matches!(extension, "tsx" | "jsx" | "ts" | "js" | "html" | "mdx"))
}

fn skip_path(project_root: &Path, path: &Path) -> bool {
    path.strip_prefix(project_root)
        .ok()
        .and_then(|relative| relative.components().next())
        .and_then(|component| component.as_os_str().to_str())
        .is_some_and(|first| matches!(first, ".dx" | "node_modules" | "target" | ".git"))
}

fn contains_unknown_class_usage(source: &str) -> bool {
    ["className={", "class={"]
        .into_iter()
        .any(|pattern| source_has_unknown_class_expression(source, pattern))
}

fn source_has_unknown_class_expression(source: &str, pattern: &str) -> bool {
    let mut rest = source;
    while let Some(start) = rest.find(pattern) {
        let expression = rest[start + pattern.len()..].trim_start();
        if class_expression_is_unknown(expression) {
            return true;
        }
        rest = &expression[1.min(expression.len())..];
    }
    false
}

fn class_expression_is_unknown(expression: &str) -> bool {
    match expression.chars().next() {
        Some('"') | Some('\'') => false,
        Some('`') => expression
            .get(1..)
            .and_then(|tail| tail.split('`').next())
            .is_some_and(|template| template.contains("${")),
        Some(_) => true,
        None => false,
    }
}
