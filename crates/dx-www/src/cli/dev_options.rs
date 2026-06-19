use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::config::{DxConfig, DxDevServerMode};
use crate::error::{DxError, DxResult};
use crate::hot_reload_protocol::{
    DX_HOT_RELOAD_PROTOCOL, DX_HOT_RELOAD_SOURCE, DX_HOT_RELOAD_VERSION_ENDPOINT,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxDevCommandOptions {
    pub(super) host: String,
    pub(super) port: u16,
    pub(super) hot_reload: bool,
    pub(super) devtools: bool,
    pub(super) server_mode: DxDevServerMode,
    pub(super) port_explicit: bool,
}

#[derive(Debug)]
pub(super) enum DxDevServerBinding {
    Bound { listener: TcpListener, port: u16 },
    Existing(DxDevExistingServer),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxDevExistingServer {
    pub(super) host: String,
    pub(super) port: u16,
    pub(super) url: String,
    pub(super) probe_url: String,
}

pub(super) fn parse_dev_options(
    args: &[String],
    config: &DxConfig,
) -> DxResult<DxDevCommandOptions> {
    let mut host = normalize_dev_host(&config.dev.host);
    let mut port = config.dev.port;
    let mut hot_reload = config.dev.hot_reload;
    let mut devtools = config.dev.devtools;
    let mut server_mode = config.dev.server_mode;
    let mut port_explicit = false;

    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--host" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| dev_options_error("--host requires a value", "dev.host"))?;
                let value = value.trim();
                if value.is_empty() || value.starts_with('-') {
                    return Err(dev_options_error(
                        "--host requires a non-empty host",
                        "dev.host",
                    ));
                }
                host = normalize_dev_host(value);
                index += 2;
            }
            "--port" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| dev_options_error("--port requires a value", "dev.port"))?;
                port = value.parse::<u16>().map_err(|_| {
                    dev_options_error(format!("Invalid dev port: {value}"), "dev.port")
                })?;
                if port == 0 {
                    return Err(dev_options_error(
                        "dev port must be greater than zero",
                        "dev.port",
                    ));
                }
                port_explicit = true;
                index += 2;
            }
            "--no-hot-reload" => {
                hot_reload = false;
                index += 1;
            }
            "--devtools" => {
                devtools = true;
                index += 1;
            }
            "--no-devtools" => {
                devtools = false;
                index += 1;
            }
            "--server" | "--server-mode" => {
                let field = if args[index].as_str() == "--server" {
                    "dev.server"
                } else {
                    "dev.server_mode"
                };
                let value = args.get(index + 1).ok_or_else(|| {
                    dev_options_error(format!("{} requires a value", args[index]), field)
                })?;
                server_mode = DxDevServerMode::from_config_value(value).ok_or_else(|| {
                    dev_options_error(
                        format!("Unsupported dev server mode: {value}"),
                        "dev.server_mode",
                    )
                })?;
                index += 2;
            }
            unknown => {
                return Err(dev_options_error(
                    format!("Unknown option for dx dev: {unknown}"),
                    "dx dev",
                ));
            }
        }
    }

    Ok(DxDevCommandOptions {
        host,
        port,
        hot_reload,
        devtools,
        server_mode,
        port_explicit,
    })
}

pub(super) fn bind_dev_listener(
    host: &str,
    requested_port: u16,
    port_explicit: bool,
) -> DxResult<DxDevServerBinding> {
    let max_attempts = if port_explicit { 1 } else { 32 };

    for offset in 0..max_attempts {
        let Some(port) = requested_port.checked_add(offset) else {
            break;
        };
        let addr = format!("{host}:{port}");
        match TcpListener::bind(&addr) {
            Ok(listener) => return Ok(DxDevServerBinding::Bound { listener, port }),
            Err(error) if error.kind() == std::io::ErrorKind::AddrInUse => {
                if let Some(existing) = probe_existing_dx_dev_server(host, port) {
                    return Ok(DxDevServerBinding::Existing(existing));
                }
                if !port_explicit {
                    continue;
                }
                return Err(occupied_explicit_dev_port_error(host, port, error));
            }
            Err(error) => {
                return Err(DxError::IoError {
                    path: None,
                    message: format!("Failed to bind to {addr}: {error}"),
                });
            }
        }
    }

    let end_port = requested_port.saturating_add(max_attempts.saturating_sub(1));
    Err(DxError::IoError {
        path: None,
        message: format!(
            "Failed to find an available dev port from {host}:{requested_port} through {host}:{end_port}"
        ),
    })
}

fn occupied_explicit_dev_port_error(host: &str, port: u16, error: std::io::Error) -> DxError {
    let probe_host = dev_probe_host(host);
    DxError::IoError {
        path: None,
        message: format!(
            "Dev port {host}:{port} is already in use and did not answer as a DX dev server at {}. No duplicate dev server was started. Stop the process using that port or choose a different --port. Bind error: {error}",
            dev_probe_url(probe_host, port),
        ),
    }
}

fn probe_existing_dx_dev_server(host: &str, port: u16) -> Option<DxDevExistingServer> {
    let probe_host = dev_probe_host(host);
    let socket_addr = dev_socket_addr(probe_host, port)
        .to_socket_addrs()
        .ok()?
        .next()?;
    let mut stream = TcpStream::connect_timeout(&socket_addr, Duration::from_millis(250)).ok()?;
    let _ = stream.set_read_timeout(Some(Duration::from_millis(350)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(350)));

    let probe_path = format!("{DX_HOT_RELOAD_VERSION_ENDPOINT}?resource=route%3A%2F");
    let request = format!(
        "GET {probe_path} HTTP/1.1\r\nHost: {probe_host}:{port}\r\nAccept: application/json\r\nConnection: close\r\n\r\n"
    );
    stream.write_all(request.as_bytes()).ok()?;

    let mut response = Vec::with_capacity(2048);
    let mut buffer = [0u8; 1024];
    while response.len() < 8192 {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(read) => response.extend_from_slice(&buffer[..read]),
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) =>
            {
                break;
            }
            Err(_) => return None,
        }
    }

    let response = String::from_utf8_lossy(&response);
    let response = response.as_ref();
    let status_ok = response.starts_with("HTTP/1.1 200") || response.starts_with("HTTP/1.0 200");
    if status_ok
        && response_has_dx_hot_reload_poll_header(response)
        && response.contains(DX_HOT_RELOAD_PROTOCOL)
        && response.contains(DX_HOT_RELOAD_SOURCE)
    {
        Some(DxDevExistingServer {
            host: probe_host.to_string(),
            port,
            url: dev_server_url(probe_host, port),
            probe_url: dev_probe_url(probe_host, port),
        })
    } else {
        None
    }
}

fn response_has_dx_hot_reload_poll_header(response: &str) -> bool {
    let headers = response
        .split_once("\r\n\r\n")
        .or_else(|| response.split_once("\n\n"))
        .map(|(headers, _body)| headers)
        .unwrap_or(response);
    headers
        .lines()
        .skip(1)
        .filter_map(|line| line.split_once(':'))
        .any(|(name, value)| {
            name.trim().eq_ignore_ascii_case("x-dx-hot-reload")
                && value.trim().eq_ignore_ascii_case("poll")
        })
}

fn dev_probe_host(host: &str) -> &str {
    match host.trim() {
        "" | "0.0.0.0" | "::" | "localhost" => "127.0.0.1",
        host => host,
    }
}

fn dev_server_url(host: &str, port: u16) -> String {
    if host.contains(':') && !host.starts_with('[') {
        format!("http://[{host}]:{port}")
    } else {
        format!("http://{host}:{port}")
    }
}

fn dev_probe_url(host: &str, port: u16) -> String {
    format!(
        "{}{DX_HOT_RELOAD_VERSION_ENDPOINT}?resource=route%3A%2F",
        dev_server_url(host, port)
    )
}

fn dev_socket_addr(host: &str, port: u16) -> String {
    if host.contains(':') && !host.starts_with('[') {
        format!("[{host}]:{port}")
    } else {
        format!("{host}:{port}")
    }
}

fn normalize_dev_host(host: &str) -> String {
    let host = host.trim();
    if host.eq_ignore_ascii_case("localhost") {
        "127.0.0.1".to_string()
    } else {
        host.to_string()
    }
}

fn dev_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dev_options_accepts_host_port_and_no_hot_reload() {
        let config = DxConfig::default();
        let args = vec![
            "--host".to_string(),
            "localhost".to_string(),
            "--port".to_string(),
            "3042".to_string(),
            "--no-hot-reload".to_string(),
        ];

        let options = parse_dev_options(&args, &config).expect("dev options");

        assert_eq!(options.host, "127.0.0.1");
        assert_eq!(options.port, 3042);
        assert!(!options.hot_reload);
        assert!(options.port_explicit);
    }

    #[test]
    fn parse_dev_options_accepts_devtools_toggles() {
        let mut config = DxConfig::default();
        config.dev.devtools = false;

        let enabled = parse_dev_options(&["--devtools".to_string()], &config).expect("dev options");
        assert!(enabled.devtools);

        let disabled = parse_dev_options(&["--no-devtools".to_string()], &DxConfig::default())
            .expect("dev options");
        assert!(!disabled.devtools);
    }

    #[test]
    fn parse_dev_options_accepts_server_mode_flags() {
        let mut config = DxConfig::default();
        config.dev.server_mode = DxDevServerMode::Axum;

        let from_server = parse_dev_options(
            &["--server".to_string(), "may-minihttp".to_string()],
            &config,
        )
        .expect("dev options");
        assert_eq!(from_server.server_mode, DxDevServerMode::MayMinihttp);

        let from_server_mode =
            parse_dev_options(&["--server-mode".to_string(), "auto".to_string()], &config)
                .expect("dev options");
        assert_eq!(from_server_mode.server_mode, DxDevServerMode::Auto);
    }

    #[test]
    fn parse_dev_options_rejects_missing_port_value() {
        let error = parse_dev_options(&["--port".to_string()], &DxConfig::default()).unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "--port requires a value");
                assert_eq!(field.as_deref(), Some("dev.port"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_dev_options_trims_host_before_normalizing_localhost() {
        let config = DxConfig::default();
        let args = vec!["--host".to_string(), " localhost ".to_string()];

        let options = parse_dev_options(&args, &config).expect("dev options");

        assert_eq!(options.host, "127.0.0.1");
        assert!(!options.port_explicit);
    }

    #[test]
    fn bind_dev_listener_falls_forward_when_default_port_is_busy() {
        let mut reserved = None;
        for port in 43100..43200 {
            let first = std::net::TcpListener::bind(("127.0.0.1", port));
            let second = std::net::TcpListener::bind(("127.0.0.1", port + 1));
            match (first, second) {
                (Ok(first), Ok(second)) => {
                    drop(second);
                    reserved = Some((port, first));
                    break;
                }
                (first, second) => {
                    drop(first);
                    drop(second);
                }
            }
        }
        let (busy_port, _busy_listener) = reserved.expect("two adjacent free ports");

        let binding = bind_dev_listener("127.0.0.1", busy_port, false).expect("fallback listener");

        match binding {
            DxDevServerBinding::Bound {
                port: selected_port,
                ..
            } => assert_eq!(selected_port, busy_port + 1),
            DxDevServerBinding::Existing(existing) => {
                panic!("expected fallback listener, found {}", existing.url);
            }
        }
    }

    #[test]
    fn bind_dev_listener_rejects_busy_explicit_port_without_fallback() {
        let mut reserved = None;
        for port in 43200..43300 {
            let first = std::net::TcpListener::bind(("127.0.0.1", port));
            let second = std::net::TcpListener::bind(("127.0.0.1", port + 1));
            match (first, second) {
                (Ok(first), Ok(second)) => {
                    drop(second);
                    reserved = Some((port, first));
                    break;
                }
                (first, second) => {
                    drop(first);
                    drop(second);
                }
            }
        }
        let (busy_port, _busy_listener) = reserved.expect("two adjacent free ports");

        let error = bind_dev_listener("127.0.0.1", busy_port, true)
            .expect_err("explicit busy port should fail instead of falling forward");

        match error {
            DxError::IoError { message, .. } => {
                assert!(
                    message.contains(&format!("127.0.0.1:{busy_port}")),
                    "{message}"
                );
                assert!(
                    message.contains("did not answer as a DX dev server"),
                    "{message}"
                );
                assert!(
                    message.contains("No duplicate dev server was started"),
                    "{message}"
                );
                assert!(message.contains("choose a different --port"), "{message}");
                assert!(
                    !message.contains(&format!("127.0.0.1:{}", busy_port + 1)),
                    "{message}"
                );
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn bind_dev_listener_reuses_existing_dx_dev_server_without_fallback() {
        let (port, server) = spawn_fake_probe_server(concat!(
            "HTTP/1.1 200 OK\r\n",
            "Content-Type: application/json; charset=utf-8\r\n",
            "x-dx-hot-reload: poll\r\n",
            "Connection: close\r\n",
            "\r\n",
            "{\"ok\":true,\"protocol\":\"dx.hot-reload.poll\",",
            "\"receipt\":{\"source\":\"dx-www-rust-dev-server\"}}"
        ));

        let binding = bind_dev_listener("127.0.0.1", port, false).expect("existing server binding");

        match binding {
            DxDevServerBinding::Existing(existing) => {
                assert_eq!(existing.port, port);
                assert_eq!(existing.url, format!("http://127.0.0.1:{port}"));
                assert_eq!(
                    existing.probe_url,
                    format!("http://127.0.0.1:{port}/_dx/hot-reload/version?resource=route%3A%2F")
                );
            }
            DxDevServerBinding::Bound { port: selected, .. } => {
                panic!("expected existing server reuse, selected {selected}");
            }
        }
        server.join().expect("fake server joined");
    }

    #[test]
    fn bind_dev_listener_rejects_protocol_body_without_dx_hot_reload_header() {
        let (port, server) = spawn_fake_probe_server(concat!(
            "HTTP/1.1 200 OK\r\n",
            "Content-Type: application/json; charset=utf-8\r\n",
            "Connection: close\r\n",
            "\r\n",
            "{\"ok\":true,\"protocol\":\"dx.hot-reload.poll\",",
            "\"receipt\":{\"source\":\"dx-www-rust-dev-server\"}}"
        ));

        let error = bind_dev_listener("127.0.0.1", port, true)
            .expect_err("missing DX hot reload header should not be reused");

        match error {
            DxError::IoError { message, .. } => {
                assert!(
                    message.contains("did not answer as a DX dev server"),
                    "{message}"
                );
                assert!(
                    message.contains("No duplicate dev server was started"),
                    "{message}"
                );
            }
            other => panic!("unexpected error: {other:?}"),
        }
        server.join().expect("fake server joined");
    }

    fn spawn_fake_probe_server(response: &'static str) -> (u16, std::thread::JoinHandle<()>) {
        let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).expect("fake server");
        let port = listener.local_addr().expect("fake server addr").port();
        let server = std::thread::spawn(move || {
            listener
                .set_nonblocking(true)
                .expect("fake server nonblocking");
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
            let (mut stream, _) = loop {
                match listener.accept() {
                    Ok(connection) => break connection,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        if std::time::Instant::now() >= deadline {
                            panic!("probe connection not received");
                        }
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                    Err(error) => panic!("probe connection failed: {error}"),
                }
            };
            let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(1)));
            let mut request = [0u8; 1024];
            let _ = std::io::Read::read(&mut stream, &mut request);
            std::io::Write::write_all(&mut stream, response.as_bytes()).expect("probe response");
        });
        (port, server)
    }
}
