use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use super::route_request_values::{decode_path_segment, decode_path_segments, parse_search_params};

const ROUTE_HANDLER_FILENAMES: &[&str] = &["route.ts", "route.tsx", "route.js", "route.jsx"];
const APP_API_ROUTE_ROOTS: &[&str] = &["app/api", "src/app/api"];

#[derive(Debug, Clone)]
pub(super) struct AppApiRouteMatch {
    pub(super) path: PathBuf,
    pub(super) params: BTreeMap<String, String>,
    pub(super) search_params: BTreeMap<String, String>,
}

pub(super) fn route_handler_match(cwd: &Path, path: &str) -> Option<AppApiRouteMatch> {
    let request_path = path;
    let path_part = path.split('?').next().unwrap_or(path).trim_end_matches('/');
    let api_path = if path_part == "/api" {
        ""
    } else {
        path_part.strip_prefix("/api/")?
    };
    if api_path
        .split('/')
        .any(|segment| segment == "." || segment == ".." || segment.contains('\\'))
    {
        return None;
    }
    for api_root in app_api_route_roots(cwd) {
        let exact_dir = if api_path.is_empty() {
            api_root
        } else {
            api_root.join(api_path)
        };
        if let Some(exact) = route_handler_file_in(&exact_dir) {
            return Some(AppApiRouteMatch {
                path: exact,
                params: BTreeMap::new(),
                search_params: parse_search_params(request_path),
            });
        }
    }
    dynamic_route_handler_match(cwd, api_path, request_path)
}

fn route_handler_file_in(directory: &Path) -> Option<PathBuf> {
    ROUTE_HANDLER_FILENAMES
        .iter()
        .map(|file_name| directory.join(file_name))
        .find(|candidate| candidate.is_file())
}

fn app_api_route_roots(cwd: &Path) -> Vec<PathBuf> {
    APP_API_ROUTE_ROOTS
        .iter()
        .map(|root| cwd.join(root))
        .collect()
}

fn dynamic_route_handler_match(cwd: &Path, api_path: &str, path: &str) -> Option<AppApiRouteMatch> {
    let request_segments = if api_path.is_empty() {
        Vec::new()
    } else {
        api_path.split('/').collect::<Vec<_>>()
    };
    let mut matches = Vec::new();
    for (root_index, api_root) in app_api_route_roots(cwd).into_iter().enumerate() {
        if !api_root.is_dir() {
            continue;
        }
        let root_score = APP_API_ROUTE_ROOTS.len().saturating_sub(root_index);
        let mut stack = vec![api_root.clone()];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let handler_path = entry.path();
                if handler_path.is_dir() {
                    stack.push(handler_path);
                    continue;
                }
                let file_name = entry.file_name();
                let Some(file_name) = file_name.to_str() else {
                    continue;
                };
                if !ROUTE_HANDLER_FILENAMES.contains(&file_name) {
                    continue;
                }
                let Some(parent) = handler_path.parent() else {
                    continue;
                };
                let Ok(relative) = parent.strip_prefix(&api_root) else {
                    continue;
                };
                let route_segments = relative
                    .components()
                    .map(|component| component.as_os_str().to_string_lossy().to_string())
                    .collect::<Vec<_>>();
                if let Some((score, params)) =
                    route_match_candidate(&route_segments, &request_segments)
                {
                    matches.push((
                        score,
                        root_score,
                        AppApiRouteMatch {
                            path: handler_path,
                            params,
                            search_params: parse_search_params(path),
                        },
                    ));
                }
            }
        }
    }
    matches
        .into_iter()
        .max_by_key(|(score, root_score, _)| (*score, *root_score))
        .map(|(_, _, route_match)| route_match)
}

fn route_match_candidate(
    route_segments: &[String],
    request_segments: &[&str],
) -> Option<(usize, BTreeMap<String, String>)> {
    let mut params = BTreeMap::new();
    let mut score = 0usize;
    let mut request_index = 0usize;
    for route_segment in route_segments {
        if is_app_router_non_path_segment(route_segment) {
            continue;
        }
        if let Some(name) = route_segment
            .strip_prefix("[[...")
            .and_then(|segment| segment.strip_suffix("]]"))
        {
            if name.is_empty() {
                return None;
            }
            params.insert(
                name.to_string(),
                decode_path_segments(&request_segments[request_index..]),
            );
            return Some((score + 1, params));
        }
        if let Some(name) = route_segment
            .strip_prefix("[...")
            .and_then(|segment| segment.strip_suffix(']'))
        {
            if name.is_empty() || request_index >= request_segments.len() {
                return None;
            }
            params.insert(
                name.to_string(),
                decode_path_segments(&request_segments[request_index..]),
            );
            return Some((
                score + request_segments.len().saturating_sub(request_index),
                params,
            ));
        }
        let request_segment = request_segments.get(request_index)?;
        if route_segment.starts_with('[') && route_segment.ends_with(']') {
            let name = route_segment
                .strip_prefix('[')
                .and_then(|segment| segment.strip_suffix(']'))?;
            if name.is_empty() {
                return None;
            }
            params.insert(name.to_string(), decode_path_segment(request_segment));
            score += 1;
        } else if route_segment == request_segment {
            score += 10;
        } else {
            return None;
        }
        request_index += 1;
    }
    (request_index == request_segments.len()).then_some((score, params))
}

fn is_app_router_non_path_segment(segment: &str) -> bool {
    is_app_router_route_group_segment(segment) || is_app_router_parallel_slot_segment(segment)
}

fn is_app_router_route_group_segment(segment: &str) -> bool {
    segment.len() > 2
        && segment.starts_with('(')
        && segment.ends_with(')')
        && !segment.starts_with("(.")
}

fn is_app_router_parallel_slot_segment(segment: &str) -> bool {
    segment.len() > 1 && segment.starts_with('@')
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::PathBuf};

    fn temp_project(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "dx-www-app-api-routes-{}-{}",
            name,
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("create temp project");
        root
    }

    fn write_route(root: &Path, relative_path: &str) {
        let path = root.join(relative_path);
        fs::create_dir_all(path.parent().expect("route parent")).expect("create route parent");
        fs::write(
            path,
            "export function GET() { return Response.json({ ok: true }); }",
        )
        .expect("write route");
    }

    #[test]
    fn route_handler_match_accepts_next_route_handler_extensions() {
        let exact_root = temp_project("tsx");
        write_route(&exact_root, "app/api/status/route.tsx");
        let exact = route_handler_match(&exact_root, "/api/status?preview=1")
            .expect("match exact route.tsx handler");
        assert_eq!(exact.path, exact_root.join("app/api/status/route.tsx"));
        assert_eq!(exact.search_params.get("preview"), Some(&"1".to_string()));
        let exact_without_query =
            route_handler_match(&exact_root, "/api/status").expect("match route.tsx handler");
        assert_eq!(
            exact_without_query.path,
            exact_root.join("app/api/status/route.tsx")
        );
        let _ = fs::remove_dir_all(exact_root);

        let dynamic_root = temp_project("js");
        write_route(&dynamic_root, "app/api/users/[id]/route.js");
        let dynamic =
            route_handler_match(&dynamic_root, "/api/users/42").expect("match dynamic route.js");
        assert_eq!(
            dynamic.path,
            dynamic_root.join("app/api/users/[id]/route.js")
        );
        assert_eq!(dynamic.params.get("id"), Some(&"42".to_string()));
        let _ = fs::remove_dir_all(dynamic_root);

        let catch_all_root = temp_project("jsx");
        write_route(&catch_all_root, "app/api/files/[...path]/route.jsx");
        let catch_all = route_handler_match(&catch_all_root, "/api/files/a/b")
            .expect("match catch-all route.jsx");
        assert_eq!(
            catch_all.path,
            catch_all_root.join("app/api/files/[...path]/route.jsx")
        );
        assert_eq!(catch_all.params.get("path"), Some(&"a/b".to_string()));
        let _ = fs::remove_dir_all(catch_all_root);

        let src_root = temp_project("src-app");
        write_route(&src_root, "src/app/api/status/route.ts");
        let src = route_handler_match(&src_root, "/api/status?preview=1")
            .expect("match src/app route.ts handler");
        assert_eq!(src.path, src_root.join("src/app/api/status/route.ts"));
        assert_eq!(src.search_params.get("preview"), Some(&"1".to_string()));
        let _ = fs::remove_dir_all(src_root);
    }

    #[test]
    fn route_handler_match_ignores_app_router_route_groups() {
        let root = temp_project("route-group");
        write_route(&root, "app/api/(internal)/status/route.tsx");

        let route_match = route_handler_match(&root, "/api/status")
            .expect("match route handler nested behind an App Router route group");
        assert_eq!(
            route_match.path,
            root.join("app/api/(internal)/status/route.tsx")
        );
        assert!(route_match.params.is_empty());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_handler_match_ignores_app_router_parallel_slots() {
        let root = temp_project("parallel-slot");
        write_route(&root, "app/api/@admin/status/route.tsx");

        let route_match = route_handler_match(&root, "/api/status")
            .expect("match route handler nested behind an App Router parallel slot");
        assert_eq!(
            route_match.path,
            root.join("app/api/@admin/status/route.tsx")
        );
        assert!(route_match.params.is_empty());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_handler_match_decodes_request_params_and_search_params() {
        let root = temp_project("decode");
        write_route(&root, "app/api/users/[id]/route.ts");

        let route_match = route_handler_match(
            &root,
            "/api/users/alice%20ng?tab=launch+plan&encoded=a%2Fb&bad=%zz",
        )
        .expect("match route handler with encoded request values");

        assert_eq!(route_match.params.get("id"), Some(&"alice ng".to_string()));
        assert_eq!(
            route_match.search_params.get("tab"),
            Some(&"launch plan".to_string())
        );
        assert_eq!(
            route_match.search_params.get("encoded"),
            Some(&"a/b".to_string())
        );
        assert_eq!(
            route_match.search_params.get("bad"),
            Some(&"%zz".to_string())
        );

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn route_handler_match_prefers_app_api_over_src_app_api_for_equal_dynamic_score() {
        let root = temp_project("root-precedence");
        write_route(&root, "app/api/users/[id]/route.ts");
        write_route(&root, "src/app/api/users/[id]/route.ts");

        let route_match =
            route_handler_match(&root, "/api/users/42").expect("match app/api dynamic route");

        assert_eq!(route_match.path, root.join("app/api/users/[id]/route.ts"));
        assert_eq!(route_match.params.get("id"), Some(&"42".to_string()));

        let _ = fs::remove_dir_all(root);
    }
}
