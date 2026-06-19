use std::collections::HashMap;
#[cfg(test)]
use std::path::Path;
use std::path::PathBuf;

use super::Cli;
#[cfg(test)]
use super::dev_http::dev_project_reload_token;
use super::dev_http::{DxCliHttpRequest, DxCliHttpResponse, dev_lookup_path};
use super::dev_response;
#[cfg(all(test, not(feature = "dev-server")))]
use super::dev_wire::{dev_response_cache_key, dev_response_headers, dev_wire_response_bytes};

impl Cli {
    #[cfg(test)]
    pub(super) fn handle_http_request(
        cwd: &PathBuf,
        request: &str,
        translations: &HashMap<String, String>,
    ) -> (String, String, String) {
        let response = Self::handle_http_response(cwd, request, translations);
        (response.status, response.content_type, response.body)
    }

    #[cfg(test)]
    pub(super) fn handle_http_response(
        cwd: &PathBuf,
        request: &str,
        translations: &HashMap<String, String>,
    ) -> DxCliHttpResponse {
        dev_response::handle_http_response(
            cwd,
            request,
            translations,
            Self::route_handler_http_response,
            Self::handle_parsed_request,
        )
    }

    pub(super) fn handle_parsed_http_response(
        cwd: &PathBuf,
        request: &DxCliHttpRequest,
        translations: &HashMap<String, String>,
    ) -> DxCliHttpResponse {
        dev_response::handle_parsed_http_response(
            cwd,
            request,
            translations,
            Self::route_handler_http_response,
            Self::handle_parsed_request,
        )
    }

    pub(super) fn dev_lookup_path(request_path: &str) -> String {
        dev_lookup_path(request_path)
    }

    #[cfg(test)]
    pub(super) fn dev_project_reload_token(cwd: &Path) -> String {
        dev_project_reload_token(cwd)
    }

    #[cfg(all(test, not(feature = "dev-server")))]
    pub(super) fn dev_response_cache_key(request: &DxCliHttpRequest) -> Option<String> {
        dev_response_cache_key(request)
    }

    #[cfg(all(test, not(feature = "dev-server")))]
    pub(super) fn dev_wire_response_bytes(
        response: &DxCliHttpResponse,
        head_request: bool,
    ) -> Vec<u8> {
        dev_wire_response_bytes(response, head_request)
    }

    #[cfg(all(test, not(feature = "dev-server")))]
    pub(super) fn dev_response_headers(
        response: &DxCliHttpResponse,
        content_length: usize,
    ) -> String {
        dev_response_headers(response, content_length)
    }
}
