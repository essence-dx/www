use std::path::Path;

use crate::app_router_segments::{self, UnsupportedAppRouteSegmentReason};

use super::app_page_routes::raw_page_route_segments_from_source_path;
use super::app_segment_files::{self, is_app_page_file_name};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AppSkippedPageRouteSummary {
    pub(super) source_path: String,
    pub(super) reason: UnsupportedAppRouteSegmentReason,
    pub(super) segment: Option<String>,
}

pub(super) fn discover_skipped_page_route_summaries(cwd: &Path) -> Vec<AppSkippedPageRouteSummary> {
    let mut skipped_routes = Vec::new();
    for app_root in app_segment_files::app_route_roots(cwd) {
        skipped_routes.extend(
            walkdir::WalkDir::new(&app_root)
                .into_iter()
                .filter_map(Result::ok)
                .filter(is_app_page_file_entry)
                .filter(|entry| skips_node_modules(entry.path()))
                .filter_map(|entry| {
                    let source_path = relative_app_source_path(cwd, entry.path());
                    let segments = raw_page_route_segments_from_source_path(&source_path)?;
                    let unsupported =
                        app_router_segments::unsupported_app_page_route_segment(&segments)?;
                    Some(AppSkippedPageRouteSummary {
                        source_path,
                        reason: unsupported.reason,
                        segment: unsupported.segment,
                    })
                }),
        );
    }
    skipped_routes.sort_by(|left, right| left.source_path.cmp(&right.source_path));
    skipped_routes
}

fn is_app_page_file_entry(entry: &walkdir::DirEntry) -> bool {
    entry.file_type().is_file()
        && entry
            .file_name()
            .to_str()
            .is_some_and(is_app_page_file_name)
}

fn skips_node_modules(path: &Path) -> bool {
    path.components()
        .all(|component| component.as_os_str().to_string_lossy() != "node_modules")
}

fn relative_app_source_path(cwd: &Path, path: &Path) -> String {
    path.strip_prefix(cwd)
        .unwrap_or(path)
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    fn temp_project(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "dx-www-app-page-route-diagnostics-{}-{}",
            name,
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

    #[test]
    fn discover_page_routes_reports_skipped_route_diagnostics() {
        let root = temp_project("skipped-route-diagnostics");
        write_page(&root, "app/()/page.tsx");
        write_page(&root, "app/[team]/[team]/page.tsx");
        write_page(&root, "app/docs/[...slug]/details/page.tsx");
        write_page(&root, "app/(marketing)/about/page.tsx");

        let skipped = discover_skipped_page_route_summaries(&root);
        let summary_for = |source_path: &str| {
            skipped
                .iter()
                .find(|summary| summary.source_path == source_path)
                .unwrap_or_else(|| panic!("missing skipped route summary for {source_path}"))
        };

        let malformed = summary_for("app/()/page.tsx");
        assert_eq!(
            malformed.reason,
            UnsupportedAppRouteSegmentReason::MalformedSegment
        );
        assert_eq!(malformed.segment.as_deref(), Some("()"));

        let duplicate = summary_for("app/[team]/[team]/page.tsx");
        assert_eq!(
            duplicate.reason,
            UnsupportedAppRouteSegmentReason::DuplicateParamName
        );
        assert_eq!(duplicate.segment.as_deref(), Some("[team]"));

        let nonterminal = summary_for("app/docs/[...slug]/details/page.tsx");
        assert_eq!(
            nonterminal.reason,
            UnsupportedAppRouteSegmentReason::NonTerminalCatchAll
        );
        assert_eq!(nonterminal.segment.as_deref(), Some("details"));

        assert!(
            skipped
                .iter()
                .all(|summary| summary.source_path != "app/(marketing)/about/page.tsx")
        );

        let _ = fs::remove_dir_all(root);
    }
}
