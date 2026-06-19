use super::super::directives::collect_top_level_directives;

/// balanced-rsc-boundaries scanner for Lane 11 Next custom-transform receipts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RscBoundaryDetection {
    pub(super) directives: Vec<String>,
    pub(super) use_client: bool,
    pub(super) use_server: bool,
    pub(super) use_cache: bool,
    pub(super) cache_directives: Vec<String>,
    pub(super) client_entry: bool,
    pub(super) server_action_file: bool,
    pub(super) client_boundary_needed: bool,
    pub(super) metadata_export_in_client: bool,
    pub(super) metadata_export_names: Vec<String>,
    pub(super) metadata_export_count: usize,
    pub(super) error_file_requires_client: bool,
    pub(super) hook_or_event_boundary: bool,
    pub(super) client_entry_reasons: Vec<&'static str>,
    pub(super) source_owned_boundary: bool,
    pub(super) runtime_proxy_generated: bool,
}

pub(super) fn collect_rsc_boundary_detections(
    source_path: &str,
    source_kind: &str,
    source: &str,
    metadata_export_names: &[String],
) -> Option<RscBoundaryDetection> {
    let metadata_export = !metadata_export_names.is_empty();
    let directives = collect_top_level_directives(source);
    let use_client = directives.contains("use client");
    let use_server = directives.contains("use server");
    let cache_directives = directives
        .iter()
        .filter(|directive| *directive == "use cache" || directive.starts_with("use cache:"))
        .cloned()
        .collect::<Vec<_>>();
    let use_cache = !cache_directives.is_empty();
    let error_file = source_kind == "error" || normalized_path(source_path).ends_with("/error.tsx");
    let hook_or_event_boundary = source_has_hook_or_event(source);
    let mut client_entry_reasons = Vec::new();
    if use_client {
        client_entry_reasons.push("use-client-directive");
    }
    if hook_or_event_boundary {
        client_entry_reasons.push("hook-or-event-handler");
    }
    if error_file {
        client_entry_reasons.push("app-error-boundary");
    }
    let client_boundary_needed = !client_entry_reasons.is_empty();

    if !(use_client
        || use_server
        || use_cache
        || error_file
        || metadata_export
        || hook_or_event_boundary)
    {
        return None;
    }

    Some(RscBoundaryDetection {
        directives: directives.into_iter().collect(),
        use_client,
        use_server,
        use_cache,
        cache_directives,
        client_entry: use_client,
        server_action_file: use_server,
        client_boundary_needed,
        metadata_export_in_client: use_client && metadata_export,
        metadata_export_names: metadata_export_names.to_vec(),
        metadata_export_count: metadata_export_names.len(),
        error_file_requires_client: error_file,
        hook_or_event_boundary,
        client_entry_reasons,
        source_owned_boundary: true,
        runtime_proxy_generated: false,
    })
}

fn source_has_hook_or_event(source: &str) -> bool {
    [
        "useState(",
        "useEffect(",
        "useReducer(",
        "useTransition(",
        "useActionState(",
        "useContext(",
        "onClick=",
        "onSubmit=",
        "onChange=",
        "onInput=",
    ]
    .into_iter()
    .any(|needle| source.contains(needle))
}

fn normalized_path(source_path: &str) -> String {
    source_path.replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_client_boundary_reasons_without_proxy_generation() {
        let boundary = collect_rsc_boundary_detections(
            "app/error.tsx",
            "error",
            r#"/* license */
"use client";
export default function Error() {
  return <button onClick={() => {}}>Try again</button>;
}
"#,
            &["metadata".to_string(), "viewport".to_string()],
        )
        .expect("boundary");

        assert!(boundary.use_client);
        assert!(boundary.client_boundary_needed);
        assert!(boundary.metadata_export_in_client);
        assert_eq!(boundary.metadata_export_count, 2);
        assert_eq!(
            boundary.metadata_export_names,
            vec!["metadata".to_string(), "viewport".to_string()]
        );
        assert!(boundary.error_file_requires_client);
        assert!(!boundary.runtime_proxy_generated);
        assert!(
            boundary
                .client_entry_reasons
                .contains(&"use-client-directive")
        );
    }

    #[test]
    fn records_cache_directives_as_source_owned_evidence() {
        let boundary = collect_rsc_boundary_detections(
            "app/page.tsx",
            "page",
            r#""use cache: private";
export default function Page() { return <main />; }
"#,
            &[],
        )
        .expect("cache boundary");

        assert!(boundary.use_cache);
        assert_eq!(boundary.cache_directives, vec!["use cache: private"]);
        assert!(!boundary.client_boundary_needed);
        assert!(boundary.source_owned_boundary);
    }
}
