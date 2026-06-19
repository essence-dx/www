use std::collections::BTreeMap;

use serde_json::{Value, json};

use super::server_contract::{DxReactRouteHandlerRequest, DxReactRouteHandlerResponse};

pub(super) fn database_api_readiness_route_handler_response(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    if request.method != "GET"
        || path != "/api/database-api/readiness"
        || !source.contains("createDatabaseApiReadinessResponse")
        || !function_body.contains("createDatabaseApiReadinessResponse()")
        || !source.contains("server/database-api/readiness.ts")
    {
        return None;
    }

    Some(DxReactRouteHandlerResponse {
        status: 200,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            ("cache-control".to_string(), "no-store".to_string()),
            (
                "x-dx-database-api-readiness".to_string(),
                "source-owned-safe-interpreter".to_string(),
            ),
        ]),
        redirect_url: None,
        body: database_api_readiness_body(),
        execution_model: "source-owned-database-api-readiness-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

pub(super) fn data_fetching_cache_readiness_route_handler_response(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    if request.method != "GET"
        || path != "/api/query-cache/readiness"
        || !source.contains("createDataFetchingCacheReadinessResponse")
        || !function_body.contains("createDataFetchingCacheReadinessResponse(request)")
        || !source.contains("server/query-cache/readiness")
    {
        return None;
    }

    Some(DxReactRouteHandlerResponse {
        status: 200,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            ("cache-control".to_string(), "no-store".to_string()),
            (
                "x-dx-query-cache-readiness".to_string(),
                "source-owned-safe-interpreter".to_string(),
            ),
        ]),
        redirect_url: None,
        body: query_cache_readiness_body(request),
        execution_model: "source-owned-data-fetching-cache-readiness-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

pub(super) fn data_fetching_cache_action_route_handler_response(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    if request.method != "POST"
        || path != "/api/query-cache/readiness"
        || !source.contains("createDataFetchingCacheActionResponse")
        || !function_body.contains("createDataFetchingCacheActionResponse(request)")
        || !source.contains("server/query-cache/readiness")
    {
        return None;
    }

    Some(DxReactRouteHandlerResponse {
        status: 200,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            ("cache-control".to_string(), "no-store".to_string()),
            (
                "x-dx-query-cache-action".to_string(),
                "source-owned-safe-interpreter".to_string(),
            ),
        ]),
        redirect_url: None,
        body: query_cache_action_body(request),
        execution_model: "source-owned-data-fetching-cache-action-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

pub(super) fn instant_route_handler_compat_response(
    source: &str,
    expression: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    if !matches!(request.method.as_str(), "GET" | "POST")
        || path != "/api/instant"
        || !source.contains("@/lib/instant/route")
        || (!source.contains("export const { POST }")
            && !source.contains("export const { GET, POST }"))
        || !expression.contains("createDxInstantRouteHandlers")
    {
        return None;
    }

    let (status, body) = if request.method == "GET" {
        instant_route_readiness_body(&request.runtime_env)
    } else {
        instant_route_handler_post_body(request, &request.runtime_env)
    };

    Some(DxReactRouteHandlerResponse {
        status,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            ("cache-control".to_string(), "no-store".to_string()),
            (
                "x-dx-instant-route-handler".to_string(),
                "source-owned-safe-interpreter".to_string(),
            ),
        ]),
        redirect_url: None,
        body,
        execution_model: "source-owned-instantdb-route-handler-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

pub(super) fn template_better_auth_readiness_route_handler_response(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    if request.method != "GET"
        || path != "/api/auth/readiness"
        || !source.contains("createTemplateBetterAuthReadiness")
        || !function_body.contains("createTemplateBetterAuthReadiness()")
        || !source.contains("server/auth/better-auth")
    {
        return None;
    }

    Some(DxReactRouteHandlerResponse {
        status: 200,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([(
            "x-dx-auth-readiness".to_string(),
            "source-owned-safe-interpreter".to_string(),
        )]),
        redirect_url: None,
        body: template_better_auth_readiness_body(&request.runtime_env),
        execution_model: "source-owned-template-better-auth-readiness-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

pub(super) fn template_better_auth_route_handler_response(
    source: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    if !matches!(request.method.as_str(), "GET" | "POST")
        || !path.starts_with("/api/auth/")
        || path == "/api/auth/readiness"
        || !source.contains("@/server/auth/better-auth")
        || !source.contains("export { GET, POST }")
    {
        return None;
    }

    Some(DxReactRouteHandlerResponse {
        status: 501,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([(
            "x-dx-auth-route-handler".to_string(),
            "source-owned-safe-interpreter".to_string(),
        )]),
        redirect_url: None,
        body: template_better_auth_route_handler_body(&request.method, &request.runtime_env),
        execution_model: "source-owned-template-better-auth-route-handler-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

pub(super) fn template_better_auth_session_route_handler_response(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    if request.method != "GET"
        || path != "/api/auth/session"
        || !source.contains("createTemplateBetterAuthSessionReceipt")
        || !function_body.contains("createTemplateBetterAuthSessionReceipt()")
        || !source.contains("server/auth/better-auth")
    {
        return None;
    }

    Some(DxReactRouteHandlerResponse {
        status: 200,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            ("cache-control".to_string(), "no-store".to_string()),
            (
                "x-dx-auth-session".to_string(),
                "source-owned-safe-interpreter".to_string(),
            ),
        ]),
        redirect_url: None,
        body: template_better_auth_session_body(&request.runtime_env),
        execution_model: "source-owned-template-better-auth-session-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

fn database_api_readiness_body() -> serde_json::Value {
    json!({
        "schema": "dx.www.template.database_api_readiness",
        "laneNumber": 4,
        "laneName": "Database + API",
        "route": "/api/database-api/readiness",
        "templateReadinessReceipt": ".dx/forge/template-readiness/database-api.json",
        "runtimeProof": false,
        "networkCalls": false,
        "hostedCredentials": false,
        "cacheEvidence": {
            "sourceOfTruth": ".dx/forge/package-status.json",
            "currentManifestSet": "cache.manifests",
            "currentManifestCountField": "cache.current_manifest_count",
            "physicalManifestCountField": "cache.physical_manifest_count",
            "stalePhysicalManifestCountField": "cache.stale_physical_manifest_count",
            "currentManifestSource": "package-status-current-manifests",
            "physicalManifestCaveatId": "physical-cache-may-include-stale-manifests",
            "laneCacheManifests": {
                "db/drizzle-sqlite": ".dx/forge/cache/db-drizzle-sqlite/0.1.0/manifest.json",
                "instantdb/react": ".dx/forge/cache/instantdb-react/0.0.0-dx.0/manifest.json",
                "supabase/client": ".dx/forge/cache/supabase-client/0.1.0/manifest.json",
                "api/trpc": ".dx/forge/cache/api-trpc/11.17.0-dx.10/manifest.json",
            },
        },
        "appRouterRoutes": [
            "app/api/instant/route.ts",
            "app/api/trpc/[trpc]/route.ts",
            "app/api/database-api/readiness/route.ts",
        ],
        "serverFiles": ["server/database-api/readiness.ts"],
        "sourceContract": {
            "schema": "dx.www.template.database_api_source_contract",
            "route": "/api/database-api/readiness",
            "runtimeProof": false,
            "networkCalls": false,
            "hostedCredentials": false,
            "schemaSurfaces": [
                {
                    "packageId": "db/drizzle-sqlite",
                    "officialName": "Database ORM",
                    "sourceFile": "db/drizzle/schema.ts",
                    "kind": "drizzle-sqlite",
                    "tables": ["users", "posts"],
                    "localProof": "Drizzle schema source defines users/posts tables, indexes, relations, and select/insert model types.",
                    "runtimeProof": false,
                    "networkCalls": false,
                    "hostedCredentials": false,
                    "appOwnedBoundary": [
                        "SQLite database path",
                        "better-sqlite3 installation",
                        "migration execution",
                        "tenant access policy",
                    ],
                },
                {
                    "packageId": "instantdb/react",
                    "officialName": "Realtime App Database",
                    "sourceFile": "lib/instant/schema.ts",
                    "kind": "instantdb-schema",
                    "entities": ["todos", "labels"],
                    "rooms": ["launch"],
                    "env": ["NEXT_PUBLIC_INSTANT_APP_ID"],
                    "localProof": "Instant schema source defines todos, labels, todoLabels, launch presence, and launchPing topic surfaces.",
                    "runtimeProof": false,
                    "networkCalls": false,
                    "hostedCredentials": false,
                    "appOwnedBoundary": [
                        "hosted Instant app id",
                        "rules and auth policy",
                        "realtime connection",
                        "storage and stream retention",
                    ],
                },
                {
                    "packageId": "supabase/client",
                    "officialName": "Backend Platform Client",
                    "sourceFile": "lib/supabase/profiles.ts",
                    "kind": "supabase-profile",
                    "tables": ["profiles"],
                    "env": [
                        "NEXT_PUBLIC_SUPABASE_URL",
                        "NEXT_PUBLIC_SUPABASE_PUBLISHABLE_KEY",
                    ],
                    "localProof": "Supabase profile source defines typed profile rows, select/upsert helpers, and a missing-config public env gate.",
                    "runtimeProof": false,
                    "networkCalls": false,
                    "hostedCredentials": false,
                    "appOwnedBoundary": [
                        "hosted Supabase project",
                        "RLS policy",
                        "auth redirect allow-list",
                        "read/write/realtime proof",
                    ],
                },
            ],
            "routeSurfaces": [
                {
                    "packageId": "instantdb/react",
                    "route": "/api/instant",
                    "methods": ["POST"],
                    "handlerFile": "app/api/instant/route.ts",
                    "runtimeProof": false,
                    "networkCalls": false,
                    "hostedCredentials": false,
                    "appOwnedBoundary": ["Instant hosted app id and route auth policy"],
                },
                {
                    "packageId": "api/trpc",
                    "route": "/api/trpc/[trpc]",
                    "methods": ["GET", "POST"],
                    "handlerFile": "app/api/trpc/[trpc]/route.ts",
                    "runtimeProof": false,
                    "networkCalls": false,
                    "hostedCredentials": false,
                    "appOwnedBoundary": ["production auth context and transport limits"],
                },
                {
                    "packageId": "api/trpc",
                    "route": "/api/database-api/readiness",
                    "methods": ["GET"],
                    "handlerFile": "app/api/database-api/readiness/route.ts",
                    "runtimeProof": false,
                    "networkCalls": false,
                    "hostedCredentials": false,
                    "appOwnedBoundary": ["provider runtime proof stays outside this local readiness route"],
                },
            ],
            "trpcProcedures": [
                {
                    "path": "health",
                    "kind": "query",
                    "sourceFile": "lib/trpc/router.ts",
                    "localProof": "Returns local request id and server timestamp through the typed router.",
                    "runtimeProof": false,
                    "appOwnedBoundary": ["deployment health policy and observability"],
                },
                {
                    "path": "launchReadiness",
                    "kind": "query",
                    "sourceFile": "lib/trpc/router.ts",
                    "localProof": "Returns the typed launch readiness result for a template input.",
                    "runtimeProof": false,
                    "appOwnedBoundary": ["production caller auth and rate limits"],
                },
                {
                    "path": "launchEvents",
                    "kind": "query",
                    "sourceFile": "lib/trpc/router.ts",
                    "localProof": "Returns typed paginated launch event fixture rows.",
                    "runtimeProof": false,
                    "appOwnedBoundary": ["durable event store and retention"],
                },
                {
                    "path": "launchEvent",
                    "kind": "mutation",
                    "sourceFile": "lib/trpc/router.ts",
                    "localProof": "Accepts a typed launch event mutation input and returns a local receipt shape.",
                    "runtimeProof": false,
                    "appOwnedBoundary": ["write authorization and persistence"],
                },
                {
                    "path": "launchFeed",
                    "kind": "subscription",
                    "sourceFile": "lib/trpc/router.ts",
                    "localProof": "The subscription procedure is source-visible but not claimed as live transport proof.",
                    "runtimeProof": false,
                    "appOwnedBoundary": [
                        "WebSocket/SSE transport",
                        "fan-out",
                        "retry",
                        "stream lifecycle",
                    ],
                },
            ],
        },
        "packages": [
            database_api_package(
                "db/drizzle-sqlite",
                "Database ORM",
                &["db/drizzle/schema.ts", "db/drizzle/dashboard-workflow.ts"],
                "Schema, query-plan, and dashboard workflow source are materialized and lock-backed.",
                &[
                    "SQLite database path",
                    "better-sqlite3 runtime install",
                    "migration rollout",
                    "tenant authorization",
                ],
            ),
            database_api_package(
                "instantdb/react",
                "Realtime App Database",
                &["lib/instant/schema.ts", "lib/instant/status.ts", "app/api/instant/route.ts"],
                "Schema, missing-config checks, and the Instant App Router route surface are materialized.",
                &[
                    "NEXT_PUBLIC_INSTANT_APP_ID",
                    "hosted rules and auth policy",
                    "realtime transport",
                    "storage and stream runtime proof",
                ],
            ),
            database_api_package(
                "supabase/client",
                "Backend Platform Client",
                &[
                    "lib/supabase/env.ts",
                    "lib/supabase/profiles.ts",
                    "lib/supabase/profile-workflow.ts",
                    "lib/supabase/.env.example",
                ],
                "Profile read model, config gate, and public-env validation are materialized without hosted credentials.",
                &[
                    "Supabase project credentials",
                    "RLS migration",
                    "Auth redirect allow-list",
                    "hosted read/write/realtime proof",
                ],
            ),
            database_api_package(
                "api/trpc",
                "Type-Safe API",
                &[
                    "lib/trpc/router.ts",
                    "lib/trpc/route-handler.ts",
                    "app/api/trpc/[trpc]/route.ts",
                    "lib/database-api/source-contract.ts",
                    "server/database-api/readiness.ts",
                    "app/api/database-api/readiness/route.ts",
                ],
                "Router, App Router route handlers, and the Database + API readiness contract are materialized and receipt-backed.",
                &[
                    "production auth context",
                    "transport and subscription policy",
                    "request limits",
                    "observability",
                ],
            ),
        ],
        "boundary": "This route executes locally and reports package readiness only; it does not open hosted database connections, install dependencies, or claim provider runtime proof.",
    })
}

fn database_api_package(
    package_id: &str,
    official_name: &str,
    front_facing_files: &[&str],
    local_proof: &str,
    app_owned_boundary: &[&str],
) -> serde_json::Value {
    json!({
        "packageId": package_id,
        "officialName": official_name,
        "status": "source-owned-adapter-boundary",
        "runtimeProof": false,
        "networkCalls": false,
        "frontFacingFiles": front_facing_files,
        "localProof": local_proof,
        "appOwnedBoundary": app_owned_boundary,
    })
}

fn query_cache_readiness_body(request: &DxReactRouteHandlerRequest) -> serde_json::Value {
    let filter = search_param_or_default(&request.search_params, "filter", "all");
    let optimistic_state = search_param_or_default(&request.search_params, "optimistic", "idle");
    let query_key = format!("dx:dashboard:projects:{filter}");
    let readiness_label = if optimistic_state == "applied" {
        "Local cache receipt applied"
    } else {
        "Local cache receipt ready"
    };

    json!({
        "schema": "dx.www.template.data_fetching_cache_readiness",
        "laneNumber": 3,
        "laneName": "State + Data Fetching",
        "route": "/api/query-cache/readiness",
        "packageId": "tanstack/query",
        "officialPackageName": "Data Fetching & Cache",
        "upstreamPackage": "@tanstack/react-query",
        "status": "source-owned-cache-readiness",
        "runtimeProof": false,
        "networkCalls": false,
        "nodeModulesRequired": false,
        "adapterBoundary": "queryclient-adapter-required",
        "cache": {
            "status": "source-owned-cache-readiness",
            "runtimeBoundary": "source-owned-template-cache",
            "upstreamAdapterBoundary": "queryclient-adapter-required",
            "packageId": "tanstack/query",
            "queryKey": query_key,
            "cacheEntryCount": 2,
            "readyEntryCount": 2,
            "staleEntryCount": 0,
            "invalidatedEntryCount": 0,
            "optimisticState": optimistic_state,
            "lastReceiptState": "App Router readiness route",
            "readinessLabel": readiness_label,
        },
        "appRouterRoutes": ["app/api/query-cache/readiness/route.ts"],
        "serverFiles": ["server/query-cache/readiness.ts"],
        "frontFacingFiles": [
            "components/template-app/dashboard-query-cache.ts",
            "server/query-cache/readiness.ts",
            "app/api/query-cache/readiness/route.ts",
        ],
        "appOwnedBoundary": [
            "live QueryClient provider",
            "production dashboard fetchers",
            "cache persistence storage",
            "broadcast channel naming",
            "browser runtime proof",
        ],
    })
}

fn query_cache_action_body(request: &DxReactRouteHandlerRequest) -> serde_json::Value {
    let filter = body_string(&request.body, "filter")
        .unwrap_or_else(|| search_param_or_default(&request.search_params, "filter", "all"));
    let optimistic_state = body_string(&request.body, "optimisticState")
        .or_else(|| body_string(&request.body, "optimistic"))
        .unwrap_or_else(|| search_param_or_default(&request.search_params, "optimistic", "idle"));
    let query_key = body_string(&request.body, "queryKey")
        .unwrap_or_else(|| format!("dx:dashboard:projects:{filter}"));
    let action = if body_string(&request.body, "action").as_deref() == Some("refresh") {
        "refresh"
    } else {
        "invalidate"
    };

    json!({
        "schema": "dx.www.template.data_fetching_cache_action_receipt",
        "route": "/api/query-cache/readiness",
        "packageId": "tanstack/query",
        "officialPackageName": "Data Fetching & Cache",
        "upstreamPackage": "@tanstack/react-query",
        "status": "source-owned-cache-action-dry-run",
        "action": action,
        "queryKey": query_key,
        "runtimeProof": false,
        "networkCalls": false,
        "nodeModulesRequired": false,
        "queryClientExecution": false,
        "adapterBoundary": "queryclient-adapter-required",
        "cache": query_cache_action_summary(action, &query_key, &filter, &optimistic_state),
        "secretValues": [],
        "appOwnedBoundary": [
            "live QueryClient provider",
            "production dashboard fetchers",
            "cache persistence storage",
            "broadcast channel naming",
            "browser runtime proof",
            "stateful QueryClient mutation execution",
        ],
    })
}

fn query_cache_action_summary(
    action: &str,
    query_key: &str,
    filter: &str,
    optimistic_state: &str,
) -> serde_json::Value {
    let project_key = format!("dx:dashboard:projects:{filter}");
    let mut entries = vec![
        (
            project_key.as_str(),
            if optimistic_state == "queued" {
                "optimistic"
            } else {
                "fresh"
            },
            false,
        ),
        (
            "dx:forge:package-reality",
            if optimistic_state == "applied" {
                "fresh"
            } else {
                "optimistic"
            },
            false,
        ),
    ];

    for (entry_key, state, invalidated) in &mut entries {
        if *entry_key != query_key {
            continue;
        }
        if action == "refresh" {
            *state = "fresh";
            *invalidated = false;
        } else {
            *state = "stale";
            *invalidated = true;
        }
    }

    let cache_entry_count = entries.len();
    let invalidated_entry_count = entries
        .iter()
        .filter(|(_, _, invalidated)| *invalidated)
        .count();
    let stale_entry_count = entries
        .iter()
        .filter(|(_, state, _)| *state == "stale")
        .count();
    let optimistic_entry_count = entries
        .iter()
        .filter(|(_, state, invalidated)| !*invalidated && *state == "optimistic")
        .count();
    let ready_entry_count = entries
        .iter()
        .filter(|(_, state, invalidated)| !*invalidated && *state != "stale")
        .count();

    json!({
        "cacheEntryCount": cache_entry_count,
        "readyEntryCount": ready_entry_count,
        "staleEntryCount": stale_entry_count,
        "invalidatedEntryCount": invalidated_entry_count,
        "optimisticEntryCount": optimistic_entry_count,
    })
}

fn search_param_or_default(
    params: &BTreeMap<String, String>,
    name: &str,
    default_value: &str,
) -> String {
    params
        .get(name)
        .filter(|value| !value.trim().is_empty())
        .cloned()
        .unwrap_or_else(|| default_value.to_string())
}

fn body_string(body: &Value, name: &str) -> Option<String> {
    body.get(name)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn instant_route_readiness_body(
    runtime_env: &BTreeMap<String, String>,
) -> (u16, serde_json::Value) {
    let app_id_configured = runtime_env
        .get("NEXT_PUBLIC_INSTANT_APP_ID")
        .is_some_and(|value| !value.trim().is_empty());

    let status = if app_id_configured {
        "configured-source-owned-adapter-boundary"
    } else {
        "provider-gated"
    };
    let http_status = if app_id_configured { 200 } else { 501 };

    (
        http_status,
        json!({
            "schema": "dx.www.template.instant_route_readiness",
            "packageId": "instantdb/react",
            "officialName": "Realtime App Database",
            "status": status,
            "httpStatus": http_status,
            "runtimeProof": false,
            "runtimeExecution": false,
            "liveProviderExecution": false,
            "networkCalls": false,
            "hostedCredentials": false,
            "nodeModulesRequired": false,
            "appIdConfigured": app_id_configured,
            "providerConfigured": app_id_configured,
            "providerBoundary": true,
            "missing": if app_id_configured {
                Vec::<&str>::new()
            } else {
                vec!["NEXT_PUBLIC_INSTANT_APP_ID"]
            },
            "route": "/api/instant",
            "boundary": if app_id_configured {
                "InstantDB app ID is configured locally; GET reports readiness only and does not claim hosted realtime transport proof."
            } else {
                "This route is source-owned and lock-backed, but hosted InstantDB app ID, rules, auth policy, and realtime transport proof stay app-owned."
            },
        }),
    )
}

fn instant_route_handler_post_body(
    request: &DxReactRouteHandlerRequest,
    runtime_env: &BTreeMap<String, String>,
) -> (u16, serde_json::Value) {
    let operation = request
        .body
        .get("op")
        .and_then(|value| value.as_str())
        .unwrap_or("unknown");
    let app_id_configured = instant_app_id_configured(runtime_env);
    let http_status = if app_id_configured { 202 } else { 501 };

    (
        http_status,
        json!({
            "ok": app_id_configured,
            "schema": "dx.www.template.instantdb_route_handler",
            "status": if app_id_configured {
                "configured-source-owned-dry-run"
            } else {
                "missing-config"
            },
            "httpStatus": http_status,
            "providerBoundary": true,
            "providerConfigured": app_id_configured,
            "method": request.method,
            "route": "/api/instant",
            "operation": operation,
            "packageId": "instantdb/react",
            "officialPackageName": "Realtime App Database",
            "upstreamPackage": "@instantdb/react",
            "requiredEnv": ["NEXT_PUBLIC_INSTANT_APP_ID"],
            "missing": if app_id_configured {
                Vec::<&str>::new()
            } else {
                vec!["NEXT_PUBLIC_INSTANT_APP_ID"]
            },
            "appIdConfigured": app_id_configured,
            "runtimeProof": false,
            "runtimeExecution": false,
            "liveProviderExecution": false,
            "networkCalls": false,
            "hostedCredentials": false,
            "nodeModulesRequired": false,
            "sourceFiles": [
                "app/api/instant/route.ts",
                "lib/instant/route.ts",
                "lib/instant/env.ts",
                "lib/instant/schema.ts",
            ],
            "schemaEvidence": {
                "entities": ["todos", "labels"],
                "links": ["todoLabels"],
                "rooms": ["launch"],
                "topics": ["launchPing"],
            },
            "appOwnedBoundary": [
                "NEXT_PUBLIC_INSTANT_APP_ID",
                "hosted Instant app rules and auth policy",
                "realtime transport proof",
                "request authorization for /api/instant",
            ],
            "message": if app_id_configured {
                "InstantDB app ID is configured locally; POST returns a local route receipt without hosted InstantDB transport, storage, auth, or stream execution."
            } else {
                "Configure NEXT_PUBLIC_INSTANT_APP_ID and hosted Instant rules before enabling the live Instant route handler."
            },
        }),
    )
}

fn instant_app_id_configured(runtime_env: &BTreeMap<String, String>) -> bool {
    runtime_env
        .get("NEXT_PUBLIC_INSTANT_APP_ID")
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
}

fn template_better_auth_readiness_body(
    runtime_env: &BTreeMap<String, String>,
) -> serde_json::Value {
    let missing_config = template_better_auth_missing_config(runtime_env);
    let credentials_configured = missing_config.is_empty();

    json!({
        "ok": true,
        "adapter": "better-auth",
        "officialPackageName": "Authentication",
        "upstreamPackage": "better-auth",
        "packageReadinessStatus": template_better_auth_package_readiness_status(credentials_configured),
        "status": "adapter-boundary",
        "liveRouteHandlersHttpStatus": 501,
        "runtimeExecution": false,
        "liveSessionExecution": false,
        "credentialsConfigured": credentials_configured,
        "databaseAdapterConfigured": false,
        "sessionStorage": "app-owned",
        "canRunRouteHandlers": false,
        "missingConfig": missing_config,
        "baseURL": template_better_auth_base_url(runtime_env),
        "adapterBoundaries": template_better_auth_adapter_boundaries(),
        "databaseBoundary": template_better_auth_database_boundary(),
        "appOwnedDatabaseAdapter": false,
        "migrationsRequired": true,
    })
}

fn template_better_auth_package_readiness_status(credentials_configured: bool) -> &'static str {
    if credentials_configured {
        "configured"
    } else {
        "missing-config"
    }
}

fn template_better_auth_route_handler_body(
    method: &str,
    runtime_env: &BTreeMap<String, String>,
) -> serde_json::Value {
    let missing_config = template_better_auth_missing_config(runtime_env);
    let credentials_configured = missing_config.is_empty();

    json!({
        "ok": false,
        "status": "adapter-boundary",
        "httpStatus": 501,
        "method": method,
        "adapter": "better-auth",
        "officialPackageName": "Authentication",
        "upstreamPackage": "better-auth",
        "runtimeExecution": false,
        "liveSessionExecution": false,
        "credentialsConfigured": credentials_configured,
        "databaseAdapterConfigured": false,
        "sessionStorage": "app-owned",
        "missingConfig": missing_config,
        "adapterBoundaries": template_better_auth_adapter_boundaries(),
        "databaseBoundary": template_better_auth_database_boundary(),
        "migrationsRequired": true,
        "message": "Configure Authentication credentials and pass an app-owned Better Auth database adapter before enabling live sessions.",
    })
}

fn template_better_auth_session_body(runtime_env: &BTreeMap<String, String>) -> serde_json::Value {
    let missing_config = template_better_auth_missing_config(runtime_env);
    let credentials_configured = missing_config.is_empty();

    json!({
        "ok": true,
        "schema": "dx.template.authentication.session_receipt",
        "status": if credentials_configured {
            "configured-anonymous-session"
        } else {
            "anonymous-session"
        },
        "httpStatus": 200,
        "route": "/api/auth/session",
        "adapter": "better-auth",
        "officialPackageName": "Authentication",
        "upstreamPackage": "better-auth",
        "authenticated": false,
        "session": null,
        "runtimeExecution": false,
        "liveSessionExecution": false,
        "credentialsConfigured": credentials_configured,
        "databaseAdapterConfigured": false,
        "sessionStorage": "app-owned",
        "missingConfig": missing_config,
        "baseURL": template_better_auth_base_url(runtime_env),
        "adapterBoundaries": template_better_auth_adapter_boundaries(),
        "databaseBoundary": template_better_auth_database_boundary(),
        "appOwnedBoundary": "Live Better Auth session lookup requires app-owned cookies, database adapter, migrations, and deployment policy.",
    })
}

fn template_better_auth_missing_config(
    runtime_env: &BTreeMap<String, String>,
) -> Vec<&'static str> {
    [
        "BETTER_AUTH_SECRET",
        "BETTER_AUTH_URL",
        "GOOGLE_CLIENT_ID",
        "GOOGLE_CLIENT_SECRET",
    ]
    .into_iter()
    .filter(|name| !runtime_env_value_present(runtime_env, name))
    .collect()
}

fn runtime_env_value_present(runtime_env: &BTreeMap<String, String>, name: &str) -> bool {
    runtime_env
        .get(name)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
}

fn template_better_auth_base_url(runtime_env: &BTreeMap<String, String>) -> String {
    runtime_env
        .get("BETTER_AUTH_URL")
        .or_else(|| runtime_env.get("NEXT_PUBLIC_BETTER_AUTH_URL"))
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "/api/auth".to_string())
}

fn template_better_auth_adapter_boundaries() -> serde_json::Value {
    json!([
        "database adapter, session storage, migrations",
        "cookie lifetime, trusted origins, and secure deployment policy",
        "Google OAuth callback URL and provider-console credentials",
    ])
}

fn template_better_auth_database_boundary() -> serde_json::Value {
    json!({
            "schema": "dx.template.authentication.database_boundary",
            "packageId": "auth/better-auth",
            "officialPackageName": "Authentication",
            "upstreamPackage": "better-auth",
            "appOwned": true,
            "runtimeProof": false,
            "requiredInput": "BetterAuthOptions[\"database\"]",
            "acceptedUpstreamShapes": [
                "Better Auth DBAdapterInstance",
                "Kysely database plus type",
                "SQLite, PostgreSQL, MySQL, D1, or Bun database object accepted by Better Auth",
            ],
            "migrationCommands": ["npx auth@latest generate", "npx auth@latest migrate"],
            "note": "Forge materializes the Authentication server boundary, but the app must choose and pass a real Better Auth database adapter before live sessions are enabled.",
    })
}
