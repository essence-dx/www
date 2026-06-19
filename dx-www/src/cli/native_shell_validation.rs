use std::path::Path;

use crate::error::{DxError, DxResult};

const REQUIRED_TAURI_CSP_TOKENS: &[&str] = &[
    "default-src 'self'",
    "connect-src",
    "ipc:",
    "http://ipc.localhost",
    "img-src",
    "asset:",
    "http://asset.localhost",
    "style-src",
    "'unsafe-inline'",
];

pub(super) fn validate_www_project(project_root: &Path) -> DxResult<()> {
    if !project_root.is_dir() {
        return Err(DxError::ProjectNotFound {
            path: project_root.to_path_buf(),
        });
    }
    for required in ["dx", "app", "styles"] {
        if !project_root.join(required).exists() {
            return Err(DxError::InvalidProjectStructure {
                message: format!(
                    "dx www native-shell requires a DX-WWW project with `{required}` at {}",
                    project_root.display()
                ),
            });
        }
    }
    Ok(())
}

pub(super) fn validate_mobile_native_shell(project_root: &Path) -> DxResult<()> {
    let cargo_toml_path = project_root.join("src-tauri/Cargo.toml");
    let cargo_toml =
        std::fs::read_to_string(&cargo_toml_path).map_err(|error| DxError::IoError {
            path: Some(cargo_toml_path.clone()),
            message: error.to_string(),
        })?;
    let cargo_toml: toml::Value =
        toml::from_str(&cargo_toml).map_err(|error| DxError::ConfigValidationError {
            message: format!("Invalid src-tauri/Cargo.toml for Android build: {error}"),
            field: Some("build.target.android.native_shell".to_string()),
        })?;
    let crate_types = mobile_crate_types(&cargo_toml);
    if !["staticlib", "cdylib", "rlib"]
        .iter()
        .all(|required| crate_types.contains(required))
    {
        return Err(DxError::ConfigValidationError {
            message: "Android builds require src-tauri/Cargo.toml to expose a mobile-compatible [lib] crate; regenerate the native shell with dx www native-shell --write --force after reviewing local native changes".to_string(),
            field: Some("build.target.android.native_shell".to_string()),
        });
    }
    let native_lib_name =
        mobile_lib_name(&cargo_toml).ok_or_else(native_shell_mobile_lib_name_error)?;
    if !has_tauri_build_dependency(&cargo_toml) {
        return Err(native_shell_tauri_build_dependency_error());
    }
    if !has_tauri_runtime_dependency(&cargo_toml) {
        return Err(native_shell_tauri_runtime_dependency_error());
    }

    validate_native_shell_build_script(project_root)?;
    validate_native_shell_lib_rs(project_root)?;
    validate_native_shell_main_rs(project_root, native_lib_name)?;

    validate_native_shell_tauri_config(project_root)?;

    Ok(())
}

fn validate_native_shell_build_script(project_root: &Path) -> DxResult<()> {
    let build_rs_path = project_root.join("src-tauri/build.rs");
    if !build_rs_path.is_file() {
        return Err(native_shell_build_script_error());
    }

    let build_rs = std::fs::read_to_string(&build_rs_path).map_err(|error| DxError::IoError {
        path: Some(build_rs_path),
        message: error.to_string(),
    })?;
    if !has_tauri_build_invocation(&build_rs) {
        return Err(native_shell_build_script_error());
    }

    Ok(())
}

fn has_tauri_build_invocation(build_rs: &str) -> bool {
    normalized_rust_source(build_rs).contains("tauri_build::build()")
}

fn validate_native_shell_lib_rs(project_root: &Path) -> DxResult<()> {
    let lib_rs_path = project_root.join("src-tauri/src/lib.rs");
    let lib_rs = std::fs::read_to_string(&lib_rs_path).map_err(|error| DxError::IoError {
        path: Some(lib_rs_path.clone()),
        message: error.to_string(),
    })?;
    if !has_tauri_mobile_entrypoint(&lib_rs) {
        return Err(native_shell_mobile_entrypoint_error());
    }
    if !has_tauri_runtime_launch(&lib_rs) {
        return Err(native_shell_runtime_launch_error());
    }

    Ok(())
}

fn has_tauri_mobile_entrypoint(lib_rs: &str) -> bool {
    let normalized = normalized_rust_source(lib_rs);
    normalized.contains("tauri::mobile_entry_point") && normalized.contains("pubfnrun()")
}

fn has_tauri_runtime_launch(lib_rs: &str) -> bool {
    let source = strip_rust_comments(lib_rs);
    let Some(run_body) = pub_run_body(&source) else {
        return false;
    };
    let normalized = normalized_source(run_body);
    normalized.contains("tauri::Builder::default()")
        && normalized.contains(".run(tauri::generate_context!())")
}

fn validate_native_shell_main_rs(project_root: &Path, native_lib_name: &str) -> DxResult<()> {
    let main_rs_path = project_root.join("src-tauri/src/main.rs");
    if !main_rs_path.is_file() {
        return Err(native_shell_desktop_entrypoint_error(native_lib_name));
    }

    let main_rs = std::fs::read_to_string(&main_rs_path).map_err(|error| DxError::IoError {
        path: Some(main_rs_path.clone()),
        message: error.to_string(),
    })?;
    if !has_desktop_entrypoint_run_call(&main_rs, native_lib_name) {
        return Err(native_shell_desktop_entrypoint_error(native_lib_name));
    }

    Ok(())
}

fn has_desktop_entrypoint_run_call(main_rs: &str, native_lib_name: &str) -> bool {
    let source = strip_rust_comments(main_rs);
    let Some(main_body) = function_body(&source, "fn main()") else {
        return false;
    };

    normalized_source(main_body).contains(&format!("{native_lib_name}::run()"))
}

fn normalized_rust_source(source: &str) -> String {
    normalized_source(&strip_rust_comments(source))
}

fn normalized_source(source: &str) -> String {
    source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

fn pub_run_body(source: &str) -> Option<&str> {
    function_body(source, "pub fn run()")
}

fn function_body<'a>(source: &'a str, signature: &str) -> Option<&'a str> {
    let function_start = source.find(signature)?;
    let body_start = function_start + source[function_start..].find('{')?;
    let mut depth = 0usize;

    for (offset, character) in source[body_start..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    let body_contents_start = body_start + 1;
                    return source.get(body_contents_start..body_start + offset);
                }
            }
            _ => {}
        }
    }

    None
}

fn strip_rust_comments(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    let mut characters = source.chars().peekable();
    let mut in_block_comment = false;

    while let Some(character) = characters.next() {
        if in_block_comment {
            if character == '*' && characters.peek() == Some(&'/') {
                characters.next();
                in_block_comment = false;
            }
            continue;
        }

        if character == '/' && characters.peek() == Some(&'/') {
            for line_character in characters.by_ref() {
                if line_character == '\n' {
                    output.push('\n');
                    break;
                }
            }
            continue;
        }

        if character == '/' && characters.peek() == Some(&'*') {
            characters.next();
            in_block_comment = true;
            continue;
        }

        output.push(character);
    }

    output
}

pub(super) fn validate_native_root(native_root: &Path) -> DxResult<()> {
    for required in ["crates/tauri", "crates/tauri-build"] {
        if !native_root.join(required).is_dir() {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Native root is missing `{required}`: {}",
                    native_root.display()
                ),
                field: Some("native-shell.native-root".to_string()),
            });
        }
    }
    Ok(())
}

fn mobile_crate_types(cargo_toml: &toml::Value) -> Vec<&str> {
    cargo_toml
        .get("lib")
        .and_then(|lib| lib.get("crate-type"))
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(toml::Value::as_str)
        .collect()
}

fn mobile_lib_name(cargo_toml: &toml::Value) -> Option<&str> {
    cargo_toml
        .get("lib")
        .and_then(|lib| lib.get("name"))
        .and_then(toml::Value::as_str)
        .filter(|name| !name.trim().is_empty())
}

fn has_tauri_build_dependency(cargo_toml: &toml::Value) -> bool {
    has_cargo_dependency(cargo_toml, "build-dependencies", "tauri-build")
}

fn has_tauri_runtime_dependency(cargo_toml: &toml::Value) -> bool {
    has_cargo_dependency(cargo_toml, "dependencies", "tauri")
}

fn has_cargo_dependency(cargo_toml: &toml::Value, section: &str, name: &str) -> bool {
    cargo_toml
        .get(section)
        .and_then(toml::Value::as_table)
        .and_then(|dependencies| dependencies.get(name))
        .is_some_and(is_valid_cargo_dependency_entry)
}

fn is_valid_cargo_dependency_entry(entry: &toml::Value) -> bool {
    match entry {
        toml::Value::String(version) => !version.trim().is_empty(),
        toml::Value::Table(table) => {
            ["version", "path", "git", "registry"]
                .iter()
                .any(|key| non_empty_toml_string(table.get(*key)))
                || table.get("workspace").and_then(toml::Value::as_bool) == Some(true)
        }
        _ => false,
    }
}

fn non_empty_toml_string(value: Option<&toml::Value>) -> bool {
    value
        .and_then(toml::Value::as_str)
        .is_some_and(|value| !value.trim().is_empty())
}

fn validate_native_shell_tauri_config(project_root: &Path) -> DxResult<()> {
    let config_path = project_root.join("src-tauri/tauri.conf.json");
    let config = std::fs::read_to_string(&config_path).map_err(|error| DxError::IoError {
        path: Some(config_path.clone()),
        message: error.to_string(),
    })?;
    let config: serde_json::Value =
        serde_json::from_str(&config).map_err(|error| DxError::ConfigValidationError {
            message: format!("Invalid src-tauri/tauri.conf.json for Android build: {error}"),
            field: Some("build.target.android.native_shell".to_string()),
        })?;

    require_tauri_config_string(&config, &["build", "frontendDist"], "../.dx/www/output")?;
    require_tauri_before_build_command(&config)?;
    require_tauri_config_bool(&config, &["build", "removeUnusedCommands"], true)?;
    let dev_port = require_tauri_dev_url(&config)?;
    require_tauri_before_dev_command(&config, dev_port)?;
    require_tauri_config_bool(&config, &["app", "withGlobalTauri"], true)?;
    require_tauri_config_non_empty_array(&config, &["app", "windows"])?;
    require_tauri_config_string_tokens(
        &config,
        &["app", "security", "csp"],
        REQUIRED_TAURI_CSP_TOKENS,
    )?;

    Ok(())
}

fn require_tauri_config_string(
    config: &serde_json::Value,
    path: &[&str],
    expected: &'static str,
) -> DxResult<()> {
    if tauri_config_value(config, path).and_then(serde_json::Value::as_str) == Some(expected) {
        return Ok(());
    }
    let path = tauri_config_path(path);
    Err(native_shell_android_config_error(format!(
        "Android builds require {path}={expected:?} in src-tauri/tauri.conf.json"
    )))
}

fn require_tauri_dev_url(config: &serde_json::Value) -> DxResult<u16> {
    let path = ["build", "devUrl"];
    let path_name = tauri_config_path(&path);
    let Some(dev_url) = tauri_config_value(config, &path).and_then(serde_json::Value::as_str)
    else {
        return Err(native_shell_android_config_error(format!(
            "Android builds require {path_name}=\"http://127.0.0.1:<port>\" in src-tauri/tauri.conf.json"
        )));
    };

    let Some(port) = dev_url.strip_prefix("http://127.0.0.1:") else {
        return Err(native_shell_android_config_error(format!(
            "Android builds require {path_name} to use http://127.0.0.1:<port> in src-tauri/tauri.conf.json"
        )));
    };
    let port = port.parse::<u16>().map_err(|_| {
        native_shell_android_config_error(format!(
            "Android builds require {path_name} to end with a valid non-zero port in src-tauri/tauri.conf.json"
        ))
    })?;
    if port == 0 {
        return Err(native_shell_android_config_error(format!(
            "Android builds require {path_name} to end with a valid non-zero port in src-tauri/tauri.conf.json"
        )));
    }

    Ok(port)
}

fn require_tauri_before_build_command(config: &serde_json::Value) -> DxResult<()> {
    let path = ["build", "beforeBuildCommand"];
    let path_name = tauri_config_path(&path);
    let Some(command) = tauri_config_command_script(config, &path) else {
        return Err(native_shell_android_config_error(format!(
            "Android builds require {path_name} to run dx www build in src-tauri/tauri.conf.json"
        )));
    };

    let tokens = command.split_whitespace().collect::<Vec<_>>();
    if tokens.as_slice() != &["dx", "www", "build"] {
        return Err(native_shell_android_config_error(format!(
            "Android builds require {path_name} to run exactly dx www build in src-tauri/tauri.conf.json"
        )));
    }

    Ok(())
}

fn require_tauri_before_dev_command(
    config: &serde_json::Value,
    expected_port: u16,
) -> DxResult<()> {
    let path = ["build", "beforeDevCommand"];
    let path_name = tauri_config_path(&path);
    let Some(command) = tauri_config_command_script(config, &path) else {
        return Err(native_shell_android_config_error(format!(
            "Android builds require {path_name} to run dx dev for the same loopback port as build.devUrl in src-tauri/tauri.conf.json"
        )));
    };

    let tokens = command.split_whitespace().collect::<Vec<_>>();
    if tokens.len() < 2 || tokens[0] != "dx" || tokens[1] != "dev" {
        return Err(native_shell_android_config_error(format!(
            "Android builds require {path_name} to run dx dev in src-tauri/tauri.conf.json"
        )));
    }

    let (host, port) = parse_single_dx_dev_command(&tokens, &path_name)?;
    if host != Some("127.0.0.1") {
        return Err(native_shell_android_config_error(format!(
            "Android builds require {path_name} to include --host 127.0.0.1 in src-tauri/tauri.conf.json"
        )));
    }

    let port = port
        .and_then(|port| port.parse::<u16>().ok())
        .filter(|port| *port != 0);
    if port != Some(expected_port) {
        return Err(native_shell_android_config_error(format!(
            "Android builds require {path_name} to include --port {expected_port} matching build.devUrl in src-tauri/tauri.conf.json"
        )));
    }

    Ok(())
}

fn parse_single_dx_dev_command<'a>(
    tokens: &'a [&str],
    path_name: &str,
) -> DxResult<(Option<&'a str>, Option<&'a str>)> {
    let mut host = None;
    let mut port = None;
    let mut index = 2usize;

    while let Some(token) = tokens.get(index).copied() {
        if token == "--host" {
            host = parse_command_option_argument(tokens, &mut index, host, path_name, "--host")?;
            continue;
        }
        if let Some(value) = token.strip_prefix("--host=") {
            host = parse_command_option_equals(value, host, path_name, "--host")?;
            index += 1;
            continue;
        }
        if token == "--port" {
            port = parse_command_option_argument(tokens, &mut index, port, path_name, "--port")?;
            continue;
        }
        if let Some(value) = token.strip_prefix("--port=") {
            port = parse_command_option_equals(value, port, path_name, "--port")?;
            index += 1;
            continue;
        }

        return Err(native_shell_android_config_error(format!(
            "Android builds require {path_name} to be a single dx dev command with only --host and --port options in src-tauri/tauri.conf.json"
        )));
    }

    Ok((host, port))
}

fn parse_command_option_argument<'a>(
    tokens: &'a [&str],
    index: &mut usize,
    current: Option<&'a str>,
    path_name: &str,
    option: &str,
) -> DxResult<Option<&'a str>> {
    if current.is_some() {
        return Err(duplicate_command_option_error(path_name, option));
    }
    let value = tokens.get(*index + 1).copied().ok_or_else(|| {
        native_shell_android_config_error(format!(
            "Android builds require {path_name} to provide a value for {option} in src-tauri/tauri.conf.json"
        ))
    })?;
    if value.starts_with('-') {
        return Err(native_shell_android_config_error(format!(
            "Android builds require {path_name} to provide a value for {option} in src-tauri/tauri.conf.json"
        )));
    }
    *index += 2;
    Ok(Some(value))
}

fn parse_command_option_equals<'a>(
    value: &'a str,
    current: Option<&'a str>,
    path_name: &str,
    option: &str,
) -> DxResult<Option<&'a str>> {
    if current.is_some() {
        return Err(duplicate_command_option_error(path_name, option));
    }
    if value.is_empty() {
        return Err(native_shell_android_config_error(format!(
            "Android builds require {path_name} to provide a value for {option} in src-tauri/tauri.conf.json"
        )));
    }
    Ok(Some(value))
}

fn duplicate_command_option_error(path_name: &str, option: &str) -> DxError {
    native_shell_android_config_error(format!(
        "Android builds require {path_name} to include {option} only once in src-tauri/tauri.conf.json"
    ))
}

fn tauri_config_command_script<'a>(
    config: &'a serde_json::Value,
    path: &[&str],
) -> Option<&'a str> {
    match tauri_config_value(config, path)? {
        serde_json::Value::String(command) => Some(command.as_str()),
        serde_json::Value::Object(command) => {
            command.get("script").and_then(serde_json::Value::as_str)
        }
        _ => None,
    }
}

fn require_tauri_config_bool(
    config: &serde_json::Value,
    path: &[&str],
    expected: bool,
) -> DxResult<()> {
    if tauri_config_value(config, path).and_then(serde_json::Value::as_bool) == Some(expected) {
        return Ok(());
    }
    let path = tauri_config_path(path);
    Err(native_shell_android_config_error(format!(
        "Android builds require {path}={expected} in src-tauri/tauri.conf.json"
    )))
}

fn require_tauri_config_non_empty_array(config: &serde_json::Value, path: &[&str]) -> DxResult<()> {
    if tauri_config_value(config, path)
        .and_then(serde_json::Value::as_array)
        .is_some_and(|values| !values.is_empty())
    {
        return Ok(());
    }
    let path = tauri_config_path(path);
    Err(native_shell_android_config_error(format!(
        "Android builds require {path} to be a non-empty array in src-tauri/tauri.conf.json"
    )))
}

fn require_tauri_config_string_tokens(
    config: &serde_json::Value,
    path: &[&str],
    required_tokens: &[&str],
) -> DxResult<()> {
    let path_name = tauri_config_path(path);
    let value = tauri_config_value(config, path)
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            native_shell_android_config_error(format!(
                "Android builds require {path_name} to be a string in src-tauri/tauri.conf.json"
            ))
        })?;

    for required_token in required_tokens {
        if !value.contains(required_token) {
            return Err(native_shell_android_config_error(format!(
                "Android builds require {path_name} to include {required_token:?} in src-tauri/tauri.conf.json"
            )));
        }
    }

    Ok(())
}

fn tauri_config_value<'a>(
    config: &'a serde_json::Value,
    path: &[&str],
) -> Option<&'a serde_json::Value> {
    let mut value = config;
    for segment in path {
        value = value.get(*segment)?;
    }
    Some(value)
}

fn tauri_config_path(path: &[&str]) -> String {
    path.join(".")
}

fn native_shell_android_config_error(message: impl Into<String>) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some("build.target.android.native_shell".to_string()),
    }
}

fn native_shell_build_script_error() -> DxError {
    native_shell_android_config_error(
        "Android builds require src-tauri/build.rs to call tauri_build::build(); regenerate the native shell with dx www native-shell --write --force after reviewing local native changes",
    )
}

fn native_shell_mobile_entrypoint_error() -> DxError {
    native_shell_android_config_error(
        "Android builds require src-tauri/src/lib.rs to define the Tauri mobile entrypoint",
    )
}

fn native_shell_runtime_launch_error() -> DxError {
    native_shell_android_config_error(
        "Android builds require src-tauri/src/lib.rs run() to launch tauri::Builder::default().run(tauri::generate_context!()); regenerate the native shell with dx www native-shell --write --force after reviewing local native changes",
    )
}

fn native_shell_mobile_lib_name_error() -> DxError {
    native_shell_android_config_error(
        "Android builds require src-tauri/Cargo.toml [lib].name so src-tauri/src/main.rs can call the native lib run() entrypoint; regenerate the native shell with dx www native-shell --write --force after reviewing local native changes",
    )
}

fn native_shell_desktop_entrypoint_error(native_lib_name: &str) -> DxError {
    native_shell_android_config_error(format!(
        "Android builds require src-tauri/src/main.rs to call {native_lib_name}::run() for the desktop entrypoint; regenerate the native shell with dx www native-shell --write --force after reviewing local native changes"
    ))
}

fn native_shell_tauri_build_dependency_error() -> DxError {
    native_shell_android_config_error(
        "Android builds require src-tauri/Cargo.toml [build-dependencies].tauri-build for tauri_build::build(); regenerate the native shell with dx www native-shell --write --force after reviewing local native changes",
    )
}

fn native_shell_tauri_runtime_dependency_error() -> DxError {
    native_shell_android_config_error(
        "Android builds require src-tauri/Cargo.toml [dependencies].tauri for the Tauri mobile entrypoint; regenerate the native shell with dx www native-shell --write --force after reviewing local native changes",
    )
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::error::DxError;

    use super::{mobile_crate_types, validate_mobile_native_shell};

    const VALID_NATIVE_SHELL_CSP: &str = "default-src 'self'; connect-src ipc: http://ipc.localhost http://127.0.0.1:3000; img-src 'self' asset: http://asset.localhost data:; style-src 'self' 'unsafe-inline'";
    const HARDENED_NATIVE_SHELL_CSP: &str = "default-src 'self'; connect-src ipc: http://ipc.localhost https: wss:; img-src 'self' asset: http://asset.localhost data:; script-src 'self'; style-src 'self' 'unsafe-inline'";
    const VALID_NATIVE_SHELL_DEV_CSP: &str = "default-src 'self'; connect-src ipc: http://ipc.localhost http://127.0.0.1:* ws://127.0.0.1:* https: wss:; img-src 'self' asset: http://asset.localhost data:; script-src 'self'; style-src 'self' 'unsafe-inline'";

    #[test]
    fn parses_mobile_crate_types_without_exact_string_matching() -> Result<(), toml::de::Error> {
        let cargo_toml: toml::Value = toml::from_str(
            r#"
            [lib]
            crate-type = [
              "rlib",
              "cdylib",
              "staticlib",
            ]
            "#,
        )?;

        let crate_types = mobile_crate_types(&cargo_toml);

        assert!(crate_types.contains(&"staticlib"));
        assert!(crate_types.contains(&"cdylib"));
        assert!(crate_types.contains(&"rlib"));
        Ok(())
    }

    #[test]
    fn rejects_mobile_native_shell_without_tauri_build_script() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        std::fs::remove_file(project.path().join("src-tauri/build.rs"))
            .expect("remove build script");

        let error = validate_mobile_native_shell(project.path())
            .expect_err("native shell validation should require src-tauri/build.rs");

        assert_native_shell_error_mentions(error, "src-tauri/build.rs");
    }

    #[test]
    fn rejects_mobile_native_shell_build_script_without_tauri_build_call() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        write_project_file(project.path(), "src-tauri/build.rs", "fn main() {}\n");

        let error = validate_mobile_native_shell(project.path())
            .expect_err("native shell validation should require tauri_build::build()");

        assert_native_shell_error_mentions(error, "tauri_build::build()");
    }

    #[test]
    fn rejects_mobile_native_shell_build_script_with_commented_tauri_build_call() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        write_project_file(
            project.path(),
            "src-tauri/build.rs",
            "fn main() {\n  // tauri_build::build();\n}\n",
        );

        let error = validate_mobile_native_shell(project.path()).expect_err(
            "native shell validation should require an active tauri_build::build() call",
        );

        assert_native_shell_error_mentions(error, "tauri_build::build()");
    }

    #[test]
    fn rejects_mobile_native_shell_without_tauri_build_dependency() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        write_project_file(
            project.path(),
            "src-tauri/Cargo.toml",
            r#"[package]
name = "dx-mobile-app-native"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
name = "dx_mobile_app_native_lib"
crate-type = ["staticlib", "cdylib", "rlib"]
"#,
        );

        let error = validate_mobile_native_shell(project.path())
            .expect_err("native shell validation should require tauri-build build-dependency");

        assert_native_shell_error_mentions(error, "tauri-build");
    }

    #[test]
    fn rejects_mobile_native_shell_tauri_build_dependency_without_source() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        write_project_file(
            project.path(),
            "src-tauri/Cargo.toml",
            r#"[package]
name = "dx-mobile-app-native"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
name = "dx_mobile_app_native_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { features = [] }
"#,
        );

        let error = validate_mobile_native_shell(project.path()).expect_err(
            "native shell validation should reject tauri-build entries without version, path, git, registry, or workspace source",
        );

        assert_native_shell_error_mentions(error, "tauri-build");
    }

    #[test]
    fn rejects_mobile_native_shell_without_tauri_runtime_dependency() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        write_project_file(
            project.path(),
            "src-tauri/Cargo.toml",
            r#"[package]
name = "dx-mobile-app-native"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
name = "dx_mobile_app_native_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { path = "../native/crates/tauri-build" }
"#,
        );

        let error = validate_mobile_native_shell(project.path())
            .expect_err("native shell validation should require tauri runtime dependency");

        assert_native_shell_error_mentions(error, "[dependencies].tauri");
    }

    #[test]
    fn rejects_mobile_native_shell_tauri_runtime_dependency_without_source() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        write_project_file(
            project.path(),
            "src-tauri/Cargo.toml",
            r#"[package]
name = "dx-mobile-app-native"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
name = "dx_mobile_app_native_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { path = "../native/crates/tauri-build" }

[dependencies]
tauri = { features = [] }
"#,
        );

        let error = validate_mobile_native_shell(project.path()).expect_err(
            "native shell validation should reject tauri entries without version, path, git, registry, or workspace source",
        );

        assert_native_shell_error_mentions(error, "[dependencies].tauri");
    }

    #[test]
    fn rejects_mobile_native_shell_without_tauri_runtime_launch() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        write_project_file(
            project.path(),
            "src-tauri/src/lib.rs",
            "#[cfg_attr(mobile, tauri::mobile_entry_point)]\npub fn run() {}\n",
        );

        let error = validate_mobile_native_shell(project.path())
            .expect_err("native shell validation should require a Tauri runtime launch");

        assert_native_shell_error_mentions(error, "tauri::Builder::default()");
    }

    #[test]
    fn rejects_mobile_native_shell_with_commented_tauri_runtime_launch() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        write_project_file(
            project.path(),
            "src-tauri/src/lib.rs",
            r#"#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  // tauri::Builder::default().run(tauri::generate_context!());
}
"#,
        );

        let error = validate_mobile_native_shell(project.path())
            .expect_err("native shell validation should require an active Tauri runtime launch");

        assert_native_shell_error_mentions(error, "tauri::Builder::default()");
    }

    #[test]
    fn rejects_mobile_native_shell_with_tauri_runtime_launch_outside_run_function() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        write_project_file(
            project.path(),
            "src-tauri/src/lib.rs",
            r#"#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {}

fn unused_runtime_launch() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("unused launch");
}
"#,
        );

        let error = validate_mobile_native_shell(project.path())
            .expect_err("native shell validation should require Tauri runtime launch inside run()");

        assert_native_shell_error_mentions(error, "tauri::Builder::default()");
    }

    #[test]
    fn accepts_mobile_native_shell_with_split_tauri_runtime_launch() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        write_project_file(
            project.path(),
            "src-tauri/src/lib.rs",
            r#"#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let builder = tauri::Builder::default();
  builder
    .run(tauri::generate_context!())
    .expect("failed to run DX WWW native shell");
}
"#,
        );

        validate_mobile_native_shell(project.path())
            .expect("split Tauri runtime launch should remain valid");
    }

    #[test]
    fn rejects_mobile_native_shell_without_global_tauri_access() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        let app_config = tauri_app_config(false, VALID_NATIVE_SHELL_CSP);
        write_tauri_config(project.path(), &app_config);

        let error = validate_mobile_native_shell(project.path())
            .expect_err("native shell validation should require app.withGlobalTauri=true");

        assert_native_shell_error_mentions(error, "app.withGlobalTauri=true");
    }

    #[test]
    fn rejects_mobile_native_shell_without_app_windows() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        let app_config = format!(
            r#"{{
    "withGlobalTauri": true,
    "security": {{
      "csp": "{VALID_NATIVE_SHELL_CSP}"
    }}
  }}"#
        );
        write_tauri_config(project.path(), &app_config);

        let error = validate_mobile_native_shell(project.path())
            .expect_err("native shell validation should require app.windows");

        assert_native_shell_error_mentions(error, "app.windows");
    }

    #[test]
    fn rejects_mobile_native_shell_without_ipc_asset_csp_sources() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        let app_config = tauri_app_config(
            true,
            "default-src 'self'; connect-src http://127.0.0.1:3000; img-src 'self' data:; style-src 'self'",
        );
        write_tauri_config(project.path(), &app_config);

        let error = validate_mobile_native_shell(project.path())
            .expect_err("native shell validation should require IPC and asset CSP sources");

        assert_native_shell_error_mentions(error, "app.security.csp");
    }

    #[test]
    fn accepts_mobile_native_shell_with_hardened_security_csp() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        let app_config = format!(
            r#"{{
    "withGlobalTauri": true,
    "windows": [
      {{
        "title": "DX Mobile App",
        "width": 420,
        "height": 760,
        "resizable": true,
        "fullscreen": false
      }}
    ],
    "security": {{
      "csp": "{HARDENED_NATIVE_SHELL_CSP}",
      "devCsp": "{VALID_NATIVE_SHELL_DEV_CSP}"
    }}
  }}"#
        );
        write_tauri_config(project.path(), &app_config);

        validate_mobile_native_shell(project.path())
            .expect("hardened CSP with devCsp should remain valid");
    }

    #[test]
    fn rejects_mobile_native_shell_without_dev_url() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        let app_config = tauri_app_config(true, VALID_NATIVE_SHELL_CSP);
        write_tauri_config_with_build(
            project.path(),
            r#"{
    "frontendDist": "../.dx/www/output",
    "beforeDevCommand": "dx dev --host 127.0.0.1 --port 3000",
    "beforeBuildCommand": "dx www build",
    "removeUnusedCommands": true
  }"#,
            &app_config,
        );

        let error = validate_mobile_native_shell(project.path())
            .expect_err("native shell validation should require build.devUrl");

        assert_native_shell_error_mentions(error, "build.devUrl");
    }

    #[test]
    fn rejects_mobile_native_shell_with_non_loopback_dev_url() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        let app_config = tauri_app_config(true, VALID_NATIVE_SHELL_CSP);
        write_tauri_config_with_dev_server(
            project.path(),
            "http://0.0.0.0:3000",
            "dx dev --host 127.0.0.1 --port 3000",
            &app_config,
        );

        let error = validate_mobile_native_shell(project.path())
            .expect_err("native shell validation should require loopback build.devUrl");

        assert_native_shell_error_mentions(error, "build.devUrl");
    }

    #[test]
    fn rejects_mobile_native_shell_without_before_dev_command() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        let app_config = tauri_app_config(true, VALID_NATIVE_SHELL_CSP);
        write_tauri_config_with_build(
            project.path(),
            r#"{
    "frontendDist": "../.dx/www/output",
    "devUrl": "http://127.0.0.1:3000",
    "beforeBuildCommand": "dx www build",
    "removeUnusedCommands": true
  }"#,
            &app_config,
        );

        let error = validate_mobile_native_shell(project.path())
            .expect_err("native shell validation should require build.beforeDevCommand");

        assert_native_shell_error_mentions(error, "build.beforeDevCommand");
    }

    #[test]
    fn rejects_mobile_native_shell_with_mismatched_dev_command_port() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        let app_config = tauri_app_config(true, VALID_NATIVE_SHELL_CSP);
        write_tauri_config_with_dev_server(
            project.path(),
            "http://127.0.0.1:3000",
            "dx dev --host 127.0.0.1 --port 3010",
            &app_config,
        );

        let error = validate_mobile_native_shell(project.path())
            .expect_err("native shell validation should require matching dev command port");

        assert_native_shell_error_mentions(error, "build.beforeDevCommand");
    }

    #[test]
    fn accepts_mobile_native_shell_with_custom_dev_port() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        let app_config = tauri_app_config(true, VALID_NATIVE_SHELL_CSP);
        write_tauri_config_with_dev_server(
            project.path(),
            "http://127.0.0.1:3010",
            "dx dev --port 3010 --host 127.0.0.1",
            &app_config,
        );

        validate_mobile_native_shell(project.path())
            .expect("custom loopback dev port should remain valid");
    }

    #[test]
    fn accepts_mobile_native_shell_with_tauri_command_objects() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        let app_config = tauri_app_config(true, VALID_NATIVE_SHELL_CSP);
        write_tauri_config_with_build(
            project.path(),
            r#"{
    "frontendDist": "../.dx/www/output",
    "devUrl": "http://127.0.0.1:3000",
    "beforeDevCommand": {
      "script": "dx dev --host 127.0.0.1 --port 3000",
      "cwd": "..",
      "wait": true
    },
    "beforeBuildCommand": {
      "script": "dx www build",
      "cwd": ".."
    },
    "removeUnusedCommands": true
  }"#,
            &app_config,
        );

        validate_mobile_native_shell(project.path())
            .expect("Tauri command object scripts should remain valid for DX native shells");
    }

    #[test]
    fn rejects_mobile_native_shell_with_chained_before_dev_command() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        let app_config = tauri_app_config(true, VALID_NATIVE_SHELL_CSP);
        write_tauri_config_with_dev_server(
            project.path(),
            "http://127.0.0.1:3000",
            "dx dev --host 127.0.0.1 --port 3000 && echo unsafe",
            &app_config,
        );

        let error = validate_mobile_native_shell(project.path())
            .expect_err("native shell validation should reject chained beforeDevCommand");

        assert_native_shell_error_mentions(error, "build.beforeDevCommand");
    }

    #[test]
    fn rejects_mobile_native_shell_without_desktop_entrypoint_run_call() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        write_project_file(project.path(), "src-tauri/src/main.rs", "fn main() {}\n");

        let error = validate_mobile_native_shell(project.path()).expect_err(
            "native shell validation should require main.rs to call the native lib run function",
        );

        assert_native_shell_error_mentions(error, "dx_mobile_app_native_lib::run()");
    }

    #[test]
    fn rejects_mobile_native_shell_with_commented_desktop_entrypoint_run_call() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        write_project_file(
            project.path(),
            "src-tauri/src/main.rs",
            "fn main() {\n  // dx_mobile_app_native_lib::run();\n}\n",
        );

        let error = validate_mobile_native_shell(project.path()).expect_err(
            "native shell validation should require an active native lib run call in main.rs",
        );

        assert_native_shell_error_mentions(error, "dx_mobile_app_native_lib::run()");
    }

    #[test]
    fn rejects_mobile_native_shell_with_wrong_desktop_entrypoint_lib_name() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        write_project_file(
            project.path(),
            "src-tauri/src/main.rs",
            "fn main() {\n  other_native_lib::run();\n}\n",
        );

        let error = validate_mobile_native_shell(project.path()).expect_err(
            "native shell validation should require main.rs to call the Cargo [lib].name run function",
        );

        assert_native_shell_error_mentions(error, "dx_mobile_app_native_lib::run()");
    }

    #[test]
    fn rejects_mobile_native_shell_with_desktop_entrypoint_run_call_outside_main_function() {
        let project = tempfile::tempdir().expect("tempdir");
        write_valid_mobile_native_shell(project.path());
        write_project_file(
            project.path(),
            "src-tauri/src/main.rs",
            r#"fn main() {}

fn unused_entrypoint() {
  dx_mobile_app_native_lib::run();
}
"#,
        );

        let error = validate_mobile_native_shell(project.path()).expect_err(
            "native shell validation should require the native lib run call inside main()",
        );

        assert_native_shell_error_mentions(error, "dx_mobile_app_native_lib::run()");
    }

    fn write_valid_mobile_native_shell(project_root: &Path) {
        write_project_file(
            project_root,
            "src-tauri/Cargo.toml",
            r#"[package]
name = "dx-mobile-app-native"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
name = "dx_mobile_app_native_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { path = "../native/crates/tauri-build" }

[dependencies]
tauri = { path = "../native/crates/tauri" }
"#,
        );
        write_project_file(
            project_root,
            "src-tauri/build.rs",
            "fn main() {\n  tauri_build::build();\n}\n",
        );
        write_project_file(
            project_root,
            "src-tauri/src/lib.rs",
            r#"#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("failed to run DX WWW native shell");
}
"#,
        );
        write_project_file(
            project_root,
            "src-tauri/src/main.rs",
            "fn main() {\n  dx_mobile_app_native_lib::run();\n}\n",
        );
        let app_config = tauri_app_config(true, VALID_NATIVE_SHELL_CSP);
        write_tauri_config(project_root, &app_config);
    }

    fn tauri_app_config(with_global_tauri: bool, csp: &str) -> String {
        format!(
            r#"{{
    "withGlobalTauri": {with_global_tauri},
    "windows": [
      {{
        "title": "DX Mobile App",
        "width": 420,
        "height": 760,
        "resizable": true,
        "fullscreen": false
      }}
    ],
    "security": {{
      "csp": "{csp}"
    }}
  }}"#
        )
    }

    fn write_tauri_config(project_root: &Path, app_config: &str) {
        write_tauri_config_with_dev_server(
            project_root,
            "http://127.0.0.1:3000",
            "dx dev --host 127.0.0.1 --port 3000",
            app_config,
        );
    }

    fn write_tauri_config_with_dev_server(
        project_root: &Path,
        dev_url: &str,
        before_dev_command: &str,
        app_config: &str,
    ) {
        let build_config = format!(
            r#"{{
    "frontendDist": "../.dx/www/output",
    "devUrl": "{dev_url}",
    "beforeDevCommand": "{before_dev_command}",
    "beforeBuildCommand": "dx www build",
    "removeUnusedCommands": true
  }}"#
        );
        write_tauri_config_with_build(project_root, &build_config, app_config);
    }

    fn write_tauri_config_with_build(project_root: &Path, build_config: &str, app_config: &str) {
        write_project_file(
            project_root,
            "src-tauri/tauri.conf.json",
            &format!(
                r#"{{
  "build": {build_config},
  "app": {app_config}
}}
"#
            ),
        );
    }

    fn write_project_file(project_root: &Path, relative_path: &str, contents: &str) {
        let path = project_root.join(relative_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create fixture parent");
        }
        std::fs::write(path, contents).expect("write fixture file");
    }

    fn assert_native_shell_error_mentions(error: DxError, expected: &str) {
        let DxError::ConfigValidationError { message, field } = error else {
            panic!("expected ConfigValidationError");
        };
        assert_eq!(field.as_deref(), Some("build.target.android.native_shell"));
        assert!(message.contains(expected), "{message}");
    }
}
