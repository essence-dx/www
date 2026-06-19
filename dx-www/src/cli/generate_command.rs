use std::path::Path;

use crate::error::{DxError, DxResult};

use super::naming::to_pascal_case;

pub(super) fn cmd_generate(cwd: &Path, gen_type: &str, name: &str) -> DxResult<()> {
    match gen_type {
        "page" | "p" => generate_page(cwd, name),
        "component" | "c" => generate_component(cwd, name),
        "api" | "a" => generate_api(cwd, name),
        "layout" | "l" => generate_layout(cwd, name),
        _ => Err(DxError::ConfigValidationError {
            message: format!("Unknown generator type: {gen_type}"),
            field: Some("type".to_string()),
        }),
    }
}

fn generate_page(cwd: &Path, name: &str) -> DxResult<()> {
    let path = cwd.join("pages").join(format!("{name}.html"));

    if path.exists() {
        return Err(DxError::IoError {
            path: Some(path.clone()),
            message: "File already exists".to_string(),
        });
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: e.to_string(),
        })?;
    }

    let content = format!(
        r#"<script lang="rust">
pub struct Props {{
    // Define props here
}}

pub async fn load() -> Props {{
    Props {{}}
}}
</script>

<page>
    <div class="p-8">
        <h1 class="text-3xl font-bold">{}</h1>
    </div>
</page>
"#,
        name
    );

    std::fs::write(&path, content).map_err(|e| DxError::IoError {
        path: Some(path.clone()),
        message: e.to_string(),
    })?;

    eprintln!("? Created pages/{name}.html");
    Ok(())
}

fn generate_component(cwd: &Path, name: &str) -> DxResult<()> {
    let pascal_name = to_pascal_case(name);
    let path = cwd.join("components").join(format!("{pascal_name}.tsx"));

    if path.exists() {
        return Err(DxError::IoError {
            path: Some(path.clone()),
            message: "File already exists".to_string(),
        });
    }

    let content = r#"<script lang="rust">
pub struct Props {
    // Define props here
}
</script>

<component>
    <div class="p-4">
        <!-- Component content -->
    </div>
</component>
"#;

    std::fs::write(&path, content).map_err(|e| DxError::IoError {
        path: Some(path.clone()),
        message: e.to_string(),
    })?;

    eprintln!("? Created components/{pascal_name}.tsx");
    Ok(())
}

fn generate_api(cwd: &Path, name: &str) -> DxResult<()> {
    let path = cwd.join("api").join(format!("{name}.rs"));

    if path.exists() {
        return Err(DxError::IoError {
            path: Some(path.clone()),
            message: "File already exists".to_string(),
        });
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: e.to_string(),
        })?;
    }

    let content = r#"use dx_www::prelude::*;

/// Handle GET requests.
pub async fn get(req: Request) -> Response {
    Response::json(&serde_json::json!({
        "message": "Hello from API"
    }))
}

/// Handle POST requests.
pub async fn post(req: Request) -> Response {
    let body: serde_json::Value = req.json().await?;
    Response::json(&body)
}
"#;

    std::fs::write(&path, content).map_err(|e| DxError::IoError {
        path: Some(path.clone()),
        message: e.to_string(),
    })?;

    eprintln!("? Created api/{name}.rs");
    Ok(())
}

fn generate_layout(cwd: &Path, name: &str) -> DxResult<()> {
    let dir = cwd.join("pages").join(name);
    let path = dir.join("_layout.html");

    if path.exists() {
        return Err(DxError::IoError {
            path: Some(path.clone()),
            message: "File already exists".to_string(),
        });
    }

    std::fs::create_dir_all(&dir).map_err(|e| DxError::IoError {
        path: Some(dir.clone()),
        message: e.to_string(),
    })?;

    let content = r#"<script lang="rust">
pub struct Props {
    children: Children,
}
</script>

<page>
    <div class="min-h-screen">
        <slot />
    </div>
</page>
"#;

    std::fs::write(&path, content).map_err(|e| DxError::IoError {
        path: Some(path.clone()),
        message: e.to_string(),
    })?;

    eprintln!("? Created pages/{name}/_layout.html");
    Ok(())
}
