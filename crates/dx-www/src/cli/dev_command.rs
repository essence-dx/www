use std::collections::HashMap;
use std::path::PathBuf;
use console::style;
#[cfg(feature = "dev-server")]
use std::sync::Arc;
#[cfg(feature = "dev-server")]
use std::sync::Mutex;
#[cfg(feature = "dev-server")]
use std::time::Duration;
#[cfg(feature = "dev-server")]
use std::time::Instant;

use crate::config::DxConfig;
#[cfg(feature = "dev-server")]
use crate::error::DxError;
use crate::error::DxResult;

use super::dev_http::{DxCliHttpRequest, DxCliHttpResponse};
#[cfg(feature = "dev-server")]
use super::dev_http::{
    dev_project_reload_token, dx_cli_request_from_axum, dx_cli_response_to_axum,
    with_dev_html_injections_token,
};
use super::dev_options::{DxDevServerBinding, bind_dev_listener, parse_dev_options};
#[cfg(feature = "dev-server")]
use super::dev_server_mode::DxDevSelectedServerMode;
use super::dev_server_mode::resolve_dev_server_mode;
use super::dev_tiny_server::serve_tiny_dev_server;
#[cfg(feature = "dev-server")]
use super::devtools;
use super::help_text::{is_help_arg, print_dev_help};

pub(super) type DxDevParsedResponder =
    fn(&PathBuf, &DxCliHttpRequest, &HashMap<String, String>) -> DxCliHttpResponse;

#[cfg(feature = "dev-server")]
#[derive(Debug, Clone)]
struct DxDevCachedAxumResponse {
    response: crate::dev::axum_server::DxDevAxumResponse,
    expires_at: Instant,
}

#[cfg(feature = "dev-server")]
type DxDevAxumResponseCache = Mutex<HashMap<String, DxDevCachedAxumResponse>>;

#[cfg(feature = "dev-server")]
const DX_DEV_AXUM_RESPONSE_CACHE_TTL: Duration = Duration::from_millis(1_000);

pub(super) fn cmd_dev<F>(
    cwd: &PathBuf,
    args: &[String],
    load_translations: F,
    respond: DxDevParsedResponder,
) -> DxResult<()>
where
    F: FnOnce() -> DxResult<HashMap<String, String>>,
{
    if args.iter().any(|arg| is_help_arg(Some(arg))) {
        print_dev_help();
        return Ok(());
    }

    let config = DxConfig::load_project(cwd)?;
    let options = parse_dev_options(args, &config)?;
    let (listener, port) =
        match bind_dev_listener(&options.host, options.port, options.port_explicit)? {
            DxDevServerBinding::Existing(existing) => {
                eprintln!();
                eprintln!("Existing DX dev server detected at {}", existing.url);
                eprintln!("   Verified by: {}", existing.probe_url);
                eprintln!("   No duplicate dev server started.");
                eprintln!("   Project: {}", config.project.name);
                eprintln!();
                return Ok(());
            }
            DxDevServerBinding::Bound { listener, port } => (listener, port),
        };

    let translations = load_translations()?;
    let mode_decision = resolve_dev_server_mode(
        cwd,
        options.server_mode,
        options.hot_reload,
        options.devtools,
    )?;

    eprintln!();
    eprintln!("{}", crate::cli::utils::rainbow_text(&format!("◆ Www {} (Dx)", env!("CARGO_PKG_VERSION"))));
    
    let name_val = config.project.name.clone();
    let local_val = style(format!("http://{}:{}", options.host, port)).cyan().to_string();
    let project_size = crate::cli::utils::get_dir_size(cwd);
    let size_val = crate::cli::utils::format_size(project_size);
    
    crate::cli::utils::print_ascii_table(
        ("Key", "Value"),
        &[
            ("Name", &name_val),
            ("Local", &local_val),
            ("Size", &size_val),
        ]
    );

    if port != options.port {
        eprintln!(
            "   {} Port {} was busy, using {}",
            style("!").yellow(),
            options.port,
            style(port).cyan()
        );
    }
    eprintln!();
    eprintln!("Press Ctrl+C to stop the server");
    eprintln!();

    let cwd = cwd.clone();
    let hot_reload = options.hot_reload;
    let devtools = options.devtools;

    #[cfg(feature = "dev-server")]
    {
        if mode_decision.selected == DxDevSelectedServerMode::MayMinihttp {
            return serve_tiny_dev_server(
                listener,
                cwd,
                translations,
                hot_reload,
                devtools,
                respond,
            );
        }

        let route_cwd = cwd.clone();
        let response_cache: Arc<DxDevAxumResponseCache> = Arc::new(Mutex::new(HashMap::new()));
        let responder = Arc::new(move |request: crate::dev::axum_server::DxDevAxumRequest| {
            let request = dx_cli_request_from_axum(request);
            if let Some(response) = devtools::devtools_cli_response(&route_cwd, &request, devtools)
            {
                return dx_cli_response_to_axum(response);
            }

            let cache_key = dev_axum_response_cache_key(&request);
            let now = Instant::now();

            if let Some(cache_key) = cache_key.as_ref() {
                if let Ok(cache) = response_cache.lock() {
                    if let Some(cached) = cache.get(cache_key) {
                        if cached.expires_at > now {
                            return cached.response.clone();
                        }
                    }
                }
            }

            let token = hot_reload.then(|| dev_project_reload_token(&route_cwd));
            
            let compile_start = Instant::now();
            let mut response = respond(&route_cwd, &request, &translations);
            let compile_elapsed = compile_start.elapsed();
            
            if hot_reload || devtools {
                response =
                    with_dev_html_injections_token(response, token.as_deref(), devtools, true);
            }
            let axum_response = dx_cli_response_to_axum(response);
            if let Some(cache_key) = cache_key {
                if should_cache_dev_axum_response(&axum_response) {
                    if let Ok(mut cache) = response_cache.lock() {
                        if cache.len() > 512 {
                            cache.clear();
                        }
                        cache.insert(
                            cache_key,
                            DxDevCachedAxumResponse {
                                response: axum_response.clone(),
                                expires_at: now + DX_DEV_AXUM_RESPONSE_CACHE_TTL,
                            },
                        );
                    }
                }
            }

            let path_str = request.path.as_str();
            if !path_str.starts_with("/_dx/") && !path_str.contains("__no_cache") && path_str != "/favicon.ico" {
                let size_bytes = axum_response.body.len();
                let method = request.method.as_str();
                
                let method_colored = match method {
                    "GET" => style(method).green(),
                    "POST" => style(method).blue(),
                    "PUT" => style(method).yellow(),
                    "DELETE" => style(method).red(),
                    "HEAD" => style(method).cyan(),
                    _ => style(method).white(),
                };

                let path_colored = if path_str == "/" {
                    style(path_str).cyan()
                } else if path_str.starts_with("/api") {
                    style(path_str).magenta()
                } else {
                    style(path_str).blue()
                };

                let render_time = compile_elapsed;
                
                // Let's just use compile_elapsed for both, since we don't have total_time yet.
                let display_time = crate::cli::utils::format_time(compile_elapsed);
                
                if axum_response.status == 200 {
                    eprintln!(
                        "{} {} {} - {} - {}",
                        style("→").dim(),
                        method_colored.bold(),
                        path_colored,
                        style(&display_time).yellow(),
                        style(crate::cli::utils::format_size(size_bytes)).cyan()
                    );
                } else if axum_response.status == 404 {
                    eprintln!("{} {} {} - {} - {}", style("⨯").yellow(), method_colored.bold(), path_colored, style("404 Not Found").yellow(), style(&display_time).yellow());
                } else if axum_response.status >= 500 {
                    eprintln!("{} {} {} - {} - {}", style("⨯").red(), method_colored.bold(), path_colored, style(format!("{} Error", axum_response.status)).red(), style(&display_time).red());
                    if let Ok(body_str) = String::from_utf8(axum_response.body.to_vec()) {
                        if let Some(start_idx) = body_str.find("<!-- DX_DEV_ERROR: ") {
                            let remaining = &body_str[start_idx + 19..];
                            if let Some(end) = remaining.find(" -->") {
                                let error_msg = &remaining[..end];
                                eprintln!("   {}", style(error_msg.replace("\n", "\n   ")).red());
                            }
                        }
                    }
                }
            }

            axum_response
        });
        let state = crate::dev::axum_server::DxDevAxumState::new(cwd, hot_reload, responder);
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|error| DxError::IoError {
                path: None,
                message: format!("Failed to start Tokio runtime for dx dev: {error}"),
            })?;
        runtime.block_on(crate::dev::axum_server::serve_dev_router(listener, state))
    }

    #[cfg(not(feature = "dev-server"))]
    {
        serve_tiny_dev_server(listener, cwd, translations, hot_reload, devtools, respond)
    }
}

#[cfg(feature = "dev-server")]
fn dev_axum_response_cache_key(request: &DxCliHttpRequest) -> Option<String> {
    if !matches!(request.method.as_str(), "GET" | "HEAD") {
        return None;
    }
    if request.path.contains("__no_cache") {
        return None;
    }

    let path_only = request.path.split('?').next().unwrap_or("/");
    if path_only == "/api" || path_only.starts_with("/api/") {
        return None;
    }
    if path_only.starts_with("/_dx/") && !path_only.starts_with("/_dx/styles/") {
        return None;
    }

    Some(format!("{} {}", request.method, request.path))
}

#[cfg(feature = "dev-server")]
fn should_cache_dev_axum_response(response: &crate::dev::axum_server::DxDevAxumResponse) -> bool {
    response.status == 200
        && (response.content_type.contains("text/html")
            || response.content_type.contains("text/css")
            || response.content_type.contains("javascript")
            || response.content_type.contains("image/svg+xml"))
}
