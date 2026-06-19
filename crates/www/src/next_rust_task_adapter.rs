//! DX-owned task graph adapter inspired by vendored `turbo-tasks`.
//!
//! This module converts DX source graph facts into stable task-input receipts
//! without linking or executing vendored Turbopack, Node/NAPI, or `node_modules`.

use std::collections::{BTreeMap, BTreeSet};

/// Schema for the DX-owned turbo-tasks-inspired task input adapter.
pub const DX_NEXT_RUST_TASK_INPUT_ADAPTER_SCHEMA: &str = "dx.nextRust.turboTasks.taskInputAdapter";

/// Stable format version for [`DxNextRustTaskInputAdapter`].
pub const DX_NEXT_RUST_TASK_INPUT_ADAPTER_FORMAT: u16 = 1;

/// Schema for the DX-owned turbo-tasks-inspired task graph adapter.
pub const DX_NEXT_RUST_TASK_GRAPH_ADAPTER_SCHEMA: &str = "dx.nextRust.turboTasks.taskGraphAdapter";

/// Stable format version for [`DxNextRustTaskGraphAdapter`].
pub const DX_NEXT_RUST_TASK_GRAPH_ADAPTER_FORMAT: u16 = 1;

/// DX-owned task input receipt derived from turbo-tasks' `TaskInput` concept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxNextRustTaskInputAdapter {
    /// Stable DX receipt schema.
    pub schema: &'static str,
    /// Stable numeric format version.
    pub format: u16,
    /// Vendored upstream crate that inspired the adapter.
    pub upstream_crate: &'static str,
    /// Upstream concept represented without executing upstream runtime code.
    pub upstream_concept: &'static str,
    /// Source graph node id.
    pub node_id: String,
    /// Source graph node kind.
    pub node_kind: String,
    /// Project-relative source path for the node.
    pub path: String,
    /// Source-owned content hash for the node.
    pub content_hash: String,
    /// Sorted dependency node ids included in this task input.
    pub dependency_node_ids: Vec<String>,
    /// Stable human-readable key used by receipts and cache manifests.
    pub input_key: String,
    /// Stable BLAKE3 fingerprint of the canonical task input.
    pub input_fingerprint: String,
    /// Whether this remains an adapter boundary instead of native turbo-tasks execution.
    pub adapter_boundary: bool,
    /// Whether this exposes Turbopack as the public DX architecture.
    pub public_architecture: bool,
    /// Whether the vendored Turbopack/turbo-tasks runtime executed.
    pub turbopack_runtime_executed: bool,
    /// Whether project-local `node_modules` are required.
    pub node_modules_required: bool,
    /// Boundary explanation for receipts and status surfaces.
    pub boundary: &'static str,
}

/// Source graph node facts accepted by the DX-owned task graph adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxNextRustTaskGraphNode {
    /// Source graph node id.
    pub node_id: String,
    /// Source graph node kind.
    pub node_kind: String,
    /// Project-relative source path for the node.
    pub path: String,
    /// Source-owned content hash for the node.
    pub content_hash: String,
    /// Dependency node ids that should affect this node's task input.
    pub dependency_node_ids: Vec<String>,
}

/// DX-owned task graph receipt derived from turbo-tasks invalidation ideas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxNextRustTaskGraphAdapter {
    /// Stable DX receipt schema.
    pub schema: &'static str,
    /// Stable numeric format version.
    pub format: u16,
    /// Vendored upstream crate that inspired the adapter.
    pub upstream_crate: &'static str,
    /// Upstream concept represented without executing upstream runtime code.
    pub upstream_concept: &'static str,
    /// Deterministically ordered task inputs for the source graph.
    pub task_inputs: Vec<DxNextRustTaskInputAdapter>,
    /// Stable BLAKE3 fingerprint for the full task input graph.
    pub graph_fingerprint: String,
    /// Current node ids that are missing from or differ from the previous fingerprint set.
    pub changed_node_ids: Vec<String>,
    /// Current node ids whose input fingerprint matches the previous fingerprint set.
    pub unchanged_node_ids: Vec<String>,
    /// Unchanged current node ids that depend on changed or already stale nodes.
    pub dependency_stale_node_ids: Vec<String>,
    /// Current node ids that need rebuild consideration: changed plus dependency-stale.
    pub affected_node_ids: Vec<String>,
    /// Whether this remains an adapter boundary instead of native turbo-tasks execution.
    pub adapter_boundary: bool,
    /// Whether this exposes Turbopack as the public DX architecture.
    pub public_architecture: bool,
    /// Whether the vendored Turbopack/turbo-tasks runtime executed.
    pub turbopack_runtime_executed: bool,
    /// Whether project-local `node_modules` are required.
    pub node_modules_required: bool,
    /// Boundary explanation for receipts and status surfaces.
    pub boundary: &'static str,
}

/// Build a DX-owned turbo-tasks-inspired task input receipt for a source graph node.
///
/// This function is intentionally small and deterministic: it turns DX source graph
/// facts into a stable task key and BLAKE3 fingerprint without linking or executing
/// vendored `turbo-tasks`, Turbopack, Node/NAPI, or `node_modules`.
#[must_use]
pub fn dx_next_rust_turbo_tasks_task_input_adapter(
    node_id: &str,
    node_kind: &str,
    path: &str,
    content_hash: &str,
    dependency_node_ids: &[&str],
) -> DxNextRustTaskInputAdapter {
    let mut dependency_node_ids = dependency_node_ids
        .iter()
        .map(|node_id| node_id.trim())
        .filter(|node_id| !node_id.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    dependency_node_ids.sort();
    dependency_node_ids.dedup();

    let input_key =
        dx_next_rust_task_input_key(node_id, node_kind, path, content_hash, &dependency_node_ids);
    let input_fingerprint = dx_next_rust_task_input_fingerprint(
        node_id,
        node_kind,
        path,
        content_hash,
        &dependency_node_ids,
    );

    DxNextRustTaskInputAdapter {
        schema: DX_NEXT_RUST_TASK_INPUT_ADAPTER_SCHEMA,
        format: DX_NEXT_RUST_TASK_INPUT_ADAPTER_FORMAT,
        upstream_crate: "turbopack/crates/turbo-tasks",
        upstream_concept: "TaskInput",
        node_id: node_id.to_string(),
        node_kind: node_kind.to_string(),
        path: path.to_string(),
        content_hash: content_hash.to_string(),
        dependency_node_ids,
        input_key,
        input_fingerprint,
        adapter_boundary: true,
        public_architecture: false,
        turbopack_runtime_executed: false,
        node_modules_required: false,
        boundary: "DX-owned source graph task input adapter; vendored turbo-tasks is reference material only",
    }
}

/// Build a DX-owned turbo-tasks-inspired task graph receipt for source graph invalidation.
///
/// The returned graph is ordered and fingerprinted independently from insertion order.
/// `previous_fingerprints` is a compact prior receipt view of `(node_id, input_fingerprint)`
/// pairs, allowing callers to identify changed and unchanged source nodes without
/// executing vendored `turbo-tasks`, Turbopack, Node/NAPI, or `node_modules`.
#[must_use]
pub fn dx_next_rust_turbo_tasks_graph_adapter(
    nodes: Vec<DxNextRustTaskGraphNode>,
    previous_fingerprints: &[(&str, &str)],
) -> DxNextRustTaskGraphAdapter {
    let previous_fingerprints = previous_fingerprints
        .iter()
        .map(|(node_id, fingerprint)| ((*node_id).to_string(), (*fingerprint).to_string()))
        .collect::<BTreeMap<_, _>>();

    let mut task_inputs = nodes
        .into_iter()
        .map(|node| {
            let dependency_node_ids = node
                .dependency_node_ids
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>();
            dx_next_rust_turbo_tasks_task_input_adapter(
                &node.node_id,
                &node.node_kind,
                &node.path,
                &node.content_hash,
                &dependency_node_ids,
            )
        })
        .collect::<Vec<_>>();
    task_inputs.sort_by(|left, right| {
        left.node_id
            .cmp(&right.node_id)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.node_kind.cmp(&right.node_kind))
    });

    let mut changed_node_ids = Vec::new();
    let mut unchanged_node_ids = Vec::new();
    for task_input in &task_inputs {
        match previous_fingerprints.get(&task_input.node_id) {
            Some(previous_fingerprint) if previous_fingerprint == &task_input.input_fingerprint => {
                unchanged_node_ids.push(task_input.node_id.clone());
            }
            _ => changed_node_ids.push(task_input.node_id.clone()),
        }
    }

    let dependency_stale_node_ids =
        collect_dependency_stale_node_ids(&task_inputs, &changed_node_ids);
    let affected_node_ids =
        collect_affected_node_ids(&changed_node_ids, &dependency_stale_node_ids);
    let graph_fingerprint = dx_next_rust_task_graph_fingerprint(&task_inputs);

    DxNextRustTaskGraphAdapter {
        schema: DX_NEXT_RUST_TASK_GRAPH_ADAPTER_SCHEMA,
        format: DX_NEXT_RUST_TASK_GRAPH_ADAPTER_FORMAT,
        upstream_crate: "turbopack/crates/turbo-tasks",
        upstream_concept: "TaskGraph",
        task_inputs,
        graph_fingerprint,
        changed_node_ids,
        unchanged_node_ids,
        dependency_stale_node_ids,
        affected_node_ids,
        adapter_boundary: true,
        public_architecture: false,
        turbopack_runtime_executed: false,
        node_modules_required: false,
        boundary: "DX-owned source graph task graph adapter; vendored turbo-tasks is reference material only",
    }
}

fn collect_dependency_stale_node_ids(
    task_inputs: &[DxNextRustTaskInputAdapter],
    changed_node_ids: &[String],
) -> Vec<String> {
    let mut stale_node_ids = changed_node_ids.iter().cloned().collect::<BTreeSet<_>>();
    let mut dependency_stale_node_ids = BTreeSet::new();

    loop {
        let mut found_new_stale_node = false;
        for task_input in task_inputs {
            if stale_node_ids.contains(&task_input.node_id) {
                continue;
            }

            if task_input
                .dependency_node_ids
                .iter()
                .any(|dependency_node_id| stale_node_ids.contains(dependency_node_id))
            {
                stale_node_ids.insert(task_input.node_id.clone());
                dependency_stale_node_ids.insert(task_input.node_id.clone());
                found_new_stale_node = true;
            }
        }

        if !found_new_stale_node {
            break;
        }
    }

    dependency_stale_node_ids.into_iter().collect()
}

fn collect_affected_node_ids(
    changed_node_ids: &[String],
    dependency_stale_node_ids: &[String],
) -> Vec<String> {
    changed_node_ids
        .iter()
        .chain(dependency_stale_node_ids)
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn dx_next_rust_task_input_key(
    node_id: &str,
    node_kind: &str,
    path: &str,
    content_hash: &str,
    dependency_node_ids: &[String],
) -> String {
    format!(
        "{DX_NEXT_RUST_TASK_INPUT_ADAPTER_SCHEMA}|{node_id}|{node_kind}|{path}|{content_hash}|deps={}",
        dependency_node_ids.join(",")
    )
}

fn dx_next_rust_task_input_fingerprint(
    node_id: &str,
    node_kind: &str,
    path: &str,
    content_hash: &str,
    dependency_node_ids: &[String],
) -> String {
    let mut canonical = String::new();
    push_canonical_task_input_field(
        &mut canonical,
        "schema",
        DX_NEXT_RUST_TASK_INPUT_ADAPTER_SCHEMA,
    );
    push_canonical_task_input_field(
        &mut canonical,
        "upstream_crate",
        "turbopack/crates/turbo-tasks",
    );
    push_canonical_task_input_field(&mut canonical, "upstream_concept", "TaskInput");
    push_canonical_task_input_field(&mut canonical, "node_id", node_id);
    push_canonical_task_input_field(&mut canonical, "node_kind", node_kind);
    push_canonical_task_input_field(&mut canonical, "path", path);
    push_canonical_task_input_field(&mut canonical, "content_hash", content_hash);
    push_canonical_task_input_field(
        &mut canonical,
        "dependency_node_ids",
        &dependency_node_ids.join("\n"),
    );

    blake3::hash(canonical.as_bytes()).to_hex().to_string()
}

fn dx_next_rust_task_graph_fingerprint(task_inputs: &[DxNextRustTaskInputAdapter]) -> String {
    let mut canonical = String::new();
    push_canonical_task_input_field(
        &mut canonical,
        "schema",
        DX_NEXT_RUST_TASK_GRAPH_ADAPTER_SCHEMA,
    );
    push_canonical_task_input_field(
        &mut canonical,
        "upstream_crate",
        "turbopack/crates/turbo-tasks",
    );
    push_canonical_task_input_field(&mut canonical, "upstream_concept", "TaskGraph");
    push_canonical_task_input_field(
        &mut canonical,
        "task_inputs",
        &task_inputs
            .iter()
            .map(|task_input| format!("{}={}", task_input.node_id, task_input.input_fingerprint))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    blake3::hash(canonical.as_bytes()).to_hex().to_string()
}

fn push_canonical_task_input_field(output: &mut String, name: &str, value: &str) {
    output.push_str(name);
    output.push('\0');
    output.push_str(&value.len().to_string());
    output.push('\0');
    output.push_str(value);
    output.push('\n');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_input_adapter_sorts_dependencies_before_fingerprinting() {
        let first = dx_next_rust_turbo_tasks_task_input_adapter(
            "route:app/page.tsx",
            "tsx-route",
            "app/page.tsx",
            "hash-route",
            &["component:LaunchPanel", "style:app.css"],
        );
        let second = dx_next_rust_turbo_tasks_task_input_adapter(
            "route:app/page.tsx",
            "tsx-route",
            "app/page.tsx",
            "hash-route",
            &["style:app.css", "component:LaunchPanel"],
        );

        assert_eq!(
            first.dependency_node_ids,
            vec!["component:LaunchPanel", "style:app.css"]
        );
        assert_eq!(first.input_key, second.input_key);
        assert_eq!(first.input_fingerprint, second.input_fingerprint);
        assert_eq!(first.schema, DX_NEXT_RUST_TASK_INPUT_ADAPTER_SCHEMA);
        assert_eq!(first.upstream_concept, "TaskInput");
        assert!(first.adapter_boundary);
        assert!(!first.public_architecture);
        assert!(!first.turbopack_runtime_executed);
        assert!(!first.node_modules_required);
    }

    #[test]
    fn task_input_adapter_fingerprint_changes_when_content_hash_changes() {
        let before = dx_next_rust_turbo_tasks_task_input_adapter(
            "style:app.css",
            "dx-style-css",
            "styles/app.css",
            "hash-before",
            &[],
        );
        let after = dx_next_rust_turbo_tasks_task_input_adapter(
            "style:app.css",
            "dx-style-css",
            "styles/app.css",
            "hash-after",
            &[],
        );

        assert_ne!(before.input_fingerprint, after.input_fingerprint);
        assert_eq!(before.input_fingerprint.len(), 64);
        assert!(
            before
                .input_fingerprint
                .chars()
                .all(|character| character.is_ascii_hexdigit())
        );
    }

    #[test]
    fn task_graph_adapter_orders_nodes_before_graph_fingerprint() {
        let style = task_graph_node(
            "style:app.css",
            "dx-style-css",
            "styles/app.css",
            "hash-style",
            &[],
        );
        let route = task_graph_node(
            "route:app/page.tsx",
            "tsx-route",
            "app/page.tsx",
            "hash-route",
            &["style:app.css"],
        );

        let first = dx_next_rust_turbo_tasks_graph_adapter(vec![style.clone(), route.clone()], &[]);
        let second = dx_next_rust_turbo_tasks_graph_adapter(vec![route, style], &[]);

        assert_eq!(
            first
                .task_inputs
                .iter()
                .map(|task_input| task_input.node_id.as_str())
                .collect::<Vec<_>>(),
            vec!["route:app/page.tsx", "style:app.css"]
        );
        assert_eq!(first.graph_fingerprint, second.graph_fingerprint);
        assert_eq!(first.schema, DX_NEXT_RUST_TASK_GRAPH_ADAPTER_SCHEMA);
        assert_eq!(first.upstream_concept, "TaskGraph");
        assert!(first.adapter_boundary);
        assert!(!first.public_architecture);
        assert!(!first.turbopack_runtime_executed);
        assert!(!first.node_modules_required);
    }

    #[test]
    fn task_graph_adapter_marks_changed_and_unchanged_nodes() {
        let route = task_graph_node(
            "route:app/page.tsx",
            "tsx-route",
            "app/page.tsx",
            "hash-route",
            &[],
        );
        let style = task_graph_node(
            "style:app.css",
            "dx-style-css",
            "styles/app.css",
            "hash-style",
            &[],
        );

        let previous =
            dx_next_rust_turbo_tasks_graph_adapter(vec![route.clone(), style.clone()], &[]);
        let unchanged_route = previous
            .task_inputs
            .iter()
            .find(|task_input| task_input.node_id == "route:app/page.tsx")
            .expect("route task input should exist");

        let changed_style = task_graph_node(
            "style:app.css",
            "dx-style-css",
            "styles/app.css",
            "hash-style-updated",
            &[],
        );
        let current = dx_next_rust_turbo_tasks_graph_adapter(
            vec![route, changed_style],
            &[(
                "route:app/page.tsx",
                unchanged_route.input_fingerprint.as_str(),
            )],
        );

        assert_eq!(current.unchanged_node_ids, vec!["route:app/page.tsx"]);
        assert_eq!(current.changed_node_ids, vec!["style:app.css"]);
        assert_eq!(current.dependency_stale_node_ids, Vec::<String>::new());
        assert_eq!(current.affected_node_ids, vec!["style:app.css"]);
        assert_ne!(current.graph_fingerprint, previous.graph_fingerprint);
        assert_eq!(current.graph_fingerprint.len(), 64);
    }

    #[test]
    fn task_graph_adapter_marks_transitive_dependents_as_stale() {
        let style = task_graph_node(
            "style:app.css",
            "dx-style-css",
            "styles/app.css",
            "hash-style",
            &[],
        );
        let component = task_graph_node(
            "component:LaunchPanel",
            "tsx-component",
            "components/launch-panel.tsx",
            "hash-component",
            &["style:app.css"],
        );
        let route = task_graph_node(
            "route:app/page.tsx",
            "tsx-route",
            "app/page.tsx",
            "hash-route",
            &["component:LaunchPanel"],
        );

        let previous = dx_next_rust_turbo_tasks_graph_adapter(
            vec![style.clone(), component.clone(), route.clone()],
            &[],
        );
        let style_fingerprint = task_input_fingerprint(&previous, "style:app.css");
        let component_fingerprint = task_input_fingerprint(&previous, "component:LaunchPanel");
        let route_fingerprint = task_input_fingerprint(&previous, "route:app/page.tsx");

        let changed_style = task_graph_node(
            "style:app.css",
            "dx-style-css",
            "styles/app.css",
            "hash-style-updated",
            &[],
        );
        let current = dx_next_rust_turbo_tasks_graph_adapter(
            vec![changed_style, component, route],
            &[
                ("style:app.css", style_fingerprint.as_str()),
                ("component:LaunchPanel", component_fingerprint.as_str()),
                ("route:app/page.tsx", route_fingerprint.as_str()),
            ],
        );

        assert_eq!(current.changed_node_ids, vec!["style:app.css"]);
        assert_eq!(
            current.unchanged_node_ids,
            vec!["component:LaunchPanel", "route:app/page.tsx"]
        );
        assert_eq!(
            current.dependency_stale_node_ids,
            vec!["component:LaunchPanel", "route:app/page.tsx"]
        );
        assert_eq!(
            current.affected_node_ids,
            vec![
                "component:LaunchPanel",
                "route:app/page.tsx",
                "style:app.css"
            ]
        );
    }

    fn task_graph_node(
        node_id: &str,
        node_kind: &str,
        path: &str,
        content_hash: &str,
        dependency_node_ids: &[&str],
    ) -> DxNextRustTaskGraphNode {
        DxNextRustTaskGraphNode {
            node_id: node_id.to_string(),
            node_kind: node_kind.to_string(),
            path: path.to_string(),
            content_hash: content_hash.to_string(),
            dependency_node_ids: dependency_node_ids
                .iter()
                .map(|node_id| (*node_id).to_string())
                .collect(),
        }
    }

    fn task_input_fingerprint(adapter: &DxNextRustTaskGraphAdapter, node_id: &str) -> String {
        adapter
            .task_inputs
            .iter()
            .find(|task_input| task_input.node_id == node_id)
            .unwrap_or_else(|| panic!("{node_id} task input should exist"))
            .input_fingerprint
            .clone()
    }
}
