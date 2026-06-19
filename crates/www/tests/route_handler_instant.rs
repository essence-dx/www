use std::collections::BTreeMap;

use dx_compiler::delivery::{
    DxReactRouteHandlerRequest, DxReactServerSource, DxReactServerSourceKind,
    execute_react_route_handler,
};

#[test]
fn instant_route_factory_get_reports_configured_readiness_without_node_runtime() {
    let response = execute_react_route_handler(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::RouteHandler,
            source_path: "app/api/instant/route.ts".to_string(),
            source: r#"import { createDxInstantRouteHandlers } from "@/lib/instant/route";

export const { GET, POST } = createDxInstantRouteHandlers();
"#
            .to_string(),
        },
        DxReactRouteHandlerRequest {
            method: "GET".to_string(),
            path: "/api/instant".to_string(),
            headers: BTreeMap::new(),
            body: serde_json::Value::Null,
            route_params: BTreeMap::new(),
            search_params: BTreeMap::new(),
            runtime_env: BTreeMap::from([(
                "NEXT_PUBLIC_INSTANT_APP_ID".to_string(),
                "app_dev_123".to_string(),
            )]),
        },
    )
    .expect("instant route factory GET readiness");

    assert_eq!(response.status, 200);
    assert_eq!(
        response.execution_model,
        "source-owned-instantdb-route-handler-interpreter"
    );
    assert_eq!(
        response
            .headers
            .get("x-dx-instant-route-handler")
            .map(String::as_str),
        Some("source-owned-safe-interpreter")
    );
    assert_eq!(
        response.body["schema"],
        "dx.launch_template.instant_route_readiness"
    );
    assert_eq!(
        response.body["status"],
        "configured-source-owned-adapter-boundary"
    );
    assert_eq!(response.body["httpStatus"], 200);
    assert_eq!(response.body["appIdConfigured"], true);
    assert_eq!(response.body["runtimeProof"], false);
    assert_eq!(response.body["networkCalls"], false);
    assert_eq!(response.body["nodeModulesRequired"], false);
}
