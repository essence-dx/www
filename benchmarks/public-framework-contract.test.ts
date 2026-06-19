const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const cliPath = path.join(root, "dx-www", "src", "cli", "mod.rs");
const cliCoreImplPath = path.join(root, "dx-www", "src", "cli", "mod_parts", "cli_core_impl.rs");
const cliForgeCommandsCPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "mod_parts",
  "cli_forge_commands_c.rs",
);
const appApiRoutesPath = path.join(root, "dx-www", "src", "cli", "app_api_routes.rs");
const appPageRoutesPath = path.join(root, "dx-www", "src", "cli", "app_page_routes.rs");
const routeRequestValuesPath = path.join(root, "dx-www", "src", "cli", "route_request_values.rs");
const appRouterRuntimeCommandPath = path.join(root, "dx-www", "src", "cli", "app_router_runtime_command.rs");
const appRouterExecutionPath = path.join(root, "dx-www", "src", "cli", "app_router_execution.rs");
const appRouterRenderPlanPath = path.join(root, "dx-www", "src", "cli", "app_router_execution", "render_plan.rs");
const appRouterSourceRenderPath = path.join(root, "dx-www", "src", "cli", "app_router_execution", "source_render.rs");
const appRouterStateRuntimePath = path.join(root, "dx-www", "src", "cli", "app_router_execution", "state_runtime.rs");
const appRouterSemanticsPath = path.join(root, "dx-www", "src", "cli", "app_router_semantics.rs");
const publicToolsPath = path.join(root, "dx-www", "src", "cli", "public_framework_tools.rs");
const publicImportsToolsPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "public_framework_tools",
  "imports.rs",
);
const helpTextPath = path.join(root, "dx-www", "src", "cli", "help_text.rs");
const defaultTemplateSourcesPath = path.join(root, "dx-www", "src", "cli", "default_template_sources.rs");
const serverContractPath = path.join(root, "core", "src", "delivery", "server_contract.rs");
const coreCargoPath = path.join(root, "core", "Cargo.toml");
const dxWwwCargoPath = path.join(root, "dx-www", "Cargo.toml");
const tsxAstPath = path.join(root, "core", "src", "delivery", "tsx_ast.rs");
const jsxLoweringPath = path.join(root, "core", "src", "delivery", "jsx_lowering.rs");
const domEventsPath = path.join(root, "core", "src", "delivery", "dom_events.rs");
const sourceEngineModPath = path.join(root, "dx-www", "src", "build", "source_engine", "mod.rs");
const sourceEngineGraphPath = path.join(root, "dx-www", "src", "build", "source_engine", "graph.rs");
const sourceEngineEcmascriptAnalysisPath = path.join(root, "dx-www", "src", "build", "source_engine", "ecmascript_analysis.rs");
const sourceEngineEcmascriptDynamicImportsPath = path.join(root, "dx-www", "src", "build", "source_engine", "ecmascript_dynamic_imports.rs");
const sourceEngineReceiptPath = path.join(root, "dx-www", "src", "build", "source_engine", "receipt.rs");
const sourceEngineEcosystemGraphPath = path.join(root, "dx-www", "src", "build", "source_engine", "ecosystem_graph.rs");
const sourceBuildEngineTestPath = path.join(root, "dx-www", "tests", "source_build_engine.rs");

function read(filePath) {
  assert.ok(fs.existsSync(filePath), `missing ${path.relative(root, filePath)}`);
  return fs.readFileSync(filePath, "utf8");
}

function readRustDirectory(relativeDir) {
  const directory = path.join(root, relativeDir);
  assert.ok(fs.existsSync(directory), `missing ${relativeDir}`);
  return fs
    .readdirSync(directory)
    .filter((entry) => entry.endsWith(".rs"))
    .sort()
    .map((entry) => read(path.join(directory, entry)))
    .join("\n");
}

function rustStaticStringArray(source, name) {
  const start = source.indexOf(name);
  assert.notEqual(start, -1, `${name} must exist`);
  const arrayStart = source.indexOf("&[", start);
  const arrayEnd = source.indexOf("];", arrayStart);
  assert.notEqual(arrayStart, -1, `${name} must have an array start`);
  assert.notEqual(arrayEnd, -1, `${name} must have an array end`);
  return [...source.slice(arrayStart, arrayEnd).matchAll(/"([^"]+)"/g)].map((match) => match[1]);
}

test("public framework CLI exposes TSX, dx-style, imports, web-perf, and Vercel deploy surfaces", () => {
  const cli = `${read(cliPath)}\n${read(cliCoreImplPath)}\n${read(cliForgeCommandsCPath)}\n${read(helpTextPath)}`;
  const tools = `${read(publicToolsPath)}\n${read(publicImportsToolsPath)}`;

  for (const dispatch of [
    '"style" =>',
    '"imports" =>',
    '"explain" =>',
    '"doctor" =>',
    '"export" =>',
    '"deploy" =>',
    'Some("web-perf")',
    'Some("packages")',
    "run_dx_style",
    "run_dx_imports",
    "run_dx_explain",
    "run_dx_doctor",
    "run_dx_packages_check",
    "run_dx_export_analyze",
    "run_dx_deploy",
    "run_dx_web_perf_check",
  ]) {
    assert.match(cli, new RegExp(dispatch.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const command of [
    "style build",
    "style watch",
    "style check",
    "imports sync",
    "imports check",
    "explain /route",
    "doctor",
    "export --analyze",
    "deploy vercel --dry-run",
    "check packages",
    "check web-perf --url <url> --device both --json",
  ]) {
    assert.match(cli, new RegExp(command.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(tools, /styles\/theme\.css/);
  assert.match(tools, /styles\/generated\.css/);
  assert.match(tools, /components\/auto-imports\.ts/);
  assert.match(tools, /\.dx\/imports\/import-map\.json/);
  assert.match(tools, /\.dx\/receipts\/check\/web-perf\/report\.json/);
  assert.match(tools, /\.dx\/routes\/\*\.json/);
  assert.match(tools, /\.dx\/receipts\/doctor\/report\.json/);
  assert.match(tools, /\.dx\/receipts\/check\/packages\.json/);
  assert.match(tools, /\.dx\/receipts\/export\/analyze\.json/);
  assert.match(tools, /dx\.framework\.riskRegister/);
  assert.match(tools, /schema_revision/);
  assert.match(tools, /tsx_runtime_parity/);
  assert.match(tools, /oversized_cli_module/);
  assert.match(tools, /performance_claims_unmeasured/);
  assert.match(tools, /score_ceiling/);
  assert.match(tools, /dx doctor cannot report 100 while critical framework parity or proof risks remain/);
  assert.match(tools, /adapter-boundary/);
  assert.match(tools, /source-only/);
  assert.match(tools, /official-lighthouse-json-import/);
  assert.match(tools, /rust-chrome-devtools-protocol/);
  assert.doesNotMatch(tools, /npm Lighthouse/);
});

test("dx new public starter uses App Router TSX and dx-style generated CSS", () => {
  const cli = [
    read(cliPath),
    read(path.join(root, "dx-www", "src", "cli", "new_command.rs")),
    read(defaultTemplateSourcesPath),
    read(path.join(root, "examples", "template", "app", "layout.tsx")),
    read(path.join(root, "examples", "template", "app", "page.tsx")),
    read(path.join(root, "examples", "template", "components", "icons", "icon.tsx")),
    read(path.join(root, "examples", "template", "lib", "utils.ts")),
    read(path.join(root, "examples", "template", "dx")),
    read(path.join(root, "examples", "template", "README.md")),
  ].join("\n");

  for (const file of [
    "app/layout.tsx",
    "app/page.tsx",
    "components/icons/icon.tsx",
    "styles/globals.css",
    "styles/theme.css",
    "styles/generated.css",
    "public/logo.svg",
    "public/icon.svg",
    "public/favicon.svg",
    ".dx/receipts/check/web-perf",
  ]) {
    assert.match(cli, new RegExp(file.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(cli, /style\(\s*mode=generated-css/s);
  assert.match(cli, /tokens=styles\/theme\.css/);
  assert.match(cli, /generated_css=styles\/generated\.css/);
  assert.match(cli, /icons\(\s*component=Icon/s);
  assert.match(cli, /source_tag=icon/);
  assert.match(cli, /runtime_tag=dx-icon/);
  assert.match(cli, /imports\(\s*map=\.dx\/imports\/import-map\.json/s);
  assert.match(cli, /barrel=components\/auto-imports\.ts/);
  assert.match(cli, /declarations=\.dx\/imports\/imports\.d\.ts/);
  assert.match(cli, /aliases=#imports,#components/);
  assert.match(cli, /used_only=true/);
  assert.match(cli, /import "\.\.\/styles\/globals\.css";/);
  assert.match(cli, /export function Icon/);
  assert.match(cli, /components\/icons\/icon\.tsx/);
  assert.match(cli, /data-icon-source="dx-icons"/);
  assert.doesNotMatch(cli, /components\/icons\/dx-icons\.tsx/);
  assert.match(cli, /"template": "minimal-www-starter"/);
  assert.match(cli, /"tooling": \["dx-style", "dx-imports", "dx-check", "serializer"\]/);
  assert.match(cli, /"node_modules_required": false/);
  assert.doesNotMatch(cli, /Tailwind-compatible DX-Style/);
  assert.doesNotMatch(cli, /tailwind-compatible theme/);
});

test("dev runtime uses the generic TSX App Router execution contract outside launch-only rendering", () => {
  const cli = `${read(cliPath)}\n${read(cliCoreImplPath)}`;
  const appApiRoutes = read(appApiRoutesPath);
  const appPageRoutes = read(appPageRoutesPath);
  const routeRequestValues = read(routeRequestValuesPath);
  const appRouterRuntimeCommand = read(appRouterRuntimeCommandPath);
  const appRouterExecution = read(appRouterExecutionPath);
  const appRouterRenderPlan = read(appRouterRenderPlanPath);
  const appRouterSourceRender = `${read(appRouterSourceRenderPath)}\n${readRustDirectory("dx-www/src/cli/app_router_execution/source_render_parts")}`;
  const appRouterStateRuntime = read(appRouterStateRuntimePath);
  const appRouterSemantics = read(appRouterSemanticsPath);
  const serverContract = read(serverContractPath);
  const coreCargo = read(coreCargoPath);
  const dxWwwCargo = read(dxWwwCargoPath);
  const tsxAst = read(tsxAstPath);
  const jsxLowering = read(jsxLoweringPath);
  const domEvents = read(domEventsPath);
  const tools = read(publicToolsPath);
  const legacyContractSuffix = new RegExp(String.raw`dx\.(tsx|framework|route|vercel)\.[A-Za-z0-9_.-]+\.v${1}`);
  for (const source of [
    appRouterSemantics,
    appRouterStateRuntime,
    appRouterRenderPlan,
    appRouterSourceRender,
    tsxAst,
    jsxLowering,
    tools,
  ]) {
    assert.doesNotMatch(source, legacyContractSuffix);
  }

  assert.match(cli, /mod app_router_semantics;/);
  assert.match(appRouterExecution, /pub\(super\) fn render_app_router_runtime/);
  assert.match(appRouterExecution, /build_tsx_app_router_semantics/);
  assert.match(appRouterExecution, /build_tsx_render_plan/);
  assert.match(appRouterExecution, /build_tsx_source_render_surface/);
  assert.match(appRouterExecution, /build_state_runtime/);
  assert.match(appRouterExecution, /"tsx_semantics"/);
  assert.match(appRouterExecution, /"state_runtime": state_runtime\.program/);
  assert.match(appRouterExecution, /"tsx_render_plan": render_plan/);
  assert.match(appRouterExecution, /"tsx_source_render": source_render/);
  assert.match(appRouterExecution, /"tsx_render_plan"/);
  assert.match(appRouterExecution, /"tsx_source_render"/);
  assert.match(appRouterExecution, /"state_runtime"/);
  assert.match(appRouterExecution, /data-dx-renderer="tsx-app-router-generic"/);
  assert.match(appRouterExecution, /data-dx-app-router-runtime="source-owned-app-router"/);
  assert.match(appRouterExecution, /data-dx-render-plan/);
  assert.match(appRouterExecution, /data-dx-render-components/);
  assert.match(appRouterExecution, /data-dx-render-edges/);
  assert.match(appRouterExecution, /data-dx-tsx-source-render/);
  assert.match(appRouterExecution, /data-dx-tsx-renderable-elements/);
  assert.match(appRouterExecution, /data-dx-tsx-component-refs/);
  assert.match(appRouterExecution, /data-dx-tsx-component-compositions/);
  assert.match(appRouterExecution, /data-dx-tsx-prop-bindings/);
  assert.match(appRouterExecution, /data-dx-tsx-source-imports/);
  assert.match(appRouterExecution, /data-dx-tsx-skipped-imports/);
  assert.match(appRouterExecution, /data-dx-tsx-static-snapshot/);
  assert.match(appRouterExecution, /data-dx-tsx-static-snapshot-elements/);
  assert.match(appRouterExecution, /data-dx-client-islands/);
  assert.match(appRouterExecution, /data-dx-effect-boundaries/);
  assert.match(appRouterExecution, /data-dx-context-boundaries/);
  assert.match(appRouterExecution, /data-dx-context-runtime/);
  assert.match(appRouterExecution, /data-dx-effect-scheduler/);
  assert.match(appRouterExecution, /static_dom_snapshot_preview/);
  assert.match(appRouterExecution, /app_router_runtime_shell_preview/);
  assert.match(appRouterExecution, /body_inner_app_router_shell_preview/);
  assert.match(appRouterExecution, /data-dx-tsx-app-router-shell/);
  assert.match(appRouterExecution, /data-dx-app-router-shell-status/);
  assert.match(appRouterExecution, /client_island_manifest_script_tag/);
  assert.match(appRouterExecution, /effect_context_manifest_script_tag/);
  assert.match(appRouterExecution, /data-dx-tsx-static-dom-preview/);
  assert.match(appRouterExecution, /data-dx-static-dom-preview-hidden/);
  assert.match(appRouterExecution, /data-dx-tsx-literal-expressions/);
  assert.match(appRouterExecution, /data-dx-dom-action-binder/);
  assert.match(appRouterExecution, /data-dx-dom-action-descriptors/);
  assert.match(appRouterExecution, /id="__DX_TSX_SOURCE_RENDER__"/);
  assert.match(appRouterExecution, /id="__DX_TSX_CLIENT_ISLANDS__"/);
  assert.match(appRouterExecution, /id="__DX_TSX_EFFECT_CONTEXT_BOUNDARIES__"/);
  assert.match(appRouterExecution, /id="__DX_CONTEXT_RUNTIME__"/);
  assert.match(appRouterExecution, /id="__DX_DOM_ACTION_BINDER__"/);
  assert.match(appRouterExecution, /data-dx-state-runtime/);
  assert.match(appRouterExecution, /id="__DX_APP_ROUTER_EXECUTION__"/);
  assert.match(appRouterExecution, /"route_params"/);
  assert.match(appRouterExecution, /"search_params"/);
  assert.match(appRouterExecution, /data-dx-route-params/);
  assert.match(appRouterExecution, /data-dx-search-params/);
  assert.match(appRouterExecution, /data-dx-state-slots/);
  assert.match(appRouterExecution, /data-dx-event-slots/);
  assert.match(appRouterExecution, /"public_authoring": "tsx"/);
  assert.match(appRouterExecution, /"legacy_page_formats": "internal-only"/);
  assert.match(cli, /app_page_routes::route_match\(cwd, &request\.path\)/);
  assert.match(cli, /app_api_routes::route_handler_match\(cwd, path\)/);
  assert.match(appPageRoutes, /pub\(super\) fn route_match/);
  assert.match(appPageRoutes, /fn match_app_route_candidate/);
  assert.match(routeRequestValues, /pub\(super\) fn parse_search_params/);
  assert.match(appApiRoutes, /fn dynamic_route_handler_match/);
  assert.match(appApiRoutes, /fn route_match_candidate/);
  assert.match(appRouterSemantics, /dx\.tsx\.appRouterSemantics/);
  assert.match(appRouterSemantics, /schema_revision/);
  assert.match(appRouterSemantics, /react-familiar-compiler-owned/);
  assert.match(appRouterSemantics, /full_react_runtime_parity/);
  assert.match(appRouterSemantics, /dx\.tsx\.stateGraph/);
  assert.match(appRouterSemantics, /compiler-owned state graph ABI/);
  assert.match(appRouterSemantics, /react_authoring_source/);
  assert.match(appRouterSemantics, /runtime_lowering/);
  assert.match(appRouterSemantics, /derived_slots/);
  assert.match(appRouterSemantics, /event_slots/);
  assert.match(appRouterSemantics, /server_actions/);
  assert.match(appRouterSemantics, /dx\.tsx\.effectContextBoundaryManifest/);
  assert.match(appRouterSemantics, /effect_context_boundaries/);
  assert.match(appRouterSemantics, /effect_scheduler_status/);
  assert.match(appRouterSemantics, /dependency-scheduler-ready/);
  assert.match(appRouterSemantics, /context_provider_count/);
  assert.match(appRouterSemantics, /dx\.tsx\.contextRuntime/);
  assert.match(appRouterSemantics, /provider-value-map-ready/);
  assert.match(appRouterSemantics, /initial_values/);
  assert.match(appRouterSemantics, /provider_value_source/);
  assert.match(appRouterSemantics, /safe_literal_context_value/);
  assert.match(appRouterSemantics, /collect_hook_usage/);
  assert.match(appRouterSemantics, /collect_event_handlers/);
  assert.match(appRouterSemantics, /unsupported-react-event-diagnostic/);
  assert.match(appRouterSemantics, /unsupported-react-event/);
  assert.match(appRouterSemantics, /implicit-client-boundary/);
  assert.match(appRouterSemantics, /still_needed_for_nextjs_runtime_parity/);
  assert.match(appRouterStateRuntime, /dx\.tsx\.stateRuntime/);
  assert.match(appRouterStateRuntime, /dx\.tsx\.effectScheduler/);
  assert.match(appRouterStateRuntime, /schema_revision/);
  assert.match(appRouterStateRuntime, /__DX_STATE_GRAPH_RUNTIME__/);
  assert.match(appRouterStateRuntime, /generated-js-client-islands/);
  assert.match(appRouterStateRuntime, /full_react_hook_runtime/);
  assert.match(appRouterStateRuntime, /full_react_effect_body_execution/);
  assert.match(appRouterStateRuntime, /scheduleEffectsForState/);
  assert.match(appRouterStateRuntime, /dx:effect-scheduled/);
  assert.match(appRouterExecution, /attachContextRuntime/);
  assert.match(appRouterExecution, /resolveContextValue/);
  assert.match(appRouterExecution, /program\.initial_values/);
  assert.match(appRouterExecution, /context_initial_values/);
  assert.match(appRouterExecution, /dx:context-runtime-ready/);
  assert.match(appRouterExecution, /dx:context-value/);
  assert.match(appRouterStateRuntime, /adapter_boundary_gaps/);
  assert.match(appRouterStateRuntime, /react_hook_policy/);
  assert.match(appRouterStateRuntime, /setSlot/);
  assert.match(appRouterStateRuntime, /applyStateOperation/);
  assert.match(appRouterStateRuntime, /set-from-input/);
  assert.match(appRouterStateRuntime, /toggle/);
  assert.match(appRouterStateRuntime, /add/);
  assert.match(appRouterStateRuntime, /subtract/);
  assert.match(appRouterStateRuntime, /set-literal/);
  assert.match(appRouterStateRuntime, /dx:state-runtime-ready/);
  assert.match(appRouterStateRuntime, /node_modules_required/);
  assert.match(appRouterRenderPlan, /dx\.tsx\.renderPlan/);
  assert.match(appRouterRenderPlan, /schema_revision/);
  assert.match(appRouterRenderPlan, /React-familiar apps with source-owned packages and no hidden dependency surface/);
  assert.match(appRouterRenderPlan, /claim_policy/);
  assert.match(appRouterRenderPlan, /source_owned_render_plan/);
  assert.match(appRouterRenderPlan, /external_runtime_required/);
  assert.match(appRouterRenderPlan, /external_runtime_executed/);
  assert.match(appRouterRenderPlan, /production_blockers/);
  assert.match(appRouterRenderPlan, /component-import-execution/);
  assert.match(appRouterRenderPlan, /props-evaluation/);
  assert.match(appRouterRenderPlan, /client-island-dom-binding/);
  assert.match(appRouterRenderPlan, /client_island_manifest/);
  assert.match(appRouterRenderPlan, /generated_dom_action_binder/);
  assert.match(appRouterRenderPlan, /effect_scheduler/);
  assert.match(appRouterRenderPlan, /context_runtime/);
  assert.match(appRouterRenderPlan, /faster than Next\.js globally/);
  assert.match(appRouterSourceRender, /dx\.tsx\.sourceRenderSurface/);
  assert.match(appRouterSourceRender, /build_tsx_source_render_surface/);
  assert.match(appRouterSourceRender, /renderable_elements/);
  assert.match(appRouterSourceRender, /component_references/);
  assert.match(appRouterSourceRender, /component_compositions/);
  assert.match(appRouterSourceRender, /source_owned_component_compositions/);
  assert.match(appRouterSourceRender, /source_owned_component_composition/);
  assert.match(appRouterSourceRender, /matching_component_import/);
  assert.match(appRouterSourceRender, /source-owned-import-static-composition/);
  assert.match(appRouterSourceRender, /dx\.tsx\.clientIslandManifest/);
  assert.match(appRouterSourceRender, /build_client_island_manifest/);
  assert.match(appRouterSourceRender, /generated-dom-action-binder/);
  assert.match(appRouterSourceRender, /component_invocation_inputs/);
  assert.match(appRouterSourceRender, /dx\.tsx\.componentInvocationInputs/);
  assert.match(appRouterSourceRender, /literal_child_expressions/);
  assert.match(appRouterSourceRender, /event_handler_props/);
  assert.match(appRouterSourceRender, /static-invocation-inputs/);
  assert.match(appRouterSourceRender, /component_return_preview/);
  assert.match(appRouterSourceRender, /dx\.tsx\.componentReturnPreview/);
  assert.match(appRouterSourceRender, /children_placeholder_count/);
  assert.match(appRouterSourceRender, /children_insertions/);
  assert.match(appRouterSourceRender, /is_children_placeholder_expression/);
  assert.match(appRouterSourceRender, /composed_static_dom_snapshot/);
  assert.match(appRouterSourceRender, /dx\.tsx\.composedStaticDomSnapshot/);
  assert.match(appRouterSourceRender, /schema_revision/);
  assert.doesNotMatch(appRouterSourceRender, /composed-static-dom-snapshot/);
  assert.match(appRouterSourceRender, /build_composed_static_dom_snapshot/);
  assert.match(appRouterSourceRender, /compose_app_router_static_shell/);
  assert.match(appRouterSourceRender, /dx\.tsx\.appRouterStaticShell/);
  assert.match(appRouterSourceRender, /layout-template-page-composition/);
  assert.match(appRouterSourceRender, /app_router_shell_child_insertions/);
  assert.match(appRouterSourceRender, /wraps page HTML with layout\/template children placeholders/);
  assert.match(appRouterSourceRender, /nested-static-dom-tree/);
  assert.match(appRouterSourceRender, /root_jsx_elements/);
  assert.match(appRouterSourceRender, /child_element_indices/);
  assert.match(appRouterSourceRender, /render_static_child_nodes/);
  assert.match(appRouterSourceRender, /importer_element_index/);
  assert.match(appRouterSourceRender, /matching_component_composition_for_element/);
  assert.match(appRouterSourceRender, /source-owned-component-preview/);
  assert.match(appRouterSourceRender, /component_preview_insertions/);
  assert.match(appRouterSourceRender, /component_prop_identifier_bindings/);
  assert.match(appRouterSourceRender, /dx\.tsx\.componentPropIdentifierBindings/);
  assert.match(appRouterSourceRender, /resolve_component_prop_identifier/);
  assert.match(appRouterSourceRender, /resolve_static_template_with_prop_bindings/);
  assert.match(appRouterSourceRender, /static-template-prop-interpolation/);
  assert.match(appRouterSourceRender, /template_literal_prop_bindings/);
  assert.match(appRouterSourceRender, /resolves_trimmed_static_template_literal_prop_binding/);
  assert.match(appRouterSourceRender, /resolve_static_conditional_with_prop_bindings/);
  assert.match(appRouterSourceRender, /static-ternary-prop-branches/);
  assert.match(appRouterSourceRender, /conditional_expression_prop_bindings/);
  assert.match(appRouterSourceRender, /resolves_static_conditional_prop_branch/);
  assert.match(appRouterSourceRender, /resolve_static_class_list_with_prop_bindings/);
  assert.match(appRouterSourceRender, /static-class-list-prop-bindings/);
  assert.match(appRouterSourceRender, /class_list_prop_bindings/);
  assert.match(appRouterSourceRender, /resolves_static_class_list_prop_bindings/);
  assert.match(appRouterSourceRender, /resolve_static_class_call_with_prop_bindings/);
  assert.match(appRouterSourceRender, /static-class-call-prop-bindings/);
  assert.match(appRouterSourceRender, /class_call_prop_bindings/);
  assert.match(appRouterSourceRender, /resolves_static_class_call_prop_bindings/);
  assert.match(appRouterSourceRender, /component_destructured_prop_aliases/);
  assert.match(appRouterSourceRender, /dx\.tsx\.componentDestructuredPropAliases/);
  assert.match(appRouterSourceRender, /function-parameter-object-pattern/);
  assert.match(appRouterSourceRender, /arrow-parameter-object-pattern/);
  assert.match(appRouterSourceRender, /prop_alias_allows_bare_identifier/);
  assert.match(appRouterSourceRender, /renamed-alias-object-pattern/);
  assert.match(appRouterSourceRender, /default-value-object-pattern/);
  assert.match(appRouterSourceRender, /prop_alias_default_value/);
  assert.match(appRouterSourceRender, /props\./);
  assert.match(appRouterSourceRender, /simple-prop-identifier-bindings/);
  assert.match(appRouterSourceRender, /component_runtime_binding_plan/);
  assert.match(appRouterSourceRender, /dx\.tsx\.componentRuntimeBindingPlan/);
  assert.match(appRouterSourceRender, /dom_action_descriptors/);
  assert.match(appRouterSourceRender, /dx\.tsx\.domActionDescriptors/);
  assert.match(appRouterSourceRender, /safe-intrinsic-actions-planned/);
  assert.match(appRouterSourceRender, /dom_event_name_from_react_attribute/);
  assert.match(appRouterSourceRender, /dom_action_binder/);
  assert.match(appRouterSourceRender, /dx\.tsx\.domActionBinder/);
  assert.match(appRouterSourceRender, /generated-js-binder-ready/);
  assert.match(appRouterSourceRender, /attachSafeDomActionBinder/);
  assert.match(appRouterSourceRender, /supportedEvents\.includes/);
  assert.match(appRouterSourceRender, /dx:dom-action-preview/);
  assert.match(appRouterSourceRender, /dx:dom-action-binder-ready/);
  assert.match(appRouterSourceRender, /state_runtime_bridge/);
  assert.match(appRouterSourceRender, /dx\.tsx\.domActionStateBridge/);
  assert.match(appRouterSourceRender, /dispatchDomActionPreviewToStateRuntime/);
  assert.match(appRouterSourceRender, /__DX_STATE_GRAPH_RUNTIME__/);
  assert.match(appRouterSourceRender, /dx:state-runtime-dispatch/);
  assert.match(appRouterSourceRender, /state_dom_reflection/);
  assert.match(appRouterSourceRender, /build_state_dom_reflection_plan/);
  assert.match(appRouterSourceRender, /dx\.tsx\.stateDomReflection/);
  assert.match(appRouterSourceRender, /text-content/);
  assert.match(appRouterSourceRender, /form-control-value/);
  assert.match(appRouterSourceRender, /form-control-checked/);
  assert.match(appRouterSourceRender, /aria-attribute/);
  assert.match(appRouterStateRuntime, /reflectStateSlotToDom/);
  assert.match(appRouterStateRuntime, /state_dom_reflection/);
  assert.match(appRouterStateRuntime, /dx:state-dom-reflection/);
  assert.match(appRouterStateRuntime, /derived_slots/);
  assert.match(appRouterStateRuntime, /derived_dom_reflection/);
  assert.match(appRouterStateRuntime, /refreshDerivedSlots/);
  assert.match(appRouterStateRuntime, /dx:derived-state-slot/);
  assert.match(appRouterStateRuntime, /data-dx-state-read/);
  assert.match(appRouterStateRuntime, /reflectStateSlotToDom\(slot\.name/);
  assert.match(appRouterSourceRender, /derived_state_dom_reflection_entry/);
  assert.match(appRouterSourceRender, /derived values when the expression has a safe runtime lowering/);
  assert.match(appRouterSourceRender, /full_react_hook_parity/);
  assert.match(appRouterSourceRender, /node_modules_required/);
  assert.match(appRouterSourceRender, /browser_listeners_attached/);
  assert.match(appRouterSourceRender, /full_react_event_execution/);
  assert.match(appRouterSourceRender, /intrinsic_form_lowering/);
  assert.match(appRouterSourceRender, /planned-from-state-graph/);
  assert.match(appRouterSourceRender, /full_dom_binding/);
  assert.match(appRouterSourceRender, /full_component_execution/);
  assert.match(appRouterSourceRender, /prop_bindings/);
  assert.match(appRouterSourceRender, /form_surfaces/);
  assert.match(appRouterSourceRender, /bounded-source-owned-tsx/);
  assert.match(appRouterSourceRender, /source_owned_imports_scanned/);
  assert.match(appRouterSourceRender, /static_dom_snapshot/);
  assert.match(appRouterSourceRender, /static_dom_snapshot_elements/);
  assert.match(appRouterSourceRender, /static_dom_snapshot_literal_expressions/);
  assert.match(appRouterSourceRender, /dx\.tsx\.staticDomSnapshot/);
  assert.match(appRouterSourceRender, /safe-common-subset/);
  assert.match(appRouterSourceRender, /full_react_execution/);
  assert.match(appRouterSourceRender, /build_static_dom_snapshot/);
  assert.match(appRouterSourceRender, /static_literal_expression/);
  assert.match(appRouterSourceRender, /StaticLiteralExpression/);
  assert.match(appRouterSourceRender, /literal expression props/);
  assert.match(appRouterSourceRender, /resolve_source_owned_import/);
  assert.match(appRouterSourceRender, /source-owned-import/);

  const nativeDomEvents = rustStaticStringArray(domEvents, "NATIVE_DOM_EVENT_NAMES");
  for (const eventName of ["click", "input", "pointermove"]) {
    assert.ok(
      nativeDomEvents.includes(eventName),
      `React-style event lowering requires ${eventName} in the source-owned DOM catalog`,
    );
  }
  assert.match(domEvents, /react_style_event_attributes_map_from_source_catalog/);
  assert.match(domEvents, /\("onClick", "click"\)/);
  assert.match(domEvents, /\("onInput", "input"\)/);
  assert.match(domEvents, /\("onPointerMove", "pointermove"\)/);
  assert.match(domEvents, /native_dom_event_names\(\)\.contains\(&dom_event\)/);
  assert.match(domEvents, /unsupported_react_style_events_are_not_silently_lowered/);
  assert.match(domEvents, /\("onMagicGesture", None\)/);
  assert.match(domEvents, /\("onclick", None\)/);
  assert.match(appRouterSourceRender, /Source-owned TSX render surface, not full JSX execution/);
  assert.match(appRouterSourceRender, /adapter_boundary_gaps/);
  assert.match(coreCargo, /oxc = \["oxc_parser", "oxc_allocator", "oxc_ast", "oxc_span"\]/);
  assert.match(dxWwwCargo, /tsx-oxc = \["dep:dx-www-compiler", "dx-www-compiler\/oxc"\]/);
  assert.match(dxWwwCargo, /cli = \[[\s\S]*"clap"[\s\S]*"tsx-oxc"[\s\S]*\]/);
  assert.match(tsxAst, /dx\.tsx\.parserBackend/);
  assert.match(tsxAst, /schema_revision/);
  assert.match(tsxAst, /DxTsxParserBackend/);
  assert.match(tsxAst, /active_backend/);
  assert.match(tsxAst, /custom-scanner/);
  assert.match(tsxAst, /oxc-parser/);
  assert.match(tsxAst, /oxc_parser::Parser/);
  assert.match(tsxAst, /SourceType::from_path/);
  assert.match(tsxAst, /Parser::new/);
  assert.match(tsxAst, /extract_oxc_imports/);
  assert.match(tsxAst, /oxc_import_declarations/);
  assert.match(tsxAst, /ImportDeclarationSpecifier/);
  assert.match(tsxAst, /parser_backend_parse\.imports/);
  assert.match(tsxAst, /extract_oxc_metadata/);
  assert.match(tsxAst, /oxc_route_metadata/);
  assert.match(tsxAst, /ExportNamedDeclaration/);
  assert.match(tsxAst, /ObjectPropertyKind/);
  assert.match(jsxLowering, /parser_backend: DxTsxParserBackend/);
  assert.match(jsxLowering, /dx\.tsx\.oxcJsxSurface/);
  assert.match(jsxLowering, /schema_revision/);
  assert.match(jsxLowering, /lower_with_oxc_jsx_surface/);
  assert.match(jsxLowering, /parse_oxc_jsx_surface/);
  assert.match(jsxLowering, /collect_oxc_jsx_element/);
  assert.match(jsxLowering, /DxReactJsxChildNode/);
  assert.match(jsxLowering, /child_nodes/);
  assert.match(jsxLowering, /parent_index/);
  assert.match(jsxLowering, /Element \{ index/);
  assert.match(jsxLowering, /JSXElementName/);
  assert.match(jsxLowering, /JSXAttributeItem/);
  assert.match(jsxLowering, /JSXChild/);
  assert.match(jsxLowering, /collect_oxc_jsx_expression/);
  assert.match(jsxLowering, /collect_oxc_call_expression/);
  assert.match(jsxLowering, /collect_oxc_argument/);
  assert.match(jsxLowering, /collect_oxc_array_expression/);
  assert.match(jsxLowering, /collect_oxc_object_expression/);
  assert.match(jsxLowering, /collect_oxc_statement_branch/);
  assert.match(appRouterSourceRender, /parser_backends/);
  assert.match(appRouterSourceRender, /oxc_validated_documents/);
  assert.match(appRouterSourceRender, /jsx_backends/);
  assert.match(appRouterSourceRender, /oxc_jsx_documents/);
  assert.match(appRouterSourceRender, /jsx_backend/);
  assert.match(appRouterRuntimeCommand, /render_app_router_runtime\(DxAppRouterRuntimeRenderInput/);
  assert.match(appRouterRuntimeCommand, /compile_react_app_route/);
  assert.match(appRouterRuntimeCommand, /render_app_router_runtime\(DxAppRouterRuntimeRenderInput/);
  assert.doesNotMatch(appRouterRuntimeCommand, /tsx_launch_runtime::render_template_app_tsx_route/);
  assert.doesNotMatch(appRouterRuntimeCommand, /tsx_launch_runtime::render_template_shell_tsx_route/);
  assert.match(serverContract, /fn exported_const_callable_signature/);
  assert.match(serverContract, /request_serialization/);
  assert.match(serverContract, /source-owned-route-handler-boundary/);
});

test("source build records DX-owned ECMAScript analysis without Next runtime takeover", () => {
  const sourceEngineMod = read(sourceEngineModPath);
  const graph = read(sourceEngineGraphPath);
  const ecmascriptAnalysis = read(sourceEngineEcmascriptAnalysisPath);
  const ecmascriptDynamicImports = read(sourceEngineEcmascriptDynamicImportsPath);
  const receipt = read(sourceEngineReceiptPath);
  const ecosystemGraph = read(sourceEngineEcosystemGraphPath);
  const sourceBuildEngineTest = read(sourceBuildEngineTestPath);

  assert.match(sourceEngineMod, /mod ecmascript_analysis;/);
  assert.match(sourceEngineMod, /mod ecmascript_dynamic_imports;/);
  assert.match(graph, /SourceBuildEcmascriptAnalysis/);
  assert.match(graph, /SourceBuildEcmascriptDynamicImportAnalysis/);
  assert.match(graph, /SourceBuildEcmascriptUnresolvedDynamicImport/);
  assert.match(graph, /SourceBuildEcmascriptUnsupportedDynamicImport/);
  assert.match(graph, /import_options_present/);
  assert.match(graph, /import_options_supported/);
  assert.match(graph, /ecmascript_analysis: SourceBuildEcmascriptAnalysis/);
  assert.match(ecmascriptAnalysis, /dx\.ecmascript\.analysis/);
  assert.match(ecmascriptAnalysis, /turbopack-ecmascript/);
  assert.match(ecmascriptAnalysis, /reference_only/);
  assert.match(ecmascriptAnalysis, /runtime_build_adoption/);
  assert.match(ecmascriptAnalysis, /public_runtime_dependency/);
  assert.match(ecmascriptAnalysis, /next-custom-transforms::track_dynamic_imports/);
  assert.match(ecmascriptAnalysis, /compiler_owns_output/);
  assert.match(ecmascriptAnalysis, /next_runtime_required/);
  assert.match(ecmascriptAnalysis, /react_runtime_required/);
  assert.match(ecmascriptAnalysis, /rsc_required/);
  assert.match(ecmascriptAnalysis, /node_modules_required/);
  assert.match(ecmascriptAnalysis, /full_nextjs_parity/);
  assert.match(ecmascriptAnalysis, /dynamic_imports/);
  assert.match(ecmascriptAnalysis, /dynamic_import_analysis/);
  assert.match(ecmascriptAnalysis, /dynamic_import_analysis_status/);
  assert.match(ecmascriptAnalysis, /static-and-unresolved/);
  assert.match(ecmascriptAnalysis, /unresolved_dynamic_imports/);
  assert.match(ecmascriptAnalysis, /unsupported_dynamic_imports/);
  assert.match(ecmascriptAnalysis, /collect_dynamic_imports/);
  assert.doesNotMatch(ecmascriptAnalysis, /fn parse_dynamic_import/);
  assert.match(ecmascriptDynamicImports, /collect_dynamic_imports/);
  assert.match(ecmascriptDynamicImports, /scan_dynamic_import_calls/);
  assert.match(ecmascriptDynamicImports, /classify_dynamic_import_call/);
  assert.doesNotMatch(ecmascriptDynamicImports, /fn dynamic_imports\(source/);
  assert.doesNotMatch(ecmascriptDynamicImports, /fn unresolved_dynamic_imports\(source/);
  assert.doesNotMatch(ecmascriptDynamicImports, /fn unsupported_dynamic_imports\(source/);
  assert.match(ecmascriptDynamicImports, /SourceBuildEcmascriptDynamicImports/);
  assert.match(ecmascriptDynamicImports, /records_unresolved_dynamic_imports_without_static_specifiers/);
  assert.match(ecmascriptDynamicImports, /non-static-dynamic-import-expression/);
  assert.match(ecmascriptDynamicImports, /records_unsupported_dynamic_import_calls/);
  assert.match(ecmascriptDynamicImports, /unsupported-dynamic-import-empty-expression/);
  assert.match(ecmascriptDynamicImports, /unsupported-dynamic-import-options/);
  assert.match(ecmascriptDynamicImports, /options_import\.import_options_present,\s*true/);
  assert.match(ecmascriptDynamicImports, /options_import\.import_options_supported,\s*false/);
  assert.match(ecmascriptDynamicImports, /static_import\.import_options_present,\s*false/);
  assert.match(ecmascriptDynamicImports, /static_import\.import_options_supported,\s*true/);
  assert.match(ecmascriptDynamicImports, /unsupported-dynamic-import-unclosed-call/);
  assert.match(ecmascriptDynamicImports, /records_dynamic_imports_with_leading_comments_and_static_templates/);
  assert.match(ecmascriptDynamicImports, /skip_ascii_ws_and_comments/);
  assert.match(ecmascriptDynamicImports, /b'`'/);
  assert.match(ecmascriptDynamicImports, /specifier\.contains\("\$\{"/);
  assert.match(ecmascriptAnalysis, /top_level_await/);
  assert.match(receipt, /dx-source-ecmascript-analysis-adapter/);
  assert.match(receipt, /Turbopack ECMAScript reference/);
  assert.match(ecosystemGraph, /ecmascriptAnalysis/);
  assert.match(ecosystemGraph, /unresolvedDynamicImportCount/);
  assert.match(ecosystemGraph, /unsupportedDynamicImportCount/);
  assert.match(ecosystemGraph, /unsupportedDynamicImportReasonCounts/);
  assert.match(ecosystemGraph, /dynamicImportOptionBoundaryCount/);
  assert.match(ecosystemGraph, /dynamicImportOptionUnsupportedCount/);
  assert.match(ecosystemGraph, /dynamicImportAnalysisStatusCounts/);
  assert.match(sourceBuildEngineTest, /unresolved_dynamic_imports/);
  assert.match(sourceBuildEngineTest, /unsupported_dynamic_imports/);
  assert.match(sourceBuildEngineTest, /dynamic_import_analysis/);
  assert.match(sourceBuildEngineTest, /unsupported-observed/);
  assert.match(sourceBuildEngineTest, /non-static-dynamic-import-expression/);
  assert.match(sourceBuildEngineTest, /unresolvedDynamicImportCount"\],\s*2/);
  assert.match(sourceBuildEngineTest, /unsupportedDynamicImportCount"\],\s*1/);
  assert.match(sourceBuildEngineTest, /unsupportedDynamicImportReasonCounts/);
  assert.match(sourceBuildEngineTest, /dynamicImportOptionBoundaryCount"\],\s*1/);
  assert.match(sourceBuildEngineTest, /dynamicImportOptionUnsupportedCount"\],\s*1/);
  assert.match(sourceBuildEngineTest, /unsupported-dynamic-import-options/);
  assert.match(sourceBuildEngineTest, /import_options_present/);
  assert.match(sourceBuildEngineTest, /import_options_supported/);
  assert.doesNotMatch(ecmascriptAnalysis, /next_runtime_required": true/);
  assert.doesNotMatch(ecmascriptAnalysis, /rsc_required": true/);
});
