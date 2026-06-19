use std::collections::BTreeMap;

use serde_json::{Value, json};

use super::server_contract::{DxReactRouteHandlerRequest, DxReactRouteHandlerResponse};

pub(super) fn database_orm_readiness_route_handler_response(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    if request.method != "GET"
        || path != "/api/database-orm/readiness"
        || !source.contains("createDatabaseOrmReadinessResponse")
        || !function_body.contains("createDatabaseOrmReadinessResponse()")
        || !source.contains("server/database-orm/readiness.ts")
    {
        return None;
    }

    let body = database_orm_readiness_body(&request.runtime_env);
    let status = readiness_http_status(&body);

    Some(DxReactRouteHandlerResponse {
        status,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            ("cache-control".to_string(), "no-store".to_string()),
            (
                "x-dx-database-orm-readiness".to_string(),
                "source-owned-safe-interpreter".to_string(),
            ),
        ]),
        redirect_url: None,
        body,
        execution_model: "source-owned-database-orm-readiness-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

fn database_orm_readiness_body(runtime_env: &BTreeMap<String, String>) -> Value {
    let missing_config = database_orm_missing_config(runtime_env);
    let configured = missing_config.is_empty();

    json!({
        "schema": "dx.www.template.database_orm_readiness",
        "packageId": "db/drizzle-sqlite",
        "officialName": "Database ORM",
        "route": "/api/database-orm/readiness",
        "status": if configured {
            "configured-source-owned-adapter-boundary"
        } else {
            "runtime-gated"
        },
        "httpStatus": if configured { 200 } else { 501 },
        "runtimeProof": false,
        "networkCalls": false,
        "hostedCredentials": false,
        "requiredConfig": [
            "DX_DATABASE_URL or DX_SQLITE_DATABASE_PATH",
            "DX_DATABASE_MIGRATIONS_REVIEWED",
            "DX_DATABASE_AUTHORIZATION_REVIEWED",
        ],
        "missingConfig": missing_config,
        "sourceOwnedSurfaces": [
            "db/drizzle/schema.ts",
            "db/drizzle/dashboard-workflow.ts",
            "server/database-orm/readiness.ts",
            "app/api/database-orm/readiness/route.ts",
        ],
        "schemaTables": ["users", "posts"],
        "appOwnedBoundary": [
            "database file or connection URL",
            "SQLite driver package installation",
            "migration review and rollout",
            "tenant authorization policy",
            "backup, retention, and audit policy",
        ],
        "message": if configured {
            "Database ORM runtime inputs are locally acknowledged; this still does not open a database, run migrations, or prove deployed data access."
        } else {
            "This route validates local database runtime readiness only; configure the database location and app-owned reviews before enabling live reads, writes, migrations, or tenant access."
        },
    })
}

fn database_orm_missing_config(runtime_env: &BTreeMap<String, String>) -> Vec<&'static str> {
    let has_database_location = runtime_env_value_present(runtime_env, "DX_DATABASE_URL")
        || runtime_env_value_present(runtime_env, "DX_SQLITE_DATABASE_PATH");
    let migrations_reviewed = runtime_env_bool(runtime_env, "DX_DATABASE_MIGRATIONS_REVIEWED");
    let authorization_reviewed =
        runtime_env_bool(runtime_env, "DX_DATABASE_AUTHORIZATION_REVIEWED");

    let mut missing = Vec::new();
    if !has_database_location {
        missing.push("DX_DATABASE_URL or DX_SQLITE_DATABASE_PATH");
    }
    if !migrations_reviewed {
        missing.push("DX_DATABASE_MIGRATIONS_REVIEWED");
    }
    if !authorization_reviewed {
        missing.push("DX_DATABASE_AUTHORIZATION_REVIEWED");
    }
    missing
}

fn runtime_env_value_present(runtime_env: &BTreeMap<String, String>, name: &str) -> bool {
    runtime_env
        .get(name)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
}

fn runtime_env_bool(runtime_env: &BTreeMap<String, String>, name: &str) -> bool {
    runtime_env
        .get(name)
        .map(|value| value.trim().eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn readiness_http_status(body: &Value) -> u16 {
    body.get("httpStatus")
        .and_then(Value::as_u64)
        .and_then(|status| u16::try_from(status).ok())
        .unwrap_or(501)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::Value;

    use super::database_orm_readiness_route_handler_response;
    use crate::delivery::server_contract::DxReactRouteHandlerRequest;

    #[test]
    fn database_orm_readiness_accepts_absolute_url_for_endpoint_match() {
        let response = database_orm_readiness_route_handler_response(
            r#"import { createDatabaseOrmReadinessResponse } from "../../../../server/database-orm/readiness.ts";

export function GET() {
  return createDatabaseOrmReadinessResponse();
}
"#,
            "createDatabaseOrmReadinessResponse()",
            &DxReactRouteHandlerRequest {
                method: "GET".to_string(),
                path: "https://example.test/api/database-orm/readiness?source=dx#ready"
                    .to_string(),
                headers: BTreeMap::new(),
                body: Value::Null,
                route_params: BTreeMap::new(),
                search_params: BTreeMap::from([("source".to_string(), "dx".to_string())]),
                runtime_env: BTreeMap::from([
                    (
                        "DX_SQLITE_DATABASE_PATH".to_string(),
                        "./data/app.db".to_string(),
                    ),
                    (
                        "DX_DATABASE_MIGRATIONS_REVIEWED".to_string(),
                        "true".to_string(),
                    ),
                    (
                        "DX_DATABASE_AUTHORIZATION_REVIEWED".to_string(),
                        "true".to_string(),
                    ),
                ]),
            },
        )
        .expect("database ORM readiness route should match absolute request URL");

        assert_eq!(response.status, 200);
        assert_eq!(
            response.execution_model,
            "source-owned-database-orm-readiness-interpreter"
        );
        assert_eq!(response.body["route"], "/api/database-orm/readiness");
        assert_eq!(response.body["runtimeProof"], false);
        assert_eq!(response.body["networkCalls"], false);
    }
}
