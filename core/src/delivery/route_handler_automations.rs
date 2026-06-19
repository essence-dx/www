use std::collections::BTreeMap;

use serde_json::{Value, json};

use super::server_contract::{DxReactRouteHandlerRequest, DxReactRouteHandlerResponse};

pub(super) fn automation_n8n_dry_run_route_handler_response(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    if path != "/api/automations/n8n/dry-run"
        || !matches!(request.method.as_str(), "GET" | "POST")
        || !source.contains("@/lib/automations/n8n/receipt")
        || !source.contains("@/lib/automations/n8n/readiness")
        || !source.contains("createDxN8nRunReceipt")
    {
        return None;
    }

    if request.method == "GET"
        && !function_body.contains("filterDxN8nConnectors")
        && !function_body.contains("buildDxN8nCredentialReadiness")
    {
        return None;
    }

    if request.method == "POST" && !function_body.contains("createDxN8nRunReceipt") {
        return None;
    }

    let (status, body) = if request.method == "GET" {
        (200, automation_n8n_get_body(request))
    } else {
        automation_n8n_post_body(request)
    };

    Some(DxReactRouteHandlerResponse {
        status,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([(
            "x-dx-automation-n8n-dry-run".to_string(),
            "source-owned-safe-interpreter".to_string(),
        )]),
        redirect_url: None,
        body,
        execution_model: "source-owned-automation-n8n-dry-run-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

fn automation_n8n_get_body(request: &DxReactRouteHandlerRequest) -> Value {
    let filter = search_param_or_default(&request.search_params, "filter", "all");
    let filter = match filter.as_str() {
        "ready" | "missing-config" | "tool-ready" => filter,
        _ => "all".to_string(),
    };
    let connector_ids = n8n_connector_ids_for_filter(&filter, &request.runtime_env);
    let provider_configured_count = connector_ids
        .iter()
        .filter(|connector_id| n8n_missing_env(connector_id, &request.runtime_env).is_empty())
        .count();
    let provider_missing_config_count = connector_ids
        .len()
        .saturating_sub(provider_configured_count);
    let connectors = connector_ids
        .into_iter()
        .map(|connector_id| {
            let connector = n8n_connector(connector_id);
            json!({
                "id": connector["id"],
                "displayName": connector["displayName"],
                "status": connector["status"],
                "authKinds": connector["authKinds"],
                "credentials": connector["credentials"],
                "sourceFile": connector["sourceFile"],
                "resources": connector["resources"],
                "operations": connector["operations"],
                "workflowNode": connector["workflowNode"],
                "readiness": n8n_credential_readiness(connector_id, &request.runtime_env),
            })
        })
        .collect::<Vec<_>>();

    json!({
        "ok": true,
        "packageId": "automations/n8n",
        "status": "provider-boundary",
        "providerBoundary": true,
        "providerConfiguredCount": provider_configured_count,
        "providerMissingConfigCount": provider_missing_config_count,
        "runtimeExecution": false,
        "liveProviderExecution": false,
        "filter": filter,
        "connectors": connectors,
        "boundary": "This route exposes local connector readiness only. Provider credentials, webhook registration, and live n8n execution remain app-owned.",
    })
}

fn automation_n8n_post_body(request: &DxReactRouteHandlerRequest) -> (u16, Value) {
    let connector_id = request
        .body
        .get("connectorId")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("slack-release-note");

    if !n8n_known_connector(connector_id) {
        return (
            400,
            json!({
                "ok": false,
                "packageId": "automations/n8n",
                "status": "bad-request",
                "message": "Automation connector must be one of the launch template connectors.",
                "runtimeExecution": false,
                "secretValues": [],
            }),
        );
    }

    let mode = match request.body.get("mode").and_then(Value::as_str) {
        Some("draft") => "draft",
        Some("run") => "run",
        _ => "dry-run",
    };
    let intent = request
        .body
        .get("intent")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("Draft a launch handoff receipt without contacting provider APIs.");
    let workflow_id = request
        .body
        .get("workflowId")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| format!("{connector_id}-launch-dry-run"));
    let readiness = n8n_credential_readiness(connector_id, &request.runtime_env);
    let receipt = n8n_run_receipt(
        connector_id,
        intent,
        mode,
        &workflow_id,
        &request.runtime_env,
    );
    let blocked = receipt["status"] == "blocked-missing-config";
    let status = if blocked { 501 } else { 202 };

    (
        status,
        json!({
            "ok": !blocked,
            "packageId": "automations/n8n",
            "status": if blocked { "missing-config" } else { "local-dry-run" },
            "httpStatus": status,
            "providerBoundary": true,
            "providerConfigured": !blocked,
            "connector": n8n_connector(connector_id),
            "readiness": readiness,
            "receipt": receipt,
            "runtimeExecution": false,
            "liveProviderExecution": false,
            "secretValues": [],
            "boundary": "The response is a local DX/Zed handoff. It does not run n8n, read secrets, register webhooks, or call provider APIs.",
        }),
    )
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

fn n8n_connector_ids_for_filter(
    filter: &str,
    runtime_env: &BTreeMap<String, String>,
) -> Vec<&'static str> {
    let ids = ["manual-trigger", "slack-release-note", "notion-launch-log"];
    ids.into_iter()
        .filter(|connector_id| match filter {
            "ready" => n8n_missing_env(connector_id, runtime_env).is_empty(),
            "missing-config" => !n8n_missing_env(connector_id, runtime_env).is_empty(),
            "tool-ready" => matches!(*connector_id, "slack-release-note" | "notion-launch-log"),
            _ => true,
        })
        .collect()
}

fn n8n_known_connector(connector_id: &str) -> bool {
    matches!(
        connector_id,
        "manual-trigger" | "slack-release-note" | "notion-launch-log"
    )
}

fn n8n_connector(connector_id: &str) -> Value {
    match connector_id {
        "manual-trigger" => json!({
            "id": "manual-trigger",
            "displayName": "Manual Trigger",
            "status": "ready",
            "authKinds": [],
            "credentials": [],
            "sourceFile": "nodes/ManualTrigger/ManualTrigger.node.ts",
            "resources": [{ "name": "Manual trigger", "value": "manualTrigger" }],
            "operations": [{
                "name": "Start workflow",
                "value": "start",
                "action": "manual.trigger",
            }],
            "workflowNode": {
                "ready": true,
                "trigger": true,
                "usableAsTool": false,
                "runMode": "metadata-ready",
            },
        }),
        "notion-launch-log" => json!({
            "id": "notion-launch-log",
            "displayName": "Notion launch log",
            "status": "needs_credential",
            "authKinds": ["api-key", "oauth2"],
            "credentials": ["notionApi", "notionOAuth2Api"],
            "sourceFile": "nodes/Notion/Notion.node.ts",
            "resources": [{ "name": "Page", "value": "page" }],
            "operations": [{
                "name": "Append page",
                "value": "append",
                "action": "notion.page.append",
            }],
            "workflowNode": {
                "ready": true,
                "trigger": false,
                "usableAsTool": true,
                "runMode": "credential-gated",
            },
        }),
        _ => json!({
            "id": "slack-release-note",
            "displayName": "Slack release note",
            "status": "needs_credential",
            "authKinds": ["token", "oauth2"],
            "credentials": ["slackApi", "slackOAuth2Api"],
            "sourceFile": "nodes/Slack/V2/SlackV2.node.ts",
            "resources": [{ "name": "Message", "value": "message" }],
            "operations": [{
                "name": "Post message",
                "value": "post",
                "action": "slack.message.post",
            }],
            "workflowNode": {
                "ready": true,
                "trigger": false,
                "usableAsTool": true,
                "runMode": "credential-gated",
            },
        }),
    }
}

fn n8n_credential_readiness(connector_id: &str, runtime_env: &BTreeMap<String, String>) -> Value {
    let connector = n8n_connector(connector_id);
    let display_name = connector["displayName"].as_str().unwrap_or("Connector");
    let credentials = connector["credentials"].clone();
    let required_env = n8n_required_env(connector_id);
    let missing_credentials = n8n_missing_env(connector_id, runtime_env);
    let credentials_configured = missing_credentials.is_empty();
    let status = if required_env.is_empty() {
        "metadata-ready"
    } else {
        "credential-gated"
    };

    json!({
        "status": status,
        "requiredEnv": required_env,
        "missingCredentials": missing_credentials,
        "credentialsConfigured": credentials_configured,
        "credentialTypes": credentials,
        "message": if required_env.is_empty() {
            format!("{display_name} can draft a local workflow receipt without secrets.")
        } else if credentials_configured {
            format!("{display_name} has local credential presence for a Zed handoff; live n8n execution still remains app-owned.")
        } else {
            format!("{display_name} needs app-owned credentials before live execution.")
        },
    })
}

fn n8n_required_env(connector_id: &str) -> Vec<&'static str> {
    match connector_id {
        "slack-release-note" => vec!["SLACK_BOT_TOKEN", "SLACK_CLIENT_ID", "SLACK_CLIENT_SECRET"],
        "notion-launch-log" => vec!["NOTION_API_KEY", "NOTION_CLIENT_ID", "NOTION_CLIENT_SECRET"],
        _ => Vec::new(),
    }
}

fn n8n_missing_env(
    connector_id: &str,
    runtime_env: &BTreeMap<String, String>,
) -> Vec<&'static str> {
    n8n_required_env(connector_id)
        .into_iter()
        .filter(|name| {
            !runtime_env
                .get(*name)
                .map(|value| !value.trim().is_empty())
                .unwrap_or(false)
        })
        .collect()
}

fn n8n_run_receipt(
    connector_id: &str,
    intent: &str,
    mode: &str,
    workflow_id: &str,
    runtime_env: &BTreeMap<String, String>,
) -> Value {
    let connector = n8n_connector(connector_id);
    let required_env = n8n_required_env(connector_id);
    let missing_credentials = n8n_missing_env(connector_id, runtime_env);
    let status = if mode == "run" && !missing_credentials.is_empty() {
        "blocked-missing-config"
    } else if mode == "run" {
        "zed-handoff-created"
    } else if mode == "draft" {
        "draft-created"
    } else {
        "local-dry-run"
    };
    let quoted_connector = serde_json::to_string(connector_id).unwrap_or_else(|_| "\"\"".into());
    let quoted_workflow = serde_json::to_string(workflow_id).unwrap_or_else(|_| "\"\"".into());

    json!({
        "schema": "dx.automation.n8n.run_receipt",
        "packageId": "automations/n8n",
        "upstreamPackage": "n8n-nodes-base",
        "connectorId": connector_id,
        "connectorName": connector["displayName"],
        "workflowId": workflow_id,
        "workflowIntent": intent,
        "mode": mode,
        "status": status,
        "requiredEnv": required_env,
        "missingCredentials": missing_credentials,
        "runtimeExecution": false,
        "secretValues": [],
        "commands": {
            "dryRun": format!(
                "dx automations run --json --dry-run --connector {quoted_connector} --workflow {quoted_workflow}"
            ),
            "run": format!(
                "dx automations run --json --connector {quoted_connector} --workflow {quoted_workflow} --mode {mode}"
            ),
        },
        "boundary": "This receipt is a local DX/Zed handoff. n8n credentials, webhook registration, and live workflow execution remain app-owned.",
        "generatedAt": "dx-www-source-owned-clock",
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use super::automation_n8n_dry_run_route_handler_response;
    use crate::delivery::server_contract::DxReactRouteHandlerRequest;

    #[test]
    fn n8n_configured_connector_run_returns_zed_handoff_without_provider_runtime() {
        let response = automation_n8n_dry_run_route_handler_response(
            r#"import { buildDxN8nCredentialReadiness } from "@/lib/automations/n8n/readiness";
import { createDxN8nRunReceipt } from "@/lib/automations/n8n/receipt";"#,
            "createDxN8nRunReceipt({ connector, intent, mode, workflowId })",
            &DxReactRouteHandlerRequest {
                method: "POST".to_string(),
                path: "/api/automations/n8n/dry-run".to_string(),
                headers: BTreeMap::new(),
                body: json!({
                    "connectorId": "slack-release-note",
                    "mode": "run",
                    "workflowId": "slack-release-note-launch-run",
                    "intent": "Prepare the launch Slack note handoff",
                }),
                route_params: BTreeMap::new(),
                search_params: BTreeMap::new(),
                runtime_env: BTreeMap::from([
                    (
                        "SLACK_BOT_TOKEN".to_string(),
                        "redacted-present".to_string(),
                    ),
                    (
                        "SLACK_CLIENT_ID".to_string(),
                        "redacted-present".to_string(),
                    ),
                    (
                        "SLACK_CLIENT_SECRET".to_string(),
                        "redacted-present".to_string(),
                    ),
                ]),
            },
        )
        .expect("n8n configured dry-run route");

        assert_eq!(response.status, 202);
        assert_eq!(response.body["ok"], true);
        assert_eq!(
            response.body["providerBoundary"], true,
            "provider boundary stays visible even after credentials are present"
        );
        assert_eq!(response.body["providerConfigured"], true);
        assert_eq!(response.body["runtimeExecution"], false);
        assert_eq!(response.body["liveProviderExecution"], false);
        assert_eq!(response.body["receipt"]["status"], "zed-handoff-created");
        assert_eq!(
            response.body["readiness"]["missingCredentials"]
                .as_array()
                .map(Vec::len),
            Some(0)
        );
        assert_eq!(response.body["readiness"]["credentialsConfigured"], true);
        assert_eq!(
            response.body["secretValues"].as_array().map(Vec::len),
            Some(0)
        );
    }

    #[test]
    fn n8n_get_readiness_accepts_absolute_url_for_endpoint_match() {
        let response = automation_n8n_dry_run_route_handler_response(
            r#"import { filterDxN8nConnectors, buildDxN8nCredentialReadiness } from "@/lib/automations/n8n/readiness";
import { createDxN8nRunReceipt } from "@/lib/automations/n8n/receipt";"#,
            "filterDxN8nConnectors(connectors, filter)",
            &DxReactRouteHandlerRequest {
                method: "GET".to_string(),
                path: "https://example.test/api/automations/n8n/dry-run?filter=tool-ready#connectors"
                    .to_string(),
                headers: BTreeMap::new(),
                body: json!({}),
                route_params: BTreeMap::new(),
                search_params: BTreeMap::from([("filter".to_string(), "tool-ready".to_string())]),
                runtime_env: BTreeMap::new(),
            },
        )
        .expect("n8n dry-run route should match absolute request URL");

        assert_eq!(response.status, 200);
        assert_eq!(
            response.execution_model,
            "source-owned-automation-n8n-dry-run-interpreter"
        );
        assert_eq!(response.body["filter"], "tool-ready");
        assert_eq!(
            response.body["connectors"].as_array().map(Vec::len),
            Some(2)
        );
        assert_eq!(response.body["runtimeExecution"], false);
    }
}
