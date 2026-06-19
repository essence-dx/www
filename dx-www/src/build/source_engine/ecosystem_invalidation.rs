use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::{Component, Path, PathBuf};

use serde_json::{Value, json};

use super::graph::normalize_path;

pub(super) fn build_graph_invalidation(
    project_root: &Path,
    nodes: &[Value],
    edges: &[Value],
    changed_paths: &[PathBuf],
) -> Value {
    if changed_paths.is_empty() {
        return empty_invalidation();
    }

    let node_ids_by_path = node_ids_by_path(nodes);
    let changed_node_ids = changed_node_ids(project_root, changed_paths, &node_ids_by_path);
    if changed_node_ids.is_empty() {
        return empty_invalidation();
    }

    let affected_node_ids = affected_node_ids(edges, &changed_node_ids);
    let emitted_output_node_ids = emitted_output_node_ids(edges, &affected_node_ids);
    let emitted_output_artifacts = emitted_output_artifacts(nodes, edges, &affected_node_ids);
    let rebuild_node_ids = rebuild_node_ids(nodes, &affected_node_ids);
    let changed_routes = routes_for_node_ids(nodes, &changed_node_ids);
    let affected_routes = routes_for_node_ids(nodes, &affected_node_ids);
    let rebuild_routes = routes_for_node_ids(nodes, &rebuild_node_ids);

    json!({
        "changedNodeIds": changed_node_ids,
        "affectedNodeIds": affected_node_ids,
        "emittedOutputNodeIds": emitted_output_node_ids,
        "emittedOutputArtifacts": emitted_output_artifacts,
        "rebuildNodeIds": rebuild_node_ids,
        "changedRoutes": changed_routes,
        "affectedRoutes": affected_routes,
        "rebuildRoutes": rebuild_routes
    })
}

fn empty_invalidation() -> Value {
    json!({
        "changedNodeIds": [],
        "affectedNodeIds": [],
        "emittedOutputNodeIds": [],
        "emittedOutputArtifacts": [],
        "rebuildNodeIds": [],
        "changedRoutes": [],
        "affectedRoutes": [],
        "rebuildRoutes": []
    })
}

fn node_ids_by_path(nodes: &[Value]) -> BTreeMap<String, BTreeSet<String>> {
    let mut ids_by_path = BTreeMap::<String, BTreeSet<String>>::new();
    for node in nodes {
        let Some(id) = node["id"].as_str() else {
            continue;
        };
        for field in ["path", "source_path"] {
            let Some(path) = node[field].as_str() else {
                continue;
            };
            ids_by_path
                .entry(path.to_string())
                .or_default()
                .insert(id.to_string());
        }
    }
    ids_by_path
}

fn changed_node_ids(
    project_root: &Path,
    changed_paths: &[PathBuf],
    node_ids_by_path: &BTreeMap<String, BTreeSet<String>>,
) -> Vec<String> {
    let mut changed_node_ids = BTreeSet::new();
    for changed_path in changed_paths {
        let changed_path = normalize_changed_path(project_root, changed_path);
        let Some(node_ids) = node_ids_by_path.get(&changed_path) else {
            continue;
        };
        changed_node_ids.extend(node_ids.iter().cloned());
    }
    changed_node_ids.into_iter().collect()
}

fn normalize_changed_path(project_root: &Path, changed_path: &Path) -> String {
    let project_root = clean_path(
        project_root
            .canonicalize()
            .unwrap_or_else(|_| project_root.to_path_buf()),
    );
    let absolute_changed_path = if changed_path.is_absolute() {
        changed_path.to_path_buf()
    } else {
        project_root.join(changed_path)
    };

    if let Ok(canonical_changed_path) = absolute_changed_path.canonicalize() {
        if let Ok(relative) = canonical_changed_path.strip_prefix(&project_root) {
            return normalize_path(relative);
        }
    }

    if let Ok(relative) = clean_path(absolute_changed_path).strip_prefix(&project_root) {
        return normalize_path(relative);
    }
    normalize_path(&clean_path(changed_path.to_path_buf()))
}

fn clean_path(path: PathBuf) -> PathBuf {
    let mut clean = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                clean.pop();
            }
            _ => clean.push(component.as_os_str()),
        }
    }
    clean
}

fn affected_node_ids(edges: &[Value], changed_node_ids: &[String]) -> Vec<String> {
    let mut incoming = BTreeMap::<String, BTreeSet<String>>::new();
    for edge in edges {
        let (Some(from), Some(to)) = (edge["from"].as_str(), edge["to"].as_str()) else {
            continue;
        };
        incoming
            .entry(to.to_string())
            .or_default()
            .insert(from.to_string());
    }

    let mut affected = Vec::new();
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::from(changed_node_ids.to_vec());
    while let Some(current) = queue.pop_front() {
        if !seen.insert(current.clone()) {
            continue;
        }
        affected.push(current.clone());
        if let Some(parents) = incoming.get(&current) {
            queue.extend(
                parents
                    .iter()
                    .filter(|parent| !seen.contains(*parent))
                    .cloned(),
            );
        }
    }
    affected
}

fn emitted_output_node_ids(edges: &[Value], affected_node_ids: &[String]) -> Vec<String> {
    let affected_node_ids = affected_node_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let mut output_node_ids = BTreeSet::new();
    for edge in edges {
        let Some(kind) = edge["kind"].as_str() else {
            continue;
        };
        if !is_emitted_output_edge_kind(kind) {
            continue;
        }
        let (Some(from), Some(to)) = (edge["from"].as_str(), edge["to"].as_str()) else {
            continue;
        };
        if affected_node_ids.contains(from) {
            output_node_ids.insert(to.to_string());
        }
    }
    output_node_ids.into_iter().collect()
}

fn emitted_output_artifacts(
    nodes: &[Value],
    edges: &[Value],
    affected_node_ids: &[String],
) -> Vec<Value> {
    let node_by_id = nodes
        .iter()
        .filter_map(|node| {
            let id = node["id"].as_str()?;
            Some((id, node))
        })
        .collect::<BTreeMap<_, _>>();
    let affected_node_ids = affected_node_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let mut artifacts = Vec::new();
    let mut seen = BTreeSet::new();

    for edge in edges {
        let Some(kind) = edge["kind"].as_str() else {
            continue;
        };
        if !is_emitted_output_edge_kind(kind) {
            continue;
        }
        let (Some(from), Some(to)) = (edge["from"].as_str(), edge["to"].as_str()) else {
            continue;
        };
        if !affected_node_ids.contains(from) {
            continue;
        }
        if !seen.insert((from.to_string(), to.to_string(), kind.to_string())) {
            continue;
        }
        let Some(node) = node_by_id.get(to) else {
            continue;
        };
        let output = node
            .get("output")
            .cloned()
            .unwrap_or_else(|| node["path"].clone());

        artifacts.push(json!({
            "id": to,
            "kind": node["kind"].clone(),
            "path": node["path"].clone(),
            "output": output,
            "sourceNodeId": from,
            "edgeKind": kind,
            "route": node["route"].clone(),
            "source_path": node["source_path"].clone(),
            "node_modules_required": node["node_modules_required"].clone(),
            "lifecycle_scripts_executed": node["lifecycle_scripts_executed"].clone(),
            "source_owned_contract": node["source_owned_contract"].clone(),
            "external_runtime_required": node["external_runtime_required"].clone(),
            "external_runtime_executed": node["external_runtime_executed"].clone()
        }));
    }

    artifacts.sort_by_key(|artifact| {
        format!(
            "{}\0{}\0{}",
            artifact["sourceNodeId"].as_str().unwrap_or_default(),
            artifact["id"].as_str().unwrap_or_default(),
            artifact["edgeKind"].as_str().unwrap_or_default()
        )
    });
    artifacts
}

fn rebuild_node_ids(nodes: &[Value], affected_node_ids: &[String]) -> Vec<String> {
    let rebuildable_by_id = nodes
        .iter()
        .filter_map(|node| {
            let id = node["id"].as_str()?;
            let kind = node["kind"].as_str()?;
            Some((id.to_string(), is_rebuildable_kind(kind)))
        })
        .collect::<BTreeMap<_, _>>();

    affected_node_ids
        .iter()
        .filter(|node_id| rebuildable_by_id.get(*node_id).copied().unwrap_or(false))
        .cloned()
        .collect()
}

fn is_emitted_output_edge_kind(kind: &str) -> bool {
    matches!(
        kind,
        "emits"
            | "emits-css-source-map"
            | "emits-placeholder"
            | "emits-server-data"
            | "links-server-data"
    )
}

fn routes_for_node_ids(nodes: &[Value], node_ids: &[String]) -> Vec<String> {
    let node_ids = node_ids.iter().map(String::as_str).collect::<BTreeSet<_>>();
    nodes
        .iter()
        .filter_map(|node| {
            let id = node["id"].as_str()?;
            if node_ids.contains(id) {
                node["route"].as_str().map(str::to_string)
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn is_rebuildable_kind(kind: &str) -> bool {
    matches!(
        kind,
        "dx-style-css"
            | "route-shell-chunk"
            | "server-data-route"
            | "source-module"
            | "source-module-chunk"
            | "tsx-route"
    )
}
