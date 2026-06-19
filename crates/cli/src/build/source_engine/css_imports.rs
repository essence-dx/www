use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

#[derive(Debug, Clone)]
pub struct FlattenedCss {
    pub source: String,
    pub imports: Vec<FlattenedCssImport>,
    pub retained_imports: Vec<RetainedCssImport>,
}

#[derive(Debug, Clone)]
pub struct FlattenedCssImport {
    pub specifier: String,
    pub path: PathBuf,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct RetainedCssImport {
    pub specifier: String,
    pub path: Option<PathBuf>,
    pub source: Option<String>,
    pub condition: Option<String>,
    pub reason: &'static str,
}

pub fn flatten_local_css_imports_with_manifest(
    project_root: &Path,
    entry_path: &Path,
    source: &str,
) -> DxResult<FlattenedCss> {
    let root = canonical_or_original(project_root);
    let mut visited = BTreeSet::new();
    visited.insert(canonical_or_original(entry_path));
    let mut imports = Vec::new();
    let mut retained_imports = Vec::new();
    let source = flatten_source(
        &root,
        entry_path,
        source,
        &mut visited,
        &mut imports,
        &mut retained_imports,
    )?;
    Ok(FlattenedCss {
        source,
        imports,
        retained_imports,
    })
}

fn flatten_source(
    project_root: &Path,
    current_path: &Path,
    source: &str,
    visited: &mut BTreeSet<PathBuf>,
    imports: &mut Vec<FlattenedCssImport>,
    retained_imports: &mut Vec<RetainedCssImport>,
) -> DxResult<String> {
    let mut flattened = String::new();

    for line in source.lines() {
        let Some(import) = css_import_directive(line) else {
            push_line(&mut flattened, line);
            continue;
        };

        if let Some(reason) = retained_import_reason(&import) {
            retained_imports.push(retained_import(
                project_root,
                current_path,
                &import,
                reason,
            )?);
            push_line(&mut flattened, line);
            continue;
        }

        let Some(import_path) = resolve_local_import(project_root, current_path, import.specifier)
        else {
            retained_imports.push(retained_import_without_source(&import, "unresolved-local"));
            push_line(&mut flattened, line);
            continue;
        };

        if !visited.insert(import_path.clone()) {
            continue;
        }

        let import_source =
            std::fs::read_to_string(&import_path).map_err(|error| DxError::IoError {
                path: Some(import_path.clone()),
                message: error.to_string(),
            })?;
        imports.push(FlattenedCssImport {
            specifier: import.specifier.to_string(),
            path: project_relative_path(project_root, &import_path),
            source: import_source.clone(),
        });
        let imported = flatten_source(
            project_root,
            &import_path,
            &import_source,
            visited,
            imports,
            retained_imports,
        )?;
        if let Some(condition) = import
            .condition
            .filter(|condition| is_media_import_condition(condition))
        {
            flattened.push_str(&wrap_media_import_source(condition, &imported));
        } else {
            flattened.push_str(&imported);
        }
    }

    Ok(flattened)
}

#[derive(Debug, Clone, Copy)]
struct CssImportDirective<'a> {
    specifier: &'a str,
    condition: Option<&'a str>,
}

fn css_import_directive(line: &str) -> Option<CssImportDirective<'_>> {
    let rest = line.trim().strip_prefix("@import ")?.trim_start();
    let (specifier, tail) = quoted_import_specifier(rest).or_else(|| url_import_specifier(rest))?;
    Some(CssImportDirective {
        specifier,
        condition: import_condition(tail),
    })
}

fn quoted_import_specifier(rest: &str) -> Option<(&str, &str)> {
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }

    let rest = &rest[quote.len_utf8()..];
    let end = rest.find(quote)?;
    let specifier = &rest[..end];
    let tail = rest[end + quote.len_utf8()..].trim();
    Some((specifier, tail))
}

fn url_import_specifier(rest: &str) -> Option<(&str, &str)> {
    let rest = rest.strip_prefix("url(")?.trim_start();
    let quote = rest.chars().next()?;
    if quote == '"' || quote == '\'' {
        let rest = &rest[quote.len_utf8()..];
        let end = rest.find(quote)?;
        let specifier = &rest[..end];
        let tail = rest[end + quote.len_utf8()..].trim_start();
        let tail = tail.strip_prefix(')')?.trim();
        return Some((specifier, tail));
    }

    let end = rest.find(')')?;
    let specifier = rest[..end].trim();
    if specifier.is_empty() {
        return None;
    }
    Some((specifier, rest[end + 1..].trim()))
}

fn import_condition(tail: &str) -> Option<&str> {
    let tail = tail.trim();
    if tail == ";" {
        return None;
    }
    let condition = tail.strip_suffix(';').unwrap_or(tail).trim();
    (!condition.is_empty()).then_some(condition)
}

fn retained_import_reason(import: &CssImportDirective<'_>) -> Option<&'static str> {
    if external_or_rooted_specifier(import.specifier) {
        return Some("external-or-rooted");
    }
    if import
        .condition
        .is_some_and(|condition| !is_media_import_condition(condition))
    {
        return Some("conditional-or-media");
    }
    None
}

fn is_media_import_condition(condition: &str) -> bool {
    let condition = condition.trim();
    if condition.is_empty() {
        return false;
    }

    let lower = condition.to_ascii_lowercase();
    !(lower.starts_with("layer")
        || lower.starts_with("supports")
        || lower.contains(" layer(")
        || lower.contains(" supports("))
}

fn wrap_media_import_source(condition: &str, source: &str) -> String {
    let mut output = String::new();
    output.push_str("@media ");
    output.push_str(condition.trim());
    output.push_str(" {\n");
    output.push_str(source);
    if !source.ends_with('\n') {
        output.push('\n');
    }
    output.push_str("}\n");
    output
}

fn retained_import(
    project_root: &Path,
    current_path: &Path,
    import: &CssImportDirective<'_>,
    reason: &'static str,
) -> DxResult<RetainedCssImport> {
    let Some(import_path) = retained_local_import_path(project_root, current_path, import, reason)
    else {
        return Ok(retained_import_without_source(import, reason));
    };
    let source = std::fs::read_to_string(&import_path).map_err(|error| DxError::IoError {
        path: Some(import_path.clone()),
        message: error.to_string(),
    })?;
    Ok(RetainedCssImport {
        specifier: import.specifier.to_string(),
        path: Some(project_relative_path(project_root, &import_path)),
        source: Some(source),
        condition: import.condition.map(str::to_string),
        reason,
    })
}

fn retained_import_without_source(
    import: &CssImportDirective<'_>,
    reason: &'static str,
) -> RetainedCssImport {
    RetainedCssImport {
        specifier: import.specifier.to_string(),
        path: None,
        source: None,
        condition: import.condition.map(str::to_string),
        reason,
    }
}

fn retained_local_import_path(
    project_root: &Path,
    current_path: &Path,
    import: &CssImportDirective<'_>,
    reason: &str,
) -> Option<PathBuf> {
    if reason != "conditional-or-media" {
        return None;
    }
    resolve_local_import(project_root, current_path, import.specifier)
}

fn resolve_local_import(
    project_root: &Path,
    current_path: &Path,
    specifier: &str,
) -> Option<PathBuf> {
    let parent = current_path.parent()?;
    for candidate in local_import_candidates(parent, specifier) {
        let candidate = canonical_or_original(&candidate);
        if candidate.is_file() && candidate.starts_with(project_root) {
            return Some(candidate);
        }
    }
    None
}

fn local_import_candidates(parent: &Path, specifier: &str) -> Vec<PathBuf> {
    let base = parent.join(specifier);
    let mut candidates = vec![base.clone()];
    if base.extension().is_none() {
        candidates.push(base.with_extension("css"));
        candidates.push(base.join("index.css"));
    }
    candidates
}

fn external_or_rooted_specifier(specifier: &str) -> bool {
    specifier.starts_with('/')
        || specifier.starts_with("//")
        || specifier.starts_with("http:")
        || specifier.starts_with("https:")
        || specifier.starts_with("data:")
}

fn canonical_or_original(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn project_relative_path(project_root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(project_root)
        .unwrap_or(path)
        .to_path_buf()
}

fn push_line(output: &mut String, line: &str) {
    output.push_str(line);
    output.push('\n');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flattens_quoted_and_url_local_imports_but_retains_conditional_or_external_imports() {
        let project = tempfile::tempdir().expect("temp project");
        let root = project.path();
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");
        std::fs::create_dir_all(root.join("tokens")).expect("tokens dir");
        std::fs::write(root.join("tokens/quoted.css"), ".quoted { color: red; }\n")
            .expect("quoted import");
        std::fs::write(
            root.join("tokens/url-theme.css"),
            ":root { --dx-url-accent: rgb(40 50 60); }\n",
        )
        .expect("url import");
        std::fs::write(root.join("tokens/print.css"), ".print { color: black; }\n")
            .expect("conditional import");

        let entry_path = root.join("styles/app.css");
        let source = concat!(
            "@import \"../tokens/quoted.css\";\n",
            "@import url(\"../tokens/url-theme.css\");\n",
            "@import url(\"https://example.com/fonts.css\");\n",
            "@import url(\"../tokens/print.css\") print;\n",
            ".card { color: var(--dx-url-accent); }\n",
        );

        let flattened =
            flatten_local_css_imports_with_manifest(root, &entry_path, source).expect("flatten");

        assert_eq!(flattened.imports.len(), 3);
        assert_eq!(flattened.retained_imports.len(), 1);
        assert_eq!(flattened.imports[0].specifier, "../tokens/quoted.css");
        assert_eq!(flattened.imports[1].specifier, "../tokens/url-theme.css");
        assert_eq!(
            flattened.imports[1].path,
            PathBuf::from("tokens/url-theme.css")
        );
        assert_eq!(flattened.imports[2].specifier, "../tokens/print.css");
        assert_eq!(flattened.imports[2].path, PathBuf::from("tokens/print.css"));
        assert_eq!(
            flattened.retained_imports[0].specifier,
            "https://example.com/fonts.css"
        );
        assert_eq!(flattened.retained_imports[0].reason, "external-or-rooted");
        assert!(flattened.source.contains(".quoted { color: red; }"));
        assert!(flattened.source.contains("--dx-url-accent: rgb(40 50 60);"));
        assert!(flattened.source.contains("@media print"));
        assert!(flattened.source.contains(".print { color: black; }"));
        assert!(
            flattened
                .source
                .contains("@import url(\"https://example.com/fonts.css\");")
        );
        assert!(
            !flattened
                .source
                .contains("@import url(\"../tokens/print.css\") print;")
        );
        assert!(
            !flattened
                .source
                .contains("@import url(\"../tokens/url-theme.css\");")
        );
    }

    #[test]
    fn flattens_local_media_imports_by_wrapping_imported_rules() {
        let project = tempfile::tempdir().expect("temp project");
        let root = project.path();
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");
        std::fs::create_dir_all(root.join("tokens")).expect("tokens dir");
        std::fs::write(
            root.join("tokens/print.css"),
            ".print-only { color: black; }\n",
        )
        .expect("print css source");

        let entry_path = root.join("styles/app.css");
        let source = concat!(
            "@import \"../tokens/print.css\" print;\n",
            ".screen-only { color: blue; }\n",
        );

        let flattened =
            flatten_local_css_imports_with_manifest(root, &entry_path, source).expect("flatten");

        assert_eq!(flattened.imports.len(), 1);
        assert!(flattened.retained_imports.is_empty());
        assert_eq!(flattened.imports[0].specifier, "../tokens/print.css");
        assert_eq!(flattened.imports[0].path, PathBuf::from("tokens/print.css"));
        assert!(flattened.source.contains("@media print {"));
        assert!(flattened.source.contains(".print-only { color: black; }"));
        assert!(flattened.source.contains(".screen-only { color: blue; }"));
        assert!(
            !flattened
                .source
                .contains("@import \"../tokens/print.css\" print;")
        );
    }

    #[test]
    fn retains_layer_and_supports_import_conditions_as_boundaries() {
        let project = tempfile::tempdir().expect("temp project");
        let root = project.path();
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");
        std::fs::create_dir_all(root.join("tokens")).expect("tokens dir");
        std::fs::write(root.join("tokens/theme.css"), ":root { --dx-gap: 1rem; }\n")
            .expect("theme import");

        let entry_path = root.join("styles/app.css");
        let source = concat!(
            "@import \"../tokens/theme.css\" layer(theme);\n",
            "@import \"../tokens/theme.css\" supports(display: grid);\n",
            ".card { display: grid; }\n",
        );

        let flattened =
            flatten_local_css_imports_with_manifest(root, &entry_path, source).expect("flatten");

        assert!(flattened.imports.is_empty());
        assert_eq!(flattened.retained_imports.len(), 2);
        assert!(
            flattened
                .retained_imports
                .iter()
                .all(|import| import.reason == "conditional-or-media")
        );
        assert!(
            flattened
                .source
                .contains("@import \"../tokens/theme.css\" layer(theme);")
        );
        assert!(
            flattened
                .source
                .contains("@import \"../tokens/theme.css\" supports(display: grid);")
        );
    }

    #[test]
    fn flattens_extensionless_and_index_css_imports() {
        let project = tempfile::tempdir().expect("temp project");
        let root = project.path();
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");
        std::fs::create_dir_all(root.join("tokens/buttons")).expect("button token dir");
        std::fs::write(root.join("tokens/theme.css"), ":root { --dx-gap: 1rem; }\n")
            .expect("theme import");
        std::fs::write(
            root.join("tokens/buttons/index.css"),
            ".button { display: inline-flex; }\n",
        )
        .expect("index import");

        let entry_path = root.join("styles/app.css");
        let source = concat!(
            "@import \"../tokens/theme\";\n",
            "@import \"../tokens/buttons\";\n",
            ".card { gap: var(--dx-gap); }\n",
        );

        let flattened =
            flatten_local_css_imports_with_manifest(root, &entry_path, source).expect("flatten");

        assert_eq!(flattened.imports.len(), 2);
        assert_eq!(flattened.imports[0].specifier, "../tokens/theme");
        assert_eq!(flattened.imports[0].path, PathBuf::from("tokens/theme.css"));
        assert_eq!(flattened.imports[1].specifier, "../tokens/buttons");
        assert_eq!(
            flattened.imports[1].path,
            PathBuf::from("tokens/buttons/index.css")
        );
        assert!(flattened.source.contains("--dx-gap: 1rem"));
        assert!(
            flattened
                .source
                .contains(".button { display: inline-flex; }")
        );
        assert!(!flattened.source.contains("@import \"../tokens/theme\";"));
        assert!(!flattened.source.contains("@import \"../tokens/buttons\";"));
    }

    #[test]
    fn retains_missing_local_imports_as_unresolved_boundaries() {
        let project = tempfile::tempdir().expect("temp project");
        let root = project.path();
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");

        let entry_path = root.join("styles/app.css");
        let source = concat!(
            "@import \"../tokens/missing\";\n",
            ".card { display: grid; }\n",
        );

        let flattened =
            flatten_local_css_imports_with_manifest(root, &entry_path, source).expect("flatten");

        assert!(flattened.imports.is_empty());
        assert_eq!(flattened.retained_imports.len(), 1);
        assert_eq!(flattened.retained_imports[0].specifier, "../tokens/missing");
        assert_eq!(flattened.retained_imports[0].reason, "unresolved-local");
        assert!(flattened.source.contains("@import \"../tokens/missing\";"));
        assert!(flattened.source.contains(".card { display: grid; }"));
    }
}
