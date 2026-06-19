import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");

function readProductionAppApiRoutes() {
  const source = readFileSync(resolve(repoRoot, "dx-www/src/cli/app_api_routes.rs"), "utf8");
  return source.split("#[cfg(test)]")[0];
}

function readRouteRequestValues() {
  return readFileSync(resolve(repoRoot, "dx-www/src/cli/route_request_values.rs"), "utf8");
}

function readServerContract() {
  return readFileSync(resolve(repoRoot, "core/src/delivery/server_contract.rs"), "utf8");
}

function readServerTests() {
  return readFileSync(resolve(repoRoot, "core/src/delivery/tests.rs"), "utf8");
}

function readDeliveryModule() {
  return readFileSync(resolve(repoRoot, "core/src/delivery/mod.rs"), "utf8");
}

function readHttpJsonRouteHandler() {
  return readFileSync(resolve(repoRoot, "core/src/delivery/route_handler_http_json.rs"), "utf8");
}

function readRouteHandlerRuntimeEnv() {
  return readFileSync(resolve(repoRoot, "dx-www/src/cli/route_handler_runtime_env.rs"), "utf8");
}

test("App API route matcher accepts Next-familiar route handlers", () => {
  const appApiRoutes = readProductionAppApiRoutes();
  const routeRequestValues = readRouteRequestValues();
  const serverContract = readServerContract();
  const serverTests = readServerTests();

  assert.match(appApiRoutes, /ROUTE_HANDLER_FILENAMES/);
  assert.match(appApiRoutes, /APP_API_ROUTE_ROOTS/);
  assert.match(appApiRoutes, /"app\/api"/);
  assert.match(appApiRoutes, /"src\/app\/api"/);
  for (const filename of ["route.ts", "route.tsx", "route.js", "route.jsx"]) {
    assert.match(appApiRoutes, new RegExp(`"${filename.replace(".", "\\.")}"`));
  }
  assert.match(serverContract, /ROUTE_HANDLER_FILENAMES/);
  assert.match(serverContract, /APP_ROUTE_HANDLER_ROOTS/);
  assert.match(serverContract, /"app\/"/);
  assert.match(serverContract, /"src\/app\/"/);
  for (const filename of ["route.ts", "route.tsx", "route.js", "route.jsx"]) {
    assert.match(serverContract, new RegExp(`"${filename.replace(".", "\\.")}"`));
  }
  assert.match(serverContract, /route_handler_suffix/);
  assert.match(serverContract, /route_endpoint_segments/);
  assert.match(serverContract, /route_endpoint_segment_visible/);
  assert.match(serverContract, /segment\.starts_with\('\('\)/);
  assert.match(serverContract, /segment\.ends_with\('\)'\)/);
  assert.match(serverContract, /!segment\.starts_with\("\(\."\)/);
  assert.match(serverContract, /segment\.starts_with\('@'\)/);
  assert.match(serverContract, /route_request_path_for_match/);
  assert.match(serverContract, /route_request_absolute_url_path/);
  assert.match(serverContract, /find\(":\/\/"\)/);
  assert.match(
    serverContract,
    /fn trpc_route_handler_compat_response\([\s\S]*let path = route_request_path_for_match\(&request\.path\);/,
  );
  assert.match(serverContract, /parse_route_handler_not_found_response/);
  assert.match(
    serverContract,
    /parse_route_handler_not_found_response\(body\)\?[\s\S]*parse_route_handler_redirect_response\(body, &request\)\?/,
  );
  assert.match(
    serverContract,
    /parse_redirect_target\(\s*target_arg,\s*request,\s*&route_handler_request_url_aliases\(function_body\),\s*\)\?/,
  );
  assert.match(serverContract, /parse_route_handler_redirect_options/);
  assert.match(serverContract, /find_first_call_args\(function_body, &\["notFound"\]\)/);
  assert.match(serverContract, /"x-dx-route-handler-not-found"\.to_string\(\)/);
  assert.match(serverContract, /parse_route_handler_web_response/);
  assert.match(
    serverContract,
    /parse_route_handler_web_response\(body, &request, &context_bindings\)\?[\s\S]*parse_route_handler_json_response\(body, &request, &context_bindings\)\?/,
  );
  assert.match(serverContract, /parse_route_handler_conditional_response/);
  assert.match(serverContract, /route_handler_else_branch_source/);
  assert.match(serverContract, /route_handler_source_starts_with_if/);
  assert.match(serverContract, /route_handler_condition_value/);
  assert.match(serverContract, /route_handler_condition_expression/);
  assert.match(serverContract, /route_handler_condition_comparison_value/);
  assert.match(serverContract, /route_handler_condition_logical_value/);
  assert.match(serverContract, /route_handler_method_aliases/);
  assert.match(serverContract, /route_handler_destructured_method_aliases/);
  assert.match(serverContract, /route_handler_request_url_aliases/);
  assert.match(serverContract, /route_handler_destructured_request_url_aliases/);
  assert.match(serverContract, /route_handler_nested_conditional_response/);
  assert.match(serverContract, /parse_route_handler_json_response_body/);
  assert.match(
    serverContract,
    /parse_route_handler_json_response_body\(\s*body_arg,\s*request,\s*&body_aliases,\s*&cookie_aliases,\s*context_bindings,\s*\)\?/,
  );
  assert.match(serverContract, /parse_route_handler_value\(\s*source,\s*request,\s*body_aliases,/);
  assert.match(serverContract, /route_handler_body_aliases/);
  assert.match(serverContract, /route_handler_body_alias_value/);
  assert.match(serverContract, /route_handler_json_body_aliases/);
  assert.match(
    serverContract,
    /route_handler_body_method_aliases\(\s*function_body,\s*body_roots,\s*&\["text", "formData"\]/,
  );
  assert.match(serverContract, /route_handler_form_data_read/);
  assert.match(serverContract, /route_handler_direct_form_data_get_name/);
  assert.match(
    serverContract,
    /route_handler_form_data_read\(\s*expression,\s*request,\s*&context_bindings\.form_data_aliases,\s*&context_bindings\.body_roots/s,
  );
  assert.match(serverContract, /route_handler_search_param_read/);
  assert.match(serverContract, /route_handler_search_param_aliases/);
  assert.match(serverContract, /route_handler_header_read/);
  assert.match(serverContract, /route_handler_header_aliases/);
  assert.match(serverContract, /route_handler_response_number_aliases/);
  assert.match(serverContract, /parse_route_handler_response_status/);
  assert.match(serverContract, /parse_safe_response_header_entries/);
  assert.match(serverContract, /parse_safe_response_header_tuple_array/);
  assert.match(serverContract, /new Headers/);
  assert.match(serverContract, /route_handler_url_property_read/);
  assert.match(serverContract, /route_handler_destructured_url_property_aliases/);
  assert.match(serverContract, /route_request_search/);
  assert.match(serverTests, /react_route_handler_runtime_supports_safe_response_status_aliases/);
  assert.match(
    serverTests,
    /react_route_handler_runtime_serializes_request_json_body_nullish_defaults/,
  );
  assert.match(serverTests, /react_route_handler_runtime_serializes_request_form_data_alias/);
  assert.match(
    serverTests,
    /directEmail: \(await request\.formData\(\)\)\.get\("email"\) \?\? "missing"/,
  );
  assert.match(
    serverTests,
    /cloneEmail: \(await request\.clone\(\)\.formData\(\)\)\.get\("email"\) \?\? "missing"/,
  );
  assert.match(
    serverTests,
    /clonedEmail: \(await cloned\.formData\(\)\)\.get\("email"\) \?\? "missing"/,
  );
  assert.match(
    serverTests,
    /react_route_handler_runtime_evaluates_request_method_alias/,
  );
  assert.match(
    serverTests,
    /react_route_handler_runtime_evaluates_destructured_request_method_alias/,
  );
  assert.match(
    serverTests,
    /react_route_handler_runtime_evaluates_destructured_request_url_alias/,
  );
  assert.match(
    serverTests,
    /react_route_handler_runtime_evaluates_simple_conditional_json_returns/,
  );
  assert.match(
    serverTests,
    /react_route_handler_runtime_evaluates_else_if_conditional_json_returns/,
  );
  assert.match(
    serverTests,
    /react_route_handler_runtime_evaluates_compound_conditional_json_returns/,
  );
  assert.match(
    serverTests,
    /react_route_handler_runtime_evaluates_parenthesized_conditional_json_returns/,
  );
  assert.match(
    serverTests,
    /react_route_handler_runtime_evaluates_literal_first_conditional_json_returns/,
  );
  assert.match(
    serverTests,
    /react_route_handler_runtime_evaluates_nested_conditional_json_returns/,
  );
  assert.match(serverTests, /react_route_handler_runtime_supports_static_headers_tuple_arrays/);
  assert.match(serverTests, /react_route_handler_runtime_supports_redirect_response_init_options/);
  assert.match(serverTests, /react_route_handler_runtime_redirects_to_request_url_alias/);
  assert.match(
    serverTests,
    /react_route_handler_runtime_reads_next_request_url_properties_without_next_runtime/,
  );
  assert.match(
    serverTests,
    /react_route_handler_runtime_reads_destructured_next_request_url_properties_without_next_runtime/,
  );
  assert.match(serverTests, /const \{ pathname, search: query, href \} = request\.nextUrl;/);
  assert.match(serverContract, /request\.nextUrl/);
  assert.match(serverContract, /new URL\(request\.url\)\.searchParams/);
  assert.match(serverContract, /find_first_call_args\(function_body, &\["new Response"\]\)/);
  assert.match(serverContract, /JSON\.stringify/);
  assert.match(serverContract, /text\/plain; charset=utf-8/);
  assert.doesNotMatch(
    serverContract,
    /let path = request\.path\.split\('\?'\)\.next\(\)\.unwrap_or\(&request\.path\);/,
  );
  assert.match(serverContract, /split_once\('\?'\)/);
  assert.match(serverContract, /split_once\('#'\)/);
  assert.match(serverContract, /route_endpoint_matches/);
  assert.match(appApiRoutes, /route_handler_file_in/);
  assert.match(appApiRoutes, /is_app_router_route_group_segment/);
  assert.match(appApiRoutes, /segment\.starts_with\('\('\)/);
  assert.match(appApiRoutes, /segment\.ends_with\('\)'\)/);
  assert.match(appApiRoutes, /!segment\.starts_with\("\(\."\)/);
  assert.match(appApiRoutes, /decode_path_segment/);
  assert.match(appApiRoutes, /decode_path_segments/);
  assert.match(appApiRoutes, /parse_search_params/);
  assert.match(routeRequestValues, /percent_decode_component/);
  assert.match(routeRequestValues, /b'\+' if plus_as_space/);
  assert.match(routeRequestValues, /decode_hex_pair/);
  assert.doesNotMatch(appApiRoutes, /entry\.file_name\(\)\s*!=\s*"route\.ts"/);
});

test("Route handler method guard responses expose DX-owned runtime boundary", () => {
  const serverContract = readServerContract();
  const serverTests = readServerTests();
  const guardStart = serverContract.indexOf("fn automatic_route_handler_options_response");
  const guardEnd = serverContract.indexOf("fn route_handler_allowed_methods");

  assert.notEqual(guardStart, -1);
  assert.notEqual(guardEnd, -1);

  const guardResponses = serverContract.slice(guardStart, guardEnd);

  assert.doesNotMatch(guardResponses, /fullNextRouteHandlerParity/);
  assert.match(guardResponses, /"runtimeBoundary"/);
  assert.match(guardResponses, /"sourceOwned": true/);
  assert.match(guardResponses, /"externalRuntimeRequired": false/);
  assert.match(guardResponses, /"externalRuntimeExecuted": false/);
  assert.match(serverTests, /response\.body\["runtimeBoundary"\]\["sourceOwned"\]/);
  assert.match(serverTests, /response\.body\.get\("fullNextRouteHandlerParity"\)\.is_none\(\)/);
});

test("HTTP JSON route options are a source-owned WWW route capability", () => {
  const deliveryModule = readDeliveryModule();
  const serverContract = readServerContract();
  const serverTests = readServerTests();
  const httpJsonRouteHandler = readHttpJsonRouteHandler();
  const routeHandlerRuntimeEnv = readRouteHandlerRuntimeEnv();

  assert.match(deliveryModule, /mod route_handler_http_json;/);
  assert.match(serverContract, /http_json_route_handler_response/);
  assert.match(
    serverContract,
    /http_json_route_handler_response\(&source\.source, body, &request\)/,
  );
  assert.match(
    serverContract,
    /http_json_route_handler_response\(source, expression, request\)/,
  );

  for (const marker of [
    "createDxHttpJsonRoute",
    "createDxHttpJsonRouteResponse",
    "DX_HTTP_JSON_ALLOWED_ORIGINS",
    "allowedOrigins",
    "requiredSearchParams",
    "proxyRequestForwarded",
    "upstreamRequestAllowed",
    "source-owned-http-json-route-policy-interpreter",
  ]) {
    assert.match(httpJsonRouteHandler, new RegExp(marker));
  }

  assert.match(
    httpJsonRouteHandler,
    /"networkCalls": false[\s\S]*"runtimeExecution": false/,
  );
  assert.doesNotMatch(httpJsonRouteHandler, /\bfetch\(|\breqwest::|\bureq::/);
  assert.match(routeHandlerRuntimeEnv, /DX_HTTP_JSON_ALLOWED_ORIGINS/);
  assert.match(serverTests, /react_route_handler_runtime_accepts_http_json_route_policy/);
  assert.match(
    serverTests,
    /react_route_handler_runtime_accepts_http_json_route_policy_from_runtime_env/,
  );
  assert.match(serverTests, /react_route_handler_runtime_rejects_http_json_route_missing_query/);
  assert.match(serverTests, /react_route_handler_runtime_accepts_http_json_factory_export/);
});
