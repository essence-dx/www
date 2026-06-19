use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use crate::error::{DxError, DxResult};

use super::preview_contract::{self, DxProductionServerActionExecutor};
use super::preview_options::parse_preview_options;

const MAX_PREVIEW_REQUEST_BYTES: usize = 1024 * 1024;

pub(super) fn cmd_preview(
    cwd: &Path,
    args: &[String],
    execute_server_action: DxProductionServerActionExecutor,
) -> DxResult<()> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        eprintln!("Usage: dx preview --production-contract [--build-dir <path>] [--port <port>]");
        eprintln!();
        eprintln!(
            "Serves only routes, immutable assets, and health checks listed in .dx/www/output/deploy-adapter.json."
        );
        return Ok(());
    }

    let options = parse_preview_options(cwd, args)?;
    let build_dir = options.build_dir;
    let port = options.port;

    let contract_path = build_dir.join("deploy-adapter.json");
    if !contract_path.is_file() {
        return Err(DxError::IoError {
            path: Some(contract_path),
            message: "Run dx build before dx preview --production-contract".to_string(),
        });
    }

    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(&addr).map_err(|e| DxError::IoError {
        path: None,
        message: format!("Failed to bind to {addr}: {e}"),
    })?;

    eprintln!();
    crate::cli::utils::print_ascii_table(&[
        ("Action", "Production-contract preview running"),
        ("Local", &console::style(format!("http://localhost:{port}")).cyan().to_string()),
        ("Output", &console::style(build_dir.display().to_string()).cyan().to_string()),
        ("Contract", &console::style(contract_path.display().to_string()).cyan().to_string()),
    ]);
    eprintln!();
    eprintln!("Press Ctrl+C to stop");

    let preview_cache = Arc::new(
        preview_contract::load_production_preview_cache(&build_dir).map_err(|message| {
            DxError::IoError {
                path: Some(contract_path.clone()),
                message,
            }
        })?,
    );

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let preview_cache = Arc::clone(&preview_cache);
                std::thread::spawn(move || {
                    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
                    serve_preview_connection(&mut stream, &preview_cache, execute_server_action);
                });
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }

    Ok(())
}

fn serve_preview_connection<T: Read + Write>(
    stream: &mut T,
    preview_cache: &preview_contract::DxProductionPreviewCache,
    execute_server_action: DxProductionServerActionExecutor,
) {
    loop {
        let request = match read_preview_http_request(stream) {
            Ok(request) => request,
            Err(error) if preview_connection_read_finished(&error) => break,
            Err(error) => {
                let response_bytes = bad_preview_request_response(&error.to_string());
                let _ = stream.write_all(&response_bytes);
                let _ = stream.flush();
                break;
            }
        };
        let keep_alive = preview_request_wants_keep_alive(&request);
        let connection = if keep_alive { "keep-alive" } else { "close" };
        let response_bytes =
            preview_contract::production_contract_wire_response_cached_with_connection(
                preview_cache,
                &request,
                execute_server_action,
                connection,
            );

        if stream.write_all(&response_bytes).is_err() || stream.flush().is_err() {
            break;
        }
        if !keep_alive {
            break;
        }
    }
}

fn preview_connection_read_finished(error: &Error) -> bool {
    matches!(
        error.kind(),
        ErrorKind::UnexpectedEof | ErrorKind::TimedOut | ErrorKind::WouldBlock
    )
}

fn preview_request_wants_keep_alive(request: &str) -> bool {
    let mut lines = request.lines();
    let request_line = lines.next().unwrap_or_default();
    let http_version = request_line.split_whitespace().nth(2).unwrap_or("HTTP/1.1");
    let http10 = http_version.eq_ignore_ascii_case("HTTP/1.0");
    let mut connection = None;

    for line in lines {
        if line.trim().is_empty() {
            break;
        }
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        if name.trim().eq_ignore_ascii_case("connection") {
            connection = Some(value.to_ascii_lowercase());
        }
    }

    if connection
        .as_deref()
        .is_some_and(|value| value.split(',').any(|part| part.trim() == "close"))
    {
        return false;
    }
    if connection
        .as_deref()
        .is_some_and(|value| value.split(',').any(|part| part.trim() == "keep-alive"))
    {
        return true;
    }
    !http10
}

fn read_preview_http_request(reader: &mut impl Read) -> std::io::Result<String> {
    let mut bytes = Vec::new();
    let mut buffer = [0u8; 8192];
    let mut header_end = None;
    let mut content_length = 0usize;

    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..read]);
        if bytes.len() > MAX_PREVIEW_REQUEST_BYTES {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "preview request exceeded 1 MiB",
            ));
        }

        if header_end.is_none() {
            if let Some(end) = find_http_header_end(&bytes) {
                content_length = parse_content_length(&bytes[..end])?;
                if end.saturating_add(content_length) > MAX_PREVIEW_REQUEST_BYTES {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "preview request body exceeded 1 MiB",
                    ));
                }
                header_end = Some(end);
            }
        }

        if let Some(end) = header_end {
            let expected_len = end.saturating_add(content_length);
            if bytes.len() >= expected_len {
                bytes.truncate(expected_len);
                break;
            }
        }
    }

    if bytes.is_empty() {
        return Err(Error::new(
            ErrorKind::UnexpectedEof,
            "empty preview request",
        ));
    }

    Ok(String::from_utf8_lossy(&bytes).to_string())
}

fn find_http_header_end(bytes: &[u8]) -> Option<usize> {
    bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|position| position + 4)
}

fn parse_content_length(header_bytes: &[u8]) -> std::io::Result<usize> {
    let headers = String::from_utf8_lossy(header_bytes);
    for line in headers.lines() {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        if name.trim().eq_ignore_ascii_case("content-length") {
            return value
                .trim()
                .parse::<usize>()
                .map_err(|_| Error::new(ErrorKind::InvalidData, "invalid Content-Length header"));
        }
    }
    Ok(0)
}

fn bad_preview_request_response(message: &str) -> Vec<u8> {
    let body = format!("Invalid preview request: {message}");
    let header = format!(
        "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain; charset=utf-8\r\nCache-Control: no-store\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let mut bytes = header.into_bytes();
    bytes.extend_from_slice(body.as_bytes());
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_preview_http_request_reads_full_body() {
        let request = b"POST /_dx/action HTTP/1.1\r\nHost: localhost\r\nContent-Length: 11\r\n\r\n{\"ok\":true}extra";
        let mut cursor = std::io::Cursor::new(&request[..]);

        let parsed = read_preview_http_request(&mut cursor).expect("request");

        assert_eq!(
            parsed,
            "POST /_dx/action HTTP/1.1\r\nHost: localhost\r\nContent-Length: 11\r\n\r\n{\"ok\":true}"
        );
    }

    #[test]
    fn read_preview_http_request_rejects_invalid_content_length() {
        let request = b"POST / HTTP/1.1\r\nContent-Length: nope\r\n\r\n{}";
        let mut cursor = std::io::Cursor::new(&request[..]);

        let error = read_preview_http_request(&mut cursor).unwrap_err();

        assert_eq!(error.kind(), ErrorKind::InvalidData);
    }

    #[test]
    fn preview_request_keep_alive_matches_http_semantics() {
        assert!(preview_request_wants_keep_alive(
            "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n"
        ));
        assert!(preview_request_wants_keep_alive(
            "GET / HTTP/1.0\r\nHost: localhost\r\nConnection: keep-alive\r\n\r\n"
        ));
        assert!(!preview_request_wants_keep_alive(
            "GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
        ));
        assert!(!preview_request_wants_keep_alive(
            "GET / HTTP/1.0\r\nHost: localhost\r\n\r\n"
        ));
    }
}
