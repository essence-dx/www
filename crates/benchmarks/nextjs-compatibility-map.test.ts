const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const compatibilityMapPath = path.join(root, "docs", "NEXTJS_COMPATIBILITY_MAP.md");
const appRouterExecutionPath = path.join(root, "dx-www", "src", "cli", "app_router_execution.rs");
const cliPath = path.join(root, "dx-www", "src", "cli", "mod.rs");
const appRouterMetadataPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "metadata.rs",
);
const appApiRoutesPath = path.join(root, "dx-www", "src", "cli", "app_api_routes.rs");
const serverContractPath = path.join(root, "core", "src", "delivery", "server_contract.rs");
const deliveryTestsPath = path.join(root, "core", "src", "delivery", "tests.rs");
const routeHandlerReceiptPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_route_handler_receipt.rs",
);
const nextNavigationPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "next_navigation.rs",
);

function read(filePath) {
  assert.ok(fs.existsSync(filePath), `missing ${path.relative(root, filePath)}`);
  return fs.readFileSync(filePath, "utf8");
}

test("Next.js compatibility map records official mirror provenance and DX boundaries", () => {
  const docs = read(compatibilityMapPath);

  assert.match(docs, /Official upstream mirror/);
  assert.match(docs, /G:\\WWW\\inspirations\\nextjs/);
  assert.match(docs, /https:\/\/github\.com\/vercel\/next\.js\.git/);
  assert.match(docs, /MIT License/);
  assert.match(docs, /f3f56ecec2f3f8cefa0f0a1323ea406740251d5c/);

  for (const feature of [
    "App Router",
    "layouts",
    "route handlers",
    "metadata",
    "server/client components",
    "server actions",
    "redirects/not-found",
    "image/font/script boundaries",
    "middleware",
    "static export",
    "cache/revalidate",
  ]) {
    assert.match(docs, new RegExp(feature.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(docs, /Reuse directly/);
  assert.match(docs, /Adapter boundary/);
  assert.match(docs, /Not useful for DX architecture/);
  assert.match(docs, /Do not use Turborepo as DX architecture/);
  assert.match(docs, /source-owned `dx` file/);
  assert.match(docs, /Forge packages/);
  assert.match(docs, /dx-style/);
  assert.match(docs, /dx-check/);
  assert.match(docs, /DX Studio\/Zed preview/);
});

test("DX App Router exposes an upstream-informed Next navigation control-flow slice", () => {
  const execution = read(appRouterExecutionPath);
  const navigation = read(nextNavigationPath);

  assert.match(execution, /mod next_navigation;/);
  assert.match(execution, /build_next_navigation_control_flow/);
  assert.match(execution, /next_navigation_head_tags/);
  assert.match(execution, /"next_navigation_control_flow"/);
  assert.match(execution, /data-dx-next-navigation-control-flow/);
  assert.match(execution, /data-dx-next-redirect/);
  assert.match(execution, /data-dx-next-not-found/);

  assert.match(navigation, /dx\.next\.appRouterControlFlow/);
  assert.match(navigation, /NEXT_REDIRECT/);
  assert.match(navigation, /NEXT_HTTP_ERROR_FALLBACK/);
  assert.match(navigation, /TemporaryRedirect/);
  assert.match(navigation, /PermanentRedirect/);
  assert.match(navigation, /redirect\(\)/);
  assert.match(navigation, /permanentRedirect\(\)/);
  assert.match(navigation, /notFound\(\)/);
  assert.match(navigation, /next\/navigation/);
  assert.match(navigation, /meta http-equiv="refresh"/);
  assert.match(navigation, /meta name="robots" content="noindex"/);
  assert.match(navigation, /node_modules_required/);
  assert.match(navigation, /source_owned_control_flow/);
  assert.match(navigation, /external_runtime_required/);
  assert.match(navigation, /external_runtime_executed/);
});

test("DX App Router records safe generateMetadata as source-owned metadata compatibility", () => {
  const execution = read(appRouterExecutionPath);
  const metadata = read(appRouterMetadataPath);
  const docs = read(compatibilityMapPath);

  assert.match(execution, /mod metadata;/);
  assert.match(execution, /metadata_sources\([\s\S]*input\.route_params,[\s\S]*input\.search_params/);
  assert.match(execution, /effective_metadata/);

  assert.match(metadata, /safe_generate_metadata_source/);
  assert.match(metadata, /source_kind": "generateMetadata"/);
  assert.match(metadata, /safe-generate-metadata-literal-return/);
  assert.match(metadata, /safe-generate-metadata-request-literal-return/);
  assert.match(metadata, /request_prop_bindings/);
  assert.match(metadata, /params\.slug/);
  assert.match(metadata, /searchParams\.preview/);
  assert.match(metadata, /safe_metadata_template_literal/);
  assert.match(metadata, /source_owned_metadata": true/);
  assert.match(metadata, /external_runtime_required": false/);
  assert.match(metadata, /external_runtime_executed": false/);
  assert.match(metadata, /node_modules_required": false/);

  assert.match(docs, /safe literal `generateMetadata`/);
  assert.match(docs, /request-bound `generateMetadata`/);
  assert.match(docs, /streamed metadata/);
});

test("DX App Router records route handler request maps without Next runtime takeover", () => {
  const appApiRoutes = read(appApiRoutesPath);
  const serverContract = read(serverContractPath);
  const routeHandlerReceipt = read(routeHandlerReceiptPath);
  const cli = read(cliPath);
  const execution = read(appRouterExecutionPath);
  const docs = read(compatibilityMapPath);

  assert.match(appApiRoutes, /pub\(super\) struct AppApiRouteMatch/);
  assert.match(appApiRoutes, /pub\(super\) fn route_handler_match/);
  assert.match(appApiRoutes, /params: BTreeMap<String, String>/);
  assert.match(appApiRoutes, /search_params: BTreeMap<String, String>/);
  assert.match(appApiRoutes, /parse_search_params/);

  assert.match(serverContract, /pub route_params: BTreeMap<String, String>/);
  assert.match(serverContract, /pub search_params: BTreeMap<String, String>/);
  assert.match(serverContract, /"request\.params"/);
  assert.match(serverContract, /"request\.searchParams"/);
  assert.match(serverContract, /request\.route_params/);
  assert.match(serverContract, /request\.search_params/);
  assert.match(serverContract, /source-owned-safe-interpreter/);
  assert.match(serverContract, /lifecycle_scripts_executed: false/);
  assert.match(serverContract, /"dx\.next\.appRouteHandlerReceipt"\.to_string\(\)/);
  assert.doesNotMatch(serverContract, /dx\.next\.appRouteHandlerReceipt\.v1/);

  assert.match(
    routeHandlerReceipt,
    /APP_ROUTE_HANDLER_RECEIPT_SCHEMA: &str = "dx\.next\.appRouteHandlerReceipt"/,
  );
  assert.match(routeHandlerReceipt, /APP_ROUTE_HANDLER_RECEIPT_FORMAT: u8 = 1/);
  assert.match(routeHandlerReceipt, /"schema": APP_ROUTE_HANDLER_RECEIPT_SCHEMA/);
  assert.match(routeHandlerReceipt, /"format": APP_ROUTE_HANDLER_RECEIPT_FORMAT/);
  assert.doesNotMatch(routeHandlerReceipt, /\.v1/);
  assert.match(routeHandlerReceipt, /route_param_count/);
  assert.match(routeHandlerReceipt, /search_param_count/);
  assert.match(routeHandlerReceipt, /response_header_count/);
  assert.match(routeHandlerReceipt, /node_modules_required/);
  assert.match(routeHandlerReceipt, /"runtime_boundary":\s*\{/);
  assert.match(routeHandlerReceipt, /"source_owned":\s*true/);
  assert.match(routeHandlerReceipt, /"external_runtime_required":\s*false/);
  assert.match(routeHandlerReceipt, /"external_runtime_executed":\s*false/);
  assert.match(routeHandlerReceipt, /x-dx-route-handler-receipt/);
  assert.match(routeHandlerReceipt, /x-dx-route-handler-request-maps/);
  assert.match(routeHandlerReceipt, /x-dx-route-handler-source-owned/);
  assert.match(routeHandlerReceipt, /x-dx-external-runtime-required/);
  assert.match(routeHandlerReceipt, /x-dx-external-runtime-executed/);

  assert.match(cli, /mod app_route_handler_receipt;/);
  assert.match(cli, /build_app_route_handler_receipt/);
  assert.match(cli, /app_route_handler_receipt_headers/);
  assert.match(cli, /APP_ROUTE_HANDLER_RECEIPT_SCHEMA\.to_string\(\)/);
  assert.doesNotMatch(cli, /dx\.next\.appRouteHandlerReceipt\.v1/);
  assert.match(cli, /app_api_route_handler_match/);
  assert.match(cli, /route_params: route_handler_match\.params\.clone\(\)/);
  assert.match(cli, /search_params: route_handler_match\.search_params\.clone\(\)/);

  assert.match(execution, /node_modules_required": false|data-dx-node-modules-required="false"/);
  assert.match(docs, /route handler request maps/);
  assert.match(docs, /route-handler receipt headers/);
  assert.doesNotMatch(docs, /dx\.next\.appRouteHandlerReceipt\.v1/);
  assert.match(docs, /not unbounded Route Handler runtime coverage/);
});

test("DX App Router accepts safe route handler context params without Next runtime takeover", () => {
  const serverContract = read(serverContractPath);
  const deliveryTests = read(deliveryTestsPath);
  const docs = read(compatibilityMapPath);

  assert.match(serverContract, /route_handler_context_bindings/);
  assert.match(serverContract, /RouteHandlerContextBindings/);
  assert.match(serverContract, /context\.params/);
  assert.match(serverContract, /params\.slug/);
  assert.match(serverContract, /node_modules_required/);
  assert.doesNotMatch(serverContract, /next\/server.*Route Handler runtime/i);

  assert.match(
    deliveryTests,
    /react_route_handler_runtime_reads_next_context_params_without_next_runtime/,
  );
  assert.match(deliveryTests, /context\.params\.slug/);
  assert.match(deliveryTests, /params\.slug/);

  assert.match(docs, /safe route-handler context params/);
  assert.match(docs, /not unbounded Route Handler runtime coverage/);
});

test("DX App Router records safe async route handler params aliases without Next runtime takeover", () => {
  const serverContract = read(serverContractPath);
  const deliveryTests = read(deliveryTestsPath);
  const docs = read(compatibilityMapPath);

  assert.match(serverContract, /route_handler_context_param_aliases/);
  assert.match(serverContract, /route_param_aliases/);
  assert.match(serverContract, /await\\s\+/);
  assert.match(serverContract, /route_handler_context_alias_value/);
  assert.match(serverContract, /node_modules_required/);
  assert.doesNotMatch(serverContract, /next\/server.*Route Handler runtime/i);

  assert.match(
    deliveryTests,
    /react_route_handler_runtime_reads_async_context_param_aliases_without_next_runtime/,
  );
  assert.match(deliveryTests, /const \{ slug \} = await params/);
  assert.match(deliveryTests, /const slugAlias = context\.params\.slug/);

  assert.match(docs, /async route-handler params aliases/);
  assert.match(docs, /not unbounded Route Handler runtime coverage/);
});

test("DX App Router serves HEAD through safe GET handlers without Next runtime takeover", () => {
  const serverContract = read(serverContractPath);
  const deliveryTests = read(deliveryTestsPath);

  assert.match(serverContract, /route_handler_export_method/);
  assert.match(serverContract, /request_method == "HEAD"/);
  assert.match(serverContract, /exported_route_handler_body\(&source\.source, export_method\)/);
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_uses_get_handler_for_head_requests_without_next_runtime/,
  );
  assert.doesNotMatch(serverContract, /next\/server.*HEAD/i);
});

test("DX App Router answers OPTIONS with source-owned method discovery", () => {
  const serverContract = read(serverContractPath);
  const deliveryTests = read(deliveryTestsPath);

  assert.match(serverContract, /automatic_route_handler_options_response/);
  assert.match(serverContract, /route_handler_allowed_methods/);
  assert.match(serverContract, /"Allow"\.to_string\(\)/);
  assert.match(serverContract, /request\.method == "OPTIONS"/);
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_answers_options_from_exported_methods_without_next_runtime/,
  );
  assert.doesNotMatch(serverContract, /next\/server.*OPTIONS/i);
});

test("DX App Router returns source-owned 405 responses for unexported methods", () => {
  const serverContract = read(serverContractPath);
  const deliveryTests = read(deliveryTestsPath);

  assert.match(serverContract, /route_handler_method_not_allowed_response/);
  assert.match(serverContract, /status: 405/);
  assert.match(serverContract, /"methodNotAllowed"/);
  assert.match(serverContract, /"Allow"\.to_string\(\)/);
  assert.match(
    deliveryTests,
    /react_route_handler_runtime_returns_405_for_unexported_methods_without_next_runtime/,
  );
  assert.doesNotMatch(serverContract, /next\/server.*Method Not Allowed/i);
});
