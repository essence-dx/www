use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use super::css_imports::FlattenedCss;
use super::graph::{SourceBuildStyleAssetReference, normalize_path, relative_path};

pub fn collect_flattened_css_asset_references(
    project_root: &Path,
    style_path: &Path,
    entry_source: &str,
    flattened: &FlattenedCss,
) -> Vec<SourceBuildStyleAssetReference> {
    let mut seen_paths = BTreeSet::new();
    let mut references = Vec::new();

    extend_css_asset_references(
        &mut references,
        &mut seen_paths,
        collect_css_asset_references(project_root, style_path, entry_source),
    );

    for import in &flattened.imports {
        let import_path = project_root.join(&import.path);
        extend_css_asset_references(
            &mut references,
            &mut seen_paths,
            collect_css_asset_references_with_origin(
                project_root,
                &import_path,
                &import.source,
                "flattened-import",
                Some(&import.specifier),
            ),
        );
    }

    for import in &flattened.retained_imports {
        let (Some(import_path), Some(import_source)) = (&import.path, &import.source) else {
            continue;
        };
        let import_path = project_root.join(import_path);
        extend_css_asset_references(
            &mut references,
            &mut seen_paths,
            collect_css_asset_references_with_origin(
                project_root,
                &import_path,
                import_source,
                "retained-import",
                Some(&import.specifier),
            ),
        );
    }

    references
}

pub fn collect_css_asset_references(
    project_root: &Path,
    style_path: &Path,
    source: &str,
) -> Vec<SourceBuildStyleAssetReference> {
    collect_css_asset_references_with_origin(project_root, style_path, source, "", None)
}

fn collect_css_asset_references_with_origin(
    project_root: &Path,
    style_path: &Path,
    source: &str,
    source_role: &str,
    import_specifier: Option<&str>,
) -> Vec<SourceBuildStyleAssetReference> {
    let canonical_root = project_root
        .canonicalize()
        .unwrap_or_else(|_| project_root.to_path_buf());
    let canonical_style = canonical_source_path(style_path);
    let source_path = normalize_path(&relative_path(&canonical_root, &canonical_style));
    let mut seen = BTreeSet::new();
    let mut references = Vec::new();

    for specifier in css_url_specifiers(source) {
        let Some(asset_path) = resolve_public_asset(project_root, style_path, &specifier) else {
            continue;
        };
        let relative = normalize_path(&relative_path(&canonical_root, &asset_path));
        if !seen.insert(relative.clone()) {
            continue;
        }
        references.push(SourceBuildStyleAssetReference {
            specifier,
            path: relative,
            source_path: source_path.clone(),
            source_role: source_role.to_string(),
            import_specifier: import_specifier.map(str::to_string),
            kind: "css-url".to_string(),
            node_modules_required: false,
        });
    }

    references
}

fn canonical_source_path(path: &Path) -> PathBuf {
    if let Ok(canonical) = path.canonicalize() {
        return canonical;
    }
    let Some(parent) = path.parent() else {
        return path.to_path_buf();
    };
    let Ok(canonical_parent) = parent.canonicalize() else {
        return path.to_path_buf();
    };
    path.file_name()
        .map(|file_name| canonical_parent.join(file_name))
        .unwrap_or(canonical_parent)
}

fn extend_css_asset_references(
    references: &mut Vec<SourceBuildStyleAssetReference>,
    seen_paths: &mut BTreeSet<String>,
    source_references: Vec<SourceBuildStyleAssetReference>,
) {
    for reference in source_references {
        if seen_paths.insert(reference.path.clone()) {
            references.push(reference);
        }
    }
}

fn css_url_specifiers(source: &str) -> Vec<String> {
    let mut rest = source;
    let mut specifiers = Vec::new();

    while let Some(start) = rest.find("url(") {
        rest = &rest[start + "url(".len()..];
        let Some(end) = rest.find(')') else {
            break;
        };
        if let Some(specifier) = css_url_specifier(&rest[..end]) {
            specifiers.push(specifier);
        }
        rest = &rest[end + 1..];
    }

    specifiers
}

fn css_url_specifier(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    let unquoted = trimmed
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .or_else(|| {
            trimmed
                .strip_prefix('\'')
                .and_then(|value| value.strip_suffix('\''))
        })
        .unwrap_or(trimmed)
        .trim();
    if unquoted.is_empty() || ignored_asset_specifier(unquoted) {
        None
    } else {
        Some(unquoted.to_string())
    }
}

fn resolve_public_asset(
    project_root: &Path,
    style_path: &Path,
    specifier: &str,
) -> Option<PathBuf> {
    let clean = specifier.split(['?', '#']).next()?.trim();
    if clean.is_empty() || ignored_asset_specifier(clean) {
        return None;
    }

    let candidate = if let Some(rooted) = clean.strip_prefix('/') {
        if rooted.starts_with("public/") {
            project_root.join(rooted)
        } else {
            project_root.join("public").join(rooted)
        }
    } else if clean.starts_with('.') {
        style_path.parent()?.join(clean)
    } else {
        return None;
    };

    verified_public_asset(project_root, candidate)
}

fn verified_public_asset(project_root: &Path, candidate: PathBuf) -> Option<PathBuf> {
    if !candidate.is_file() {
        return None;
    }
    let public_root = project_root.join("public").canonicalize().ok()?;
    let canonical_candidate = candidate.canonicalize().ok()?;
    canonical_candidate
        .starts_with(public_root)
        .then_some(canonical_candidate)
}

fn ignored_asset_specifier(specifier: &str) -> bool {
    specifier.starts_with('#')
        || specifier.starts_with("//")
        || specifier.starts_with("http:")
        || specifier.starts_with("https:")
        || specifier.starts_with("data:")
        || specifier.starts_with("blob:")
}

#[cfg(test)]
mod tests {
    use super::super::css_imports::{FlattenedCss, FlattenedCssImport, RetainedCssImport};
    use super::*;

    #[test]
    fn collects_root_public_css_url_references() {
        let project = tempfile::tempdir().expect("temp project");
        let root = project.path();
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");
        std::fs::create_dir_all(root.join("public/icons")).expect("public dir");
        std::fs::write(root.join("public/icons/mark.svg"), "<svg />").expect("asset");

        let references = collect_css_asset_references(
            root,
            &root.join("styles/app.css"),
            ".hero { background-image: url(\"/icons/mark.svg?v=1#hash\"); }\n",
        );

        assert_eq!(references.len(), 1);
        assert_eq!(references[0].specifier, "/icons/mark.svg?v=1#hash");
        assert_eq!(references[0].path, "public/icons/mark.svg");
        assert_eq!(references[0].source_path, "styles/app.css");
        assert_eq!(references[0].source_role, "");
        assert_eq!(references[0].import_specifier, None);
        assert_eq!(references[0].kind, "css-url");
        assert!(!references[0].node_modules_required);
    }

    #[test]
    fn collects_relative_css_url_references_from_flattened_import_origins() {
        let project = tempfile::tempdir().expect("temp project");
        let root = project.path();
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");
        std::fs::create_dir_all(root.join("components/card")).expect("component dir");
        std::fs::create_dir_all(root.join("public/icons")).expect("public dir");
        std::fs::write(root.join("public/icons/card.svg"), "<svg />").expect("asset");

        let flattened = FlattenedCss {
            source: ".card { display: grid; }\n".to_string(),
            imports: vec![FlattenedCssImport {
                specifier: "../components/card/card.css".to_string(),
                path: PathBuf::from("components/card/card.css"),
                source: ".card { background-image: url(\"../../public/icons/card.svg\"); }\n"
                    .to_string(),
            }],
            retained_imports: Vec::new(),
        };

        let references = collect_flattened_css_asset_references(
            root,
            &root.join("styles/app.css"),
            ".app { display: block; }\n",
            &flattened,
        );

        assert_eq!(references.len(), 1);
        assert_eq!(references[0].specifier, "../../public/icons/card.svg");
        assert_eq!(references[0].path, "public/icons/card.svg");
        assert_eq!(references[0].source_path, "components/card/card.css");
        assert_eq!(references[0].source_role, "flattened-import");
        assert_eq!(
            references[0].import_specifier.as_deref(),
            Some("../components/card/card.css")
        );
        assert_eq!(references[0].kind, "css-url");
        assert!(!references[0].node_modules_required);
    }

    #[test]
    fn collects_relative_css_url_references_from_retained_import_origins() {
        let project = tempfile::tempdir().expect("temp project");
        let root = project.path();
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");
        std::fs::create_dir_all(root.join("tokens")).expect("token dir");
        std::fs::create_dir_all(root.join("public/icons")).expect("public dir");
        std::fs::write(root.join("public/icons/layer.svg"), "<svg />").expect("asset");

        let flattened = FlattenedCss {
            source: "@import \"../tokens/layered.css\" layer(theme);\n".to_string(),
            imports: Vec::new(),
            retained_imports: vec![RetainedCssImport {
                specifier: "../tokens/layered.css".to_string(),
                path: Some(PathBuf::from("tokens/layered.css")),
                source: Some(
                    ".card { mask-image: url(\"../public/icons/layer.svg\"); }\n".to_string(),
                ),
                condition: Some("layer(theme)".to_string()),
                reason: "conditional-or-media",
            }],
        };

        let references = collect_flattened_css_asset_references(
            root,
            &root.join("styles/app.css"),
            ".app { display: block; }\n",
            &flattened,
        );

        assert_eq!(references.len(), 1);
        assert_eq!(references[0].specifier, "../public/icons/layer.svg");
        assert_eq!(references[0].path, "public/icons/layer.svg");
        assert_eq!(references[0].source_path, "tokens/layered.css");
        assert_eq!(references[0].source_role, "retained-import");
        assert_eq!(
            references[0].import_specifier.as_deref(),
            Some("../tokens/layered.css")
        );
        assert_eq!(references[0].kind, "css-url");
        assert!(!references[0].node_modules_required);
    }
}
