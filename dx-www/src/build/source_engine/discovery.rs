use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::app_router_segments::{
    UnsupportedAppRouteSegment, UnsupportedAppRouteSegmentReason,
    has_unsupported_app_page_route_segments, unsupported_app_page_route_segment,
};
use crate::error::DxResult;

const APP_ROUTE_ROOTS: &[&str] = &["app", "src/app"];
const APP_PAGE_FILENAMES: &[&str] = &["page.tsx", "page.jsx", "page.ts", "page.js"];

#[derive(Debug, Clone, Default)]
pub struct SourceBuildInputs {
    pub routes: Vec<PathBuf>,
    pub skipped_routes: Vec<SourceSkippedAppRoute>,
    pub route_handlers: Vec<PathBuf>,
    pub styles: Vec<PathBuf>,
    pub content_documents: Vec<PathBuf>,
    pub assets: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceSkippedAppRoute {
    pub path: PathBuf,
    pub reason: UnsupportedAppRouteSegmentReason,
    pub segment: Option<String>,
}

pub fn discover_source_inputs(project_root: &Path) -> DxResult<SourceBuildInputs> {
    let mut inputs = SourceBuildInputs::default();

    collect_app_routes(project_root, &mut inputs.routes, &mut inputs.skipped_routes);
    collect_app_route_handlers(project_root, &mut inputs.route_handlers);
    collect_extension_files(&project_root.join("styles"), &["css"], &mut inputs.styles);
    collect_content_documents(project_root, &mut inputs.content_documents);
    collect_all_files(&project_root.join("public"), &mut inputs.assets);

    inputs.routes.sort();
    inputs
        .skipped_routes
        .sort_by(|left, right| left.path.cmp(&right.path));
    inputs.route_handlers.sort();
    inputs.styles.sort();
    inputs.content_documents.sort();
    inputs.assets.sort();

    Ok(inputs)
}

fn collect_content_documents(project_root: &Path, documents: &mut Vec<PathBuf>) {
    for folder in ["docs", "content"] {
        collect_extension_files(&project_root.join(folder), &["md", "mdx"], documents);
    }
}

fn collect_app_routes(
    project_root: &Path,
    routes: &mut Vec<PathBuf>,
    skipped_routes: &mut Vec<SourceSkippedAppRoute>,
) {
    for app_dir in app_route_roots(project_root) {
        for entry in WalkDir::new(&app_dir)
            .into_iter()
            .filter_entry(|entry| !is_ignored_dir(entry.path()))
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            let relative = entry.path().strip_prefix(&app_dir).unwrap_or(entry.path());
            if is_app_page_file_name(&entry.file_name().to_string_lossy()) {
                if let Some(unsupported) = unsupported_app_route_segment(relative) {
                    skipped_routes.push(SourceSkippedAppRoute {
                        path: entry.path().to_path_buf(),
                        reason: unsupported.reason,
                        segment: unsupported.segment,
                    });
                } else {
                    routes.push(entry.path().to_path_buf());
                }
            }
        }
    }
}

fn collect_app_route_handlers(project_root: &Path, route_handlers: &mut Vec<PathBuf>) {
    for app_dir in app_route_roots(project_root) {
        for entry in WalkDir::new(app_dir)
            .into_iter()
            .filter_entry(|entry| !is_ignored_dir(entry.path()))
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            let file_name = entry.file_name().to_string_lossy().to_ascii_lowercase();
            if matches!(
                file_name.as_str(),
                "route.ts" | "route.tsx" | "route.js" | "route.jsx"
            ) {
                route_handlers.push(entry.path().to_path_buf());
            }
        }
    }
}

fn app_route_roots(project_root: &Path) -> Vec<PathBuf> {
    APP_ROUTE_ROOTS
        .iter()
        .map(|root| project_root.join(root))
        .filter(|root| root.is_dir())
        .collect()
}

fn collect_extension_files(root: &Path, extensions: &[&str], files: &mut Vec<PathBuf>) {
    if !root.is_dir() {
        return;
    }

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| !is_ignored_dir(entry.path()))
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let Some(extension) = entry.path().extension().and_then(|value| value.to_str()) else {
            continue;
        };
        if extensions
            .iter()
            .any(|expected| extension.eq_ignore_ascii_case(expected))
        {
            files.push(entry.path().to_path_buf());
        }
    }
}

fn collect_all_files(root: &Path, files: &mut Vec<PathBuf>) {
    if !root.is_dir() {
        return;
    }

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| !is_ignored_dir(entry.path()))
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        files.push(entry.path().to_path_buf());
    }
}

fn is_ignored_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
            matches!(
                name,
                "node_modules" | ".git" | ".dx" | "target" | "dist" | "build"
            )
        })
}

fn is_app_page_file_name(file_name: &str) -> bool {
    APP_PAGE_FILENAMES
        .iter()
        .any(|expected| file_name.eq_ignore_ascii_case(expected))
}

fn unsupported_app_route_segment(relative_path: &Path) -> Option<UnsupportedAppRouteSegment> {
    let parent = relative_path.parent()?;
    let segments = parent
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    if !has_unsupported_app_page_route_segments(&segments) {
        return None;
    }
    unsupported_app_page_route_segment(&segments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_project(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "dx-www-source-discovery-{name}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("create temp project");
        root
    }

    fn write_page(root: &Path, relative_path: &str) {
        let path = root.join(relative_path);
        fs::create_dir_all(path.parent().expect("page parent")).expect("create page parent");
        fs::write(path, "export default function Page() { return <main />; }").expect("write page");
    }

    fn relative_path(root: &Path, path: &Path) -> String {
        path.strip_prefix(root)
            .unwrap_or(path)
            .components()
            .map(|component| component.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/")
    }

    #[test]
    fn source_discovery_skips_unsupported_app_page_route_shapes() {
        let root = temp_project("route-shape-validation");
        write_page(&root, "app/()/page.tsx");
        write_page(&root, "app/@/page.tsx");
        write_page(&root, "app/docs/[...slug]/details/page.tsx");
        write_page(&root, "app/[team]/[team]/page.tsx");
        write_page(&root, "app/docs/[...slug]/page.tsx");
        write_page(&root, "app/files/[path]/[[...rest]]/page.tsx");
        write_page(&root, "src/app/(shop)/products/[id]/page.tsx");

        let inputs = discover_source_inputs(&root).expect("discover source inputs");
        let routes = inputs
            .routes
            .iter()
            .map(|path| relative_path(&root, path))
            .collect::<Vec<_>>();

        assert!(!routes.contains(&"app/()/page.tsx".to_string()));
        assert!(!routes.contains(&"app/@/page.tsx".to_string()));
        assert!(!routes.contains(&"app/docs/[...slug]/details/page.tsx".to_string()));
        assert!(!routes.contains(&"app/[team]/[team]/page.tsx".to_string()));
        assert!(routes.contains(&"app/docs/[...slug]/page.tsx".to_string()));
        assert!(routes.contains(&"app/files/[path]/[[...rest]]/page.tsx".to_string()));
        assert!(routes.contains(&"src/app/(shop)/products/[id]/page.tsx".to_string()));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn source_discovery_reports_skipped_unsupported_app_page_route_diagnostics() {
        let root = temp_project("route-shape-diagnostics");
        write_page(&root, "app/()/page.tsx");
        write_page(&root, "app/[team]/[team]/page.tsx");
        write_page(&root, "app/docs/[...slug]/details/page.tsx");
        write_page(&root, "app/(shop)/products/[id]/page.tsx");

        let inputs = discover_source_inputs(&root).expect("discover source inputs");
        let skipped_for = |source_path: &str| {
            inputs
                .skipped_routes
                .iter()
                .find(|route| relative_path(&root, &route.path) == source_path)
                .unwrap_or_else(|| panic!("missing skipped route diagnostic for {source_path}"))
        };

        let malformed = skipped_for("app/()/page.tsx");
        assert_eq!(
            malformed.reason,
            UnsupportedAppRouteSegmentReason::MalformedSegment
        );
        assert_eq!(malformed.segment.as_deref(), Some("()"));

        let duplicate = skipped_for("app/[team]/[team]/page.tsx");
        assert_eq!(
            duplicate.reason,
            UnsupportedAppRouteSegmentReason::DuplicateParamName
        );
        assert_eq!(duplicate.segment.as_deref(), Some("[team]"));

        let nonterminal = skipped_for("app/docs/[...slug]/details/page.tsx");
        assert_eq!(
            nonterminal.reason,
            UnsupportedAppRouteSegmentReason::NonTerminalCatchAll
        );
        assert_eq!(nonterminal.segment.as_deref(), Some("details"));

        assert!(
            inputs
                .skipped_routes
                .iter()
                .all(|route| relative_path(&root, &route.path)
                    != "app/(shop)/products/[id]/page.tsx")
        );

        let _ = fs::remove_dir_all(root);
    }
}
