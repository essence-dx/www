use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};

/// adapter-boundary-conflicts maps detected Next custom-transform surfaces into
/// DX-owned dx-check receipt diagnostics without enforcing Next runtime behavior.
pub(super) fn collect_next_custom_transform_conflicts(
    rsc_boundaries: &[Value],
    server_actions: &[Value],
    page_config_exports: &[Value],
    dynamic_imports: &[Value],
    font_loaders: &[Value],
    metadata_exports: &[Value],
) -> Vec<Value> {
    let mut conflicts = Vec::new();

    collect_rsc_boundary_conflicts(rsc_boundaries, &mut conflicts);
    collect_server_action_conflicts(server_actions, &mut conflicts);
    collect_page_config_conflicts(page_config_exports, &mut conflicts);
    collect_dynamic_import_conflicts(dynamic_imports, &mut conflicts);
    collect_font_loader_conflicts(font_loaders, &mut conflicts);
    collect_metadata_conflicts(metadata_exports, &mut conflicts);

    conflicts
}

fn collect_rsc_boundary_conflicts(rsc_boundaries: &[Value], conflicts: &mut Vec<Value>) {
    for boundary in rsc_boundaries {
        let source_path = boundary["source_path"].as_str().unwrap_or_default();
        if boundary["use_client"].as_bool().unwrap_or(false)
            && boundary["use_server"].as_bool().unwrap_or(false)
        {
            conflicts.push(conflict(
                source_path,
                "client-and-server-directives",
                "Next custom transforms reject files that combine use client and use server.",
            ));
        }
        if boundary["metadata_export_in_client"]
            .as_bool()
            .unwrap_or(false)
        {
            let metadata_export_names = boundary
                .get("metadata_export_names")
                .and_then(Value::as_array)
                .map(|names| {
                    names
                        .iter()
                        .filter_map(Value::as_str)
                        .map(str::to_string)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            conflicts.push(client_metadata_export_conflict(
                source_path,
                &metadata_export_names,
            ));
        }
        if boundary["error_file_requires_client"]
            .as_bool()
            .unwrap_or(false)
            && !boundary["use_client"].as_bool().unwrap_or(false)
        {
            conflicts.push(conflict(
                source_path,
                "error-boundary-without-use-client",
                "Next treats app error boundaries as client components; DX records the requirement without rewriting the file.",
            ));
        }
    }
}

fn collect_server_action_conflicts(server_actions: &[Value], conflicts: &mut Vec<Value>) {
    for action in server_actions {
        if !action["async_export"].as_bool().unwrap_or(true) {
            conflicts.push(conflict(
                action["source_path"].as_str().unwrap_or_default(),
                "sync-server-action",
                "Next server actions require async functions; DX records the mismatch without proxy generation.",
            ));
        }
    }
}

fn collect_page_config_conflicts(page_config_exports: &[Value], conflicts: &mut Vec<Value>) {
    for config in page_config_exports {
        let Some(issue) = config.get("compatibility_issue").and_then(Value::as_str) else {
            continue;
        };
        let source_path = config["source_path"].as_str().unwrap_or_default();
        match issue {
            "config-re-export" => conflicts.push(conflict(
                source_path,
                "page-config-re-export",
                "Next custom transforms reject re-exported legacy config; DX records the boundary without rewriting.",
            )),
            "config-must-be-object" => conflicts.push(conflict(
                source_path,
                "invalid-page-config-shape",
                "Next custom transforms expect legacy config to be an object literal.",
            )),
            "config-object-spread-unsupported" => conflicts.push(conflict(
                source_path,
                "invalid-page-config-spread",
                "Next custom transforms reject object spread inside legacy config; DX records this as receipt evidence.",
            )),
            _ => {}
        }
    }

    let page_config_sources = grouped_names(page_config_exports);
    for (source_path, names) in page_config_sources {
        if names.contains("config") && names.len() > 1 {
            conflicts.push(conflict(
                &source_path,
                "mixed-page-config-exports",
                "The legacy config export and route segment config exports are both present.",
            ));
        }
    }
}

fn collect_dynamic_import_conflicts(dynamic_imports: &[Value], conflicts: &mut Vec<Value>) {
    for dynamic_import in dynamic_imports {
        if dynamic_import["call"].as_str() == Some("next/dynamic")
            && dynamic_import["ssr_false"].as_bool().unwrap_or(false)
        {
            conflicts.push(conflict(
                dynamic_import["source_path"].as_str().unwrap_or_default(),
                "next-dynamic-ssr-false",
                "Next RSC validation rejects ssr: false in Server Components; DX keeps this as an adapter receipt.",
            ));
        }
        let Some(issue) = dynamic_import
            .get("compatibility_issue")
            .and_then(Value::as_str)
        else {
            continue;
        };
        let source_path = dynamic_import["source_path"].as_str().unwrap_or_default();
        match issue {
            "next-dynamic-missing-loader" => conflicts.push(conflict(
                source_path,
                "next-dynamic-missing-loader",
                "Next custom transforms require next/dynamic to receive a loader argument.",
            )),
            "next-dynamic-too-many-args" => conflicts.push(conflict(
                source_path,
                "next-dynamic-too-many-args",
                "Next custom transforms accept at most two next/dynamic arguments.",
            )),
            "next-dynamic-options-must-be-object" => conflicts.push(conflict(
                source_path,
                "next-dynamic-options-must-be-object",
                "Next custom transforms require next/dynamic options to be an object literal.",
            )),
            _ => {}
        }
    }
}

fn collect_font_loader_conflicts(font_loaders: &[Value], conflicts: &mut Vec<Value>) {
    for font in font_loaders {
        let Some(issue) = font.get("compatibility_issue").and_then(Value::as_str) else {
            continue;
        };
        let source_path = font["source_path"].as_str().unwrap_or_default();
        match issue {
            "font-loader-namespace-import" => conflicts.push(conflict(
                source_path,
                "font-loader-namespace-import",
                "Next font loaders reject namespace imports; DX records this without generating CSS imports.",
            )),
            "font-loader-call-outside-module-scope" => conflicts.push(conflict(
                source_path,
                "font-loader-call-outside-module-scope",
                "Next font loaders require calls assigned to const bindings at module scope.",
            )),
            "font-loader-call-must-be-const" => conflicts.push(conflict(
                source_path,
                "font-loader-call-must-be-const",
                "Next font loader calls must be assigned to const bindings.",
            )),
            "font-loader-options-spread-unsupported" => conflicts.push(conflict(
                source_path,
                "font-loader-options-spread-unsupported",
                "Next font loader arguments are recorded as unsupported when options use spread syntax.",
            )),
            _ => {}
        }
    }
}

fn collect_metadata_conflicts(metadata_exports: &[Value], conflicts: &mut Vec<Value>) {
    for metadata in metadata_exports {
        if metadata["metadata_conflict"].as_bool().unwrap_or(false) {
            conflicts.push(conflict(
                metadata["source_path"].as_str().unwrap_or_default(),
                "metadata-and-generateMetadata",
                "Next rejects exporting metadata and generateMetadata from the same server entry.",
            ));
        }
        if metadata["viewport_conflict"].as_bool().unwrap_or(false) {
            conflicts.push(conflict(
                metadata["source_path"].as_str().unwrap_or_default(),
                "viewport-and-generateViewport",
                "Next rejects exporting viewport and generateViewport from the same server entry.",
            ));
        }
        let Some(issues) = metadata
            .get("compatibility_issues")
            .and_then(Value::as_array)
        else {
            continue;
        };
        if issues
            .iter()
            .any(|issue| issue.as_str() == Some("metadata-re-export"))
        {
            conflicts.push(conflict(
                metadata["source_path"].as_str().unwrap_or_default(),
                "metadata-re-export",
                "Next metadata re-exports are recorded as source receipt evidence before adapter work.",
            ));
        }
    }
}

fn grouped_names(values: &[Value]) -> BTreeMap<String, BTreeSet<String>> {
    let mut grouped = BTreeMap::new();
    for value in values {
        let Some(source_path) = value.get("source_path").and_then(Value::as_str) else {
            continue;
        };
        let Some(name) = value.get("name").and_then(Value::as_str) else {
            continue;
        };
        grouped
            .entry(source_path.to_string())
            .or_insert_with(BTreeSet::new)
            .insert(name.to_string());
    }
    grouped
}

fn conflict(source_path: &str, kind: &str, message: &str) -> Value {
    json!({
        "source_path": source_path,
        "kind": kind,
        "message": message,
        "diagnostic_status": "dx-check-receipt-only",
    })
}

fn client_metadata_export_conflict(source_path: &str, metadata_export_names: &[String]) -> Value {
    let surface_summary = if metadata_export_names.is_empty() {
        "metadata/viewport exports".to_string()
    } else {
        metadata_export_names.join(", ")
    };
    json!({
        "source_path": source_path,
        "kind": "client-metadata-export",
        "message": format!(
            "Next rejects metadata/viewport exports in client-marked files; DX recorded: {surface_summary}."
        ),
        "metadata_export_names": metadata_export_names,
        "metadata_export_count": metadata_export_names.len(),
        "diagnostic_status": "dx-check-receipt-only",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_conflicts_as_dx_check_receipts_only() {
        let conflicts = collect_next_custom_transform_conflicts(
            &[json!({
                "source_path": "app/page.tsx",
                "use_client": true,
                "use_server": true,
                "metadata_export_in_client": false,
                "metadata_export_names": [],
                "metadata_export_count": 0,
                "error_file_requires_client": false,
            })],
            &[],
            &[],
            &[],
            &[],
            &[],
        );

        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0]["kind"], "client-and-server-directives");
        assert_eq!(conflicts[0]["diagnostic_status"], "dx-check-receipt-only");
    }
}
