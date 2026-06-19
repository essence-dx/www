#![allow(clippy::too_many_arguments)]
use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::route_handler_ai::{
    ai_factory_route_handler_response, ai_local_stream_route_handler_response,
    ai_provider_boundary_route_handler_response,
};
use super::route_handler_automations::automation_n8n_dry_run_route_handler_response;
use super::route_handler_body_boundary::{
    RouteHandlerUnsupportedBodyRead, route_handler_unsupported_body_alias_read,
    route_handler_unsupported_body_aliases, route_handler_unsupported_body_read_message,
    route_handler_unsupported_request_body_read,
};
use super::route_handler_compat::{
    data_fetching_cache_action_route_handler_response,
    data_fetching_cache_readiness_route_handler_response,
    database_api_readiness_route_handler_response, instant_route_handler_compat_response,
    template_better_auth_readiness_route_handler_response,
    template_better_auth_route_handler_response,
    template_better_auth_session_route_handler_response,
};
use super::route_handler_database_orm::database_orm_readiness_route_handler_response;
use super::route_handler_fumadocs::{
    fumadocs_llms_route_handler_response, fumadocs_openapi_proxy_route_handler_response,
    fumadocs_search_route_handler_response,
};
use super::route_handler_http_json::http_json_route_handler_response;
use super::route_handler_instant_readiness::instant_readiness_route_handler_response;
use super::route_handler_payments::{
    payments_stripe_checkout_route_handler_response,
    payments_stripe_readiness_route_handler_response,
    payments_stripe_webhook_route_handler_response,
};
use super::route_handler_supabase::supabase_readiness_route_handler_response;
use super::tsx_ast::parse_tsx_module;

const ROUTE_HANDLER_FILENAMES: &[&str] = &["route.ts", "route.tsx", "route.js", "route.jsx"];
const APP_ROUTE_HANDLER_ROOTS: &[&str] = &["app/", "src/app/"];

/// Source file that can produce a DX-WWW server contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactServerSource {
    /// Server file kind.
    pub kind: DxReactServerSourceKind,
    /// Project-relative source path.
    pub source_path: String,
    /// Raw TypeScript source.
    pub source: String,
}

/// Supported React-shaped server source kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxReactServerSourceKind {
    /// `app/api/**/route.ts` route handler.
    RouteHandler,
    /// `server/loaders.ts` exported loader function.
    Loader,
    /// `server/actions.ts` exported server action function.
    Action,
}

/// Analyze-only server contract for one source-owned server file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactServerContract {
    /// Server file kind.
    pub kind: DxReactServerSourceKind,
    /// Project-relative source path.
    pub source_path: String,
    /// Route endpoint for `app/api/**/route.ts` files.
    pub endpoint: Option<String>,
    /// Exported callable server functions.
    pub exports: Vec<DxReactServerExport>,
    /// Execution posture used while generating this contract.
    pub execution_model: String,
    /// Whether package lifecycle scripts were executed while producing this contract.
    pub lifecycle_scripts_executed: bool,
}

/// One exported callable in a server contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactServerExport {
    /// Export name.
    pub name: String,
    /// HTTP method for route handlers.
    pub http_method: Option<String>,
    /// Whether the export is declared async.
    pub async_export: bool,
    /// Request serialization policy for compiler/runtime handoff.
    pub request_serialization: String,
    /// Response serialization policy for compiler/runtime handoff.
    pub response_serialization: String,
}

/// Minimal source-owned validation schema for a compiled server action boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactServerActionSchema {
    /// Validation mode selected by the compiler.
    pub mode: String,
    /// Stable hash of the schema shape. The source text is not stored in receipts.
    pub source_hash: String,
    /// Object fields expected by this schema.
    #[serde(default)]
    pub fields: Vec<DxReactServerActionSchemaField>,
}

/// One validated object field in a server-action schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactServerActionSchemaField {
    /// Field name.
    pub name: String,
    /// DX-WWW validation type.
    pub value_type: String,
    /// Whether the field must be present.
    pub required: bool,
    /// Allowed string literal values for union fields.
    #[serde(default)]
    pub allowed_values: Vec<String>,
}

/// Request passed to the safe route-handler interpreter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactRouteHandlerRequest {
    /// HTTP method.
    pub method: String,
    /// Request path.
    pub path: String,
    /// Lowercase HTTP request headers.
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
    /// Parsed JSON request body or raw text body projected into the safe interpreter.
    #[serde(default)]
    pub body: Value,
    /// Dynamic App Router route params captured by the DX route matcher.
    #[serde(default)]
    pub route_params: BTreeMap<String, String>,
    /// Query params captured by the DX route matcher.
    #[serde(default)]
    pub search_params: BTreeMap<String, String>,
    /// Allowlisted runtime environment values projected into safe interpreters.
    #[serde(default)]
    pub runtime_env: BTreeMap<String, String>,
}

impl DxReactRouteHandlerRequest {
    pub(crate) fn path_for_match(&self) -> &str {
        route_request_path_for_match(&self.path)
    }

    pub(crate) fn query_params(&self) -> BTreeMap<String, String> {
        route_request_search_params(self)
    }
}

/// Response produced by the safe route-handler interpreter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DxReactRouteHandlerResponse {
    /// HTTP status code.
    pub status: u16,
    /// Response content type.
    pub content_type: String,
    /// Safe response headers emitted by the route handler.
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
    /// Redirect target when the handler returns a redirect helper.
    #[serde(default)]
    pub redirect_url: Option<String>,
    /// JSON response body.
    pub body: Value,
    /// Execution posture used for this response.
    pub execution_model: String,
    /// Whether package lifecycle scripts were executed.
    pub lifecycle_scripts_executed: bool,
}

/// Compiled invocation protocol for a source-owned server action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactServerActionProtocol {
    /// Project-relative source path.
    pub source_path: String,
    /// Exported action function name.
    pub export_name: String,
    /// Stable source action identifier.
    pub action_id: String,
    /// Internal endpoint used by compiled client event calls.
    pub endpoint: String,
    /// Request serialization policy.
    pub request_serialization: String,
    /// Response serialization policy.
    pub response_serialization: String,
    /// Runtime request validation schema compiled from the action signature.
    pub request_schema: DxReactServerActionSchema,
    /// Runtime response validation schema compiled from the action return shape.
    pub response_schema: DxReactServerActionSchema,
    /// CSRF hook posture.
    pub csrf_hook: String,
    /// Session hook posture.
    pub session_hook: String,
    /// Replay protection posture.
    pub replay_protection: String,
    /// Receipt policy for mutation calls.
    pub receipt_policy: String,
    /// Execution posture used while generating this protocol.
    pub execution_model: String,
    /// Whether package lifecycle scripts were executed while producing this protocol.
    pub lifecycle_scripts_executed: bool,
}

/// Request passed to the safe server-action interpreter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DxReactServerActionRequest {
    /// Stable action identifier from a compiled protocol.
    pub action_id: String,
    /// JSON payload supplied by the compiled client event.
    pub payload: Value,
    /// CSRF token from the app/session boundary.
    pub csrf_token: Option<String>,
    /// Session identifier from the app auth boundary.
    pub session_id: Option<String>,
    /// Idempotency key that makes retries replay-safe.
    pub idempotency_key: Option<String>,
}

/// Receipt proving how a server action was invoked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactServerActionReceipt {
    /// Stable receipt identifier.
    pub receipt_id: String,
    /// Stable action identifier from the compiled protocol.
    pub action_id: String,
    /// Project-relative source path.
    pub source_path: String,
    /// Exported action function name.
    pub export_name: String,
    /// Hash of the session identifier; never stores the raw session value.
    pub session_hash: String,
    /// Hash of the idempotency key; never stores the raw key value.
    pub idempotency_key_hash: String,
    /// Hash of the JSON payload.
    pub payload_hash: String,
    /// Hash of the JSON response body.
    pub response_hash: String,
    /// Hash of the request validation schema.
    pub request_schema_hash: String,
    /// Hash of the response validation schema.
    pub response_schema_hash: String,
    /// Whether request payload validation ran and passed.
    pub request_validated: bool,
    /// Whether response payload validation ran and passed.
    pub response_validated: bool,
    /// Redacted validation findings. Raw payload/session values are never stored.
    #[serde(default)]
    pub validation_errors: Vec<String>,
    /// Whether this invocation satisfied replay-safety requirements.
    pub replay_safe: bool,
}

/// Response produced by the safe server-action interpreter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DxReactServerActionResponse {
    /// Stable action identifier from the compiled protocol.
    pub action_id: String,
    /// JSON response body.
    pub body: Value,
    /// Replay-safe invocation receipt.
    pub receipt: DxReactServerActionReceipt,
    /// Execution posture used for this response.
    pub execution_model: String,
    /// Whether package lifecycle scripts were executed.
    pub lifecycle_scripts_executed: bool,
}

/// Response produced by the safe loader interpreter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DxReactServerLoaderResponse {
    /// Project-relative source path.
    pub source_path: String,
    /// Exported loader function name.
    pub export_name: String,
    /// JSON response body.
    pub body: Value,
    /// Hash of the JSON response body.
    pub response_hash: String,
    /// Execution posture used for this response.
    pub execution_model: String,
    /// Whether package lifecycle scripts were executed.
    pub lifecycle_scripts_executed: bool,
}

/// Route-local server data manifest emitted for async server component pages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DxReactServerDataManifest {
    /// Manifest schema version.
    pub version: u32,
    /// Route path that requested the loader data.
    pub route: String,
    /// Project-relative App Router page source path.
    pub route_source_path: String,
    /// Whether this route data path requires `node_modules`.
    pub node_modules_required: bool,
    /// Whether package lifecycle scripts were executed.
    pub lifecycle_scripts_executed: bool,
    /// Loader outputs resolved for this route.
    pub entries: Vec<DxReactServerDataEntry>,
}

/// One evaluated loader binding used by a route.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DxReactServerDataEntry {
    /// Local variable that receives the loader value in the route source.
    pub binding: String,
    /// Local callable name used in the route source.
    pub local_name: String,
    /// Exported loader function name.
    pub export_name: String,
    /// Project-relative loader source path.
    pub source_path: String,
    /// Evaluated loader value.
    pub value: Value,
    /// Stable hash of the evaluated value.
    pub value_hash: String,
    /// Execution posture used for the evaluation.
    pub execution_model: String,
    /// Whether package lifecycle scripts were executed.
    pub lifecycle_scripts_executed: bool,
}

/// Compile source-owned server files into explicit DX-WWW server contracts.
pub fn compile_react_server_contracts(
    sources: &[DxReactServerSource],
) -> Vec<DxReactServerContract> {
    sources
        .iter()
        .map(|source| DxReactServerContract {
            kind: source.kind,
            source_path: source.source_path.clone(),
            endpoint: match source.kind {
                DxReactServerSourceKind::RouteHandler => route_endpoint(&source.source_path),
                DxReactServerSourceKind::Loader | DxReactServerSourceKind::Action => None,
            },
            exports: server_exports(source),
            execution_model: "analyze-only".to_string(),
            lifecycle_scripts_executed: false,
        })
        .collect()
}

/// Evaluate loaders referenced by one async server component route.
pub fn compile_react_server_data_manifest(
    route: &str,
    route_source_path: &str,
    route_source: &str,
    server_sources: &[DxReactServerSource],
) -> Result<DxReactServerDataManifest, String> {
    let imports = route_loader_imports(route_source);
    let calls = route_loader_calls(route_source);
    let mut entries = Vec::new();

    for call in calls {
        let Some(import) = imports
            .iter()
            .find(|import| import.local_name == call.local_name)
        else {
            continue;
        };
        let Some(source) = server_sources.iter().find(|source| {
            source.kind == DxReactServerSourceKind::Loader
                && source.source_path.replace('\\', "/") == import.source_path
        }) else {
            return Err(format!(
                "route `{route_source_path}` imports missing loader source `{}`",
                import.source_path
            ));
        };
        let response = execute_react_server_loader(source, &import.export_name)?;
        entries.push(DxReactServerDataEntry {
            binding: call.binding,
            local_name: import.local_name.clone(),
            export_name: response.export_name,
            source_path: response.source_path,
            value: response.body,
            value_hash: response.response_hash,
            execution_model: response.execution_model,
            lifecycle_scripts_executed: response.lifecycle_scripts_executed,
        });
    }

    entries.sort_by(|left, right| {
        left.binding
            .cmp(&right.binding)
            .then(left.source_path.cmp(&right.source_path))
            .then(left.export_name.cmp(&right.export_name))
    });

    Ok(DxReactServerDataManifest {
        version: 1,
        route: route.to_string(),
        route_source_path: route_source_path.to_string(),
        node_modules_required: false,
        lifecycle_scripts_executed: false,
        entries,
    })
}

/// Execute a supported `server/loaders.ts` export through the safe DX-WWW interpreter.
pub fn execute_react_server_loader(
    source: &DxReactServerSource,
    export_name: &str,
) -> Result<DxReactServerLoaderResponse, String> {
    if source.kind != DxReactServerSourceKind::Loader {
        return Err("source is not a server loader file".to_string());
    }
    if !exported_functions(&source.source)
        .iter()
        .any(|export| export.name == export_name)
    {
        return Err(format!("server loader does not export `{export_name}`"));
    }
    let body = exported_function_body(&source.source, export_name)
        .ok_or_else(|| format!("server loader `{export_name}` must be an exported function"))?;
    let return_object = returned_object_literal(body).ok_or_else(|| {
        "server loader must return an object literal for the stable DX server contract".to_string()
    })?;
    let response_body = Value::Object(parse_object_literal(return_object)?);
    Ok(DxReactServerLoaderResponse {
        source_path: source.source_path.clone(),
        export_name: export_name.to_string(),
        response_hash: short_hash(&stable_json(&response_body)),
        body: response_body,
        execution_model: "source-owned-safe-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

/// Compile source-owned server actions into client invocation protocols.
pub fn compile_react_server_action_protocols(
    sources: &[DxReactServerSource],
) -> Vec<DxReactServerActionProtocol> {
    sources
        .iter()
        .filter(|source| source.kind == DxReactServerSourceKind::Action)
        .flat_map(|source| {
            exported_functions(&source.source)
                .into_iter()
                .map(move |export| server_action_protocol(source, export))
        })
        .collect()
}

/// Execute a supported `app/api/**/route.ts` handler through the safe DX-WWW interpreter.
pub fn execute_react_route_handler(
    source: &DxReactServerSource,
    request: DxReactRouteHandlerRequest,
) -> Result<DxReactRouteHandlerResponse, String> {
    if source.kind != DxReactServerSourceKind::RouteHandler {
        return Err("source is not a route handler".to_string());
    }
    if !route_endpoint(&source.source_path)
        .as_deref()
        .is_some_and(|endpoint| route_endpoint_matches(endpoint, &request.path))
    {
        return Err("route handler path does not match request path".to_string());
    }
    if http_method(&request.method).is_none() {
        return Err(format!(
            "unsupported route handler method `{}`",
            request.method
        ));
    }

    let export_method = route_handler_export_method(&source.source, &request.method);
    let Some(body) = exported_route_handler_body(&source.source, export_method) else {
        if let Some(response) =
            template_better_auth_route_handler_response(&source.source, &request)
        {
            return Ok(response);
        }
        if let Some(response) =
            exported_const_route_handler_boundary(&source.source, export_method, &request)
        {
            return Ok(response);
        }
        if request.method == "OPTIONS" {
            if let Some(response) = automatic_route_handler_options_response(&source.source) {
                return Ok(response);
            }
        }
        if let Some(response) =
            route_handler_method_not_allowed_response(&source.source, &request.method)
        {
            return Ok(response);
        }
        return Err(route_handler_missing_export_message(&request.method));
    };
    if let Some(response) = parse_route_handler_not_found_response(body)? {
        return Ok(response);
    }
    if let Some(response) = parse_route_handler_redirect_response(body, &request)? {
        return Ok(response);
    }
    if let Some(response) = http_json_route_handler_response(&source.source, body, &request) {
        return Ok(response);
    }
    if let Some(response) =
        database_api_readiness_route_handler_response(&source.source, body, &request)
    {
        return Ok(response);
    }
    if let Some(response) =
        database_orm_readiness_route_handler_response(&source.source, body, &request)
    {
        return Ok(response);
    }
    if let Some(response) =
        supabase_readiness_route_handler_response(&source.source, body, &request)
    {
        return Ok(response);
    }
    if let Some(response) = instant_readiness_route_handler_response(&source.source, body, &request)
    {
        return Ok(response);
    }
    if let Some(response) =
        data_fetching_cache_readiness_route_handler_response(&source.source, body, &request)
    {
        return Ok(response);
    }
    if let Some(response) =
        data_fetching_cache_action_route_handler_response(&source.source, body, &request)
    {
        return Ok(response);
    }
    if let Some(response) =
        template_better_auth_readiness_route_handler_response(&source.source, body, &request)
    {
        return Ok(response);
    }
    if let Some(response) =
        template_better_auth_session_route_handler_response(&source.source, body, &request)
    {
        return Ok(response);
    }
    if let Some(response) =
        automation_n8n_dry_run_route_handler_response(&source.source, body, &request)
    {
        return Ok(response);
    }
    if let Some(response) = ai_local_stream_route_handler_response(&source.source, body, &request) {
        return Ok(response);
    }
    if let Some(response) =
        ai_provider_boundary_route_handler_response(&source.source, body, &request)
    {
        return Ok(response);
    }
    if let Some(response) =
        payments_stripe_checkout_route_handler_response(&source.source, body, &request)
    {
        return Ok(response);
    }
    if let Some(response) =
        payments_stripe_readiness_route_handler_response(&source.source, body, &request)
    {
        return Ok(response);
    }
    if let Some(response) =
        payments_stripe_webhook_route_handler_response(&source.source, body, &request)
    {
        return Ok(response);
    }
    if let Some(response) = fumadocs_llms_route_handler_response(&source.source, body, &request) {
        return Ok(response);
    }
    let context_bindings = route_handler_context_bindings(&source.source, export_method, body);
    if let Some(response) =
        parse_route_handler_conditional_response(body, &request, &context_bindings)?
    {
        return Ok(response);
    }
    if let Some(response) = parse_route_handler_web_response(body, &request, &context_bindings)? {
        return Ok(response);
    }
    if let Some(response) = parse_route_handler_json_response(body, &request, &context_bindings)? {
        return Ok(response);
    }

    let return_object = returned_object_literal(body).ok_or_else(|| {
        "route handler must return an object literal for the stable DX route contract".to_string()
    })?;
    let json_body =
        parse_route_handler_object_literal(return_object, body, &request, &context_bindings)?;
    let status = json_body
        .get("status")
        .and_then(Value::as_u64)
        .and_then(|status| u16::try_from(status).ok())
        .unwrap_or(200);

    Ok(DxReactRouteHandlerResponse {
        status,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::new(),
        redirect_url: None,
        body: Value::Object(json_body),
        execution_model: "source-owned-safe-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

/// Execute a supported `server/actions.ts` export through the safe DX-WWW interpreter.
pub fn execute_react_server_action(
    source: &DxReactServerSource,
    request: DxReactServerActionRequest,
) -> Result<DxReactServerActionResponse, String> {
    if source.kind != DxReactServerSourceKind::Action {
        return Err("source is not a server action file".to_string());
    }
    let (source_path, export_name) = request
        .action_id
        .split_once('#')
        .ok_or_else(|| "server action id must use `source#export`".to_string())?;
    if source_path != source.source_path {
        return Err("server action source path does not match request action id".to_string());
    }
    if !exported_functions(&source.source)
        .iter()
        .any(|export| export.name == export_name)
    {
        return Err(format!("server action does not export `{export_name}`"));
    }

    let _csrf_token = non_empty_secret(request.csrf_token.as_deref(), "csrf token")?;
    let session_id = non_empty_secret(request.session_id.as_deref(), "session id")?;
    let idempotency_key = non_empty_secret(request.idempotency_key.as_deref(), "idempotency key")?;
    let body = exported_function_body(&source.source, export_name)
        .ok_or_else(|| format!("server action `{export_name}` must be an exported function"))?;
    let request_schema = action_request_schema_for_export(&source.source, export_name);
    let request_errors = validate_schema_value(&request.payload, &request_schema, "payload");
    if !request_errors.is_empty() {
        return Err(format!(
            "server action request validation failed: {}",
            request_errors.join("; ")
        ));
    }
    let return_object = returned_object_literal(body).ok_or_else(|| {
        "server action must return an object literal for the stable DX server contract".to_string()
    })?;
    let response_body = Value::Object(parse_object_literal(return_object)?);
    let response_schema = action_response_schema_for_export(&source.source, export_name);
    let response_errors = validate_schema_value(&response_body, &response_schema, "response");
    if !response_errors.is_empty() {
        return Err(format!(
            "server action response validation failed: {}",
            response_errors.join("; ")
        ));
    }
    let receipt = server_action_receipt(
        &request.action_id,
        source_path,
        export_name,
        &request.payload,
        &response_body,
        session_id,
        idempotency_key,
        &request_schema,
        &response_schema,
    );

    Ok(DxReactServerActionResponse {
        action_id: request.action_id,
        body: response_body,
        receipt,
        execution_model: "source-owned-safe-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

fn server_exports(source: &DxReactServerSource) -> Vec<DxReactServerExport> {
    exported_functions(&source.source)
        .into_iter()
        .filter(|export| match source.kind {
            DxReactServerSourceKind::RouteHandler => http_method(&export.name).is_some(),
            DxReactServerSourceKind::Loader | DxReactServerSourceKind::Action => true,
        })
        .map(|export| DxReactServerExport {
            http_method: http_method(&export.name).map(str::to_string),
            request_serialization: request_serialization(&export.parameters),
            response_serialization: response_serialization(&source.source),
            name: export.name,
            async_export: export.async_export,
        })
        .collect()
}

fn server_action_protocol(
    source: &DxReactServerSource,
    export: ExportedFunction,
) -> DxReactServerActionProtocol {
    let action_id = server_action_id(&source.source_path, &export.name);
    let request_schema = action_request_schema(&export.parameters);
    let response_schema = action_response_schema(source, &export.name);
    let response_serialization = if response_schema.mode == "typed-object" {
        "typed-json-object".to_string()
    } else {
        response_serialization(&source.source)
    };
    DxReactServerActionProtocol {
        source_path: source.source_path.clone(),
        export_name: export.name,
        endpoint: server_action_endpoint(&action_id),
        action_id,
        request_serialization: request_serialization(&export.parameters),
        response_serialization,
        request_schema,
        response_schema,
        csrf_hook: "required".to_string(),
        session_hook: "required".to_string(),
        replay_protection: "idempotency-key".to_string(),
        receipt_policy: "hashes-source-session-payload-response".to_string(),
        execution_model: "protocol-only".to_string(),
        lifecycle_scripts_executed: false,
    }
}

fn action_request_schema_for_export(source: &str, export_name: &str) -> DxReactServerActionSchema {
    exported_functions(source)
        .into_iter()
        .find(|export| export.name == export_name)
        .map(|export| action_request_schema(&export.parameters))
        .unwrap_or_else(json_value_schema)
}

fn action_request_schema(parameters: &str) -> DxReactServerActionSchema {
    let Some(first_parameter) = split_top_level_entries(parameters).first().copied() else {
        return json_value_schema();
    };
    let Some((_, type_source)) = first_parameter.split_once(':') else {
        return json_value_schema();
    };
    typed_object_schema_from_type(type_source.trim()).unwrap_or_else(json_value_schema)
}

fn action_response_schema_for_export(source: &str, export_name: &str) -> DxReactServerActionSchema {
    action_response_schema(
        &DxReactServerSource {
            kind: DxReactServerSourceKind::Action,
            source_path: String::new(),
            source: source.to_string(),
        },
        export_name,
    )
}

fn action_response_schema(
    source: &DxReactServerSource,
    export_name: &str,
) -> DxReactServerActionSchema {
    let Some(body) = exported_function_body(&source.source, export_name) else {
        return json_value_schema();
    };
    let Some(return_object) = returned_object_literal(body) else {
        return json_value_schema();
    };
    let fields = response_schema_fields(return_object);
    if fields.is_empty() {
        json_value_schema()
    } else {
        action_schema("typed-object", fields)
    }
}

fn typed_object_schema_from_type(type_source: &str) -> Option<DxReactServerActionSchema> {
    let source = type_source.trim();
    let object = if source.starts_with('{') {
        strip_object_braces(source)?
    } else {
        return None;
    };
    let fields = type_schema_fields(object);
    (!fields.is_empty()).then(|| action_schema("typed-object", fields))
}

fn action_schema(
    mode: &str,
    fields: Vec<DxReactServerActionSchemaField>,
) -> DxReactServerActionSchema {
    let mut schema = DxReactServerActionSchema {
        mode: mode.to_string(),
        source_hash: String::new(),
        fields,
    };
    schema.source_hash = action_schema_hash(&schema);
    schema
}

fn json_value_schema() -> DxReactServerActionSchema {
    action_schema("json-value", Vec::new())
}

fn action_schema_hash(schema: &DxReactServerActionSchema) -> String {
    short_hash(&format!(
        "{}:{}",
        schema.mode,
        serde_json::to_string(&schema.fields).unwrap_or_default()
    ))
}

fn type_schema_fields(source: &str) -> Vec<DxReactServerActionSchemaField> {
    let mut fields = split_top_level_type_entries(source)
        .into_iter()
        .filter_map(type_schema_field)
        .collect::<Vec<_>>();
    fields.sort_by(|left, right| left.name.cmp(&right.name));
    fields
}

fn type_schema_field(entry: &str) -> Option<DxReactServerActionSchemaField> {
    let (name_source, type_source) = entry.split_once(':')?;
    let mut name = name_source.trim().trim_start_matches("readonly ").trim();
    let required = !name.ends_with('?');
    if !required {
        name = name.trim_end_matches('?').trim();
    }
    let name = name.trim_matches('"').trim_matches('\'');
    if name.is_empty() {
        return None;
    }
    let (value_type, allowed_values) = schema_type_from_ts(type_source.trim());
    Some(DxReactServerActionSchemaField {
        name: name.to_string(),
        value_type,
        required,
        allowed_values,
    })
}

fn schema_type_from_ts(source: &str) -> (String, Vec<String>) {
    let source = source.trim().trim_end_matches(';').trim();
    let string_literals = source
        .split('|')
        .filter_map(|part| parse_quoted_value(part.trim()).map(|(value, _)| value))
        .collect::<Vec<_>>();
    if !string_literals.is_empty() && string_literals.len() == source.split('|').count() {
        return ("string-literal-union".to_string(), string_literals);
    }
    let normalized = source.replace(' ', "");
    match normalized.as_str() {
        "string" => ("string".to_string(), Vec::new()),
        "number" => ("number".to_string(), Vec::new()),
        "boolean" => ("boolean".to_string(), Vec::new()),
        "string[]" | "Array<string>" => ("array<string>".to_string(), Vec::new()),
        "number[]" | "Array<number>" => ("array<number>".to_string(), Vec::new()),
        "boolean[]" | "Array<boolean>" => ("array<boolean>".to_string(), Vec::new()),
        _ => ("json-value".to_string(), Vec::new()),
    }
}

fn response_schema_fields(source: &str) -> Vec<DxReactServerActionSchemaField> {
    let mut fields = split_top_level_entries(source)
        .into_iter()
        .filter_map(|entry| {
            let (key, value) = entry.split_once(':')?;
            let name = key.trim().trim_matches('"').trim_matches('\'');
            if name.is_empty() {
                return None;
            }
            Some(DxReactServerActionSchemaField {
                name: name.to_string(),
                value_type: schema_type_from_value(value.trim()),
                required: true,
                allowed_values: Vec::new(),
            })
        })
        .collect::<Vec<_>>();
    fields.sort_by(|left, right| left.name.cmp(&right.name));
    fields
}

fn schema_type_from_value(source: &str) -> String {
    let source = source.trim().trim_end_matches(',').trim();
    if matches!(source, "true" | "false") {
        "boolean".to_string()
    } else if parse_quoted_value(source).is_some() {
        "string".to_string()
    } else if source.parse::<i64>().is_ok() || source.parse::<f64>().is_ok() {
        "number".to_string()
    } else if source.starts_with('[') {
        "array".to_string()
    } else if source.starts_with('{') {
        "object".to_string()
    } else {
        "json-value".to_string()
    }
}

fn validate_schema_value(
    value: &Value,
    schema: &DxReactServerActionSchema,
    label: &str,
) -> Vec<String> {
    if schema.mode != "typed-object" {
        return Vec::new();
    }
    let Some(object) = value.as_object() else {
        return vec![format!("{label} expected object")];
    };
    let mut errors = Vec::new();
    for field in &schema.fields {
        let field_label = format!("{label}.{}", field.name);
        let Some(field_value) = object.get(&field.name) else {
            if field.required {
                errors.push(format!("{field_label} is required"));
            }
            continue;
        };
        if field_value.is_null() && !field.required {
            continue;
        }
        if !schema_field_accepts_value(field, field_value) {
            errors.push(schema_field_error(field, &field_label));
        }
    }
    errors
}

fn schema_field_accepts_value(field: &DxReactServerActionSchemaField, value: &Value) -> bool {
    match field.value_type.as_str() {
        "string" => value.is_string(),
        "number" => value.is_number(),
        "boolean" => value.is_boolean(),
        "array<string>" => value
            .as_array()
            .is_some_and(|items| items.iter().all(Value::is_string)),
        "array<number>" => value
            .as_array()
            .is_some_and(|items| items.iter().all(Value::is_number)),
        "array<boolean>" => value
            .as_array()
            .is_some_and(|items| items.iter().all(Value::is_boolean)),
        "string-literal-union" => value
            .as_str()
            .is_some_and(|actual| field.allowed_values.iter().any(|allowed| allowed == actual)),
        "array" => value.is_array(),
        "object" => value.is_object(),
        "json-value" => true,
        _ => true,
    }
}

fn schema_field_error(field: &DxReactServerActionSchemaField, label: &str) -> String {
    match field.value_type.as_str() {
        "string-literal-union" => {
            format!("{label} expected one of allowed string literals")
        }
        value_type => format!("{label} expected {value_type}"),
    }
}

fn split_top_level_type_entries(source: &str) -> Vec<&str> {
    let mut entries = Vec::new();
    let mut start = 0usize;
    let mut cursor = 0usize;
    let mut quote = None;
    let mut depth = 0usize;
    while cursor < source.len() {
        let ch = source[cursor..].chars().next().unwrap_or_default();
        if let Some(active_quote) = quote {
            if ch == active_quote {
                quote = None;
            } else if ch == '\\' {
                cursor += ch.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..]
                        .chars()
                        .next()
                        .unwrap_or_default()
                        .len_utf8();
                    continue;
                }
            }
            cursor += ch.len_utf8();
            continue;
        }
        match ch {
            '"' | '\'' | '`' => quote = Some(ch),
            '{' | '[' | '(' | '<' => depth += 1,
            '}' | ']' | ')' | '>' => depth = depth.saturating_sub(1),
            ',' | ';' if depth == 0 => {
                let entry = source[start..cursor].trim();
                if !entry.is_empty() {
                    entries.push(entry);
                }
                start = cursor + ch.len_utf8();
            }
            _ => {}
        }
        cursor += ch.len_utf8();
    }
    let entry = source[start..].trim();
    if !entry.is_empty() {
        entries.push(entry);
    }
    entries
}

fn server_action_id(source_path: &str, export_name: &str) -> String {
    format!("{source_path}#{export_name}")
}

fn server_action_endpoint(action_id: &str) -> String {
    format!(
        "/.dx/actions/{}",
        short_hash(action_id).trim_start_matches("blake3:")
    )
}

fn server_action_receipt(
    action_id: &str,
    source_path: &str,
    export_name: &str,
    payload: &Value,
    response_body: &Value,
    session_id: &str,
    idempotency_key: &str,
    request_schema: &DxReactServerActionSchema,
    response_schema: &DxReactServerActionSchema,
) -> DxReactServerActionReceipt {
    let payload_hash = short_hash(&stable_json(payload));
    let response_hash = short_hash(&stable_json(response_body));
    let session_hash = short_hash(session_id);
    let idempotency_key_hash = short_hash(idempotency_key);
    let receipt_id = short_hash(&format!(
        "{action_id}:{payload_hash}:{response_hash}:{session_hash}:{idempotency_key_hash}"
    ));
    DxReactServerActionReceipt {
        receipt_id,
        action_id: action_id.to_string(),
        source_path: source_path.to_string(),
        export_name: export_name.to_string(),
        session_hash,
        idempotency_key_hash,
        payload_hash,
        response_hash,
        request_schema_hash: request_schema.source_hash.clone(),
        response_schema_hash: response_schema.source_hash.clone(),
        request_validated: true,
        response_validated: true,
        validation_errors: Vec::new(),
        replay_safe: true,
    }
}

fn non_empty_secret<'a>(value: Option<&'a str>, name: &str) -> Result<&'a str, String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("{name} is required for server action invocation"))
}

fn stable_json(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "null".to_string())
}

fn short_hash(value: &str) -> String {
    let hash = blake3::hash(value.as_bytes()).to_hex().to_string();
    format!("blake3:{}", &hash[..16])
}

fn exported_function_body<'a>(source: &'a str, name: &str) -> Option<&'a str> {
    let needle = format!("function {name}");
    let function_index = source.find(&needle)?;
    let parameters_start = source[function_index..]
        .find('(')
        .map(|offset| function_index + offset)?;
    let parameters_end = find_balanced_delimiter(source, parameters_start, '(', ')')?;
    let block_start = source[parameters_end..]
        .find('{')
        .map(|offset| parameters_end + offset)?;
    let block_end = find_balanced_block(source, block_start)?;
    Some(&source[block_start + 1..block_end])
}

fn exported_route_handler_body<'a>(source: &'a str, name: &str) -> Option<&'a str> {
    exported_function_body(source, name).or_else(|| exported_const_function_body(source, name))
}

fn automatic_route_handler_options_response(source: &str) -> Option<DxReactRouteHandlerResponse> {
    let allowed_methods = route_handler_allowed_methods(source);
    if allowed_methods.is_empty() {
        return None;
    }
    let allow_header = allowed_methods.join(", ");

    Some(DxReactRouteHandlerResponse {
        status: 200,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            ("Allow".to_string(), allow_header),
            (
                "x-dx-route-handler-options".to_string(),
                "source-owned-method-discovery".to_string(),
            ),
        ]),
        redirect_url: None,
        body: serde_json::json!({
            "ok": true,
            "schema": "dx.next.routeHandlerOptions",
            "allowedMethods": allowed_methods,
            "nodeModulesRequired": false,
            "runtimeBoundary": {
                "sourceOwned": true,
                "externalRuntimeRequired": false,
                "externalRuntimeExecuted": false,
            },
        }),
        execution_model: "source-owned-route-handler-options".to_string(),
        lifecycle_scripts_executed: false,
    })
}

fn route_handler_method_not_allowed_response(
    source: &str,
    request_method: &str,
) -> Option<DxReactRouteHandlerResponse> {
    let allowed_methods = route_handler_allowed_methods(source);
    if allowed_methods.is_empty() || allowed_methods.contains(&request_method) {
        return None;
    }
    let allow_header = allowed_methods.join(", ");

    Some(DxReactRouteHandlerResponse {
        status: 405,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([("Allow".to_string(), allow_header)]),
        redirect_url: None,
        body: serde_json::json!({
            "ok": false,
            "schema": "dx.next.routeHandlerMethodGuard",
            "method": request_method,
            "methodNotAllowed": true,
            "allowedMethods": allowed_methods,
            "nodeModulesRequired": false,
            "runtimeBoundary": {
                "sourceOwned": true,
                "externalRuntimeRequired": false,
                "externalRuntimeExecuted": false,
            },
        }),
        execution_model: "source-owned-route-handler-method-guard".to_string(),
        lifecycle_scripts_executed: false,
    })
}

fn route_handler_allowed_methods(source: &str) -> Vec<&'static str> {
    let has_get = route_handler_method_is_exported(source, "GET");
    let has_head = route_handler_method_is_exported(source, "HEAD");
    let mut methods = Vec::new();

    if has_get {
        push_route_handler_method(&mut methods, "GET");
    }
    if has_get || has_head {
        push_route_handler_method(&mut methods, "HEAD");
    }
    for method in ["POST", "PUT", "PATCH", "DELETE"] {
        if route_handler_method_is_exported(source, method) {
            push_route_handler_method(&mut methods, method);
        }
    }
    if !methods.is_empty() {
        push_route_handler_method(&mut methods, "OPTIONS");
    }

    methods
}

fn route_handler_method_is_exported(source: &str, method: &str) -> bool {
    exported_route_handler_body(source, method).is_some()
        || exported_const_handler_expression(source, method).is_some()
}

fn push_route_handler_method(methods: &mut Vec<&'static str>, method: &'static str) {
    if !methods.contains(&method) {
        methods.push(method);
    }
}

fn exported_const_function_body<'a>(source: &'a str, name: &str) -> Option<&'a str> {
    let expression = exported_const_handler_expression(source, name)?;
    let trimmed = expression.trim_start();
    let block_start = expression.find('{')?;
    let callable_prefix = &expression[..block_start];
    if !trimmed.starts_with("function")
        && !trimmed.starts_with("async function")
        && !callable_prefix.contains("=>")
    {
        return None;
    }
    let block_end = find_balanced_block(expression, block_start)?;
    Some(&expression[block_start + 1..block_end])
}

fn exported_const_route_handler_boundary(
    source: &str,
    name: &str,
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let expression = exported_const_handler_expression(source, name)?;
    if let Some(response) = http_json_route_handler_response(source, expression, request) {
        return Some(response);
    }
    if let Some(response) =
        fumadocs_openapi_proxy_route_handler_response(source, expression, request)
    {
        return Some(response);
    }
    if let Some(response) = fumadocs_search_route_handler_response(source, expression, request) {
        return Some(response);
    }
    if let Some(response) = ai_factory_route_handler_response(source, expression, request) {
        return Some(response);
    }
    let package_id = if expression.contains("createDxAIChatRoute")
        || expression.contains("createDxAgentRoute")
    {
        "ai/vercel-ai"
    } else if expression.contains("dxTrpcRouteHandler") {
        "api/trpc"
    } else if expression.contains("createDxInstantRouteHandlers") {
        "instantdb/react"
    } else {
        "app/route-handler"
    };
    if package_id == "api/trpc" {
        if let Some(response) = trpc_route_handler_compat_response(request) {
            return Some(response);
        }
    }
    if package_id == "instantdb/react" {
        if let Some(response) = instant_route_handler_compat_response(source, expression, request) {
            return Some(response);
        }
    }
    let export_kind = if destructured_const_export_expression(source, name).is_some() {
        "destructured-const"
    } else {
        "const"
    };
    let handler_expression = expression
        .split_whitespace()
        .take(6)
        .collect::<Vec<_>>()
        .join(" ");

    Some(DxReactRouteHandlerResponse {
        status: 501,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::new(),
        redirect_url: None,
        body: serde_json::json!({
            "ok": false,
            "status": "route-handler-boundary",
            "method": request.method,
            "path": request.path,
            "packageId": package_id,
            "exportKind": export_kind,
            "handlerExpression": handler_expression,
            "message": "DX-WWW detected this App Router route handler export, but the safe source-owned interpreter cannot execute the helper factory yet.",
            "next": "Materialize a source-owned function export or add a dedicated Forge interpreter for this handler factory before claiming runtime parity.",
            "lifecycleScriptsExecuted": false,
        }),
        execution_model: "source-owned-route-handler-boundary".to_string(),
        lifecycle_scripts_executed: false,
    })
}

fn trpc_route_handler_compat_response(
    request: &DxReactRouteHandlerRequest,
) -> Option<DxReactRouteHandlerResponse> {
    let path = route_request_path_for_match(&request.path);
    let procedure = path.trim_end_matches('/').strip_prefix("/api/trpc/")?;

    let body = match (request.method.as_str(), procedure) {
        ("GET", "health") => serde_json::json!({
            "status": "ok",
            "procedure": "health.query",
            "requestId": "dx-trpc-health-source-owned",
            "serverTime": "dx-www-source-owned-clock",
        }),
        ("GET", "launchReadiness") => serde_json::json!({
            "template": "dx-www",
            "api": "trpc",
            "ready": true,
            "procedure": "launchReadiness.query",
        }),
        ("GET", "launchEvents") => serde_json::json!({
            "items": [
                {
                    "id": "launch-001",
                    "event": "viewed",
                    "route": "/launch",
                    "summary": "Launch surface opened",
                },
                {
                    "id": "launch-002",
                    "event": "validated",
                    "route": "/launch",
                    "summary": "Template package checks passed",
                },
            ],
            "nextCursor": 2,
            "total": 4,
            "procedure": "launchEvents.query",
        }),
        ("POST", "launchEvent") => serde_json::json!({
            "accepted": true,
            "event": request
                .body
                .get("event")
                .and_then(Value::as_str)
                .unwrap_or("validated"),
            "route": request
                .body
                .get("route")
                .and_then(Value::as_str)
                .unwrap_or("/launch"),
            "requestId": "dx-trpc-launch-event-source-owned",
            "serverTime": "dx-www-source-owned-clock",
            "procedure": "launchEvent.mutation",
        }),
        _ => return None,
    };

    Some(DxReactRouteHandlerResponse {
        status: 200,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([
            (
                "x-dx-route-handler-receipt".to_string(),
                "dx.next.appRouteHandlerReceipt".to_string(),
            ),
            (
                "x-dx-trpc-compat".to_string(),
                "source-owned-safe-interpreter".to_string(),
            ),
        ]),
        redirect_url: None,
        body,
        execution_model: "source-owned-trpc-compat-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    })
}

fn exported_const_handler_expression<'a>(source: &'a str, name: &str) -> Option<&'a str> {
    let direct = format!("export const {name}");
    if let Some(start) = source.find(&direct) {
        let equals = source[start..].find('=').map(|offset| start + offset + 1)?;
        let end = find_statement_end(source, equals).unwrap_or(source.len());
        return Some(source[equals..end].trim());
    }

    destructured_const_export_expression(source, name)
}

fn destructured_const_export_expression<'a>(source: &'a str, name: &str) -> Option<&'a str> {
    let mut cursor = 0usize;
    while let Some(relative_start) = source[cursor..].find("export const") {
        let start = cursor + relative_start;
        let fields_start = skip_ascii_whitespace(source, start + "export const".len());
        if !source[fields_start..].starts_with('{') {
            cursor = fields_start.saturating_add(1);
            continue;
        }
        let Some(fields_end) = find_balanced_delimiter(source, fields_start, '{', '}') else {
            cursor = fields_start.saturating_add(1);
            continue;
        };
        let fields = &source[fields_start + 1..fields_end];
        if destructured_const_export_contains(fields, name) {
            let equals = source[fields_end..]
                .find('=')
                .map(|offset| fields_end + offset + 1)?;
            let end = find_statement_end(source, equals).unwrap_or(source.len());
            return Some(source[equals..end].trim());
        }
        cursor = fields_end.saturating_add(1);
    }

    None
}

fn destructured_const_export_contains(fields: &str, name: &str) -> bool {
    split_top_level_entries(fields).into_iter().any(|field| {
        let exported_name = field
            .split_once(':')
            .map(|(exported, _)| exported)
            .unwrap_or(field)
            .trim();
        exported_name == name
    })
}

fn skip_ascii_whitespace(source: &str, mut cursor: usize) -> usize {
    while cursor < source.len() && source.as_bytes()[cursor].is_ascii_whitespace() {
        cursor += 1;
    }
    cursor
}

fn find_statement_end(source: &str, mut cursor: usize) -> Option<usize> {
    let mut quote = None;
    let mut depth = 0usize;
    while cursor < source.len() {
        let ch = source[cursor..].chars().next()?;
        if let Some(active_quote) = quote {
            if ch == active_quote {
                quote = None;
            } else if ch == '\\' {
                cursor += ch.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..].chars().next()?.len_utf8();
                    continue;
                }
            }
            cursor += ch.len_utf8();
            continue;
        }
        match ch {
            '"' | '\'' | '`' => quote = Some(ch),
            '(' | '[' | '{' | '<' => depth += 1,
            ')' | ']' | '}' | '>' => depth = depth.saturating_sub(1),
            ';' if depth == 0 => return Some(cursor),
            _ => {}
        }
        cursor += ch.len_utf8();
    }
    None
}

fn returned_object_literal(function_body: &str) -> Option<&str> {
    let return_index = function_body.find("return")?;
    let object_start = function_body[return_index..]
        .find('{')
        .map(|offset| return_index + offset)?;
    let object_end = find_balanced_block(function_body, object_start)?;
    Some(&function_body[object_start + 1..object_end])
}

fn parse_object_literal(source: &str) -> Result<Map<String, Value>, String> {
    let mut map = Map::new();
    for entry in split_top_level_entries(source) {
        let Some((key, value)) = entry.split_once(':') else {
            continue;
        };
        let key = key.trim().trim_matches('"').trim_matches('\'').to_string();
        if key.is_empty() {
            continue;
        }
        map.insert(key, parse_literal_value(value.trim())?);
    }
    Ok(map)
}

fn parse_route_handler_object_literal(
    source: &str,
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
    context_bindings: &RouteHandlerContextBindings,
) -> Result<Map<String, Value>, String> {
    let body_aliases = route_handler_effective_body_aliases(function_body, context_bindings);
    let cookie_aliases = route_handler_cookie_aliases(function_body);
    let mut map = Map::new();
    for entry in split_top_level_entries(source) {
        let Some((key, value)) = entry.split_once(':') else {
            let key = entry.trim().trim_end_matches(',').trim();
            if !key.is_empty() {
                map.insert(
                    key.to_string(),
                    parse_route_handler_value(
                        key,
                        request,
                        &body_aliases,
                        &cookie_aliases,
                        context_bindings,
                    )?,
                );
            }
            continue;
        };
        let key = key.trim().trim_matches('"').trim_matches('\'').to_string();
        if key.is_empty() {
            continue;
        }
        map.insert(
            key,
            parse_route_handler_value(
                value.trim(),
                request,
                &body_aliases,
                &cookie_aliases,
                context_bindings,
            )?,
        );
    }
    Ok(map)
}

fn parse_route_handler_json_response(
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
    context_bindings: &RouteHandlerContextBindings,
) -> Result<Option<DxReactRouteHandlerResponse>, String> {
    let Some(args) = find_first_call_args(function_body, &["NextResponse.json", "Response.json"])
    else {
        return Ok(None);
    };
    let Some(body_arg) = args.first() else {
        return Err("json response helper requires a body argument".to_string());
    };
    let body_aliases = route_handler_effective_body_aliases(function_body, context_bindings);
    let cookie_aliases = route_handler_cookie_aliases(function_body);
    let body = parse_route_handler_json_response_body(
        body_arg,
        request,
        &body_aliases,
        &cookie_aliases,
        context_bindings,
    )?;
    let response_header_aliases = route_handler_response_header_aliases(function_body);
    let options = args
        .get(1)
        .and_then(|arg| strip_object_braces(arg))
        .map(|arg| {
            parse_route_handler_response_options(
                arg,
                &route_handler_response_number_aliases(function_body),
                &response_header_aliases,
            )
        })
        .transpose()?
        .unwrap_or_default();
    let content_type = options
        .headers
        .get("content-type")
        .cloned()
        .unwrap_or_else(|| "application/json; charset=utf-8".to_string());

    Ok(Some(DxReactRouteHandlerResponse {
        status: options.status.unwrap_or(200),
        content_type,
        headers: options.headers,
        redirect_url: None,
        body,
        execution_model: "source-owned-safe-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    }))
}

fn parse_route_handler_json_response_body(
    source: &str,
    request: &DxReactRouteHandlerRequest,
    body_aliases: &[String],
    cookie_aliases: &BTreeMap<String, RouteHandlerCookieAlias>,
    context_bindings: &RouteHandlerContextBindings,
) -> Result<Value, String> {
    if let Some(object) = strip_object_braces(source) {
        return parse_route_handler_object_literal_with_aliases(
            object,
            request,
            body_aliases,
            cookie_aliases,
            context_bindings,
        )
        .map(Value::Object);
    }
    parse_route_handler_value(
        source,
        request,
        body_aliases,
        cookie_aliases,
        context_bindings,
    )
}

fn parse_route_handler_conditional_response(
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
    context_bindings: &RouteHandlerContextBindings,
) -> Result<Option<DxReactRouteHandlerResponse>, String> {
    let Some(branch) = route_handler_first_conditional_return(function_body) else {
        return Ok(None);
    };
    let body_aliases = route_handler_effective_body_aliases(function_body, context_bindings);
    let cookie_aliases = route_handler_cookie_aliases(function_body);
    let Some(condition_matches) = route_handler_condition_value(
        branch.condition,
        request,
        &body_aliases,
        &cookie_aliases,
        context_bindings,
    )?
    else {
        return Ok(None);
    };
    if condition_matches {
        return parse_route_handler_response_from_source(branch.body, request, context_bindings);
    }
    let selected_source =
        if let Some(alternate_source) = route_handler_else_branch_source(branch.remainder) {
            if route_handler_source_starts_with_if(alternate_source) {
                return parse_route_handler_conditional_response(
                    alternate_source,
                    request,
                    context_bindings,
                );
            }
            alternate_source
        } else {
            branch.remainder
        };
    parse_route_handler_response_from_source(selected_source, request, context_bindings)
}

fn parse_route_handler_response_from_source(
    source: &str,
    request: &DxReactRouteHandlerRequest,
    context_bindings: &RouteHandlerContextBindings,
) -> Result<Option<DxReactRouteHandlerResponse>, String> {
    if let Some(response) =
        route_handler_nested_conditional_response(source, request, context_bindings)?
    {
        return Ok(Some(response));
    }
    if let Some(response) = parse_route_handler_web_response(source, request, context_bindings)? {
        return Ok(Some(response));
    }
    parse_route_handler_json_response(source, request, context_bindings)
}

fn route_handler_nested_conditional_response(
    source: &str,
    request: &DxReactRouteHandlerRequest,
    context_bindings: &RouteHandlerContextBindings,
) -> Result<Option<DxReactRouteHandlerResponse>, String> {
    if route_handler_source_starts_with_if(source) {
        return parse_route_handler_conditional_response(source, request, context_bindings);
    }
    Ok(None)
}

struct RouteHandlerConditionalReturn<'a> {
    condition: &'a str,
    body: &'a str,
    remainder: &'a str,
}

fn route_handler_first_conditional_return(
    source: &str,
) -> Option<RouteHandlerConditionalReturn<'_>> {
    let mut cursor = 0usize;
    while let Some(index) = source[cursor..].find("if") {
        let if_start = cursor + index;
        let after_if = if_start + 2;
        if !route_handler_keyword_boundary(source, if_start, after_if) {
            cursor = after_if;
            continue;
        }
        let condition_start = skip_ascii_whitespace(source, after_if);
        if !source[condition_start..].starts_with('(') {
            cursor = after_if;
            continue;
        }
        let condition_end = find_balanced_delimiter(source, condition_start, '(', ')')?;
        let block_start = skip_ascii_whitespace(source, condition_end + 1);
        if !source[block_start..].starts_with('{') {
            cursor = after_if;
            continue;
        }
        let block_end = find_balanced_block(source, block_start)?;
        let body = &source[block_start + 1..block_end];
        if !body.contains("return")
            || !(body.contains("Response.json")
                || body.contains("NextResponse.json")
                || body.contains("new Response"))
        {
            cursor = block_end + 1;
            continue;
        }
        return Some(RouteHandlerConditionalReturn {
            condition: &source[condition_start + 1..condition_end],
            body,
            remainder: &source[block_end + 1..],
        });
    }
    None
}

fn route_handler_else_branch_source(source: &str) -> Option<&str> {
    let else_start = skip_ascii_whitespace(source, 0);
    let else_end = else_start + "else".len();
    if else_end > source.len()
        || !source[else_start..].starts_with("else")
        || !route_handler_keyword_boundary(source, else_start, else_end)
    {
        return None;
    }
    let branch_start = skip_ascii_whitespace(source, else_end);
    let branch_source = &source[branch_start..];
    if route_handler_source_starts_with_if(branch_source) {
        return Some(branch_source);
    }
    if !branch_source.starts_with('{') {
        return None;
    }
    let block_end = find_balanced_block(source, branch_start)?;
    Some(&source[branch_start + 1..block_end])
}

fn route_handler_source_starts_with_if(source: &str) -> bool {
    let if_start = skip_ascii_whitespace(source, 0);
    let if_end = if_start + 2;
    if_end <= source.len()
        && source[if_start..].starts_with("if")
        && route_handler_keyword_boundary(source, if_start, if_end)
}

fn route_handler_keyword_boundary(source: &str, start: usize, end: usize) -> bool {
    let before = source[..start]
        .chars()
        .next_back()
        .map(route_handler_identifier_char)
        .unwrap_or(false);
    let after = source[end..]
        .chars()
        .next()
        .map(route_handler_identifier_char)
        .unwrap_or(false);
    !before && !after
}

fn route_handler_identifier_char(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '_' | '$')
}

fn route_handler_condition_value(
    source: &str,
    request: &DxReactRouteHandlerRequest,
    body_aliases: &[String],
    cookie_aliases: &BTreeMap<String, RouteHandlerCookieAlias>,
    context_bindings: &RouteHandlerContextBindings,
) -> Result<Option<bool>, String> {
    let expression = route_handler_condition_expression(source);
    for operator in ["||", "&&"] {
        if let Some(value) = route_handler_condition_logical_value(
            expression,
            operator,
            request,
            body_aliases,
            cookie_aliases,
            context_bindings,
        )? {
            return Ok(Some(value));
        }
    }
    for operator in ["!==", "==="] {
        let (left, right) = split_top_level_operator(expression, operator);
        if let Some(right) = right {
            return route_handler_condition_comparison_value(
                left,
                right,
                operator,
                request,
                body_aliases,
                cookie_aliases,
                context_bindings,
            );
        }
    }
    if let Some(inner) = expression.strip_prefix('!') {
        let Some(value) = route_handler_condition_value(
            inner,
            request,
            body_aliases,
            cookie_aliases,
            context_bindings,
        )?
        else {
            return Ok(None);
        };
        return Ok(Some(!value));
    }
    let value = route_handler_condition_operand(
        expression,
        request,
        body_aliases,
        cookie_aliases,
        context_bindings,
    )?;
    Ok(Some(route_handler_value_truthy(&value)))
}

fn route_handler_condition_comparison_value(
    left: &str,
    right: &str,
    operator: &str,
    request: &DxReactRouteHandlerRequest,
    body_aliases: &[String],
    cookie_aliases: &BTreeMap<String, RouteHandlerCookieAlias>,
    context_bindings: &RouteHandlerContextBindings,
) -> Result<Option<bool>, String> {
    let compared = if let Ok(right_value) = parse_literal_value(right) {
        let left_value = parse_route_handler_value(
            left,
            request,
            body_aliases,
            cookie_aliases,
            context_bindings,
        )?;
        Some((left_value, right_value))
    } else if let Ok(left_value) = parse_literal_value(left) {
        let right_value = parse_route_handler_value(
            right,
            request,
            body_aliases,
            cookie_aliases,
            context_bindings,
        )?;
        Some((left_value, right_value))
    } else {
        None
    };
    let Some((left_value, right_value)) = compared else {
        return Ok(None);
    };
    Ok(Some(if operator == "===" {
        left_value == right_value
    } else {
        left_value != right_value
    }))
}

fn route_handler_condition_expression(source: &str) -> &str {
    let mut expression = source.trim();
    while let Some(inner) = route_handler_outer_parenthesized_expression(expression) {
        expression = inner.trim();
    }
    expression
}

fn route_handler_outer_parenthesized_expression(source: &str) -> Option<&str> {
    let source = source.trim();
    if !source.starts_with('(') {
        return None;
    }
    let end = find_balanced_delimiter(source, 0, '(', ')')?;
    (end + 1 == source.len()).then(|| &source[1..end])
}

fn route_handler_condition_logical_value(
    source: &str,
    operator: &str,
    request: &DxReactRouteHandlerRequest,
    body_aliases: &[String],
    cookie_aliases: &BTreeMap<String, RouteHandlerCookieAlias>,
    context_bindings: &RouteHandlerContextBindings,
) -> Result<Option<bool>, String> {
    let (left, right) = split_top_level_operator(source, operator);
    let Some(right) = right else {
        return Ok(None);
    };
    let Some(left) = route_handler_condition_value(
        left,
        request,
        body_aliases,
        cookie_aliases,
        context_bindings,
    )?
    else {
        return Ok(None);
    };

    if operator == "||" && left {
        return Ok(Some(true));
    }
    if operator == "&&" && !left {
        return Ok(Some(false));
    }

    route_handler_condition_value(
        right,
        request,
        body_aliases,
        cookie_aliases,
        context_bindings,
    )
}

fn route_handler_condition_operand(
    source: &str,
    request: &DxReactRouteHandlerRequest,
    body_aliases: &[String],
    cookie_aliases: &BTreeMap<String, RouteHandlerCookieAlias>,
    context_bindings: &RouteHandlerContextBindings,
) -> Result<Value, String> {
    parse_route_handler_value(
        source.trim(),
        request,
        body_aliases,
        cookie_aliases,
        context_bindings,
    )
}

fn route_handler_value_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(value) => *value,
        Value::Number(value) => value.as_f64().map(|value| value != 0.0).unwrap_or(true),
        Value::String(value) => !value.is_empty(),
        Value::Array(_) | Value::Object(_) => true,
    }
}

fn parse_route_handler_web_response(
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
    context_bindings: &RouteHandlerContextBindings,
) -> Result<Option<DxReactRouteHandlerResponse>, String> {
    let Some(args) = find_first_call_args(function_body, &["new Response"]) else {
        return Ok(None);
    };
    let body_aliases = route_handler_effective_body_aliases(function_body, context_bindings);
    let cookie_aliases = route_handler_cookie_aliases(function_body);
    let body_arg = args.first().map(String::as_str).unwrap_or("null");
    let body = parse_route_handler_web_response_body(
        body_arg,
        request,
        &body_aliases,
        &cookie_aliases,
        context_bindings,
    )?;
    let response_header_aliases = route_handler_response_header_aliases(function_body);
    let options = args
        .get(1)
        .and_then(|arg| strip_object_braces(arg))
        .map(|arg| {
            parse_route_handler_response_options(
                arg,
                &route_handler_response_number_aliases(function_body),
                &response_header_aliases,
            )
        })
        .transpose()?
        .unwrap_or_default();
    let content_type =
        options
            .headers
            .get("content-type")
            .cloned()
            .unwrap_or_else(|| match &body {
                Value::Object(_) | Value::Array(_) => "application/json; charset=utf-8".to_string(),
                _ => "text/plain; charset=utf-8".to_string(),
            });

    Ok(Some(DxReactRouteHandlerResponse {
        status: options.status.unwrap_or(200),
        content_type,
        headers: options.headers,
        redirect_url: None,
        body,
        execution_model: "source-owned-safe-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    }))
}

fn parse_route_handler_web_response_body(
    source: &str,
    request: &DxReactRouteHandlerRequest,
    body_aliases: &[String],
    cookie_aliases: &BTreeMap<String, RouteHandlerCookieAlias>,
    context_bindings: &RouteHandlerContextBindings,
) -> Result<Value, String> {
    let source = source.trim();
    if source.is_empty() || matches!(source, "null" | "undefined") {
        return Ok(Value::Null);
    }
    if let Some(args) = find_first_call_args(source, &["JSON.stringify"]) {
        let Some(body_arg) = args.first() else {
            return Err("JSON.stringify response body requires an argument".to_string());
        };
        let object = strip_object_braces(body_arg)
            .ok_or_else(|| "JSON.stringify response body must be an object literal".to_string())?;
        return parse_route_handler_object_literal_with_aliases(
            object,
            request,
            body_aliases,
            cookie_aliases,
            context_bindings,
        )
        .map(Value::Object);
    }
    parse_route_handler_value(
        source,
        request,
        body_aliases,
        cookie_aliases,
        context_bindings,
    )
}

fn parse_route_handler_not_found_response(
    function_body: &str,
) -> Result<Option<DxReactRouteHandlerResponse>, String> {
    let Some(args) = find_first_call_args(function_body, &["notFound"]) else {
        return Ok(None);
    };
    if !args.is_empty() {
        return Err("notFound helper does not accept arguments".to_string());
    }

    Ok(Some(DxReactRouteHandlerResponse {
        status: 404,
        content_type: "application/json; charset=utf-8".to_string(),
        headers: BTreeMap::from([(
            "x-dx-route-handler-not-found".to_string(),
            "source-owned-safe-interpreter".to_string(),
        )]),
        redirect_url: None,
        body: serde_json::json!({
            "notFound": true,
            "status": 404,
            "nextHelper": "notFound",
        }),
        execution_model: "source-owned-safe-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    }))
}

fn parse_route_handler_redirect_response(
    function_body: &str,
    request: &DxReactRouteHandlerRequest,
) -> Result<Option<DxReactRouteHandlerResponse>, String> {
    let Some(args) = find_first_call_args(
        function_body,
        &["NextResponse.redirect", "Response.redirect", "redirect"],
    ) else {
        return Ok(None);
    };
    let Some(target_arg) = args.first() else {
        return Err("redirect helper requires a target argument".to_string());
    };
    let target = parse_redirect_target(
        target_arg,
        request,
        &route_handler_request_url_aliases(function_body),
    )?;
    let number_aliases = route_handler_response_number_aliases(function_body);
    let response_header_aliases = route_handler_response_header_aliases(function_body);
    let options = args
        .get(1)
        .map(|value| {
            parse_route_handler_redirect_options(value, &number_aliases, &response_header_aliases)
        })
        .transpose()?
        .unwrap_or_default();
    let status = options.status.unwrap_or(307);
    let mut headers = options.headers;
    headers.insert("location".to_string(), target.clone());

    Ok(Some(DxReactRouteHandlerResponse {
        status,
        content_type: "application/json; charset=utf-8".to_string(),
        headers,
        redirect_url: Some(target.clone()),
        body: serde_json::json!({
            "redirect": target,
            "status": status,
        }),
        execution_model: "source-owned-safe-interpreter".to_string(),
        lifecycle_scripts_executed: false,
    }))
}

fn parse_route_handler_redirect_options(
    source: &str,
    number_aliases: &BTreeMap<String, u16>,
    header_aliases: &BTreeMap<String, BTreeMap<String, String>>,
) -> Result<RouteHandlerResponseOptions, String> {
    let source = source.trim();
    if let Some(status) = parse_route_handler_response_status(source, number_aliases) {
        return Ok(RouteHandlerResponseOptions {
            status: Some(status),
            headers: BTreeMap::new(),
        });
    }
    let Some(options) = strip_object_braces(source) else {
        return Ok(RouteHandlerResponseOptions::default());
    };
    parse_route_handler_response_options(options, number_aliases, header_aliases)
}

fn parse_route_handler_object_literal_with_aliases(
    source: &str,
    request: &DxReactRouteHandlerRequest,
    body_aliases: &[String],
    cookie_aliases: &BTreeMap<String, RouteHandlerCookieAlias>,
    context_bindings: &RouteHandlerContextBindings,
) -> Result<Map<String, Value>, String> {
    let mut map = Map::new();
    for entry in split_top_level_entries(source) {
        let Some((key, value)) = entry.split_once(':') else {
            let key = entry.trim().trim_end_matches(',').trim();
            if !key.is_empty() {
                map.insert(
                    key.to_string(),
                    parse_route_handler_value(
                        key,
                        request,
                        body_aliases,
                        cookie_aliases,
                        context_bindings,
                    )?,
                );
            }
            continue;
        };
        let key = key.trim().trim_matches('"').trim_matches('\'').to_string();
        if key.is_empty() {
            continue;
        }
        map.insert(
            key,
            parse_route_handler_value(
                value.trim(),
                request,
                body_aliases,
                cookie_aliases,
                context_bindings,
            )?,
        );
    }
    Ok(map)
}

fn parse_route_handler_value(
    source: &str,
    request: &DxReactRouteHandlerRequest,
    body_aliases: &[String],
    cookie_aliases: &BTreeMap<String, RouteHandlerCookieAlias>,
    context_bindings: &RouteHandlerContextBindings,
) -> Result<Value, String> {
    let expression = source.trim().trim_end_matches(';').trim();
    if let Ok(value) = parse_literal_value(expression) {
        return Ok(value);
    }
    if context_bindings
        .method_aliases
        .iter()
        .any(|alias| expression == alias)
    {
        return Ok(Value::String(request.method.clone()));
    }
    if matches!(
        context_bindings
            .request_url_aliases
            .get(expression)
            .map(String::as_str),
        Some("path" | "url")
    ) {
        return Ok(Value::String(request.path.clone()));
    }
    match expression {
        "request.method" => return Ok(Value::String(request.method.clone())),
        "request.path" | "request.url" => return Ok(Value::String(request.path.clone())),
        "request.body" => return Ok(request.body.clone()),
        "request.params" => return Ok(serde_json::json!(request.route_params)),
        "request.searchParams" => {
            return Ok(serde_json::json!(route_request_search_params(request)));
        }
        _ => {}
    }
    if let Some(value) =
        route_handler_url_property_read(expression, request, &context_bindings.url_roots)
    {
        return Ok(value);
    }
    if let Some(property) = context_bindings.url_property_aliases.get(expression) {
        if let Some(value) = route_request_url_property_value(request, property) {
            return Ok(value);
        }
    }
    if let Some(path) = expression.strip_prefix("request.body.") {
        return Ok(json_path_value(&request.body, path));
    }
    if let Some(path) = expression.strip_prefix("request.params.") {
        return Ok(route_request_map_value(&request.route_params, path));
    }
    if let Some(path) = expression.strip_prefix("request.searchParams.") {
        return Ok(route_request_map_value(
            &route_request_search_params(request),
            path,
        ));
    }
    if let Some((param_name, default)) = route_handler_search_param_read(
        expression,
        &context_bindings.search_param_roots,
        &context_bindings.url_roots,
    ) {
        return Ok(route_search_param_value(request, &param_name, default));
    }
    if let Some(alias) = context_bindings.search_param_aliases.get(expression) {
        return Ok(route_search_param_value(
            request,
            &alias.param_name,
            alias.default.clone(),
        ));
    }
    if let Some((header_name, default)) =
        route_handler_header_read(expression, &context_bindings.header_roots)
    {
        return Ok(route_header_value(request, &header_name, default));
    }
    if let Some(alias) = context_bindings.header_aliases.get(expression) {
        return Ok(route_header_value(
            request,
            &alias.header_name,
            alias.default.clone(),
        ));
    }
    if let Some(value) = route_handler_context_map_value(
        expression,
        &context_bindings.route_param_roots,
        &request.route_params,
    ) {
        return Ok(value);
    }
    let effective_search_params = route_request_search_params(request);
    if let Some(value) = route_handler_context_map_value(
        expression,
        &context_bindings.search_param_roots,
        &effective_search_params,
    ) {
        return Ok(value);
    }
    if let Some(value) = route_handler_context_alias_value(
        expression,
        &context_bindings.route_param_aliases,
        &request.route_params,
    ) {
        return Ok(value);
    }
    if let Some(alias) = context_bindings.body_field_aliases.get(expression) {
        return Ok(json_path_value_or_missing(&request.body, &alias.field_name)
            .unwrap_or_else(|| alias.default.clone()));
    }
    if let Some(read) = route_handler_unsupported_body_alias_read(
        expression,
        &context_bindings.unsupported_body_aliases,
    ) {
        return Err(route_handler_unsupported_body_read_message(&read));
    }
    if let Some(value) = route_handler_form_data_read(
        expression,
        request,
        &context_bindings.form_data_aliases,
        &context_bindings.body_roots,
    ) {
        return Ok(value);
    }
    if let Some(read) =
        route_handler_unsupported_request_body_read(expression, &context_bindings.body_roots)
    {
        return Err(route_handler_unsupported_body_read_message(&read));
    }
    if let Some(value) =
        route_handler_request_body_read(expression, request, &context_bindings.body_roots)
    {
        return Ok(value);
    }
    if let Some(read) = route_handler_cookie_read(expression, &context_bindings.cookie_roots) {
        return Ok(cookie_read_value(request, &read));
    }
    if let Some(alias) = cookie_aliases.get(expression) {
        return Ok(cookie_read_value(request, alias));
    }
    if let Some(value) = route_handler_text_body_alias_value(
        expression,
        request,
        &context_bindings.text_body_aliases,
    ) {
        return Ok(value);
    }
    if let Some(value) = route_handler_body_alias_value(expression, request, body_aliases) {
        return Ok(value);
    }
    Err(format!("unsupported route handler literal `{expression}`"))
}

fn route_handler_text_body_alias_value(
    expression: &str,
    request: &DxReactRouteHandlerRequest,
    text_body_aliases: &[String],
) -> Option<Value> {
    let (read, default) = split_top_level_nullish(expression);
    let default = default.and_then(|value| parse_literal_value(value).ok());
    for alias in text_body_aliases {
        if route_handler_body_alias_matches(read, alias) {
            return Some(route_handler_text_body_value(request, default));
        }
        if route_handler_body_alias_path(read, alias).is_some() {
            return Some(default.unwrap_or(Value::Null));
        }
    }
    None
}

fn route_handler_body_alias_value(
    expression: &str,
    request: &DxReactRouteHandlerRequest,
    body_aliases: &[String],
) -> Option<Value> {
    let (read, default) = split_top_level_nullish(expression);
    let default = default.and_then(|value| parse_literal_value(value).ok());
    for alias in body_aliases {
        if route_handler_body_alias_matches(read, alias) {
            return Some(request.body.clone());
        }
        if let Some(path) = route_handler_body_alias_path(read, alias) {
            return Some(
                default
                    .clone()
                    .map(|default| {
                        json_path_value_or_missing(&request.body, &path).unwrap_or(default)
                    })
                    .unwrap_or_else(|| json_path_value(&request.body, &path)),
            );
        }
    }
    None
}

pub(super) fn route_handler_body_alias_matches(read: &str, alias: &str) -> bool {
    let Some(mut rest) = read.strip_prefix(alias) else {
        return false;
    };
    loop {
        rest = rest.trim_start();
        if rest.is_empty() {
            return true;
        }
        if let Some(after_non_null_assertion) = rest.strip_prefix('!') {
            rest = after_non_null_assertion;
            continue;
        }
        return route_handler_path_typescript_suffix(rest);
    }
}

pub(super) fn route_handler_body_alias_path(read: &str, alias: &str) -> Option<String> {
    let suffix = read.strip_prefix(alias)?;
    route_handler_json_path_suffix(suffix)
}

fn route_handler_json_path_suffix(source: &str) -> Option<String> {
    let mut rest = source.trim_start();
    let mut segments = Vec::new();

    loop {
        rest = rest.trim_start();
        if rest.is_empty() {
            break;
        }

        if let Some(after_non_null_assertion) = rest.strip_prefix('!') {
            rest = after_non_null_assertion;
            continue;
        }

        if route_handler_path_typescript_suffix(rest) {
            break;
        }

        if let Some(after_prefix) = rest.strip_prefix("?.[").or_else(|| rest.strip_prefix("[")) {
            let (key, consumed) = parse_quoted_value(after_prefix)?;
            if key.is_empty() || key.contains('.') {
                return None;
            }
            let after_bracket = after_prefix[consumed..].trim_start().strip_prefix("]")?;
            segments.push(key);
            rest = after_bracket;
            continue;
        }

        if let Some(after_prefix) = rest.strip_prefix("?.").or_else(|| rest.strip_prefix(".")) {
            let (segment, after_segment) = route_handler_json_path_identifier(after_prefix)?;
            segments.push(segment.to_string());
            rest = after_segment;
            continue;
        }

        return None;
    }

    (!segments.is_empty()).then(|| segments.join("."))
}

fn route_handler_json_path_identifier(source: &str) -> Option<(&str, &str)> {
    let end = source
        .char_indices()
        .find_map(|(index, character)| (!route_handler_identifier_char(character)).then_some(index))
        .unwrap_or(source.len());
    (end > 0).then(|| (&source[..end], &source[end..]))
}

fn route_handler_request_body_read(
    expression: &str,
    request: &DxReactRouteHandlerRequest,
    body_roots: &[String],
) -> Option<Value> {
    let (read, default) = split_top_level_nullish(expression);
    let read = route_handler_normalized_await_expression(read);
    let default = default.and_then(|value| parse_literal_value(value).ok());
    for root in body_roots {
        if read == format!("{root}.body") {
            return Some(route_handler_body_read_value(
                request,
                "body",
                default.clone(),
            ));
        }
        let body_root = format!("{root}.body");
        if let Some(path) = read
            .strip_prefix(&body_root)
            .and_then(route_handler_json_path_suffix)
        {
            return Some(
                default
                    .clone()
                    .map(|default| {
                        json_path_value_or_missing(&request.body, &path).unwrap_or(default)
                    })
                    .unwrap_or_else(|| json_path_value(&request.body, &path)),
            );
        }
        for method in ["json", "text", "formData"] {
            if let Some(path) = route_handler_request_body_projection_path(read, root, method) {
                return Some(route_handler_body_projection_value(
                    request,
                    method,
                    &path,
                    default.clone(),
                ));
            }
            if route_handler_request_body_method_read(read, root, method) {
                return Some(route_handler_body_read_value(
                    request,
                    method,
                    default.clone(),
                ));
            }
        }
    }
    None
}

fn route_handler_request_body_projection_path(
    expression: &str,
    root: &str,
    method: &str,
) -> Option<String> {
    let expression = expression.trim();
    if !expression.starts_with('(') {
        return None;
    }
    let close = find_balanced_delimiter(expression, 0, '(', ')')?;
    let path = route_handler_request_body_projection_suffix_path(&expression[close + 1..])?;
    let inner = route_handler_normalized_await_expression(&expression[1..close]);
    route_handler_request_body_method_read(inner, root, method).then_some(path)
}

fn route_handler_request_body_projection_suffix_path(source: &str) -> Option<String> {
    route_handler_json_path_suffix(source)
}

fn route_handler_request_body_method_read(expression: &str, root: &str, method: &str) -> bool {
    let getter = format!("{root}.{method}");
    find_first_call_args(expression, &[getter.as_str()]).is_some()
}

pub(super) fn route_handler_normalized_await_expression(source: &str) -> &str {
    let mut expression = source.trim();
    loop {
        let trimmed = expression.trim();
        if let Some(rest) = trimmed.strip_prefix("await ") {
            expression = rest;
            continue;
        }
        if trimmed.starts_with('(')
            && find_balanced_delimiter(trimmed, 0, '(', ')')
                .is_some_and(|end| end == trimmed.len().saturating_sub(1))
        {
            expression = &trimmed[1..trimmed.len().saturating_sub(1)];
            continue;
        }
        return trimmed;
    }
}

fn route_handler_body_read_value(
    request: &DxReactRouteHandlerRequest,
    method: &str,
    default: Option<Value>,
) -> Value {
    match method {
        "text" => route_handler_text_body_value(request, default),
        _ if request.body.is_null() => default.unwrap_or(Value::Null),
        _ => request.body.clone(),
    }
}

fn route_handler_body_projection_value(
    request: &DxReactRouteHandlerRequest,
    method: &str,
    path: &str,
    default: Option<Value>,
) -> Value {
    if method == "text" {
        return default.unwrap_or(Value::Null);
    }
    default
        .map(|default| json_path_value_or_missing(&request.body, path).unwrap_or(default))
        .unwrap_or_else(|| json_path_value(&request.body, path))
}

fn route_handler_text_body_value(
    request: &DxReactRouteHandlerRequest,
    default: Option<Value>,
) -> Value {
    if request.body.is_null() {
        return default.unwrap_or(Value::Null);
    }
    match &request.body {
        Value::String(value) => Value::String(value.clone()),
        _ => Value::String(
            serde_json::to_string(&request.body).unwrap_or_else(|_| request.body.to_string()),
        ),
    }
}

/// Safe route-handler context roots for direct reads like `context.params.slug` and `params.slug`.
#[derive(Debug, Clone, Default, PartialEq)]
struct RouteHandlerContextBindings {
    route_param_roots: Vec<String>,
    route_param_aliases: BTreeMap<String, String>,
    search_param_roots: Vec<String>,
    search_param_aliases: BTreeMap<String, RouteHandlerSearchParamAlias>,
    url_roots: Vec<String>,
    url_property_aliases: BTreeMap<String, String>,
    header_roots: Vec<String>,
    header_aliases: BTreeMap<String, RouteHandlerHeaderAlias>,
    cookie_roots: Vec<String>,
    body_roots: Vec<String>,
    body_aliases: Vec<String>,
    form_data_aliases: Vec<String>,
    text_body_aliases: Vec<String>,
    unsupported_body_aliases: BTreeMap<String, RouteHandlerUnsupportedBodyRead>,
    method_aliases: Vec<String>,
    request_url_aliases: BTreeMap<String, String>,
    body_field_aliases: BTreeMap<String, RouteHandlerBodyFieldAlias>,
}

#[derive(Debug, Clone, PartialEq)]
struct RouteHandlerSearchParamAlias {
    param_name: String,
    default: Value,
}

#[derive(Debug, Clone, PartialEq)]
struct RouteHandlerHeaderAlias {
    header_name: String,
    default: Value,
}

#[derive(Debug, Clone, PartialEq)]
struct RouteHandlerBodyFieldAlias {
    field_name: String,
    default: Value,
}

fn route_handler_context_bindings(
    source: &str,
    method: &str,
    function_body: &str,
) -> RouteHandlerContextBindings {
    let mut bindings = RouteHandlerContextBindings::default();
    let Some(parameters) = exported_functions(source)
        .into_iter()
        .find(|export| export.name == method)
        .map(|export| export.parameters)
    else {
        return bindings;
    };
    bindings.url_roots = route_handler_url_roots(function_body);
    bindings.url_property_aliases =
        route_handler_destructured_url_property_aliases(function_body, &bindings.url_roots);
    bindings.header_roots = route_handler_header_roots(function_body);
    bindings.cookie_roots = route_handler_cookie_roots(function_body);
    bindings.body_roots = route_handler_request_body_roots(function_body);
    bindings.body_aliases = route_handler_body_aliases(function_body, &bindings.body_roots);
    bindings.form_data_aliases =
        route_handler_body_method_aliases(function_body, &bindings.body_roots, &["formData"]);
    bindings.text_body_aliases =
        route_handler_text_body_aliases(function_body, &bindings.body_roots);
    bindings.unsupported_body_aliases =
        route_handler_unsupported_body_aliases(function_body, &bindings.body_roots);
    bindings.method_aliases = route_handler_method_aliases(function_body);
    bindings.request_url_aliases = route_handler_request_url_aliases(function_body);
    let entries = split_top_level_entries(&parameters);
    if let Some(context_parameter) = entries.get(1).map(|entry| entry.trim()) {
        extend_route_handler_context_roots(
            &mut bindings.route_param_roots,
            context_parameter,
            "params",
        );
        extend_route_handler_context_roots(
            &mut bindings.search_param_roots,
            context_parameter,
            "searchParams",
        );
    }
    bindings
        .search_param_roots
        .extend(route_handler_destructured_search_param_roots(
            function_body,
            &bindings.url_roots,
        ));
    bindings.route_param_roots.sort();
    bindings.route_param_roots.dedup();
    bindings.search_param_roots.sort();
    bindings.search_param_roots.dedup();
    bindings.route_param_aliases =
        route_handler_context_param_aliases(function_body, &bindings.route_param_roots);
    bindings.search_param_aliases = route_handler_search_param_aliases(
        function_body,
        &bindings.search_param_roots,
        &bindings.url_roots,
    );
    bindings.header_aliases = route_handler_header_aliases(function_body, &bindings.header_roots);
    bindings.body_field_aliases =
        route_handler_json_body_field_aliases(function_body, &bindings.body_roots);
    bindings
}

fn extend_route_handler_context_roots(roots: &mut Vec<String>, parameter: &str, field: &str) {
    let binding = parameter
        .split_once(':')
        .map(|(binding, _)| binding)
        .unwrap_or(parameter)
        .trim();
    if let Some(inner) = strip_object_braces(binding) {
        for entry in split_top_level_entries(inner) {
            let entry = entry.trim();
            if entry == field {
                roots.push(field.to_string());
                continue;
            }
            let Some((left, right)) = entry.split_once(':') else {
                continue;
            };
            if left.trim() != field {
                continue;
            }
            if let Some(alias) = route_handler_context_identifier(right.trim()) {
                roots.push(alias);
            }
        }
        return;
    }
    if let Some(identifier) = route_handler_context_identifier(binding) {
        roots.push(format!("{identifier}.{field}"));
    }
}

fn route_handler_context_identifier(source: &str) -> Option<String> {
    let identifier = source.trim();
    if identifier.is_empty()
        || !identifier
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '_' | '$'))
        || identifier
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_digit())
    {
        return None;
    }
    Some(identifier.to_string())
}

fn route_handler_context_map_value(
    expression: &str,
    roots: &[String],
    values: &BTreeMap<String, String>,
) -> Option<Value> {
    for root in roots {
        if expression == root {
            return Some(serde_json::json!(values));
        }
        if let Some(path) = expression.strip_prefix(&format!("{root}.")) {
            return Some(route_request_map_value(values, path));
        }
    }
    None
}

fn route_handler_context_alias_value(
    expression: &str,
    aliases: &BTreeMap<String, String>,
    values: &BTreeMap<String, String>,
) -> Option<Value> {
    aliases
        .get(expression)
        .map(|key| route_request_map_value(values, key))
}

fn route_handler_url_roots(function_body: &str) -> Vec<String> {
    let mut roots = vec!["request.nextUrl".to_string()];
    if let Ok(alias_re) = regex::Regex::new(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*(?::[^=;\n]+)?\s*=\s*(?:new\s+URL\(\s*request\.url\s*\)|request\.nextUrl)(?:\s+as\s+[^;\n]+)?\s*;?"#,
    ) {
        roots.extend(
            alias_re
                .captures_iter(function_body)
                .filter_map(|capture| capture.get(1).map(|alias| alias.as_str().to_string())),
        );
    }
    roots.sort();
    roots.dedup();
    roots
}

fn route_handler_destructured_url_property_aliases(
    function_body: &str,
    url_roots: &[String],
) -> BTreeMap<String, String> {
    let mut aliases = BTreeMap::new();
    let Ok(destructured_re) =
        regex::Regex::new(r#"(?m)\b(?:const|let)\s*\{([^}]+)\}\s*(?::[^=;\n]+)?\s*=\s*([^;\n]+)"#)
    else {
        return aliases;
    };

    for capture in destructured_re.captures_iter(function_body) {
        let Some(source) = capture.get(2).map(|value| value.as_str()) else {
            continue;
        };
        if !route_handler_destructured_search_param_source_is_url(source, url_roots) {
            continue;
        }
        let Some(fields) = capture.get(1).map(|value| value.as_str()) else {
            continue;
        };
        for field in split_top_level_entries(fields) {
            let Some((property, alias)) = route_handler_destructured_param_alias(field) else {
                continue;
            };
            if matches!(property.as_str(), "pathname" | "href" | "search") {
                aliases.insert(alias, property);
            }
        }
    }

    aliases
}

fn route_handler_destructured_search_param_roots(
    function_body: &str,
    url_roots: &[String],
) -> Vec<String> {
    let mut roots = Vec::new();
    let Ok(destructured_re) =
        regex::Regex::new(r#"(?m)\b(?:const|let)\s*\{([^}]+)\}\s*(?::[^=;\n]+)?\s*=\s*([^;\n]+)"#)
    else {
        return roots;
    };

    for capture in destructured_re.captures_iter(function_body) {
        let Some(source) = capture.get(2).map(|value| value.as_str()) else {
            continue;
        };
        if !route_handler_destructured_search_param_source_is_url(source, url_roots) {
            continue;
        }
        let Some(fields) = capture.get(1).map(|value| value.as_str()) else {
            continue;
        };
        for field in split_top_level_entries(fields) {
            let Some((property, alias)) = route_handler_destructured_param_alias(field) else {
                continue;
            };
            if property == "searchParams" {
                roots.push(alias);
            }
        }
    }

    roots.sort();
    roots.dedup();
    roots
}

fn route_handler_destructured_search_param_source_is_url(
    source: &str,
    url_roots: &[String],
) -> bool {
    let source = source.trim().trim_end_matches(';').trim();
    if url_roots.iter().any(|root| root == source) {
        return true;
    }
    find_first_call_args(source, &["new URL"])
        .and_then(|args| args.first().map(|arg| arg.trim() == "request.url"))
        .unwrap_or(false)
}

fn route_handler_search_param_aliases(
    function_body: &str,
    search_roots: &[String],
    url_roots: &[String],
) -> BTreeMap<String, RouteHandlerSearchParamAlias> {
    let mut aliases = BTreeMap::new();
    let Ok(alias_re) = regex::Regex::new(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*(?::[^=;\n]+)?\s*=\s*([^;\n]+)"#,
    ) else {
        return aliases;
    };

    for capture in alias_re.captures_iter(function_body) {
        let Some(alias) = capture.get(1).map(|value| value.as_str().to_string()) else {
            continue;
        };
        let Some(source) = capture.get(2).map(|value| value.as_str()) else {
            continue;
        };
        let Some((param_name, default)) =
            route_handler_search_param_read(source, search_roots, url_roots)
        else {
            continue;
        };
        aliases.insert(
            alias,
            RouteHandlerSearchParamAlias {
                param_name,
                default,
            },
        );
    }

    aliases
}

fn route_handler_search_param_read(
    source: &str,
    search_roots: &[String],
    url_roots: &[String],
) -> Option<(String, Value)> {
    let (read, default) = split_top_level_nullish(source);
    let param_name = route_handler_search_param_get_name(read, search_roots, url_roots)?;
    let default = default
        .and_then(|value| parse_literal_value(value).ok())
        .unwrap_or(Value::Null);
    Some((param_name, default))
}

fn route_handler_search_param_get_name(
    source: &str,
    search_roots: &[String],
    url_roots: &[String],
) -> Option<String> {
    for root in route_handler_search_param_roots(search_roots, url_roots) {
        let getter = format!("{root}.get");
        if let Some(args) = find_first_call_args(source, &[getter.as_str()]) {
            let param = args.first()?;
            if let Ok(Value::String(name)) = parse_literal_value(param) {
                return Some(name);
            }
        }
    }
    None
}

fn route_handler_search_param_roots(search_roots: &[String], url_roots: &[String]) -> Vec<String> {
    let mut roots = search_roots.to_vec();
    roots.push("new URL(request.url).searchParams".to_string());
    roots.extend(url_roots.iter().map(|root| format!("{root}.searchParams")));
    roots.sort();
    roots.dedup();
    roots
}

fn route_handler_url_property_read(
    expression: &str,
    request: &DxReactRouteHandlerRequest,
    url_roots: &[String],
) -> Option<Value> {
    let property = route_handler_url_property_name(expression, url_roots)?;
    route_request_url_property_value(request, property)
}

fn route_request_url_property_value(
    request: &DxReactRouteHandlerRequest,
    property: &str,
) -> Option<Value> {
    match property {
        "pathname" => Some(Value::String(
            route_request_path_for_match(&request.path).to_string(),
        )),
        "href" => Some(Value::String(request.path.clone())),
        "search" => Some(Value::String(route_request_search(&request.path))),
        _ => None,
    }
}

fn route_handler_url_property_name(source: &str, url_roots: &[String]) -> Option<&'static str> {
    let source = source.trim().trim_end_matches(';').trim();
    for root in route_handler_url_property_roots(url_roots) {
        for property in ["pathname", "href", "search"] {
            if source == format!("{root}.{property}") {
                return Some(property);
            }
        }
    }
    None
}

fn route_handler_url_property_roots(url_roots: &[String]) -> Vec<String> {
    let mut roots = url_roots.to_vec();
    roots.push("new URL(request.url)".to_string());
    roots.sort();
    roots.dedup();
    roots
}

fn route_search_param_value(
    request: &DxReactRouteHandlerRequest,
    param_name: &str,
    default: Value,
) -> Value {
    route_request_search_params(request)
        .get(param_name)
        .map(|value| Value::String(value.clone()))
        .unwrap_or(default)
}

fn route_handler_header_roots(function_body: &str) -> Vec<String> {
    let mut roots = vec!["request.headers".to_string()];
    if function_body.contains("headers().get") {
        roots.push("headers()".to_string());
    }
    if let Ok(alias_re) = regex::Regex::new(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*(?::[^=;\n]+)?\s*=\s*request\.headers\b"#,
    ) {
        roots.extend(
            alias_re
                .captures_iter(function_body)
                .filter_map(|capture| capture.get(1).map(|alias| alias.as_str().to_string())),
        );
    }
    if let Ok(alias_re) = regex::Regex::new(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*(?::[^=;\n]+)?\s*=\s*headers\(\)"#,
    ) {
        roots.extend(
            alias_re
                .captures_iter(function_body)
                .filter_map(|capture| capture.get(1).map(|alias| alias.as_str().to_string())),
        );
    }
    roots.extend(route_handler_destructured_header_roots(function_body));
    roots.sort();
    roots.dedup();
    roots
}

fn route_handler_destructured_header_roots(function_body: &str) -> Vec<String> {
    let mut roots = Vec::new();
    let Ok(destructured_re) =
        regex::Regex::new(r#"(?m)\b(?:const|let)\s*\{([^}]+)\}\s*(?::[^=;\n]+)?\s*=\s*request\b"#)
    else {
        return roots;
    };

    for capture in destructured_re.captures_iter(function_body) {
        let Some(fields) = capture.get(1).map(|value| value.as_str()) else {
            continue;
        };
        for field in split_top_level_entries(fields) {
            let Some((property, alias)) = route_handler_destructured_param_alias(field) else {
                continue;
            };
            if property == "headers" {
                roots.push(alias);
            }
        }
    }

    roots.sort();
    roots.dedup();
    roots
}

fn route_handler_method_aliases(function_body: &str) -> Vec<String> {
    let mut aliases = Vec::new();
    let Ok(alias_re) = regex::Regex::new(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*(?::[^=;\n]+)?\s*=\s*request\.method\b"#,
    ) else {
        return aliases;
    };
    aliases.extend(
        alias_re
            .captures_iter(function_body)
            .filter_map(|capture| capture.get(1).map(|alias| alias.as_str().to_string())),
    );
    aliases.extend(route_handler_destructured_method_aliases(function_body));
    aliases.sort();
    aliases.dedup();
    aliases
}

fn route_handler_destructured_method_aliases(function_body: &str) -> Vec<String> {
    let mut aliases = Vec::new();
    let Ok(destructured_re) =
        regex::Regex::new(r#"(?m)\b(?:const|let)\s*\{([^}]+)\}\s*(?::[^=;\n]+)?\s*=\s*request\b"#)
    else {
        return aliases;
    };
    for capture in destructured_re.captures_iter(function_body) {
        let Some(fields) = capture.get(1).map(|value| value.as_str()) else {
            continue;
        };
        for field in split_top_level_entries(fields) {
            let Some((property, alias)) = route_handler_destructured_param_alias(field) else {
                continue;
            };
            if property == "method" {
                aliases.push(alias);
            }
        }
    }
    aliases.sort();
    aliases.dedup();
    aliases
}

fn route_handler_request_url_aliases(function_body: &str) -> BTreeMap<String, String> {
    let mut aliases = BTreeMap::new();
    let Ok(alias_re) = regex::Regex::new(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*(?::[^=;\n]+)?\s*=\s*request\.(path|url)\b"#,
    ) else {
        return aliases;
    };
    for capture in alias_re.captures_iter(function_body) {
        let Some(alias) = capture.get(1).map(|value| value.as_str().to_string()) else {
            continue;
        };
        let Some(property) = capture.get(2).map(|value| value.as_str().to_string()) else {
            continue;
        };
        aliases.insert(alias, property);
    }
    aliases.extend(route_handler_destructured_request_url_aliases(
        function_body,
    ));
    aliases
}

fn route_handler_destructured_request_url_aliases(function_body: &str) -> BTreeMap<String, String> {
    let mut aliases = BTreeMap::new();
    let Ok(destructured_re) =
        regex::Regex::new(r#"(?m)\b(?:const|let)\s*\{([^}]+)\}\s*(?::[^=;\n]+)?\s*=\s*request\b"#)
    else {
        return aliases;
    };
    for capture in destructured_re.captures_iter(function_body) {
        let Some(fields) = capture.get(1).map(|value| value.as_str()) else {
            continue;
        };
        for field in split_top_level_entries(fields) {
            let Some((property, alias)) = route_handler_destructured_param_alias(field) else {
                continue;
            };
            if matches!(property.as_str(), "path" | "url") {
                aliases.insert(alias, property);
            }
        }
    }
    aliases
}

fn route_handler_header_aliases(
    function_body: &str,
    header_roots: &[String],
) -> BTreeMap<String, RouteHandlerHeaderAlias> {
    let mut aliases = BTreeMap::new();
    let Ok(alias_re) = regex::Regex::new(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*(?::[^=;\n]+)?\s*=\s*([^;\n]+)"#,
    ) else {
        return aliases;
    };

    for capture in alias_re.captures_iter(function_body) {
        let Some(alias) = capture.get(1).map(|value| value.as_str().to_string()) else {
            continue;
        };
        let Some(source) = capture.get(2).map(|value| value.as_str()) else {
            continue;
        };
        let Some((header_name, default)) = route_handler_header_read(source, header_roots) else {
            continue;
        };
        aliases.insert(
            alias,
            RouteHandlerHeaderAlias {
                header_name,
                default,
            },
        );
    }

    aliases
}

fn route_handler_header_read(source: &str, header_roots: &[String]) -> Option<(String, Value)> {
    let (read, default) = split_top_level_nullish(source);
    let header_name = route_handler_header_get_name(read, header_roots)?;
    let default = default
        .and_then(|value| parse_literal_value(value).ok())
        .unwrap_or(Value::Null);
    Some((header_name, default))
}

fn route_handler_header_get_name(source: &str, header_roots: &[String]) -> Option<String> {
    for root in header_roots {
        let getter = format!("{root}.get");
        if let Some(args) = find_first_call_args(source, &[getter.as_str()]) {
            let header = args.first()?;
            if let Ok(Value::String(name)) = parse_literal_value(header) {
                return Some(name);
            }
        }
    }
    None
}

fn route_header_value(
    request: &DxReactRouteHandlerRequest,
    header_name: &str,
    default: Value,
) -> Value {
    request_header(request, header_name)
        .map(|value| Value::String(value.to_string()))
        .unwrap_or(default)
}

fn route_handler_form_data_read(
    source: &str,
    request: &DxReactRouteHandlerRequest,
    body_aliases: &[String],
    body_roots: &[String],
) -> Option<Value> {
    let (read, default) = split_top_level_nullish(source);
    for alias in body_aliases {
        let getter = format!("{alias}.get");
        let Some(args) = find_first_call_args(read, &[getter.as_str()]) else {
            continue;
        };
        let param = args.first()?;
        let Ok(Value::String(field_name)) = parse_literal_value(param) else {
            return None;
        };
        let default = default
            .and_then(|value| parse_literal_value(value).ok())
            .unwrap_or(Value::Null);
        return Some(route_form_data_value(request, &field_name).unwrap_or(default));
    }
    for root in body_roots {
        let Some(field_name) = route_handler_direct_form_data_get_name(read, root) else {
            continue;
        };
        let default = default
            .and_then(|value| parse_literal_value(value).ok())
            .unwrap_or(Value::Null);
        return Some(route_form_data_value(request, &field_name).unwrap_or(default));
    }
    None
}

fn route_handler_direct_form_data_get_name(source: &str, root: &str) -> Option<String> {
    let expression = source.trim();
    if !expression.starts_with('(') {
        return None;
    }
    let close = find_balanced_delimiter(expression, 0, '(', ')')?;
    let suffix = expression[close + 1..].trim_start();
    let args = find_first_call_args(suffix, &[".get"])?;
    let param = args.first()?;
    let Ok(Value::String(field_name)) = parse_literal_value(param) else {
        return None;
    };
    let inner = route_handler_normalized_await_expression(&expression[1..close]);
    route_handler_request_body_method_read(inner, root, "formData").then_some(field_name)
}

fn route_form_data_value(request: &DxReactRouteHandlerRequest, field_name: &str) -> Option<Value> {
    let Value::Object(fields) = &request.body else {
        return None;
    };
    fields.get(field_name).cloned()
}

pub(super) fn split_top_level_nullish(source: &str) -> (&str, Option<&str>) {
    split_top_level_operator(source, "??")
}

fn split_top_level_assignment(source: &str) -> (&str, Option<&str>) {
    split_top_level_operator(source, "=")
}

fn split_top_level_operator<'a>(source: &'a str, operator: &str) -> (&'a str, Option<&'a str>) {
    let mut cursor = 0usize;
    let mut quote = None;
    let mut depth = 0usize;
    while cursor < source.len() {
        if quote.is_none() && depth == 0 && source[cursor..].starts_with(operator) {
            return (
                source[..cursor].trim(),
                Some(source[cursor + operator.len()..].trim()),
            );
        }
        let ch = source[cursor..].chars().next().unwrap_or_default();
        if let Some(active_quote) = quote {
            if ch == active_quote {
                quote = None;
            } else if ch == '\\' {
                cursor += ch.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..]
                        .chars()
                        .next()
                        .unwrap_or_default()
                        .len_utf8();
                    continue;
                }
            }
            cursor += ch.len_utf8();
            continue;
        }
        match ch {
            '"' | '\'' | '`' => quote = Some(ch),
            '{' | '[' | '(' => depth += 1,
            '}' | ']' | ')' => depth = depth.saturating_sub(1),
            _ => {}
        }
        cursor += ch.len_utf8();
    }
    (source.trim(), None)
}

fn route_handler_context_param_aliases(
    function_body: &str,
    roots: &[String],
) -> BTreeMap<String, String> {
    let mut aliases = BTreeMap::new();
    let root_pattern = roots
        .iter()
        .map(|root| regex::escape(root))
        .collect::<Vec<_>>()
        .join("|");
    if root_pattern.is_empty() {
        return aliases;
    }

    if let Ok(destructured_re) = regex::Regex::new(&format!(
        r#"(?m)\b(?:const|let)\s*\{{([^}}]+)\}}\s*=\s*(?:await\s+)?(?:{root_pattern})\b"#
    )) {
        for capture in destructured_re.captures_iter(function_body) {
            let Some(fields) = capture.get(1).map(|value| value.as_str()) else {
                continue;
            };
            for field in split_top_level_entries(fields) {
                if let Some((param_key, alias)) = route_handler_destructured_param_alias(field) {
                    aliases.insert(alias, param_key);
                }
            }
        }
    }

    if let Ok(direct_re) = regex::Regex::new(&format!(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*=\s*(?:{root_pattern})\.([A-Za-z_][A-Za-z0-9_-]*)\b"#
    )) {
        for capture in direct_re.captures_iter(function_body) {
            let Some(alias) = capture.get(1).map(|value| value.as_str().to_string()) else {
                continue;
            };
            let Some(param_key) = capture.get(2).map(|value| value.as_str().to_string()) else {
                continue;
            };
            aliases.insert(alias, param_key);
        }
    }

    aliases
}

fn route_handler_json_body_field_aliases(
    function_body: &str,
    body_roots: &[String],
) -> BTreeMap<String, RouteHandlerBodyFieldAlias> {
    let mut aliases = BTreeMap::new();
    let Some(root_pattern) = route_handler_body_root_expression_pattern(body_roots) else {
        return aliases;
    };
    let type_postfix_pattern = route_handler_body_alias_type_postfix_pattern();
    let Ok(destructured_re) = regex::Regex::new(&format!(
        r#"(?m)\b(?:const|let)\s*\{{([^}}]+)\}}\s*(?::[^=;\n]+)?\s*=\s*(?:\(\s*)?(?:await\s+)?(?:{root_pattern})\.json\(\)\s*\)?{type_postfix_pattern}"#
    )) else {
        return aliases;
    };
    for capture in destructured_re.captures_iter(function_body) {
        let Some(fields) = capture.get(1).map(|value| value.as_str()) else {
            continue;
        };
        for field in split_top_level_entries(fields) {
            if let Some((field_name, alias, default)) =
                route_handler_destructured_json_field_alias(field)
            {
                aliases.insert(
                    alias,
                    RouteHandlerBodyFieldAlias {
                        field_name,
                        default,
                    },
                );
            }
        }
    }
    aliases
}

fn route_handler_destructured_json_field_alias(source: &str) -> Option<(String, String, Value)> {
    let (field, default) = split_top_level_assignment(source);
    let field = field.trim();
    let default = match default {
        Some(value) => parse_literal_value(value).ok()?,
        None => Value::Null,
    };
    let (body_key, alias) = field
        .split_once(':')
        .map(|(body_key, alias)| (body_key.trim(), alias.trim()))
        .unwrap_or((field, field));
    let body_key = route_handler_context_identifier(body_key)?;
    let alias = route_handler_context_identifier(alias)?;
    Some((body_key, alias, default))
}

fn route_handler_destructured_param_alias(source: &str) -> Option<(String, String)> {
    let field = source.trim();
    let (param_key, alias) = field
        .split_once(':')
        .map(|(param_key, alias)| (param_key.trim(), alias.trim()))
        .unwrap_or((field, field));
    let param_key = route_handler_context_identifier(param_key)?;
    let alias = route_handler_context_identifier(alias)?;
    Some((param_key, alias))
}

fn route_request_map_value(values: &BTreeMap<String, String>, key: &str) -> Value {
    if key.is_empty()
        || !key.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '_' || character == '-'
        })
    {
        return Value::Null;
    }
    values
        .get(key)
        .map(|value| Value::String(value.clone()))
        .unwrap_or(Value::Null)
}

fn route_handler_request_body_roots(function_body: &str) -> Vec<String> {
    let mut roots = vec!["request".to_string(), "request.clone()".to_string()];
    roots.extend(route_handler_request_clone_aliases(function_body));
    if let Ok(alias_re) = regex::Regex::new(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*(?::[^=;\n]+)?\s*=\s*request(?:\s+as\s+[^;\n]+)?\s*;?\s*$"#,
    ) {
        roots.extend(
            alias_re
                .captures_iter(function_body)
                .filter_map(|capture| capture.get(1).map(|alias| alias.as_str().to_string())),
        );
    }
    roots.sort();
    roots.dedup();
    roots
}

fn route_handler_request_clone_aliases(function_body: &str) -> Vec<String> {
    let Ok(alias_re) = regex::Regex::new(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*(?::[^=;\n]+)?\s*=\s*request\.clone\(\)(?:\s+as\s+[^;\n]+)?"#,
    ) else {
        return Vec::new();
    };
    let mut aliases = alias_re
        .captures_iter(function_body)
        .filter_map(|capture| capture.get(1).map(|alias| alias.as_str().to_string()))
        .collect::<Vec<_>>();
    aliases.sort();
    aliases.dedup();
    aliases
}

fn route_handler_body_aliases(function_body: &str, body_roots: &[String]) -> Vec<String> {
    let mut aliases = route_handler_json_body_aliases(function_body, body_roots);
    aliases.extend(route_handler_body_method_aliases(
        function_body,
        body_roots,
        &["text", "formData"],
    ));
    aliases.extend(route_handler_raw_body_aliases(function_body, body_roots));
    aliases.sort();
    aliases.dedup();
    aliases
}

fn route_handler_effective_body_aliases(
    function_body: &str,
    context_bindings: &RouteHandlerContextBindings,
) -> Vec<String> {
    let mut aliases = context_bindings.body_aliases.clone();
    aliases.extend(route_handler_body_aliases(
        function_body,
        &context_bindings.body_roots,
    ));
    aliases.sort();
    aliases.dedup();
    aliases
}

fn route_handler_text_body_aliases(function_body: &str, body_roots: &[String]) -> Vec<String> {
    route_handler_body_method_aliases(function_body, body_roots, &["text"])
}

fn route_handler_json_body_aliases(function_body: &str, body_roots: &[String]) -> Vec<String> {
    let Some(root_pattern) = route_handler_body_root_expression_pattern(body_roots) else {
        return Vec::new();
    };
    let type_postfix_pattern = route_handler_body_alias_type_postfix_pattern();
    let Ok(alias_re) = regex::Regex::new(&format!(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*(?::[^=;\n]+)?\s*=\s*(?:\(\s*)?(?:await\s+)?(?:{root_pattern})\.json\(\)\s*\)?{type_postfix_pattern}"#
    )) else {
        return Vec::new();
    };
    route_handler_alias_captures(function_body, &alias_re)
}

fn route_handler_body_method_aliases(
    function_body: &str,
    body_roots: &[String],
    methods: &[&str],
) -> Vec<String> {
    let Some(root_pattern) = route_handler_body_root_pattern(body_roots) else {
        return Vec::new();
    };
    let method_pattern = methods.join("|");
    let Ok(alias_re) = regex::Regex::new(&format!(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*(?::[^=;\n]+)?\s*=\s*(?:\(\s*)?(?:await\s+)?(?:{root_pattern})\.(?:{method_pattern})\(\)\s*\)?(?:\s+as\s+[^;\n]+)?"#
    )) else {
        return Vec::new();
    };
    route_handler_alias_captures(function_body, &alias_re)
}

fn route_handler_raw_body_aliases(function_body: &str, body_roots: &[String]) -> Vec<String> {
    let Some(root_pattern) = route_handler_body_root_pattern(body_roots) else {
        return Vec::new();
    };
    let Ok(alias_re) = regex::Regex::new(&format!(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*(?::[^=;\n]+)?\s*=\s*(?:\(\s*)?(?:{root_pattern})\.body\b\s*\)?(?:\s+as\s+[^;\n]+)?"#
    )) else {
        return Vec::new();
    };
    route_handler_alias_captures(function_body, &alias_re)
}

fn route_handler_alias_captures(function_body: &str, alias_re: &regex::Regex) -> Vec<String> {
    let mut aliases = alias_re
        .captures_iter(function_body)
        .filter_map(|capture| capture.get(1).map(|alias| alias.as_str().to_string()))
        .collect::<Vec<_>>();
    aliases.sort();
    aliases.dedup();
    aliases
}

pub(super) fn route_handler_body_root_pattern(body_roots: &[String]) -> Option<String> {
    let mut root_patterns = body_roots
        .iter()
        .map(|root| regex::escape(root))
        .collect::<Vec<_>>();
    root_patterns.sort_by(|left, right| right.len().cmp(&left.len()).then_with(|| left.cmp(right)));
    (!root_patterns.is_empty()).then(|| root_patterns.join("|"))
}

fn route_handler_body_root_expression_pattern(body_roots: &[String]) -> Option<String> {
    let root_pattern = route_handler_body_root_pattern(body_roots)?;
    Some(format!(
        r#"(?:{root_pattern}|\(\s*(?:{root_pattern})\s+as\s+[^)]+\))"#
    ))
}

fn route_handler_body_alias_type_postfix_pattern() -> &'static str {
    r#"(?:\s+as\s+[^;\n]+|\s+satisfies\s+[^;\n]+)?"#
}

fn route_handler_path_typescript_suffix(rest: &str) -> bool {
    let rest = rest.trim_start();
    rest.starts_with("as ") || rest.starts_with("satisfies ")
}

#[derive(Debug, Clone, PartialEq)]
struct RouteHandlerCookieAlias {
    cookie_name: String,
    property: RouteHandlerCookieProperty,
    default: Value,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RouteHandlerCookieProperty {
    Value,
    Name,
    Object,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct RouteHandlerResponseOptions {
    status: Option<u16>,
    headers: BTreeMap<String, String>,
}

fn route_handler_cookie_roots(function_body: &str) -> Vec<String> {
    let mut roots = vec!["request.cookies".to_string()];
    if function_body.contains("cookies().get") {
        roots.push("cookies()".to_string());
    }
    if let Ok(alias_re) = regex::Regex::new(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*=\s*(?:request\.cookies|cookies\(\))(?:\s+as\s+[^;\n]+)?\s*;?"#,
    ) {
        roots.extend(
            alias_re
                .captures_iter(function_body)
                .filter_map(|capture| capture.get(1).map(|alias| alias.as_str().to_string())),
        );
    }
    roots.extend(route_handler_destructured_cookie_roots(function_body));
    roots.sort();
    roots.dedup();
    roots
}

fn route_handler_destructured_cookie_roots(function_body: &str) -> Vec<String> {
    let mut roots = Vec::new();
    let Ok(destructured_re) =
        regex::Regex::new(r#"(?m)\b(?:const|let)\s*\{([^}]+)\}\s*(?::[^=;\n]+)?\s*=\s*request\b"#)
    else {
        return roots;
    };

    for capture in destructured_re.captures_iter(function_body) {
        let Some(fields) = capture.get(1).map(|value| value.as_str()) else {
            continue;
        };
        for field in split_top_level_entries(fields) {
            let Some((property, alias)) = route_handler_destructured_param_alias(field) else {
                continue;
            };
            if property == "cookies" {
                roots.push(alias);
            }
        }
    }

    roots.sort();
    roots.dedup();
    roots
}

fn route_handler_cookie_aliases(function_body: &str) -> BTreeMap<String, RouteHandlerCookieAlias> {
    let mut aliases = BTreeMap::new();
    let roots = route_handler_cookie_roots(function_body);
    let Ok(alias_re) =
        regex::Regex::new(r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*=\s*([^;\n]+)"#)
    else {
        return aliases;
    };
    for capture in alias_re.captures_iter(function_body) {
        let Some(alias) = capture.get(1).map(|value| value.as_str().to_string()) else {
            continue;
        };
        let Some(source) = capture.get(2).map(|value| value.as_str()) else {
            continue;
        };
        let Some(read) = route_handler_cookie_read(source, &roots) else {
            continue;
        };
        aliases.insert(alias, read);
    }
    aliases
}

fn route_handler_cookie_read(source: &str, roots: &[String]) -> Option<RouteHandlerCookieAlias> {
    let (read, default) = split_top_level_nullish(source);
    let mut cookie_read = route_handler_cookie_get(read, roots)?;
    let default = default
        .and_then(|value| parse_literal_value(value).ok())
        .unwrap_or(Value::Null);
    cookie_read.default = default;
    Some(cookie_read)
}

fn route_handler_cookie_get(source: &str, roots: &[String]) -> Option<RouteHandlerCookieAlias> {
    for root in roots {
        let getter = format!("{root}.get");
        let mut cursor = 0usize;
        while let Some(index) = source[cursor..].find(&getter) {
            let name_start = cursor + index;
            let after_name = name_start + getter.len();
            let paren = source[after_name..]
                .char_indices()
                .find_map(|(offset, ch)| match ch {
                    '(' => Some(after_name + offset),
                    ch if ch.is_whitespace() => None,
                    _ => Some(usize::MAX),
                })?;
            if paren == usize::MAX {
                cursor = after_name;
                continue;
            }
            let end = find_balanced_delimiter(source, paren, '(', ')')?;
            let args = split_top_level_entries(&source[paren + 1..end]);
            let param = args.first()?;
            let Ok(Value::String(cookie_name)) = parse_literal_value(param) else {
                return None;
            };
            return Some(RouteHandlerCookieAlias {
                cookie_name,
                property: route_handler_cookie_property(&source[end + 1..]),
                default: Value::Null,
            });
        }
    }
    None
}

fn route_handler_cookie_property(suffix: &str) -> RouteHandlerCookieProperty {
    let suffix = suffix.trim_start();
    if suffix.starts_with("?.name") || suffix.starts_with(".name") {
        return RouteHandlerCookieProperty::Name;
    }
    if suffix.starts_with("?.value") || suffix.starts_with(".value") {
        return RouteHandlerCookieProperty::Value;
    }
    RouteHandlerCookieProperty::Object
}

fn cookie_read_value(
    request: &DxReactRouteHandlerRequest,
    read: &RouteHandlerCookieAlias,
) -> Value {
    let Some(value) = cookie_value(request, &read.cookie_name) else {
        return read.default.clone();
    };
    match read.property {
        RouteHandlerCookieProperty::Value => Value::String(value),
        RouteHandlerCookieProperty::Name => Value::String(read.cookie_name.clone()),
        RouteHandlerCookieProperty::Object => serde_json::json!({
            "name": read.cookie_name.clone(),
            "value": value,
        }),
    }
}

fn route_handler_response_number_aliases(function_body: &str) -> BTreeMap<String, u16> {
    let mut aliases = BTreeMap::new();
    let Ok(alias_re) = regex::Regex::new(
        r#"(?m)\b(?:const|let)\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?::[^=;\n]+)?\s*=\s*([0-9]+)(?:\s+as\s+const)?\s*;?"#,
    ) else {
        return aliases;
    };
    for capture in alias_re.captures_iter(function_body) {
        let Some(alias) = capture.get(1).map(|value| value.as_str().to_string()) else {
            continue;
        };
        let Some(status) = capture
            .get(2)
            .and_then(|value| parse_route_handler_literal_status(value.as_str()))
        else {
            continue;
        };
        aliases.insert(alias, status);
    }
    aliases
}

fn cookie_value(request: &DxReactRouteHandlerRequest, name: &str) -> Option<String> {
    let cookie_header = request_header(request, "cookie")?;
    cookie_header.split(';').find_map(|part| {
        let (key, value) = part.trim().split_once('=')?;
        (key.trim() == name).then(|| value.trim().to_string())
    })
}

fn request_header<'a>(request: &'a DxReactRouteHandlerRequest, name: &str) -> Option<&'a str> {
    request
        .headers
        .get(name)
        .or_else(|| {
            request
                .headers
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(name))
                .map(|(_, value)| value)
        })
        .map(String::as_str)
}

pub(super) fn find_first_call_args(source: &str, names: &[&str]) -> Option<Vec<String>> {
    for name in names {
        let mut cursor = 0usize;
        while let Some(index) = source[cursor..].find(name) {
            let name_start = cursor + index;
            let after_name = name_start + name.len();
            let paren = source[after_name..]
                .char_indices()
                .find_map(|(offset, ch)| match ch {
                    '(' => Some(after_name + offset),
                    ch if ch.is_whitespace() => None,
                    _ => Some(usize::MAX),
                })?;
            if paren != usize::MAX {
                let end = find_balanced_delimiter(source, paren, '(', ')')?;
                return Some(
                    split_top_level_entries(&source[paren + 1..end])
                        .into_iter()
                        .map(str::to_string)
                        .collect(),
                );
            }
            cursor = after_name;
        }
    }
    None
}

fn strip_object_braces(source: &str) -> Option<&str> {
    let source = source.trim();
    let start = source.find('{')?;
    let end = find_balanced_block(source, start)?;
    Some(&source[start + 1..end])
}

fn strip_array_brackets(source: &str) -> Option<&str> {
    let source = source.trim();
    let start = source.find('[')?;
    let end = find_balanced_delimiter(source, start, '[', ']')?;
    Some(&source[start + 1..end])
}

fn parse_route_handler_response_options(
    source: &str,
    number_aliases: &BTreeMap<String, u16>,
    header_aliases: &BTreeMap<String, BTreeMap<String, String>>,
) -> Result<RouteHandlerResponseOptions, String> {
    let mut options = RouteHandlerResponseOptions::default();
    for entry in split_top_level_entries(source) {
        let Some((key, value)) = entry.split_once(':') else {
            continue;
        };
        let key = key.trim().trim_matches('"').trim_matches('\'');
        match key {
            "status" => {
                options.status = parse_route_handler_response_status(value.trim(), number_aliases);
            }
            "headers" => {
                options.headers.extend(parse_safe_response_header_entries(
                    value.trim(),
                    header_aliases,
                )?);
            }
            _ => {}
        }
    }
    Ok(options)
}

fn parse_route_handler_response_status(
    source: &str,
    number_aliases: &BTreeMap<String, u16>,
) -> Option<u16> {
    let source = source.trim().trim_end_matches(',').trim();
    parse_route_handler_literal_status(source)
        .or_else(|| number_aliases.get(source).copied())
        .filter(|status| route_handler_valid_http_status(*status))
}

fn parse_route_handler_literal_status(source: &str) -> Option<u16> {
    parse_literal_value(source)
        .ok()
        .and_then(|value| value.as_u64())
        .and_then(|value| u16::try_from(value).ok())
}

fn route_handler_valid_http_status(status: u16) -> bool {
    (100..=599).contains(&status)
}

fn route_handler_response_header_aliases(
    function_body: &str,
) -> BTreeMap<String, BTreeMap<String, String>> {
    let mut aliases = BTreeMap::new();
    let Ok(alias_re) =
        regex::Regex::new(r#"(?m)\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*=\s*([^;\n]+)"#)
    else {
        return aliases;
    };
    for capture in alias_re.captures_iter(function_body) {
        let Some(alias) = capture.get(1).map(|value| value.as_str().to_string()) else {
            continue;
        };
        let Some(source) = capture.get(2).map(|value| value.as_str().trim()) else {
            continue;
        };
        let header_source = source.starts_with("new Headers")
            || strip_object_braces(source).is_some()
            || strip_array_brackets(source).is_some();
        if !header_source {
            continue;
        }
        let headers = parse_safe_response_header_entries_raw(source).unwrap_or_default();
        aliases.insert(alias, headers);
    }

    for mutation in route_handler_response_header_mutations(function_body) {
        let Some(headers) = aliases.get_mut(&mutation.alias) else {
            continue;
        };
        apply_safe_response_header_mutation(
            headers,
            &mutation.method,
            mutation.name,
            mutation.value,
        );
    }
    aliases
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RouteHandlerResponseHeaderMutation {
    alias: String,
    method: String,
    name: String,
    value: String,
}

fn route_handler_response_header_mutations(
    function_body: &str,
) -> Vec<RouteHandlerResponseHeaderMutation> {
    let Ok(mutation_re) =
        regex::Regex::new(r#"\b([A-Za-z_$][A-Za-z0-9_$]*)\s*\.\s*(set|append)\s*\("#)
    else {
        return Vec::new();
    };
    let mut mutations = Vec::new();
    for capture in mutation_re.captures_iter(function_body) {
        let Some(alias) = capture.get(1).map(|value| value.as_str().to_string()) else {
            continue;
        };
        let Some(method) = capture.get(2).map(|value| value.as_str().to_string()) else {
            continue;
        };
        let Some(full_match) = capture.get(0) else {
            continue;
        };
        let paren = full_match.end().saturating_sub(1);
        let Some(end) = find_balanced_delimiter(function_body, paren, '(', ')') else {
            continue;
        };
        let args = split_top_level_entries(&function_body[paren + 1..end]);
        if args.len() < 2 {
            continue;
        }
        let Ok(Value::String(name)) = parse_literal_value(args[0]) else {
            continue;
        };
        let name = name.to_ascii_lowercase();
        if name.is_empty() || !safe_response_header_name(&name) {
            continue;
        }
        let Ok(Value::String(value)) = parse_literal_value(args[1]) else {
            continue;
        };
        mutations.push(RouteHandlerResponseHeaderMutation {
            alias,
            method,
            name,
            value,
        });
    }
    mutations
}

fn apply_safe_response_header_mutation(
    headers: &mut BTreeMap<String, String>,
    method: &str,
    name: String,
    value: String,
) {
    if method == "append" {
        if let Some(existing) = headers.get_mut(&name) {
            if name == "set-cookie" {
                existing.push('\n');
            } else {
                existing.push_str(", ");
            }
            existing.push_str(&value);
            return;
        }
    }
    headers.insert(name, value);
}

fn parse_safe_response_headers(source: &str) -> Result<BTreeMap<String, String>, String> {
    let mut headers = BTreeMap::new();
    for entry in split_top_level_entries(source) {
        let Some((key, value)) = entry.split_once(':') else {
            continue;
        };
        let key = key
            .trim()
            .trim_matches('"')
            .trim_matches('\'')
            .to_ascii_lowercase();
        if key.is_empty() || !safe_response_header_name(&key) {
            continue;
        }
        let value = parse_literal_value(value.trim())?;
        if let Some(value) = value.as_str() {
            headers.insert(key, value.to_string());
        }
    }
    Ok(headers)
}

fn parse_safe_response_header_entries(
    source: &str,
    header_aliases: &BTreeMap<String, BTreeMap<String, String>>,
) -> Result<BTreeMap<String, String>, String> {
    let source = source.trim().trim_end_matches(',').trim();
    if let Some(headers) = header_aliases.get(source) {
        return Ok(headers.clone());
    }
    parse_safe_response_header_entries_raw(source)
}

fn parse_safe_response_header_entries_raw(
    source: &str,
) -> Result<BTreeMap<String, String>, String> {
    let source = source.trim();
    if let Some(args) = find_first_call_args(source, &["new Headers"]) {
        if let Some(first_arg) = args.first() {
            if let Some(headers) = strip_object_braces(first_arg) {
                return parse_safe_response_headers(headers);
            }
            if let Some(entries) = strip_array_brackets(first_arg) {
                return parse_safe_response_header_tuple_array(entries);
            }
        }
        return Ok(BTreeMap::new());
    }
    if let Some(headers) = strip_object_braces(source) {
        return parse_safe_response_headers(headers);
    }
    if let Some(entries) = strip_array_brackets(source) {
        return parse_safe_response_header_tuple_array(entries);
    }
    Ok(BTreeMap::new())
}

fn parse_safe_response_header_tuple_array(
    source: &str,
) -> Result<BTreeMap<String, String>, String> {
    let mut headers = BTreeMap::new();
    for entry in split_top_level_entries(source) {
        let Some(tuple) = strip_array_brackets(entry) else {
            continue;
        };
        let parts = split_top_level_entries(tuple);
        if parts.len() != 2 {
            continue;
        }
        let key = parse_literal_value(parts[0])?;
        let Some(key) = key.as_str().map(str::to_ascii_lowercase) else {
            continue;
        };
        if key.is_empty() || !safe_response_header_name(&key) {
            continue;
        }
        let value = parse_literal_value(parts[1])?;
        if let Some(value) = value.as_str() {
            headers.insert(key, value.to_string());
        }
    }
    Ok(headers)
}

fn safe_response_header_name(name: &str) -> bool {
    matches!(
        name,
        "cache-control" | "content-type" | "location" | "vary" | "etag" | "set-cookie"
    ) || name.starts_with("x-")
}

fn parse_redirect_target(
    source: &str,
    request: &DxReactRouteHandlerRequest,
    request_url_aliases: &BTreeMap<String, String>,
) -> Result<String, String> {
    let source = source.trim();
    if let Ok(Value::String(value)) = parse_literal_value(source) {
        return Ok(value);
    }
    if matches!(
        request_url_aliases.get(source).map(String::as_str),
        Some("path" | "url")
    ) {
        return Ok(request.path.clone());
    }
    if source == "request.url" {
        return Ok(request.path.clone());
    }
    if source.starts_with("new URL(") {
        let args = find_first_call_args(source, &["new URL"])
            .ok_or_else(|| "unsupported redirect URL expression".to_string())?;
        if let Some(first) = args.first() {
            if let Ok(Value::String(value)) = parse_literal_value(first) {
                return Ok(value);
            }
        }
    }
    Err(format!("unsupported redirect target `{source}`"))
}

fn json_path_value(value: &Value, path: &str) -> Value {
    json_path_value_or_missing(value, path).unwrap_or(Value::Null)
}

fn json_path_value_or_missing(value: &Value, path: &str) -> Option<Value> {
    let mut current = value;
    for segment in path.split('.') {
        if segment.is_empty() {
            return None;
        }
        match current {
            Value::Object(map) => {
                let next = map.get(segment)?;
                current = next;
            }
            _ => return None,
        }
    }
    Some(current.clone())
}

fn split_top_level_entries(source: &str) -> Vec<&str> {
    let mut entries = Vec::new();
    let mut start = 0usize;
    let mut cursor = 0usize;
    let mut quote = None;
    let mut depth = 0usize;
    while cursor < source.len() {
        let ch = source[cursor..].chars().next().unwrap_or_default();
        if let Some(active_quote) = quote {
            if ch == active_quote {
                quote = None;
            } else if ch == '\\' {
                cursor += ch.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..]
                        .chars()
                        .next()
                        .unwrap_or_default()
                        .len_utf8();
                    continue;
                }
            }
            cursor += ch.len_utf8();
            continue;
        }
        match ch {
            '"' | '\'' | '`' => quote = Some(ch),
            '{' | '[' | '(' => depth += 1,
            '}' | ']' | ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                let entry = source[start..cursor].trim();
                if !entry.is_empty() {
                    entries.push(entry);
                }
                start = cursor + ch.len_utf8();
            }
            _ => {}
        }
        cursor += ch.len_utf8();
    }
    let entry = source[start..].trim();
    if !entry.is_empty() {
        entries.push(entry);
    }
    entries
}

fn parse_literal_value(source: &str) -> Result<Value, String> {
    let source = source
        .trim()
        .trim_end_matches(';')
        .trim()
        .trim_end_matches(" as const")
        .trim();
    if matches!(source, "true" | "false") {
        return Ok(Value::Bool(source == "true"));
    }
    if source == "null" {
        return Ok(Value::Null);
    }
    if let Some((value, _)) = parse_quoted_value(source) {
        return Ok(Value::String(value));
    }
    if let Ok(value) = source.parse::<i64>() {
        return Ok(Value::Number(value.into()));
    }
    Err(format!("unsupported route handler literal `{source}`"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RouteLoaderImport {
    local_name: String,
    export_name: String,
    source_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RouteLoaderCall {
    binding: String,
    local_name: String,
}

fn route_loader_imports(route_source: &str) -> Vec<RouteLoaderImport> {
    let module = parse_tsx_module("app/page.tsx", route_source);
    let mut imports = module
        .imports
        .into_iter()
        .filter(|import| import.source.contains("server/loaders"))
        .flat_map(|import| {
            let source_path = loader_source_path(&import.source);
            let mut imported = import
                .specifiers
                .into_iter()
                .filter(|specifier| !specifier.type_only)
                .map(|specifier| RouteLoaderImport {
                    local_name: specifier.local,
                    export_name: specifier.imported,
                    source_path: source_path.clone(),
                })
                .collect::<Vec<_>>();
            if let Some(default) = import.default.filter(|_| !import.type_only) {
                imported.push(RouteLoaderImport {
                    local_name: default,
                    export_name: "default".to_string(),
                    source_path,
                });
            }
            imported
        })
        .collect::<Vec<_>>();
    imports.sort_by(|left, right| left.local_name.cmp(&right.local_name));
    imports.dedup();
    imports
}

fn loader_source_path(import_source: &str) -> String {
    if import_source.ends_with(".ts") {
        import_source
            .trim_start_matches("../")
            .trim_start_matches("./")
            .trim_start_matches("@/")
            .to_string()
    } else {
        format!(
            "{}.ts",
            import_source
                .trim_start_matches("../")
                .trim_start_matches("./")
                .trim_start_matches("@/")
        )
    }
}

fn route_loader_calls(route_source: &str) -> Vec<RouteLoaderCall> {
    let Ok(re) = regex::Regex::new(
        r#"\b(?:const|let)\s+([A-Za-z_$][A-Za-z0-9_$]*)\s*=\s*(?:await\s+)?([A-Za-z_$][A-Za-z0-9_$]*)\s*\(\s*\)"#,
    ) else {
        return Vec::new();
    };
    let mut calls = re
        .captures_iter(route_source)
        .filter_map(|capture| {
            Some(RouteLoaderCall {
                binding: capture.get(1)?.as_str().to_string(),
                local_name: capture.get(2)?.as_str().to_string(),
            })
        })
        .collect::<Vec<_>>();
    calls.sort_by(|left, right| left.binding.cmp(&right.binding));
    calls.dedup();
    calls
}

fn parse_quoted_value(source: &str) -> Option<(String, usize)> {
    let quote = source.chars().next()?;
    if !matches!(quote, '"' | '\'') {
        return None;
    }
    let mut cursor = quote.len_utf8();
    let start = cursor;
    while cursor < source.len() {
        let ch = source[cursor..].chars().next()?;
        if ch == quote {
            return Some((source[start..cursor].to_string(), cursor + ch.len_utf8()));
        }
        if ch == '\\' {
            cursor += ch.len_utf8();
            if cursor < source.len() {
                cursor += source[cursor..].chars().next()?.len_utf8();
                continue;
            }
        }
        cursor += ch.len_utf8();
    }
    None
}

fn find_balanced_block(source: &str, cursor: usize) -> Option<usize> {
    find_balanced_delimiter(source, cursor, '{', '}')
}

fn find_balanced_delimiter(
    source: &str,
    mut cursor: usize,
    open: char,
    close: char,
) -> Option<usize> {
    let mut quote = None;
    let mut depth = 0usize;
    while cursor < source.len() {
        let ch = source[cursor..].chars().next()?;
        if let Some(active_quote) = quote {
            if ch == active_quote {
                quote = None;
            } else if ch == '\\' {
                cursor += ch.len_utf8();
                if cursor < source.len() {
                    cursor += source[cursor..].chars().next()?.len_utf8();
                    continue;
                }
            }
            cursor += ch.len_utf8();
            continue;
        }
        match ch {
            '"' | '\'' | '`' => quote = Some(ch),
            _ if ch == open => depth += 1,
            _ if ch == close => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(cursor);
                }
            }
            _ => {}
        }
        cursor += ch.len_utf8();
    }
    None
}

fn exported_functions(source: &str) -> Vec<ExportedFunction> {
    let mut exports = Vec::new();
    if let Ok(function_re) = regex::Regex::new(
        r#"(?m)export\s+(async\s+)?function\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(([^)]*)\)"#,
    ) {
        exports.extend(function_re.captures_iter(source).filter_map(|capture| {
            Some(ExportedFunction {
                async_export: capture.get(1).is_some(),
                name: capture.get(2)?.as_str().to_string(),
                parameters: capture
                    .get(3)
                    .map(|parameters| parameters.as_str().trim().to_string())
                    .unwrap_or_default(),
            })
        }));
    }

    if let Ok(const_re) =
        regex::Regex::new(r#"(?m)export\s+const\s+([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(async\s+)?"#)
    {
        exports.extend(const_re.captures_iter(source).filter_map(|capture| {
            let name = capture.get(1)?.as_str().to_string();
            let (async_export, parameters) = exported_const_callable_signature(source, &name)
                .unwrap_or((capture.get(2).is_some(), String::new()));
            Some(ExportedFunction {
                async_export,
                name,
                parameters,
            })
        }));
    }

    exports
}

fn exported_const_callable_signature(source: &str, name: &str) -> Option<(bool, String)> {
    let expression = exported_const_handler_expression(source, name)?;
    let trimmed = expression.trim_start();
    let async_export = trimmed.starts_with("async ");
    if let Some(parameters) = parenthesized_callable_parameters(trimmed) {
        return Some((
            async_export || trimmed.starts_with("async function"),
            parameters,
        ));
    }
    let arrow_index = trimmed.find("=>")?;
    let candidate = trimmed[..arrow_index].trim();
    let candidate = candidate.strip_prefix("async ").unwrap_or(candidate).trim();
    (!candidate.is_empty()
        && candidate
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '$')))
    .then(|| (async_export, candidate.to_string()))
}

fn parenthesized_callable_parameters(expression: &str) -> Option<String> {
    let parameters_start = expression.find('(')?;
    let prefix = expression[..parameters_start].trim();
    let parameters_end = find_balanced_delimiter(expression, parameters_start, '(', ')')?;
    let after_parameters = expression[parameters_end + 1..].trim_start();
    if prefix.ends_with("function")
        || prefix == "async function"
        || after_parameters.starts_with("=>")
    {
        return Some(
            expression[parameters_start + 1..parameters_end]
                .trim()
                .to_string(),
        );
    }
    None
}

fn route_endpoint(source_path: &str) -> Option<String> {
    let path = source_path.replace('\\', "/");
    let path = APP_ROUTE_HANDLER_ROOTS
        .iter()
        .find_map(|root| path.strip_prefix(root))?;
    let filename = route_handler_suffix(path)?;
    let route_path = &path[..path.len().saturating_sub(filename.len() + 1)];
    let route_segments = route_endpoint_segments(route_path);
    Some(format!("/{}", route_segments.join("/")))
}

fn route_handler_suffix(path: &str) -> Option<&'static str> {
    ROUTE_HANDLER_FILENAMES.iter().copied().find(|filename| {
        path.len() > filename.len()
            && path.ends_with(filename)
            && path.as_bytes()[path.len() - filename.len() - 1] == b'/'
    })
}

fn route_endpoint_segments(route_path: &str) -> Vec<&str> {
    route_path
        .split('/')
        .filter(|segment| route_endpoint_segment_visible(segment))
        .collect()
}

fn route_endpoint_segment_visible(segment: &str) -> bool {
    !(segment.is_empty()
        || segment.starts_with('@')
        || segment.starts_with('(') && segment.ends_with(')') && !segment.starts_with("(."))
}

fn route_endpoint_matches(pattern: &str, actual: &str) -> bool {
    let actual = route_request_path_for_match(actual);
    let pattern_segments = pattern
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let actual_segments = actual
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let mut actual_index = 0usize;
    for pattern_segment in pattern_segments {
        if pattern_segment.starts_with("[[...") && pattern_segment.ends_with("]]") {
            return true;
        }
        if pattern_segment.starts_with("[...") && pattern_segment.ends_with(']') {
            return actual_index < actual_segments.len();
        }
        let Some(actual_segment) = actual_segments.get(actual_index) else {
            return false;
        };
        if !(pattern_segment.starts_with('[') && pattern_segment.ends_with(']'))
            && pattern_segment != *actual_segment
        {
            return false;
        }
        actual_index += 1;
    }
    actual_index == actual_segments.len()
}

fn route_request_path_for_match(actual: &str) -> &str {
    let actual = route_request_absolute_url_path(actual);
    let actual = actual
        .split_once('?')
        .map(|(path, _)| path)
        .unwrap_or(actual);
    actual
        .split_once('#')
        .map(|(path, _)| path)
        .unwrap_or(actual)
}

fn route_request_search(actual: &str) -> String {
    let actual = route_request_absolute_url_path(actual);
    let Some((_, query_and_hash)) = actual.split_once('?') else {
        return String::new();
    };
    let query = query_and_hash
        .split_once('#')
        .map(|(query, _)| query)
        .unwrap_or(query_and_hash);
    if query.is_empty() {
        String::new()
    } else {
        format!("?{query}")
    }
}

fn route_request_search_params(request: &DxReactRouteHandlerRequest) -> BTreeMap<String, String> {
    let mut params = route_request_query_params(&request.path);
    params.extend(request.search_params.clone());
    params
}

fn route_request_query_params(actual: &str) -> BTreeMap<String, String> {
    let mut params = BTreeMap::new();
    let actual = route_request_absolute_url_path(actual);
    let Some((_, query_and_hash)) = actual.split_once('?') else {
        return params;
    };
    let query = query_and_hash
        .split_once('#')
        .map(|(query, _)| query)
        .unwrap_or(query_and_hash);
    for pair in query.split('&').filter(|pair| !pair.is_empty()) {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        let key = route_request_decode_query_component(key);
        if !key.is_empty() {
            params.insert(key, route_request_decode_query_component(value));
        }
    }
    params
}

fn route_request_decode_query_component(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0usize;

    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                if let Some(byte) =
                    route_request_decode_hex_pair(bytes[index + 1], bytes[index + 2])
                {
                    decoded.push(byte);
                    index += 3;
                } else {
                    decoded.push(bytes[index]);
                    index += 1;
                }
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }

    String::from_utf8_lossy(&decoded).into_owned()
}

fn route_request_decode_hex_pair(high: u8, low: u8) -> Option<u8> {
    Some(route_request_hex_value(high)? << 4 | route_request_hex_value(low)?)
}

fn route_request_hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn route_request_absolute_url_path(actual: &str) -> &str {
    let Some(scheme_end) = actual.find("://") else {
        return actual;
    };
    let after_scheme = scheme_end + 3;
    let authority_and_path = &actual[after_scheme..];
    let Some(path_start) = authority_and_path.find('/') else {
        return "/";
    };
    &authority_and_path[path_start..]
}

fn http_method(name: &str) -> Option<&'static str> {
    match name {
        "GET" => Some("GET"),
        "POST" => Some("POST"),
        "PUT" => Some("PUT"),
        "PATCH" => Some("PATCH"),
        "DELETE" => Some("DELETE"),
        "OPTIONS" => Some("OPTIONS"),
        "HEAD" => Some("HEAD"),
        _ => None,
    }
}

fn route_handler_export_method<'a>(source: &str, request_method: &'a str) -> &'a str {
    if request_method == "HEAD" && route_handler_method_is_exported(source, "HEAD") {
        "HEAD"
    } else if request_method == "HEAD" {
        "GET"
    } else {
        request_method
    }
}

fn route_handler_missing_export_message(request_method: &str) -> String {
    if request_method == "HEAD" {
        "route handler does not export `HEAD` or fallback `GET`".to_string()
    } else {
        format!("route handler does not export `{request_method}`")
    }
}

fn request_serialization(parameters: &str) -> String {
    if parameters.trim().is_empty() {
        "json-value".to_string()
    } else if parameters.contains("Request") || parameters.contains("request") {
        "web-request".to_string()
    } else if action_request_schema(parameters).mode == "typed-object" {
        "typed-json-object".to_string()
    } else {
        "json-value".to_string()
    }
}

fn response_serialization(source: &str) -> String {
    if source.contains("Response.json") {
        "json-response".to_string()
    } else if source.contains("new Response") {
        "web-response".to_string()
    } else if source.contains("return {") {
        "json-object".to_string()
    } else {
        "json-value".to_string()
    }
}

struct ExportedFunction {
    name: String,
    parameters: String,
    async_export: bool,
}
