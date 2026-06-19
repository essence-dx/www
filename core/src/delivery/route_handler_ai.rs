use std::collections::BTreeMap;

use serde_json::{Value, json};

use super::server_contract::{DxReactRouteHandlerRequest, DxReactRouteHandlerResponse};

const DX_AI_EXTENDED_ROUTES_ENABLE_ENV: &str = "DX_ENABLE_EXTENDED_AI_ROUTES";

pub(super) fn ai_local_stream_route_handler_response(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    if let Some(boundary) = ai_provider_boundary(path).filter(|boundary| boundary.extended_route) {
        let route_is_gated = source_contains_provider_boundary(source, function_body);
        let route_is_disabled = !ai_extended_route_enabled(&request.runtime_env);
        let provider_is_missing = !ai_provider_env_configured(boundary, &request.runtime_env);
        if route_is_gated && (route_is_disabled || provider_is_missing) {
            return Some(ai_provider_boundary_response(
                boundary,
                &request.body,
                &request.runtime_env,
            ));
        }
    }

    let body = match path {
        "/api/ai/text-stream"
            if request.method == "GET"
                && source.contains("@/lib/ai/text-stream")
                && source.contains("createDxLaunchTextStreamResponse")
                && function_body.contains("createDxLaunchTextStream(")
                && function_body.contains("createDxLaunchTextStreamResponse") =>
        {
            ai_text_stream_body()
        }
        "/api/ai/ui-stream"
            if request.method == "POST"
                && source.contains("@/lib/ai/ui-message-stream")
                && source.contains("createDxLaunchUIMessageStreamResponse")
                && function_body.contains("createDxLaunchUIMessageStream(")
                && function_body.contains("createDxLaunchUIMessageStreamResponse") =>
        {
            ai_ui_stream_body(request)
        }
        _ => return None,
    };

    Some(DxReactRouteHandlerResponse {
        status: 200,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([(
            "x-dx-ai-local-stream".to_string(),
            "source-owned-safe-interpreter".to_string(),
        )]),
        redirect_url: None,
        body,
        execution_model: "source-owned-ai-local-stream-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

pub(super) fn ai_provider_boundary_route_handler_response(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    let boundary = ai_provider_boundary(path)?;
    if request.method != boundary.method
        || !source_contains_provider_boundary(source, function_body)
        || !source.contains(&format!("process.env.{}", boundary.required_env))
    {
        return None;
    }

    Some(ai_provider_boundary_response(
        boundary,
        &request.body,
        &request.runtime_env,
    ))
}

pub(super) fn ai_factory_route_handler_response(
    source: &str,
    expression: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = request.path_for_match().trim_end_matches('/');
    let boundary = ai_provider_boundary(path)?;
    if request.method != boundary.method || !ai_factory_matches(source, expression, boundary.path) {
        return None;
    }

    Some(ai_provider_boundary_response(
        boundary,
        &request.body,
        &request.runtime_env,
    ))
}

fn ai_factory_matches(source: &str, expression: &str, path: &str) -> bool {
    let factory_name = match path {
        "/api/ai/chat" => "createDxAIChatRoute",
        "/api/ai/agent" => "createDxAgentRoute",
        _ => return false,
    };
    source.contains(factory_name) && expression.contains(factory_name)
}

fn ai_text_stream_body() -> Value {
    let chunks = [
        "DX text stream bridge is wired.\n",
        "Connect it to live launch receipts before production.\n",
    ];
    let text = chunks.concat();

    json!({
        "ok": true,
        "schema": "dx.ai.local_stream_receipt",
        "packageId": "ai/vercel-ai",
        "status": "source-owned-local-stream-dry-run",
        "endpoint": "/api/ai/text-stream",
        "streamKind": "text-stream",
        "adapterBoundary": "ai-sdk-stream-adapter-boundary",
        "runtimeExecution": false,
        "providerRuntime": false,
        "modelStreaming": false,
        "liveStreamingTransport": false,
        "networkCalls": false,
        "nodeModulesRequired": false,
        "chunks": chunks,
        "text": text,
        "secretValues": [],
        "boundary": "This is a local DX stream receipt. It does not execute the AI SDK stream transport, call a model provider, or claim live streaming runtime proof."
    })
}

fn ai_ui_stream_body(request: &DxReactRouteHandlerRequest) -> Value {
    let text = request
        .body
        .get("text")
        .and_then(Value::as_str)
        .filter(|text| !text.trim().is_empty())
        .unwrap_or(
            "DX UI message stream bridge is wired. Connect it to real route work before launch.",
        );

    json!({
        "ok": true,
        "schema": "dx.ai.local_stream_receipt",
        "packageId": "ai/vercel-ai",
        "status": "source-owned-local-stream-dry-run",
        "endpoint": "/api/ai/ui-stream",
        "streamKind": "ui-message-stream",
        "adapterBoundary": "ai-sdk-stream-adapter-boundary",
        "runtimeExecution": false,
        "providerRuntime": false,
        "modelStreaming": false,
        "liveStreamingTransport": false,
        "networkCalls": false,
        "nodeModulesRequired": false,
        "request": {
            "text": text,
        },
        "events": [
            {
                "type": "text-start",
                "id": "dx-launch-ui-stream",
            },
            {
                "type": "text-delta",
                "id": "dx-launch-ui-stream",
                "delta": text,
            },
            {
                "type": "text-end",
                "id": "dx-launch-ui-stream",
            },
        ],
        "secretValues": [],
        "boundary": "This is a local DX UI message stream receipt. It does not execute AI SDK stream transport, call a model provider, or claim live streaming runtime proof."
    })
}

struct AiProviderBoundary {
    path: &'static str,
    method: &'static str,
    provider: &'static str,
    capability: &'static str,
    required_env: &'static str,
    app_owned_boundary: &'static str,
    configured_readiness_only: bool,
    extended_route: bool,
}

fn ai_provider_boundary(path: &str) -> Option<&'static AiProviderBoundary> {
    match path {
        "/api/ai/chat" => Some(&AiProviderBoundary {
            path: "/api/ai/chat",
            method: "POST",
            provider: "openai-compatible",
            capability: "chat-stream",
            required_env: "AI_PROVIDER_API_KEY",
            app_owned_boundary: "Set AI_PROVIDER_API_KEY in the app environment to stream model output.",
            configured_readiness_only: true,
            extended_route: false,
        }),
        "/api/ai/text-stream" => Some(&AiProviderBoundary {
            path: "/api/ai/text-stream",
            method: "GET",
            provider: "openai-compatible",
            capability: "text-stream-bridge",
            required_env: "AI_PROVIDER_API_KEY",
            app_owned_boundary: "This extended AI route is outside the default launch AI proof surface. Set DX_ENABLE_EXTENDED_AI_ROUTES=true after app-owned streaming policy and browser proof are ready.",
            configured_readiness_only: false,
            extended_route: true,
        }),
        "/api/ai/ui-stream" => Some(&AiProviderBoundary {
            path: "/api/ai/ui-stream",
            method: "POST",
            provider: "openai-compatible",
            capability: "ui-message-stream-bridge",
            required_env: "AI_PROVIDER_API_KEY",
            app_owned_boundary: "This extended AI route is outside the default launch AI proof surface. Set DX_ENABLE_EXTENDED_AI_ROUTES=true after app-owned UI stream policy and browser proof are ready.",
            configured_readiness_only: false,
            extended_route: true,
        }),
        "/api/ai/agent" => Some(&AiProviderBoundary {
            path: "/api/ai/agent",
            method: "POST",
            provider: "openai-compatible",
            capability: "agent-loop",
            required_env: "AI_PROVIDER_API_KEY",
            app_owned_boundary: "Set AI_PROVIDER_API_KEY and review app-approved tools before running the agent loop.",
            configured_readiness_only: false,
            extended_route: true,
        }),
        "/api/ai/image" => Some(&AiProviderBoundary {
            path: "/api/ai/image",
            method: "POST",
            provider: "openai-compatible",
            capability: "image-generation",
            required_env: "AI_PROVIDER_API_KEY",
            app_owned_boundary: "Set AI_PROVIDER_API_KEY before generating launch image assets.",
            configured_readiness_only: false,
            extended_route: true,
        }),
        "/api/ai/object" => Some(&AiProviderBoundary {
            path: "/api/ai/object",
            method: "POST",
            provider: "openai-compatible",
            capability: "object-generation",
            required_env: "AI_PROVIDER_API_KEY",
            app_owned_boundary: "Set AI_PROVIDER_API_KEY before generating structured launch objects.",
            configured_readiness_only: false,
            extended_route: true,
        }),
        "/api/ai/speech" => Some(&AiProviderBoundary {
            path: "/api/ai/speech",
            method: "POST",
            provider: "openai-compatible",
            capability: "speech-generation",
            required_env: "AI_PROVIDER_API_KEY",
            app_owned_boundary: "Set AI_PROVIDER_API_KEY before generating launch speech audio.",
            configured_readiness_only: false,
            extended_route: true,
        }),
        "/api/ai/transcribe" => Some(&AiProviderBoundary {
            path: "/api/ai/transcribe",
            method: "POST",
            provider: "openai-compatible",
            capability: "audio-transcription",
            required_env: "AI_PROVIDER_API_KEY",
            app_owned_boundary: "Set AI_PROVIDER_API_KEY before transcribing launch audio.",
            configured_readiness_only: false,
            extended_route: true,
        }),
        "/api/ai/upload-file" => Some(&AiProviderBoundary {
            path: "/api/ai/upload-file",
            method: "POST",
            provider: "openai-compatible",
            capability: "provider-file-upload",
            required_env: "AI_PROVIDER_API_KEY",
            app_owned_boundary: "Set AI_PROVIDER_API_KEY before uploading launch files to a model provider.",
            configured_readiness_only: false,
            extended_route: true,
        }),
        "/api/ai/video" => Some(&AiProviderBoundary {
            path: "/api/ai/video",
            method: "POST",
            provider: "gateway",
            capability: "video-generation",
            required_env: "AI_GATEWAY_API_KEY",
            app_owned_boundary: "Set AI_GATEWAY_API_KEY and gateway routing before generating launch video assets.",
            configured_readiness_only: false,
            extended_route: true,
        }),
        _ => None,
    }
}

fn source_contains_provider_boundary(source: &str, function_body: &str) -> bool {
    source.contains("@/lib/ai/provider-boundary")
        && source.contains("createDxAiMissingProviderResponse")
        && (function_body.contains("createDxAiMissingProviderResponse")
            || function_body.contains("createDxAiExtendedRouteDisabledResponse"))
}

fn ai_provider_env_configured(
    boundary: &AiProviderBoundary,
    runtime_env: &BTreeMap<String, String>,
) -> bool {
    runtime_env
        .get(boundary.required_env)
        .map(|value| !value.trim().is_empty())
        .unwrap_or(false)
}

fn ai_provider_configured_readiness(
    boundary: &AiProviderBoundary,
    runtime_env: &BTreeMap<String, String>,
) -> bool {
    (boundary.configured_readiness_only || boundary.extended_route)
        && ai_provider_env_configured(boundary, runtime_env)
        && !ai_extended_route_disabled(boundary, runtime_env)
}

fn ai_extended_route_enabled(runtime_env: &BTreeMap<String, String>) -> bool {
    runtime_env
        .get(DX_AI_EXTENDED_ROUTES_ENABLE_ENV)
        .map(|value| value.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn ai_extended_route_disabled(
    boundary: &AiProviderBoundary,
    runtime_env: &BTreeMap<String, String>,
) -> bool {
    boundary.extended_route && !ai_extended_route_enabled(runtime_env)
}

fn ai_provider_boundary_response(
    boundary: &AiProviderBoundary,
    body: &Value,
    runtime_env: &BTreeMap<String, String>,
) -> DxReactRouteHandlerResponse {
    DxReactRouteHandlerResponse {
        status: ai_provider_http_status(boundary, runtime_env),
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([(
            "x-dx-ai-provider-boundary".to_string(),
            "source-owned-safe-interpreter".to_string(),
        )]),
        redirect_url: None,
        body: ai_provider_boundary_body(boundary, body, runtime_env),
        execution_model: "source-owned-ai-provider-boundary-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    }
}

fn ai_provider_http_status(
    boundary: &AiProviderBoundary,
    runtime_env: &BTreeMap<String, String>,
) -> u16 {
    if ai_provider_configured_readiness(boundary, runtime_env) {
        202
    } else {
        501
    }
}

fn ai_provider_boundary_body(
    boundary: &AiProviderBoundary,
    body: &Value,
    runtime_env: &BTreeMap<String, String>,
) -> Value {
    let credentials_configured = ai_provider_env_configured(boundary, runtime_env);
    if ai_extended_route_disabled(boundary, runtime_env) {
        return json!({
            "schema": "dx.ai.extended_route_boundary",
            "ok": false,
            "packageId": "ai/vercel-ai",
            "status": "extended-route-disabled",
            "httpStatus": 501,
            "endpoint": boundary.path,
            "route": boundary.path,
            "provider": boundary.provider,
            "capability": boundary.capability,
            "requiredEnv": [DX_AI_EXTENDED_ROUTES_ENABLE_ENV, boundary.required_env],
            "credentialsConfigured": credentials_configured,
            "providerConfigured": credentials_configured,
            "adapterBoundary": "extended-provider-route-boundary",
            "proofSurface": "outside-default-ai-surface",
            "defaultAiSurface": false,
            "providerBoundary": true,
            "runtimeExecution": false,
            "liveProviderExecution": false,
            "runtimeProof": false,
            "liveProviderProof": false,
            "modelStreaming": false,
            "providerRuntime": false,
            "secretValues": [],
            "appOwnedBoundary": boundary.app_owned_boundary,
            "boundary": "This extended AI route is outside the default launch proof surface. Enable it explicitly after app-owned policy, billing, moderation, and runtime proof are ready."
        });
    }

    let configured_readiness = ai_provider_configured_readiness(boundary, runtime_env);
    let http_status = ai_provider_http_status(boundary, runtime_env);
    let status = if configured_readiness {
        "provider-configured-readiness-only"
    } else if credentials_configured {
        "provider-runtime-boundary"
    } else {
        "missing-config"
    };

    json!({
        "schema": if configured_readiness {
            "dx.ai.provider_readiness"
        } else {
            "dx.ai.provider_boundary"
        },
        "ok": configured_readiness,
        "packageId": "ai/vercel-ai",
        "status": status,
        "httpStatus": http_status,
        "endpoint": boundary.path,
        "provider": boundary.provider,
        "capability": boundary.capability,
        "requiredEnv": [boundary.required_env],
        "credentialsConfigured": credentials_configured,
        "providerConfigured": credentials_configured,
        "adapterBoundary": "provider-credential-boundary",
        "providerBoundary": true,
        "runtimeExecution": false,
        "liveProviderExecution": false,
        "runtimeProof": false,
        "liveProviderProof": false,
        "modelStreaming": false,
        "providerRuntime": false,
        "secretValues": [],
        "message": body.get("message").and_then(Value::as_str),
        "requestId": body.get("requestId").and_then(Value::as_str),
        "request": {
            "message": body.get("message").and_then(Value::as_str),
            "requestId": body.get("requestId").and_then(Value::as_str),
        },
        "appOwnedBoundary": boundary.app_owned_boundary,
        "boundary": "This response is a local DX/Zed provider-boundary receipt. It does not call a model, stream tokens, read secret values, bill a provider, or execute tools."
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use super::ai_provider_boundary_route_handler_response;
    use crate::delivery::server_contract::DxReactRouteHandlerRequest;

    #[test]
    fn extended_image_route_enabled_with_credentials_returns_readiness_handoff() {
        let response = ai_provider_boundary_route_handler_response(
            r#"import {
  createDxAiExtendedRouteDisabledResponse,
  createDxAiMissingProviderResponse,
  isDxAiExtendedRouteEnabled,
} from "@/lib/ai/provider-boundary";

const openai = createOpenAI({
  apiKey: process.env.AI_PROVIDER_API_KEY,
});"#,
            r#"if (!isDxAiExtendedRouteEnabled()) {
  return createDxAiExtendedRouteDisabledResponse({
    route: "/api/ai/image",
    provider: "openai-compatible",
    capability: "image-generation",
    requiredEnv: "AI_PROVIDER_API_KEY",
    credentialsConfigured: Boolean(process.env.AI_PROVIDER_API_KEY),
    appOwnedBoundary: "Enable after policy review.",
  });
}

if (!process.env.AI_PROVIDER_API_KEY) {
  return createDxAiMissingProviderResponse({
    provider: "openai-compatible",
    capability: "image-generation",
    requiredEnv: "AI_PROVIDER_API_KEY",
    appOwnedBoundary: "Set AI_PROVIDER_API_KEY before generating launch image assets.",
  });
}"#,
            &DxReactRouteHandlerRequest {
                method: "POST".to_string(),
                path: "/api/ai/image".to_string(),
                headers: BTreeMap::new(),
                body: json!({
                    "prompt": "Source-owned image route readiness",
                }),
                route_params: BTreeMap::new(),
                search_params: BTreeMap::new(),
                runtime_env: BTreeMap::from([
                    (
                        "DX_ENABLE_EXTENDED_AI_ROUTES".to_string(),
                        "true".to_string(),
                    ),
                    (
                        "AI_PROVIDER_API_KEY".to_string(),
                        "redacted-present".to_string(),
                    ),
                ]),
            },
        )
        .expect("enabled image route boundary response");

        assert_eq!(response.status, 202);
        assert_eq!(response.body["schema"], "dx.ai.provider_readiness");
        assert_eq!(response.body["ok"], true);
        assert_eq!(
            response.body["status"],
            "provider-configured-readiness-only"
        );
        assert_eq!(
            response.body["providerBoundary"], true,
            "provider boundary stays visible even after credentials are present"
        );
        assert_eq!(response.body["providerConfigured"], true);
        assert_eq!(response.body["runtimeExecution"], false);
        assert_eq!(response.body["liveProviderExecution"], false);
        assert_eq!(response.body["liveProviderProof"], false);
        assert_eq!(response.body["providerRuntime"], false);
        assert_eq!(
            response.body["secretValues"].as_array().map(Vec::len),
            Some(0)
        );
    }
}
