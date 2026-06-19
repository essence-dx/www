use std::path::{Path, PathBuf};

use serde::Serialize;

use super::default_template_sources::{
    DEFAULT_TEMPLATE_CORE_SOURCE_FILES, DefaultTemplateSourceFile,
};
use crate::error::{DxError, DxResult};

pub(crate) const DEFAULT_TEMPLATE_CORE_SOURCE_RECEIPT_FILE: &str =
    ".dx/forge/template-core-sources.json";
const DEFAULT_TEMPLATE_CONTENT_HASH_ALGORITHM: &str = "blake3";
const TEMPLATE_LOCAL_TOOLING_FILE_NAMES: &[&str] = &[
    "package.json",
    "package-lock.json",
    "npm-shrinkwrap.json",
    "pnpm-lock.yaml",
    "yarn.lock",
    "bun.lock",
    "bun.lockb",
    "next.config.js",
    "next.config.cjs",
    "next.config.mjs",
    "next.config.ts",
    "source.config.ts",
    "next-env.d.ts",
    "tsconfig.json",
    "jsconfig.json",
];
const TEMPLATE_LOCAL_TOOLING_DIR_NAMES: &[&str] = &["node_modules", ".next"];

#[derive(Debug, Serialize)]
struct DefaultTemplateCoreSourcesReceipt<'a> {
    schema: &'static str,
    owner: &'static str,
    source: &'static str,
    node_modules_required: bool,
    content_hash_algorithm: &'static str,
    aggregate_content_hash: String,
    path_policy: &'static str,
    materialized_file_count: usize,
    files: Vec<DefaultTemplateCoreSourceReceiptFile<'a>>,
}

#[derive(Debug, Serialize)]
struct DefaultTemplateCoreSourceReceiptFile<'a> {
    source_file: &'a str,
    materialized_file: &'a str,
    role: &'a str,
    bytes: usize,
    content_hash_algorithm: &'static str,
    content_hash: String,
}

pub(crate) fn write_default_template_source_files(project_dir: &Path) -> DxResult<()> {
    write_default_template_source_file_set(project_dir, DEFAULT_TEMPLATE_CORE_SOURCE_FILES)
}

pub(crate) fn read_default_template_source_text(source_file: &str) -> DxResult<String> {
    let relative = Path::new(source_file);
    if source_file.is_empty() || relative.is_absolute() {
        return Err(DxError::ConfigValidationError {
            message: format!("Default template source path must be relative: {source_file}"),
            field: Some("source_file".to_string()),
        });
    }

    let mut attempted = Vec::new();
    for root in default_template_source_roots() {
        let mut candidates = vec![root.join(source_file)];
        if let Ok(template_relative) = relative.strip_prefix("examples/template") {
            candidates.push(root.join(template_relative));
        }

        for candidate in candidates {
            attempted.push(candidate.display().to_string());
            if candidate.is_file() {
                return std::fs::read_to_string(&candidate).map_err(|e| DxError::IoError {
                    path: Some(candidate),
                    message: e.to_string(),
                });
            }
        }
    }

    Err(DxError::ConfigValidationError {
        message: format!(
            "Default template source file `{source_file}` was not found. Tried: {}",
            attempted.join(", ")
        ),
        field: Some("source_file".to_string()),
    })
}

fn write_default_template_source_file_set(
    project_dir: &Path,
    source_files: &[DefaultTemplateSourceFile],
) -> DxResult<()> {
    let validated_targets = validate_default_template_source_files(project_dir, source_files)?;
    let mut files = Vec::with_capacity(source_files.len());
    let mut aggregate_hasher = blake3::Hasher::new();

    for (source_file, target) in source_files.iter().zip(validated_targets.iter()) {
        let content = read_default_template_source_text(source_file.source_file)?;
        write_template_source_file(target, &content)?;
        let content_hash = blake3::hash(content.as_bytes()).to_hex().to_string();
        update_aggregate_hash(&mut aggregate_hasher, source_file, &content_hash);
        files.push(DefaultTemplateCoreSourceReceiptFile {
            source_file: source_file.source_file,
            materialized_file: source_file.materialized_file,
            role: source_file.role,
            bytes: content.len(),
            content_hash_algorithm: DEFAULT_TEMPLATE_CONTENT_HASH_ALGORITHM,
            content_hash,
        });
    }

    let receipt = DefaultTemplateCoreSourcesReceipt {
        schema: "dx.www.default_template.core_sources",
        owner: "dx-www",
        source: "dx new default-template-materializer",
        node_modules_required: false,
        content_hash_algorithm: DEFAULT_TEMPLATE_CONTENT_HASH_ALGORITHM,
        aggregate_content_hash: aggregate_hasher.finalize().to_hex().to_string(),
        path_policy: "relative-no-traversal",
        materialized_file_count: files.len(),
        files,
    };
    let receipt_path = project_dir.join(DEFAULT_TEMPLATE_CORE_SOURCE_RECEIPT_FILE);
    if let Some(parent) = receipt_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: e.to_string(),
        })?;
    }
    std::fs::write(
        &receipt_path,
        serde_json::to_string_pretty(&receipt).map_err(|e| DxError::InternalError {
            message: e.to_string(),
        })?,
    )
    .map_err(|e| DxError::IoError {
        path: Some(receipt_path),
        message: e.to_string(),
    })?;

    Ok(())
}

fn validate_default_template_source_files(
    project_dir: &Path,
    source_files: &[DefaultTemplateSourceFile],
) -> DxResult<Vec<PathBuf>> {
    source_files
        .iter()
        .map(|source_file| safe_template_target_path(project_dir, source_file.materialized_file))
        .collect()
}

fn write_template_source_file(target: &Path, content: &str) -> DxResult<()> {
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).map_err(|e| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: e.to_string(),
        })?;
    }
    std::fs::write(target, content).map_err(|e| DxError::IoError {
        path: Some(target.to_path_buf()),
        message: e.to_string(),
    })
}

fn default_template_source_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Some(value) = std::env::var_os("DX_WWW_REPO_ROOT") {
        push_unique_path(&mut roots, PathBuf::from(value));
    }
    if let Some(value) = std::env::var_os("DX_WWW_TEMPLATE_ROOT") {
        let template_root = PathBuf::from(value);
        push_unique_path(&mut roots, template_root.clone());
        if let Some(parent) = template_root.parent().and_then(Path::parent) {
            push_unique_path(&mut roots, parent.to_path_buf());
        }
    }
    push_unique_path(
        &mut roots,
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from(".")),
    );
    if let Ok(exe) = std::env::current_exe() {
        for ancestor in exe.ancestors() {
            push_unique_path(&mut roots, ancestor.to_path_buf());
            push_unique_path(&mut roots, ancestor.join("www"));
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        for ancestor in cwd.ancestors() {
            push_unique_path(&mut roots, ancestor.to_path_buf());
        }
    }
    roots
}

fn push_unique_path(paths: &mut Vec<PathBuf>, path: PathBuf) {
    if !paths.iter().any(|existing| existing == &path) {
        paths.push(path);
    }
}

fn update_aggregate_hash(
    aggregate_hasher: &mut blake3::Hasher,
    source_file: &DefaultTemplateSourceFile,
    content_hash: &str,
) {
    aggregate_hasher.update(source_file.source_file.as_bytes());
    aggregate_hasher.update(b"\0");
    aggregate_hasher.update(source_file.materialized_file.as_bytes());
    aggregate_hasher.update(b"\0");
    aggregate_hasher.update(source_file.role.as_bytes());
    aggregate_hasher.update(b"\0");
    aggregate_hasher.update(content_hash.as_bytes());
    aggregate_hasher.update(b"\n");
}

fn safe_template_target_path(project_dir: &Path, materialized_file: &str) -> DxResult<PathBuf> {
    let path = Path::new(materialized_file);
    if materialized_file.is_empty() || path.is_absolute() {
        return Err(DxError::ConfigValidationError {
            message: format!("Default template source path must be relative: {materialized_file}"),
            field: Some("dx new default template".to_string()),
        });
    }
    reject_template_local_artifact_path(materialized_file)?;

    let mut target = project_dir.to_path_buf();
    for component in path.components() {
        match component {
            std::path::Component::Normal(part) => target.push(part),
            _ => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Default template source path cannot escape project: {materialized_file}"
                    ),
                    field: Some("dx new default template".to_string()),
                });
            }
        }
    }

    Ok(target)
}

fn reject_template_local_artifact_path(materialized_file: &str) -> DxResult<()> {
    let path = Path::new(materialized_file);
    if let Some(file_name) = path.file_name().and_then(|file_name| file_name.to_str()) {
        if TEMPLATE_LOCAL_TOOLING_FILE_NAMES
            .iter()
            .any(|blocked| file_name.eq_ignore_ascii_case(blocked))
        {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "Default template must not materialize template-local tooling artifact: {materialized_file}"
                ),
                field: Some("dx new default template".to_string()),
            });
        }
    }

    for component in path.components() {
        if let std::path::Component::Normal(part) = component {
            if let Some(part) = part.to_str() {
                if TEMPLATE_LOCAL_TOOLING_DIR_NAMES
                    .iter()
                    .any(|blocked| part.eq_ignore_ascii_case(blocked))
                {
                    return Err(DxError::ConfigValidationError {
                        message: format!(
                            "Default template must not materialize template-local tooling artifact: {materialized_file}"
                        ),
                        field: Some("dx new default template".to_string()),
                    });
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json::Value;

    use super::*;

    #[test]
    fn default_template_materializer_writes_source_receipt_hashes_and_files() {
        let project = tempfile::tempdir().expect("temp project");

        write_default_template_source_files(project.path()).expect("materialize template sources");

        let receipt_path = project
            .path()
            .join(DEFAULT_TEMPLATE_CORE_SOURCE_RECEIPT_FILE);
        let receipt: Value = serde_json::from_str(
            &fs::read_to_string(&receipt_path).expect("read core source receipt"),
        )
        .expect("parse core source receipt");

        assert_eq!(receipt["schema"], "dx.www.default_template.core_sources");
        assert_eq!(receipt["owner"], "dx-www");
        assert_eq!(receipt["source"], "dx new default-template-materializer");
        assert_eq!(receipt["node_modules_required"], false);
        assert_eq!(
            receipt["content_hash_algorithm"],
            DEFAULT_TEMPLATE_CONTENT_HASH_ALGORITHM
        );
        assert_eq!(receipt["path_policy"], "relative-no-traversal");
        assert_eq!(
            receipt["materialized_file_count"],
            DEFAULT_TEMPLATE_CORE_SOURCE_FILES.len()
        );

        let files = receipt["files"].as_array().expect("receipt files");
        assert_eq!(files.len(), DEFAULT_TEMPLATE_CORE_SOURCE_FILES.len());

        let mut aggregate_hasher = blake3::Hasher::new();
        for source_file in DEFAULT_TEMPLATE_CORE_SOURCE_FILES {
            let content =
                read_default_template_source_text(source_file.source_file).expect("read source");
            let materialized_path = project.path().join(source_file.materialized_file);
            assert_eq!(
                fs::read_to_string(&materialized_path).expect("read materialized source"),
                content
            );

            let content_hash = blake3::hash(content.as_bytes()).to_hex().to_string();
            update_aggregate_hash(&mut aggregate_hasher, source_file, &content_hash);

            let file_receipt = files
                .iter()
                .find(|file| {
                    file.get("materialized_file").and_then(Value::as_str)
                        == Some(source_file.materialized_file)
                })
                .expect("file receipt");
            assert_eq!(file_receipt["source_file"], source_file.source_file);
            assert_eq!(file_receipt["role"], source_file.role);
            assert_eq!(file_receipt["bytes"], content.len());
            assert_eq!(file_receipt["content_hash"], content_hash);
            assert_eq!(
                file_receipt["content_hash_algorithm"],
                DEFAULT_TEMPLATE_CONTENT_HASH_ALGORITHM
            );
        }

        assert_eq!(
            receipt["aggregate_content_hash"],
            aggregate_hasher.finalize().to_hex().to_string()
        );
    }

    #[test]
    fn default_template_materializer_rejects_template_local_tooling_artifacts() {
        let project = tempfile::tempdir().expect("temp project");

        for materialized_file in [
            "package.json",
            "app/package.json",
            "package-lock.json",
            "npm-shrinkwrap.json",
            "pnpm-lock.yaml",
            "yarn.lock",
            "bun.lock",
            "bun.lockb",
            "next.config.js",
            "next.config.cjs",
            "next.config.mjs",
            "next.config.ts",
            "next-env.d.ts",
            "tsconfig.json",
            "jsconfig.json",
            "node_modules/pkg/index.js",
            ".next/server/app.js",
        ] {
            let error = safe_template_target_path(project.path(), materialized_file)
                .expect_err("template-local artifact should be rejected");
            assert!(
                error
                    .to_string()
                    .contains("template-local tooling artifact"),
                "{materialized_file}: {error}"
            );
        }
    }

    #[test]
    fn default_template_materializer_rejects_invalid_source_set_before_writing() {
        let project = tempfile::tempdir().expect("temp project");
        let source_files = [
            DefaultTemplateSourceFile {
                source_file: "test#WelcomeCard",
                materialized_file: "components/local/WelcomeCard.tsx",
                role: "test-valid-starter-file",
            },
            DefaultTemplateSourceFile {
                source_file: "test#PackageJson",
                materialized_file: "package.json",
                role: "bad-template-local-package",
            },
        ];

        let error = write_default_template_source_file_set(project.path(), &source_files)
            .expect_err("invalid template source set should fail before writes");

        assert!(
            error
                .to_string()
                .contains("template-local tooling artifact")
        );
        assert!(
            !project
                .path()
                .join("components/local/WelcomeCard.tsx")
                .exists()
        );
        assert!(
            !project
                .path()
                .join(DEFAULT_TEMPLATE_CORE_SOURCE_RECEIPT_FILE)
                .exists()
        );
    }
}
