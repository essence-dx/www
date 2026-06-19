use std::collections::BTreeMap;

use serde_json::{Map, Value, json};

pub(super) const DX_APP_ROUTER_SERVER_DATA_SCHEMA: &str = "dx.appRouter.serverData";
pub(super) const DX_APP_ROUTER_SERVER_DATA_FORMAT: u32 = 1;
const DX_APP_ROUTER_SERVER_DATA_SCHEMA_REVISION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DxServerDataSurfaceStatus {
    SourceOwnedSafeLoaderData,
    NoLoaderBindings,
    AdapterBoundary,
}

impl DxServerDataSurfaceStatus {
    pub(super) fn for_entry_count(entry_count: usize) -> Self {
        if entry_count == 0 {
            Self::NoLoaderBindings
        } else {
            Self::SourceOwnedSafeLoaderData
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::SourceOwnedSafeLoaderData => "source-owned-safe-loader-data",
            Self::NoLoaderBindings => "no-loader-bindings",
            Self::AdapterBoundary => "adapter-boundary",
        }
    }

    fn execution_model(self) -> &'static str {
        match self {
            Self::SourceOwnedSafeLoaderData => "source-owned-safe-interpreter",
            Self::NoLoaderBindings => "not-required",
            Self::AdapterBoundary => "unsupported-safe-loader-shape",
        }
    }

    fn limits(self) -> Vec<&'static str> {
        let mut limits = vec![
            "Records DX-owned safe loader data only.",
            "Does not execute arbitrary JavaScript, React Server Components, Node APIs, package lifecycle scripts, or Next.js runtime loaders.",
        ];
        if self == Self::AdapterBoundary {
            limits.push("Unsupported loader shapes stay adapter-boundary instead of falling back to a fake runtime.");
        }
        limits
    }
}

pub(super) fn insert_server_data_surface_metadata(
    object: &mut Map<String, Value>,
    status: DxServerDataSurfaceStatus,
    entry_count: usize,
) {
    object.insert(
        "schema".to_string(),
        json!(DX_APP_ROUTER_SERVER_DATA_SCHEMA),
    );
    object.insert(
        "format".to_string(),
        json!(DX_APP_ROUTER_SERVER_DATA_FORMAT),
    );
    object.insert(
        "schema_revision".to_string(),
        json!(DX_APP_ROUTER_SERVER_DATA_SCHEMA_REVISION),
    );
    object.insert("status".to_string(), json!(status.as_str()));
    object.insert("entry_count".to_string(), json!(entry_count));
    object.insert(
        "execution_model".to_string(),
        json!(status.execution_model()),
    );
    object.insert("source_owned_contract".to_string(), json!(true));
    object.insert("external_runtime_required".to_string(), json!(false));
    object.insert("external_runtime_executed".to_string(), json!(false));
    object.insert("limits".to_string(), json!(status.limits()));
}

pub(super) fn insert_server_data_adapter_boundary(
    object: &mut Map<String, Value>,
    reason: &str,
    build_output_emitted: bool,
    runtime_request_values: bool,
) {
    object.insert(
        "adapter_boundary".to_string(),
        json!({
            "kind": "server-data-loader",
            "reason": reason,
            "build_output_emitted": build_output_emitted,
            "runtime_request_values": runtime_request_values,
            "source_owned_contract": true,
            "external_runtime_required": false,
            "external_runtime_executed": false,
        }),
    );
}

pub(super) fn server_data_request_contract(
    mode: &str,
    route_params: &BTreeMap<String, String>,
    search_params: &BTreeMap<String, String>,
    build_time_contract_inputs: bool,
    runtime_request_values: bool,
) -> Value {
    json!({
        "mode": mode,
        "route_params": route_params,
        "search_params": search_params,
        "build_time_contract_inputs": build_time_contract_inputs,
        "runtime_request_values": runtime_request_values,
        "source_owned_contract": true,
        "external_runtime_request_values": false,
    })
}

pub(super) fn insert_build_time_server_data_request_contracts(
    object: &mut Map<String, Value>,
    route_params: &BTreeMap<String, String>,
    search_params: &BTreeMap<String, String>,
) {
    let request = server_data_request_contract(
        "static-route-contract-inputs",
        route_params,
        search_params,
        true,
        false,
    );
    object.insert("request".to_string(), request.clone());
    object.insert("build_time_request_props".to_string(), request);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_marks_no_loader_routes_without_fake_runtime() {
        let mut object = Map::new();

        insert_server_data_surface_metadata(
            &mut object,
            DxServerDataSurfaceStatus::NoLoaderBindings,
            0,
        );

        assert_eq!(object["schema"], DX_APP_ROUTER_SERVER_DATA_SCHEMA);
        assert_eq!(object["format"], DX_APP_ROUTER_SERVER_DATA_FORMAT);
        assert_eq!(object["status"], "no-loader-bindings");
        assert_eq!(object["entry_count"], 0);
        assert_eq!(object["execution_model"], "not-required");
        assert_eq!(object["source_owned_contract"], true);
        assert_eq!(object["external_runtime_required"], false);
        assert_eq!(object["external_runtime_executed"], false);
    }

    #[test]
    fn adapter_boundary_marks_unsupported_loader_without_fake_runtime() {
        let mut object = Map::new();

        insert_server_data_adapter_boundary(
            &mut object,
            "server loader must return a supported object literal",
            false,
            true,
        );

        assert_eq!(object["adapter_boundary"]["kind"], "server-data-loader");
        assert_eq!(
            object["adapter_boundary"]["reason"],
            "server loader must return a supported object literal"
        );
        assert_eq!(object["adapter_boundary"]["build_output_emitted"], false);
        assert_eq!(object["adapter_boundary"]["runtime_request_values"], true);
        assert_eq!(object["adapter_boundary"]["source_owned_contract"], true);
        assert_eq!(
            object["adapter_boundary"]["external_runtime_required"],
            false
        );
        assert_eq!(
            object["adapter_boundary"]["external_runtime_executed"],
            false
        );
    }

    #[test]
    fn request_contract_marks_static_inputs_as_source_owned() {
        let route_params = BTreeMap::from([("team".to_string(), "sample-team".to_string())]);
        let search_params = BTreeMap::from([("tab".to_string(), "sample-tab".to_string())]);

        let request = server_data_request_contract(
            "static-route-contract-inputs",
            &route_params,
            &search_params,
            true,
            false,
        );

        assert_eq!(request["mode"], "static-route-contract-inputs");
        assert_eq!(request["route_params"]["team"], "sample-team");
        assert_eq!(request["search_params"]["tab"], "sample-tab");
        assert_eq!(request["build_time_contract_inputs"], true);
        assert_eq!(request["runtime_request_values"], false);
        assert_eq!(request["source_owned_contract"], true);
        assert_eq!(request["external_runtime_request_values"], false);
    }

    #[test]
    fn build_time_request_insertion_reuses_one_request_contract() {
        let route_params = BTreeMap::from([("team".to_string(), "sample-team".to_string())]);
        let search_params = BTreeMap::from([("tab".to_string(), "sample-tab".to_string())]);
        let mut object = Map::new();

        insert_build_time_server_data_request_contracts(&mut object, &route_params, &search_params);

        assert_eq!(object["request"], object["build_time_request_props"]);
        assert_eq!(object["request"]["mode"], "static-route-contract-inputs");
        assert_eq!(object["request"]["route_params"]["team"], "sample-team");
        assert_eq!(object["request"]["search_params"]["tab"], "sample-tab");
        assert_eq!(object["request"]["build_time_contract_inputs"], true);
        assert_eq!(object["request"]["runtime_request_values"], false);
        assert_eq!(object["request"]["source_owned_contract"], true);
        assert_eq!(object["request"]["external_runtime_request_values"], false);
    }
}
