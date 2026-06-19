use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::graph::{
    SourceBuildContentDocument, SourceBuildContentFrontmatter, SourceBuildMdxCompatibilityOptions,
    hash_bytes, normalize_path, read_file, relative_path,
};

pub fn compile_content_documents(
    project_root: &Path,
    documents: &[PathBuf],
) -> DxResult<Vec<SourceBuildContentDocument>> {
    let mut compiled = Vec::new();

    for document in documents {
        let bytes = read_file(document)?;
        let source =
            String::from_utf8(bytes.clone()).map_err(|error| DxError::CompilationError {
                message: error.to_string(),
                file: document.to_path_buf(),
                src: None,
                span: None,
            })?;
        let relative = relative_path(project_root, document);
        let kind = content_kind(document);
        let mdx_options = (kind == "mdx").then(|| mdx_compatibility_options(&source));

        compiled.push(SourceBuildContentDocument {
            path: normalize_path(&relative),
            kind: kind.to_string(),
            hash: hash_bytes(&bytes),
            size: bytes.len() as u64,
            frontmatter: frontmatter(&source),
            heading_count: heading_count(&source),
            code_block_count: fenced_code_block_count(&source),
            mdx_options,
            node_modules_required: false,
            runtime_proof: false,
            adapter_boundary: "source-receipt-only-no-mdx-compile-evaluate-or-react-runtime"
                .to_string(),
        });
    }

    Ok(compiled)
}

fn content_kind(path: &Path) -> &'static str {
    match path
        .extension()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("mdx") => "mdx",
        _ => "markdown",
    }
}

fn frontmatter(source: &str) -> SourceBuildContentFrontmatter {
    if has_frontmatter_block(source, "---") {
        return SourceBuildContentFrontmatter {
            present: true,
            format: Some("yaml".to_string()),
        };
    }
    if has_frontmatter_block(source, "+++") {
        return SourceBuildContentFrontmatter {
            present: true,
            format: Some("toml".to_string()),
        };
    }

    SourceBuildContentFrontmatter {
        present: false,
        format: None,
    }
}

fn has_frontmatter_block(source: &str, marker: &str) -> bool {
    let mut lines = source.lines();
    if lines.next().map(str::trim) != Some(marker) {
        return false;
    }

    lines.any(|line| line.trim() == marker)
}

fn heading_count(source: &str) -> usize {
    source.lines().filter(|line| is_heading_line(line)).count()
}

fn is_heading_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    let hashes = trimmed
        .chars()
        .take_while(|character| *character == '#')
        .count();
    (1..=6).contains(&hashes) && trimmed.chars().nth(hashes) == Some(' ')
}

fn fenced_code_block_count(source: &str) -> usize {
    let fence_lines = source
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            trimmed.starts_with("```") || trimmed.starts_with("~~~")
        })
        .count();
    fence_lines / 2
}

fn mdx_compatibility_options(source: &str) -> SourceBuildMdxCompatibilityOptions {
    SourceBuildMdxCompatibilityOptions {
        schema: "dx.www.mdxCompatibilityOptions".to_string(),
        informed_by: "turbopack-mdx::MdxTransformOptions".to_string(),
        provider_import_source: "@mdx-js/react".to_string(),
        provider_import_required: false,
        react_runtime_required: false,
        rsc_required: false,
        node_modules_required: false,
        full_mdx_pipeline_parity: false,
        development: true,
        jsx: false,
        jsx_runtime: None,
        jsx_import_source: None,
        mdx_type: if uses_gfm_constructs(source) {
            "gfm".to_string()
        } else {
            "commonmark".to_string()
        },
        transform_status: "adapter-boundary-no-mdx-compile-executed".to_string(),
    }
}

fn uses_gfm_constructs(source: &str) -> bool {
    source.lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with("- [ ] ")
            || trimmed.starts_with("- [x] ")
            || (trimmed.starts_with('|') && trimmed.ends_with('|'))
    })
}
