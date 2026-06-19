use super::native_shell_naming::{kebab_case, native_lib_name};

#[derive(Debug)]
pub(super) struct NativeShellFile {
    pub(super) relative_path: &'static str,
    pub(super) contents: String,
}

pub(super) fn native_shell_files(
    project_name: &str,
    product_name: &str,
    identifier: &str,
    bridge: &str,
    dev_url: &str,
    tauri_crate: &str,
    tauri_build_crate: &str,
) -> Vec<NativeShellFile> {
    vec![
        NativeShellFile {
            relative_path: "src-tauri/Cargo.toml",
            contents: cargo_toml(project_name, tauri_crate, tauri_build_crate),
        },
        NativeShellFile {
            relative_path: "src-tauri/build.rs",
            contents: "fn main() {\n  tauri_build::build();\n}\n".to_string(),
        },
        NativeShellFile {
            relative_path: "src-tauri/src/main.rs",
            contents: main_rs(project_name),
        },
        NativeShellFile {
            relative_path: "src-tauri/src/lib.rs",
            contents: lib_rs(project_name, bridge),
        },
        NativeShellFile {
            relative_path: "src-tauri/tauri.conf.json",
            contents: tauri_config(product_name, identifier, dev_url),
        },
    ]
}

fn cargo_toml(project_name: &str, tauri_crate: &str, tauri_build_crate: &str) -> String {
    let native_name = format!("{}-native", kebab_case(project_name));
    let native_lib_name = native_lib_name(project_name);

    format!(
        r#"[package]
name = "{native_name}"
version = "0.1.0"
edition = "2021"
rust-version = "1.77.2"
publish = false

[lib]
name = "{native_lib_name}"
crate-type = ["staticlib", "cdylib", "rlib"]

[workspace]

[build-dependencies]
tauri-build = {{ path = "{tauri_build_crate}" }}

[dependencies]
serde = {{ version = "1", features = ["derive"] }}
tauri = {{ path = "{tauri_crate}" }}
"#,
        native_name = native_name,
        native_lib_name = native_lib_name,
    )
}

fn main_rs(project_name: &str) -> String {
    format!(
        r#"#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {{
  {}::run();
}}
"#,
        native_lib_name(project_name)
    )
}

fn lib_rs(project_name: &str, bridge: &str) -> String {
    format!(
        r#"use serde::Serialize;

#[derive(Serialize)]
struct DxNativeShellContract {{
  app: &'static str,
  www_route: &'static str,
  native_bridge: &'static str,
  native_shell: &'static str,
}}

#[tauri::command]
fn dx_native_shell_contract() -> DxNativeShellContract {{
  DxNativeShellContract {{
    app: "{project_name}",
    www_route: "/",
    native_bridge: "{bridge}",
    native_shell: "tauri-webview",
  }}
}}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {{
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![dx_native_shell_contract])
    .run(tauri::generate_context!())
    .expect("failed to run DX WWW native shell");
}}
"#
    )
}

fn tauri_config(product_name: &str, identifier: &str, dev_url: &str) -> String {
    serde_json::to_string_pretty(&serde_json::json!({
        "$schema": "../../../native/crates/tauri-schema-generator/schemas/config.schema.json",
        "productName": product_name,
        "version": "0.1.0",
        "identifier": identifier,
        "build": {
            "frontendDist": "../.dx/www/output",
            "devUrl": dev_url,
            "beforeDevCommand": format!("dx dev --host 127.0.0.1 --port {}", dev_url.rsplit(':').next().unwrap_or("3000")),
            "beforeBuildCommand": "dx www build",
            "removeUnusedCommands": true
        },
        "app": {
            "withGlobalTauri": true,
            "windows": [{
                "title": product_name,
                "width": 420,
                "height": 760,
                "minWidth": 360,
                "minHeight": 640,
                "resizable": true,
                "fullscreen": false
            }],
            "security": {
                "csp": format!(
                    "default-src 'self'; connect-src ipc: http://ipc.localhost {}; img-src 'self' asset: http://asset.localhost data:; style-src 'self' 'unsafe-inline'",
                    dev_url
                )
            }
        },
        "bundle": {
            "active": true,
            "targets": "all"
        }
    }))
    .map(|json| format!("{json}\n"))
    .unwrap_or_else(|_| "{}\n".to_string())
}

#[cfg(test)]
mod tests {
    use super::{NativeShellFile, native_shell_files};

    fn file_contents<'a>(files: &'a [NativeShellFile], path: &str) -> Result<&'a str, String> {
        files
            .iter()
            .find(|file| file.relative_path == path)
            .map(|file| file.contents.as_str())
            .ok_or_else(|| format!("missing native shell file: {path}"))
    }

    #[test]
    fn native_shell_files_emit_mobile_ready_tauri_entrypoints() -> Result<(), String> {
        let files = native_shell_files(
            "dx-mobile-app",
            "DX Mobile App",
            "com.essencefromexistence.dx.mobile",
            "dx-native-mobile-auth",
            "http://127.0.0.1:3000",
            "../native/crates/tauri",
            "../native/crates/tauri-build",
        );

        let cargo_toml = file_contents(&files, "src-tauri/Cargo.toml")?;
        let build_rs = file_contents(&files, "src-tauri/build.rs")?;
        let main_rs = file_contents(&files, "src-tauri/src/main.rs")?;
        let lib_rs = file_contents(&files, "src-tauri/src/lib.rs")?;
        let tauri_config = file_contents(&files, "src-tauri/tauri.conf.json")?;
        let normalized_lib_rs = lib_rs
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect::<String>();
        let cargo_manifest: toml::Value =
            toml::from_str(cargo_toml).map_err(|error| error.to_string())?;
        let tauri_config: serde_json::Value =
            serde_json::from_str(tauri_config).map_err(|error| error.to_string())?;
        let tauri_dependency_path = cargo_manifest
            .get("dependencies")
            .and_then(toml::Value::as_table)
            .and_then(|dependencies| dependencies.get("tauri"))
            .and_then(toml::Value::as_table)
            .and_then(|tauri| tauri.get("path"))
            .and_then(toml::Value::as_str);
        let native_lib_name = cargo_manifest
            .get("lib")
            .and_then(toml::Value::as_table)
            .and_then(|lib| lib.get("name"))
            .and_then(toml::Value::as_str)
            .ok_or_else(|| "missing native shell [lib].name".to_string())?;

        assert!(cargo_toml.contains("[lib]"));
        assert_eq!(native_lib_name, "dx_mobile_app_native_lib");
        assert!(cargo_toml.contains(r#"crate-type = ["staticlib", "cdylib", "rlib"]"#));
        assert!(cargo_toml.contains("[build-dependencies]"));
        assert!(cargo_toml.contains(r#"tauri-build = { path = "../native/crates/tauri-build" }"#));
        assert_eq!(tauri_dependency_path, Some("../native/crates/tauri"));
        assert!(build_rs.contains("tauri_build::build();"));
        assert!(main_rs.contains(&format!("{native_lib_name}::run();")));
        assert!(lib_rs.contains("tauri::mobile_entry_point"));
        assert!(lib_rs.contains("pub fn run()"));
        assert!(normalized_lib_rs.contains("tauri::Builder::default()"));
        assert!(normalized_lib_rs.contains(".run(tauri::generate_context!())"));
        assert!(lib_rs.contains("dx-native-mobile-auth"));
        assert_eq!(
            tauri_config
                .pointer("/build/devUrl")
                .and_then(serde_json::Value::as_str),
            Some("http://127.0.0.1:3000")
        );
        assert_eq!(
            tauri_config
                .pointer("/build/beforeDevCommand")
                .and_then(serde_json::Value::as_str),
            Some("dx dev --host 127.0.0.1 --port 3000")
        );
        assert_eq!(
            tauri_config
                .pointer("/app/withGlobalTauri")
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );
        assert!(
            tauri_config
                .pointer("/app/windows")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|windows| !windows.is_empty())
        );
        let csp = tauri_config
            .pointer("/app/security/csp")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| "missing native shell app.security.csp".to_string())?;
        for required_token in [
            "default-src 'self'",
            "connect-src",
            "ipc:",
            "http://ipc.localhost",
            "img-src",
            "asset:",
            "http://asset.localhost",
            "style-src",
            "'unsafe-inline'",
        ] {
            assert!(
                csp.contains(required_token),
                "missing CSP token: {required_token}"
            );
        }

        Ok(())
    }
}
