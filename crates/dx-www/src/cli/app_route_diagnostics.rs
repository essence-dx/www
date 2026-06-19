use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use dx_compiler::delivery::{DxTsxDiagnostic, parse_tsx_module};

use crate::error::{DxError, DxResult};
use crate::parser::style::extract_class_attribute_tokens;

use super::app_segment_files;
use super::css_diagnostics;
use super::dx_style_support::unsupported_scanned_classes;

const APP_SEGMENT_DIAGNOSTIC_FILES: [&str; 5] = [
    "layout.tsx",
    "template.tsx",
    "loading.tsx",
    "error.tsx",
    "not-found.tsx",
];
const APP_ROOT_DIAGNOSTIC_FILES: [&str; 1] = ["global-error.tsx"];

pub(super) fn validate_app_route_source(project_root: &Path, route_path: &Path) -> DxResult<()> {
    validate_source_file(project_root, route_path)?;
    validate_related_app_route_sources(project_root, route_path)?;
    css_diagnostics::validate_style_sources(project_root)
}

pub(super) fn validate_app_route_handlers(project_root: &Path) -> DxResult<()> {
    for source_path in app_route_handler_diagnostic_paths(project_root) {
        validate_source_file(project_root, &source_path)?;
    }

    Ok(())
}

fn validate_related_app_route_sources(project_root: &Path, route_path: &Path) -> DxResult<()> {
    for source_path in app_segment_diagnostic_paths(project_root, route_path)
        .into_iter()
        .chain(component_diagnostic_paths(project_root))
    {
        if source_path == route_path {
            continue;
        }
        validate_source_file(project_root, &source_path)?;
    }

    Ok(())
}

fn validate_source_file(project_root: &Path, source_file: &Path) -> DxResult<()> {
    let source = std::fs::read_to_string(source_file).map_err(|source| DxError::FileReadError {
        path: source_file.to_path_buf(),
        source,
    })?;
    let source_path = project_relative_slash_path(project_root, source_file);
    let module = parse_tsx_module(&source_path, &source);

    if let Some(diagnostic) = module.diagnostics.first() {
        return Err(tsx_parse_error(&source_path, &source, diagnostic));
    }

    if let Some(error) = dx_style_source_error(project_root, &source_path, &source) {
        return Err(error);
    }

    Ok(())
}

fn app_segment_diagnostic_paths(project_root: &Path, route_path: &Path) -> Vec<PathBuf> {
    let app_root = app_segment_files::app_root_for_route(project_root, route_path)
        .unwrap_or_else(|| project_root.join("app"));
    let route_dir = route_path.parent().unwrap_or(app_root.as_path());
    let mut dirs = vec![app_root.clone()];
    if let Ok(relative_route_dir) = route_dir.strip_prefix(&app_root) {
        let mut current = app_root.clone();
        for component in relative_route_dir.components() {
            current.push(component.as_os_str());
            dirs.push(current.clone());
        }
    }
    dirs.sort();

    let mut paths = Vec::new();
    for file_name in APP_ROOT_DIAGNOSTIC_FILES {
        let path = app_root.join(file_name);
        if path.is_file() {
            paths.push(path);
        }
    }
    for dir in dirs {
        for file_name in APP_SEGMENT_DIAGNOSTIC_FILES {
            let path = dir.join(file_name);
            if path.is_file() {
                paths.push(path);
            }
        }
    }
    paths.sort();
    paths.dedup();
    paths
}

fn app_route_handler_diagnostic_paths(project_root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for app_root in app_segment_files::app_route_roots(project_root) {
        collect_app_route_handler_diagnostic_paths(&app_root, &mut paths);
    }
    paths.sort();
    paths.dedup();
    paths
}

fn collect_app_route_handler_diagnostic_paths(dir: &Path, paths: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            if !is_skipped_source_dir(&path) {
                collect_app_route_handler_diagnostic_paths(&path, paths);
            }
            continue;
        }

        if is_route_handler_source(&path) {
            paths.push(path);
        }
    }
}

fn component_diagnostic_paths(project_root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    collect_component_diagnostic_paths(&project_root.join("components"), &mut paths);
    paths.sort();
    paths.dedup();
    paths
}

fn collect_component_diagnostic_paths(dir: &Path, paths: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            if !is_skipped_source_dir(&path) {
                collect_component_diagnostic_paths(&path, paths);
            }
            continue;
        }

        if path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| matches!(extension, "tsx" | "jsx"))
        {
            paths.push(path);
        }
    }
}

fn is_route_handler_source(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| matches!(name, "route.ts" | "route.tsx" | "route.js" | "route.jsx"))
}

fn is_skipped_source_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| matches!(name, ".dx" | ".git" | "node_modules" | "target"))
}

pub(super) fn app_route_compile_error(
    project_root: &Path,
    route_path: &Path,
    message: String,
) -> DxError {
    let source_path = project_relative_slash_path(project_root, route_path);
    match std::fs::read_to_string(route_path) {
        Ok(source) => DxError::CompilationError {
            message,
            file: PathBuf::from(source_path),
            src: Some(source),
            span: Some(miette::SourceSpan::new(0usize.into(), 1usize)),
        },
        Err(_) => DxError::CompilationError {
            message,
            file: PathBuf::from(source_path),
            src: None,
            span: None,
        },
    }
}

fn tsx_parse_error(source_path: &str, source: &str, diagnostic: &DxTsxDiagnostic) -> DxError {
    let span_len = diagnostic
        .span
        .end
        .saturating_sub(diagnostic.span.start)
        .max(1);
    DxError::ParseError {
        message: diagnostic.message.clone(),
        file: PathBuf::from(source_path),
        line: Some(diagnostic.span.line.max(1) as u32),
        column: Some(diagnostic.span.column.max(1) as u32),
        src: Some(source.to_string()),
        span: Some(miette::SourceSpan::new(
            diagnostic.span.start.into(),
            span_len,
        )),
    }
}

fn dx_style_source_error(project_root: &Path, source_path: &str, source: &str) -> Option<DxError> {
    let scanned_classes = extract_class_attribute_tokens(source)
        .into_iter()
        .collect::<BTreeSet<_>>();
    let authored_classes = authored_css_class_names(project_root);
    let unsupported = unsupported_scanned_classes(&scanned_classes)
        .into_iter()
        .find(|item| !authored_classes.contains(&item.class_name))?;
    let marker = dx_style_source_marker(&unsupported.class_name);
    let span_start = source
        .find(&marker)
        .or_else(|| source.find("className"))
        .unwrap_or(0);
    let span_len = marker.len().max(1);

    Some(DxError::CompilationError {
        message: format!(
            "dx-style unsupported class `{}`: {}",
            unsupported.class_name, unsupported.reason
        ),
        file: PathBuf::from(source_path),
        src: Some(source.to_string()),
        span: Some(miette::SourceSpan::new(span_start.into(), span_len)),
    })
}

fn authored_css_class_names(project_root: &Path) -> BTreeSet<String> {
    let mut class_names = BTreeSet::new();
    collect_authored_css_class_names(&project_root.join("styles"), &mut class_names);
    collect_authored_css_class_names(&project_root.join("app"), &mut class_names);
    collect_authored_css_class_names(&project_root.join("components"), &mut class_names);
    class_names
}

fn collect_authored_css_class_names(path: &Path, class_names: &mut BTreeSet<String>) {
    let Ok(metadata) = std::fs::metadata(path) else {
        return;
    };
    if metadata.is_file() {
        if path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("css"))
        {
            if let Ok(source) = std::fs::read_to_string(path) {
                collect_css_class_selectors(&source, class_names);
            }
        }
        return;
    }
    if !metadata.is_dir() || is_skipped_source_dir(path) {
        return;
    }

    let Ok(entries) = std::fs::read_dir(path) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        collect_authored_css_class_names(&entry.path(), class_names);
    }
}

fn collect_css_class_selectors(source: &str, class_names: &mut BTreeSet<String>) {
    let bytes = source.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] != b'.' {
            index += 1;
            continue;
        }

        let start = index + 1;
        if start >= bytes.len() || !is_css_identifier_start(bytes[start]) {
            index += 1;
            continue;
        }

        let mut end = start + 1;
        while end < bytes.len() && is_css_identifier_continue(bytes[end]) {
            end += 1;
        }
        if let Some(class_name) = source.get(start..end) {
            class_names.insert(class_name.to_string());
        }
        index = end;
    }
}

fn is_css_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || matches!(byte, b'_' | b'-')
}

fn is_css_identifier_continue(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-')
}

fn dx_style_source_marker(class_name: &str) -> String {
    if let Some(prefix) = class_name.strip_prefix("dx-grouping-error:unclosed-group:") {
        return format!("{prefix}:(");
    }
    class_name.to_string()
}

fn project_relative_slash_path(project_root: &Path, path: &Path) -> String {
    path.strip_prefix(project_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn app_route_handler_diagnostics_scan_handlers_and_skip_node_modules() {
        let project = tempdir().expect("temp project");
        for route in [
            "app/api/health/route.ts",
            "app/api/dashboard/route.tsx",
            "app/api/status/route.js",
            "app/api/session/route.jsx",
            "app/node_modules/pkg/route.ts",
        ] {
            let path = project.path().join(route);
            fs::create_dir_all(path.parent().expect("route parent")).expect("create parent");
            fs::write(
                path,
                "export function GET() { return Response.json({ ok: true }); }\n",
            )
            .expect("write route");
        }
        fs::write(
            project.path().join("app/api/health/not-route.ts"),
            "export const ignored = true;\n",
        )
        .expect("write ignored source");

        let routes = app_route_handler_diagnostic_paths(project.path())
            .into_iter()
            .map(|path| project_relative_slash_path(project.path(), &path))
            .collect::<Vec<_>>();

        assert_eq!(
            routes,
            vec![
                "app/api/dashboard/route.tsx",
                "app/api/health/route.ts",
                "app/api/session/route.jsx",
                "app/api/status/route.js",
            ]
        );
    }

    #[test]
    fn app_route_handler_diagnostics_report_parse_errors_with_source_span() {
        let project = tempdir().expect("temp project");
        let route = project.path().join("app/api/health/route.ts");
        fs::create_dir_all(route.parent().expect("route parent")).expect("create route parent");
        fs::write(
            &route,
            "export async function GET() {\n  return Response.json({ ok: true });\n",
        )
        .expect("write invalid route handler");

        let error = validate_app_route_handlers(project.path()).expect_err("route handler error");

        match error {
            DxError::ParseError {
                file, src, span, ..
            } => {
                assert_eq!(file, PathBuf::from("app/api/health/route.ts"));
                assert!(
                    src.as_deref()
                        .is_some_and(|source| source.contains("export async function GET()"))
                );
                assert!(span.is_some());
            }
            _ => panic!("expected route handler parse error"),
        }
    }

    #[test]
    fn app_route_diagnostics_validate_root_global_error_boundary() {
        let project = tempdir().expect("temp project");
        let route = project.path().join("app/dashboard/page.tsx");
        fs::create_dir_all(route.parent().expect("route parent")).expect("create route parent");
        fs::write(
            &route,
            "export default function Dashboard() { return <main>Dashboard</main>; }\n",
        )
        .expect("write dashboard page");

        fs::write(
            project.path().join("app/global-error.tsx"),
            "export default function GlobalError() {\n  return <html><body>Broken</body></html>;\n",
        )
        .expect("write invalid global error");

        let error = validate_app_route_source(project.path(), &route)
            .expect_err("root global-error.tsx syntax should be reported");

        match error {
            DxError::ParseError {
                file, src, span, ..
            } => {
                assert_eq!(file, PathBuf::from("app/global-error.tsx"));
                assert!(
                    src.as_deref()
                        .is_some_and(|source| source.contains("GlobalError"))
                );
                assert!(span.is_some());
            }
            _ => panic!("expected global error parse error"),
        }
    }

    #[test]
    fn app_route_diagnostics_validate_src_app_segment_files() {
        let project = tempdir().expect("temp project");
        let route = project.path().join("src/app/dashboard/page.tsx");
        fs::create_dir_all(route.parent().expect("route parent")).expect("create route parent");
        fs::write(
            &route,
            "export default function Dashboard() { return <main>Dashboard</main>; }\n",
        )
        .expect("write dashboard page");
        fs::write(
            project.path().join("src/app/layout.tsx"),
            "export default function Layout({ children }) {\n  return <html><body>{children}</body></html>;\n",
        )
        .expect("write invalid src/app layout");

        let error = validate_app_route_source(project.path(), &route)
            .expect_err("src/app layout syntax should be reported");

        match error {
            DxError::ParseError {
                file, src, span, ..
            } => {
                assert_eq!(file, PathBuf::from("src/app/layout.tsx"));
                assert!(
                    src.as_deref()
                        .is_some_and(|source| source.contains("Layout"))
                );
                assert!(span.is_some());
            }
            _ => panic!("expected src/app layout parse error"),
        }
    }

    #[test]
    fn app_route_diagnostics_allow_authored_tailwind_like_css_classes() {
        let project = tempdir().expect("temp project");
        let component = project.path().join("components/panel.tsx");
        let style = project.path().join("styles/globals.css");
        fs::create_dir_all(component.parent().expect("component parent"))
            .expect("create component parent");
        fs::create_dir_all(style.parent().expect("style parent")).expect("create style parent");
        fs::write(
            &style,
            ".content-docs-i18n-panel { display: grid; }\n.dashboard-panel { padding: 1rem; }\n",
        )
        .expect("write authored css");
        fs::write(
            &component,
            "export function Panel() { return <section className=\"dashboard-panel content-docs-i18n-panel\">Docs</section>; }\n",
        )
        .expect("write component");

        validate_source_file(project.path(), &component).expect("authored css classes are allowed");
    }

    #[test]
    fn app_route_diagnostics_still_report_missing_tailwind_like_css_classes() {
        let project = tempdir().expect("temp project");
        let component = project.path().join("components/panel.tsx");
        fs::create_dir_all(component.parent().expect("component parent"))
            .expect("create component parent");
        fs::write(
            &component,
            "export function Panel() { return <section className=\"content-docs-i18n-panel\">Docs</section>; }\n",
        )
        .expect("write component");

        let error = validate_source_file(project.path(), &component)
            .expect_err("missing authored css class should still fail");

        match error {
            DxError::CompilationError { message, .. } => {
                assert!(message.contains("content-docs-i18n-panel"));
                assert!(message.contains("dx-style unsupported class"));
            }
            _ => panic!("expected dx-style compilation error"),
        }
    }

    #[test]
    fn app_route_handler_diagnostics_scan_src_app_handlers() {
        let project = tempdir().expect("temp project");
        let route = project.path().join("src/app/api/health/route.ts");
        fs::create_dir_all(route.parent().expect("route parent")).expect("create route parent");
        fs::write(
            &route,
            "export function GET() { return Response.json({ ok: true }); }\n",
        )
        .expect("write src/app route handler");

        let routes = app_route_handler_diagnostic_paths(project.path())
            .into_iter()
            .map(|path| project_relative_slash_path(project.path(), &path))
            .collect::<Vec<_>>();

        assert_eq!(routes, vec!["src/app/api/health/route.ts"]);
    }
}
