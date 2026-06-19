use std::path::{Path, PathBuf};

use super::Cli;
use super::app_page_routes;

pub(super) fn route_from_app_path(cwd: &Path, app_route_path: &Path) -> String {
    let relative = Cli::relative_cli_path(cwd, app_route_path);
    app_page_routes::route_path_from_page_source_path(&relative).unwrap_or_else(|| "/".to_string())
}

pub(super) fn app_build_output_dir(cwd: &Path, app_route_path: &Path) -> PathBuf {
    let route = route_from_app_path(cwd, app_route_path);
    if route == "/" {
        PathBuf::from("app")
    } else {
        PathBuf::from("app").join(route.trim_start_matches('/'))
    }
}
