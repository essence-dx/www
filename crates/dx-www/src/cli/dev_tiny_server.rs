use std::collections::HashMap;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use crate::error::{DxError, DxResult};

use super::dev_command::DxDevParsedResponder;
use super::dev_wire::{
    DxDevResponseCache, dev_wire_request_error_response, dev_wire_request_keep_alive,
    handle_http_wire_response_cached_with_connection, new_dev_response_cache,
    read_http_wire_request,
};

struct DxTinyDevServerState {
    cwd: PathBuf,
    translations: HashMap<String, String>,
    response_cache: Arc<DxDevResponseCache>,
    hot_reload: bool,
    devtools: bool,
    respond: DxDevParsedResponder,
}

pub(super) fn serve_tiny_dev_server(
    listener: TcpListener,
    cwd: PathBuf,
    translations: HashMap<String, String>,
    hot_reload: bool,
    devtools: bool,
    respond: DxDevParsedResponder,
) -> DxResult<()> {
    let state = Arc::new(DxTinyDevServerState {
        cwd,
        translations,
        response_cache: Arc::new(new_dev_response_cache()),
        hot_reload,
        devtools,
        respond,
    });

    let workers = tiny_dev_worker_count();
    for worker_index in 1..workers {
        let listener = listener.try_clone().map_err(|error| DxError::IoError {
            path: None,
            message: format!("Failed to clone tiny dev listener: {error}"),
        })?;
        let state = Arc::clone(&state);
        std::thread::Builder::new()
            .name(format!("dx-tiny-dev-{worker_index}"))
            .spawn(move || accept_tiny_dev_connections(listener, state))
            .map_err(|error| DxError::IoError {
                path: None,
                message: format!("Failed to start tiny dev worker: {error}"),
            })?;
    }

    accept_tiny_dev_connections(listener, state);
    Ok(())
}

fn tiny_dev_worker_count() -> usize {
    std::thread::available_parallelism()
        .map(|parallelism| parallelism.get().saturating_mul(4))
        .unwrap_or(16)
        .clamp(16, 64)
}

fn accept_tiny_dev_connections(listener: TcpListener, state: Arc<DxTinyDevServerState>) {
    loop {
        match listener.accept() {
            Ok((stream, _address)) => serve_tiny_dev_connection(stream, &state),
            Err(error) => {
                eprintln!("Tiny dev connection error: {error}");
                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

fn serve_tiny_dev_connection(mut stream: TcpStream, state: &DxTinyDevServerState) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(15)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(15)));

    loop {
        match read_http_wire_request(&mut stream) {
            Ok(Some(request)) => {
                let keep_alive = dev_wire_request_keep_alive(&request);
                let response = handle_http_wire_response_cached_with_connection(
                    &state.cwd,
                    &request,
                    &state.translations,
                    &state.response_cache,
                    state.hot_reload,
                    state.devtools,
                    state.respond,
                    keep_alive,
                );

                if stream.write_all(&response).is_err() || stream.flush().is_err() {
                    break;
                }
                if !keep_alive {
                    break;
                }
            }
            Ok(None) => break,
            Err(error) => {
                let response = dev_wire_request_error_response(&error);
                let _ = stream.write_all(&response);
                let _ = stream.flush();
                break;
            }
        }
    }
}
