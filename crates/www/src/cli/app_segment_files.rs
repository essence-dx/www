use std::path::{Path, PathBuf};

use dx_compiler::delivery::{DxReactAppSegmentKind, DxReactAppSegmentSource};

const APP_ROUTER_SOURCE_EXTENSIONS: &[&str] = &["tsx", "jsx", "ts", "js"];
const APP_ROUTER_SOURCE_ROOTS: &[&str] = &["app", "src/app"];

pub(super) fn app_route_roots(cwd: &Path) -> Vec<PathBuf> {
    APP_ROUTER_SOURCE_ROOTS
        .iter()
        .map(|root| cwd.join(root))
        .filter(|root| root.is_dir())
        .collect()
}

pub(super) fn app_root_for_route(cwd: &Path, app_route_path: &Path) -> Option<PathBuf> {
    APP_ROUTER_SOURCE_ROOTS
        .iter()
        .map(|root| cwd.join(root))
        .find(|root| app_route_path.starts_with(root))
}

pub(super) fn is_app_page_file_name(file_name: &str) -> bool {
    is_app_special_file_name(file_name, "page")
}

pub(super) fn strip_app_page_file_name(normalized_source_path: &str) -> Option<&str> {
    let path = strip_app_root_prefix(normalized_source_path)?;
    strip_app_special_file_name(path, "page")
}

pub(super) fn push_app_segment_sources(
    cwd: &Path,
    sources: &mut Vec<DxReactAppSegmentSource>,
    dir: &Path,
) {
    push_app_segment_source(cwd, sources, dir, "layout", DxReactAppSegmentKind::Layout);
    push_app_segment_source(
        cwd,
        sources,
        dir,
        "template",
        DxReactAppSegmentKind::Template,
    );
    push_app_segment_source(cwd, sources, dir, "loading", DxReactAppSegmentKind::Loading);
    push_app_segment_source(cwd, sources, dir, "error", DxReactAppSegmentKind::Error);
    push_app_segment_source(
        cwd,
        sources,
        dir,
        "not-found",
        DxReactAppSegmentKind::NotFound,
    );
}

fn push_app_segment_source(
    cwd: &Path,
    sources: &mut Vec<DxReactAppSegmentSource>,
    dir: &Path,
    stem: &str,
    kind: DxReactAppSegmentKind,
) {
    let Some(path) = first_existing_app_special_file(dir, stem) else {
        return;
    };
    let Ok(source) = std::fs::read_to_string(&path) else {
        return;
    };
    sources.push(DxReactAppSegmentSource {
        kind,
        source_path: relative_cli_path(cwd, &path),
        source,
    });
}

fn first_existing_app_special_file(dir: &Path, stem: &str) -> Option<std::path::PathBuf> {
    APP_ROUTER_SOURCE_EXTENSIONS
        .iter()
        .map(|extension| dir.join(format!("{stem}.{extension}")))
        .find(|path| path.is_file())
}

fn is_app_special_file_name(file_name: &str, stem: &str) -> bool {
    APP_ROUTER_SOURCE_EXTENSIONS
        .iter()
        .any(|extension| file_name == format!("{stem}.{extension}"))
}

fn strip_app_root_prefix(path: &str) -> Option<&str> {
    APP_ROUTER_SOURCE_ROOTS
        .iter()
        .find_map(|root| path.strip_prefix(&format!("{root}/")))
}

fn strip_app_special_file_name<'a>(path: &'a str, stem: &str) -> Option<&'a str> {
    for extension in APP_ROUTER_SOURCE_EXTENSIONS {
        let file_name = format!("{stem}.{extension}");
        if path == file_name {
            return Some("");
        }
        let suffix = format!("/{file_name}");
        if let Some(route) = path.strip_suffix(&suffix) {
            return Some(route);
        }
    }
    None
}

fn relative_cli_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn page_helpers_accept_next_familiar_extensions() {
        assert!(is_app_page_file_name("page.tsx"));
        assert!(is_app_page_file_name("page.jsx"));
        assert!(is_app_page_file_name("page.ts"));
        assert!(is_app_page_file_name("page.js"));
        assert!(!is_app_page_file_name("route.ts"));
        assert_eq!(strip_app_page_file_name("app/page.jsx"), Some(""));
        assert_eq!(
            strip_app_page_file_name("app/(marketing)/dashboard/page.js"),
            Some("(marketing)/dashboard")
        );
        assert_eq!(
            strip_app_page_file_name("src/app/(marketing)/dashboard/page.js"),
            Some("(marketing)/dashboard")
        );
    }

    #[test]
    fn app_route_roots_include_src_app_when_present() {
        let root =
            std::env::temp_dir().join(format!("dx-www-app-route-roots-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("src/app")).expect("src app dir");

        assert_eq!(app_route_roots(&root), vec![root.join("src/app")]);
        assert_eq!(
            app_root_for_route(&root, &root.join("src/app/blog/page.tsx")),
            Some(root.join("src/app"))
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn segment_sources_accept_next_familiar_extensions() {
        let root =
            std::env::temp_dir().join(format!("dx-www-app-segment-files-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let app = root.join("app");
        fs::create_dir_all(&app).expect("app dir");
        fs::write(
            app.join("layout.jsx"),
            "export default function Layout() {}",
        )
        .expect("layout");
        fs::write(
            app.join("template.js"),
            "export default function Template() {}",
        )
        .expect("template");
        fs::write(
            app.join("loading.ts"),
            "export default function Loading() {}",
        )
        .expect("loading");
        fs::write(app.join("error.jsx"), "export default function Error() {}").expect("error");
        fs::write(
            app.join("not-found.js"),
            "export default function NotFound() {}",
        )
        .expect("not found");

        let mut sources = Vec::new();
        push_app_segment_sources(&root, &mut sources, &app);
        let pairs = sources
            .iter()
            .map(|source| (source.kind, source.source_path.as_str()))
            .collect::<Vec<_>>();

        assert!(pairs.contains(&(DxReactAppSegmentKind::Layout, "app/layout.jsx")));
        assert!(pairs.contains(&(DxReactAppSegmentKind::Template, "app/template.js")));
        assert!(pairs.contains(&(DxReactAppSegmentKind::Loading, "app/loading.ts")));
        assert!(pairs.contains(&(DxReactAppSegmentKind::Error, "app/error.jsx")));
        assert!(pairs.contains(&(DxReactAppSegmentKind::NotFound, "app/not-found.js")));

        let _ = fs::remove_dir_all(root);
    }
}
