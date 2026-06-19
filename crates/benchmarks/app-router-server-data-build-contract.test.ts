const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const cliCoreImplModulePath = path.join(root, "dx-www", "src", "cli", "mod_parts", "cli_core_impl.rs");
const cliBuildCommandModulePath = path.join(root, "dx-www", "src", "cli", "build_command.rs");
const buildCommandModulePath = path.join(root, "dx-www", "src", "cli", "app_router_build_command.rs");
const buildOutputModulePath = path.join(root, "dx-www", "src", "cli", "app_router_build_output.rs");
const runtimeCommandModulePath = path.join(root, "dx-www", "src", "cli", "app_router_runtime_command.rs");
const appRouterExecutionPath = path.join(root, "dx-www", "src", "cli", "app_router_execution.rs");
const serverDataSurfaceModulePath = path.join(root, "dx-www", "src", "cli", "app_router_server_data.rs");
const serverDataBuildManifestModulePath = path.join(root, "dx-www", "src", "cli", "app_server_data_manifest.rs");
const serverDataProofTestPath = path.join(root, "dx-www", "tests", "app_router_server_data.rs");
const sourceBuildGraphPath = path.join(root, "dx-www", "src", "build", "source_engine", "graph.rs");
const sourceBuildRouteOutputPath = path.join(root, "dx-www", "src", "build", "source_engine", "route_output.rs");
const sourceBuildReceiptPath = path.join(root, "dx-www", "src", "build", "source_engine", "receipt.rs");
const sourceBuildServerDataPath = path.join(root, "dx-www", "src", "build", "source_engine", "server_data.rs");
const sourceBuildServerDataProofPath = path.join(root, "dx-www", "tests", "source_build_server_data.rs");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("default App Router build has a real server-data route contract", () => {
  const homePage = read("examples/template/app/page.tsx");
  const cli = read("dx-www/src/cli/mod.rs");
  const cliCoreImpl = fs.existsSync(cliCoreImplModulePath)
    ? fs.readFileSync(cliCoreImplModulePath, "utf8")
    : "";
  const appRouterBuildCommand = fs.existsSync(buildCommandModulePath)
    ? fs.readFileSync(buildCommandModulePath, "utf8")
    : "";
  const buildCommandModule = fs.existsSync(cliBuildCommandModulePath)
    ? fs.readFileSync(cliBuildCommandModulePath, "utf8")
    : "";
  const appRouterRuntimeCommand = fs.existsSync(runtimeCommandModulePath)
    ? fs.readFileSync(runtimeCommandModulePath, "utf8")
    : "";
  const buildOutputModule = fs.existsSync(buildOutputModulePath)
    ? fs.readFileSync(buildOutputModulePath, "utf8")
    : "";
  const appRouterExecution = fs.existsSync(appRouterExecutionPath)
    ? fs.readFileSync(appRouterExecutionPath, "utf8")
    : "";
  const serverDataSurfaceModule = fs.existsSync(serverDataSurfaceModulePath)
    ? fs.readFileSync(serverDataSurfaceModulePath, "utf8")
    : "";
  const serverDataBuildManifestModule = fs.existsSync(serverDataBuildManifestModulePath)
    ? fs.readFileSync(serverDataBuildManifestModulePath, "utf8")
    : "";
  const serverDataProofTest = fs.existsSync(serverDataProofTestPath)
    ? fs.readFileSync(serverDataProofTestPath, "utf8")
    : "";
  const sourceBuildGraph = fs.existsSync(sourceBuildGraphPath)
    ? fs.readFileSync(sourceBuildGraphPath, "utf8")
    : "";
  const sourceBuildRouteOutput = fs.existsSync(sourceBuildRouteOutputPath)
    ? fs.readFileSync(sourceBuildRouteOutputPath, "utf8")
    : "";
  const sourceBuildReceipt = fs.existsSync(sourceBuildReceiptPath)
    ? fs.readFileSync(sourceBuildReceiptPath, "utf8")
    : "";
  const sourceBuildServerData = fs.existsSync(sourceBuildServerDataPath)
    ? fs.readFileSync(sourceBuildServerDataPath, "utf8")
    : "";
  const sourceBuildServerDataProof = fs.existsSync(sourceBuildServerDataProofPath)
    ? fs.readFileSync(sourceBuildServerDataProofPath, "utf8")
    : "";
  const serverContract = read("core/src/delivery/server_contract.rs");

  assert.match(homePage, /export const metadata = \{/);
  assert.match(homePage, /title: "Enhanced Development Experience"/);
  assert.match(homePage, /description: "Orchestrate your code, don't just own it\."/);
  assert.match(homePage, /export default function HomePage\(\)/);
  assert.match(homePage, /data-dx-template="minimal"/);
  assert.match(homePage, /src="\/logo\.svg"/);
  assert.match(homePage, /Dx WWW/);
  assert.doesNotMatch(homePage, /data-dx-proof-links="state-runtime islands"/);
  assert.doesNotMatch(homePage, /href="\/state-runtime"/);
  assert.doesNotMatch(homePage, /href="\/islands"/);
  assert.doesNotMatch(homePage, /data-dx-no-js-fallback="preserved"/);
  assert.doesNotMatch(homePage, /action="\/state-runtime"/);
  assert.doesNotMatch(
    homePage,
    /from "next"|TemplateShell|TemplateLandingPage|templateRouteContract|DxIntlProvider|loadDxMessages|dxDefaultLocale/,
  );

  assert.ok(fs.existsSync(buildOutputModulePath), "App Router build output should live outside giant cli/mod.rs");
  assert.ok(fs.existsSync(buildCommandModulePath), "App Router build command glue should live outside giant cli/mod.rs");
  assert.match(cli, /mod app_router_build_command;/);
  assert.match(cli, /mod app_router_build_output;/);
  assert.match(appRouterBuildCommand, /use super::app_router_build_output::\{[\s\S]*DxAppServerDataOutputInput[\s\S]*\};/);
  assert.match(appRouterBuildCommand, /use super::app_router_build_output::\{[\s\S]*write_app_server_data_contract[\s\S]*\};/);
  assert.match(
    cliCoreImpl,
    /compile_app_router_build_outputs\(\s*app_router_build_command::DxAppRouterBuildCommandInput/,
  );
  assert.match(appRouterBuildCommand, /pub\(super\) struct DxAppRouterBuildCommandInput/);
  assert.match(appRouterBuildCommand, /pub\(super\) struct DxAppRouterBuildCommandOutput/);
  assert.match(appRouterBuildCommand, /pub\(super\) fn compile_app_router_build_outputs/);
  assert.match(appRouterBuildCommand, /app_route_diagnostics::validate_app_route_handlers/);
  assert.match(appRouterBuildCommand, /app_route_diagnostics::validate_app_route_source/);
  assert.match(appRouterBuildCommand, /app_route_diagnostics::app_route_compile_error/);
  assert.match(appRouterBuildCommand, /app_segment_files::is_app_page_file_name/);
  assert.match(appRouterBuildCommand, /app_page_routes::route_path_from_page_source_path\(&relative\)\.is_some\(\)/);
  assert.doesNotMatch(cli, /for app_dir in app_route_roots[\s\S]{0,2000}write_app_router_execution_contract/);
  assert.match(cli, /mod app_router_server_data;/);
  assert.ok(fs.existsSync(serverDataSurfaceModulePath), "server-data surface metadata should live in a shared small module");
  assert.match(serverDataSurfaceModule, /DX_APP_ROUTER_SERVER_DATA_SCHEMA/);
  assert.match(serverDataSurfaceModule, /DX_APP_ROUTER_SERVER_DATA_FORMAT/);
  assert.match(serverDataSurfaceModule, /DxServerDataSurfaceStatus/);
  assert.match(serverDataSurfaceModule, /insert_server_data_surface_metadata/);
  assert.match(serverDataSurfaceModule, /insert_server_data_adapter_boundary/);
  assert.match(serverDataSurfaceModule, /server_data_request_contract/);
  assert.match(serverDataSurfaceModule, /insert_build_time_server_data_request_contracts/);
  assert.match(serverDataSurfaceModule, /request_contract_marks_static_inputs_as_source_owned/);
  assert.match(serverDataSurfaceModule, /"build_output_emitted": build_output_emitted/);
  assert.match(serverDataSurfaceModule, /"runtime_request_values": runtime_request_values/);
  assert.match(serverDataSurfaceModule, /source-owned-safe-loader-data/);
  assert.match(serverDataSurfaceModule, /no-loader-bindings/);
  assert.match(serverDataSurfaceModule, /adapter-boundary/);
  assert.match(appRouterExecution, /insert_server_data_surface_metadata/);
  assert.match(appRouterExecution, /insert_server_data_adapter_boundary/);
  assert.match(buildOutputModule, /insert_server_data_surface_metadata/);
  assert.match(buildOutputModule, /insert_server_data_adapter_boundary/);
  assert.match(appRouterBuildCommand, /DxAppRouterExecutionOutputInput/);
  assert.match(appRouterBuildCommand, /write_app_router_execution_contract\(\s*DxAppRouterExecutionOutputInput/);
  assert.match(cliCoreImpl, /server_sources: &server_sources/);
  assert.match(appRouterRuntimeCommand, /server_sources: &Cli::react_server_sources\(cwd\)/);
  assert.doesNotMatch(cli, /fn write_app_router_execution_contract\(/);
  assert.doesNotMatch(cli, /write_app_router_execution_contract\(\s*DxAppRouterExecutionOutputInput/);
  assert.match(appRouterBuildCommand, /DxAppClientIslandsOutputInput/);
  assert.match(appRouterBuildCommand, /write_app_client_islands_contract\(\s*DxAppClientIslandsOutputInput/);
  assert.doesNotMatch(cli, /fn write_app_client_islands_contract\(/);
  assert.doesNotMatch(cli, /compile_react_client_islands/);
  assert.doesNotMatch(cli, /react_client_island_micro_js_bundle/);
  assert.doesNotMatch(cli, /write_app_client_islands_contract\(\s*DxAppClientIslandsOutputInput/);
  assert.match(appRouterBuildCommand, /DxAppStreamingPlanOutputInput/);
  assert.match(appRouterBuildCommand, /write_app_streaming_plan\(\s*DxAppStreamingPlanOutputInput/);
  assert.doesNotMatch(cli, /fn write_app_streaming_plan\(/);
  assert.doesNotMatch(cli, /write_app_streaming_plan\(\s*DxAppStreamingPlanOutputInput/);
  assert.match(appRouterBuildCommand, /DxAppGeneratedStyleAssetsOutputInput/);
  assert.match(appRouterBuildCommand, /write_app_generated_style_assets\(\s*DxAppGeneratedStyleAssetsOutputInput/);
  assert.doesNotMatch(cli, /fn write_app_generated_style_assets\(/);
  assert.doesNotMatch(cli, /write_app_generated_style_assets\(\s*DxAppGeneratedStyleAssetsOutputInput/);
  assert.doesNotMatch(cli, /fn safe_generated_asset_path\(/);
  assert.match(buildOutputModule, /pub\(super\) struct DxAppRouterExecutionOutputInput/);
  assert.match(buildOutputModule, /pub\(super\) server_sources: &'a \[DxReactServerSource\]/);
  assert.match(buildOutputModule, /pub\(super\) fn write_app_router_execution_contract/);
  assert.match(buildOutputModule, /server_sources: input\.server_sources/);
  assert.match(buildOutputModule, /DxAppRouterRequestValueMode::BuildTimeContractInputs/);
  assert.match(buildOutputModule, /struct DxAppStaticRouteRequestProps/);
  assert.match(buildOutputModule, /fn build_static_app_route_request_props/);
  assert.match(buildOutputModule, /fn static_route_param_names/);
  assert.match(buildOutputModule, /fn static_search_param_names/);
  assert.match(buildOutputModule, /fn static_search_param_aliases/);
  assert.match(buildOutputModule, /fn collect_static_identifier_dot_accesses/);
  assert.match(buildOutputModule, /fn collect_static_destructured_search_param_names/);
  assert.match(buildOutputModule, /search_param_destructured_name/);
  assert.match(buildOutputModule, /build_time_request_props/);
  assert.match(serverDataSurfaceModule, /static-route-contract-inputs/);
  assert.match(
    buildOutputModule,
    /insert_build_time_server_data_request_contracts\(\s*object,\s*&request_props\.route_params,\s*&request_props\.search_params,\s*\)/,
  );
  assert.match(serverDataSurfaceModule, /"build_time_contract_inputs": build_time_contract_inputs/);
  assert.match(serverDataSurfaceModule, /"runtime_request_values": runtime_request_values/);
  assert.match(serverDataSurfaceModule, /"source_owned_contract": true/);
  assert.match(serverDataSurfaceModule, /"external_runtime_required": false/);
  assert.match(serverDataSurfaceModule, /"external_runtime_executed": false/);
  assert.doesNotMatch(serverDataSurfaceModule, /full_nextjs_runtime_parity|next-parity|next_parity/);
  assert.match(
    serverDataSurfaceModule,
    /server_data_request_contract\(\s*"static-route-contract-inputs",\s*route_params,\s*search_params,\s*true,\s*false,/,
  );
  assert.match(appRouterExecution, /server_sources: &'a \[DxReactServerSource\]/);
  assert.match(appRouterExecution, /enum DxAppRouterRequestValueMode/);
  assert.match(appRouterExecution, /request_value_mode: DxAppRouterRequestValueMode/);
  assert.match(appRouterExecution, /DxAppRouterRequestValueMode::Runtime/);
  assert.match(appRouterExecution, /compile_react_server_data_manifest/);
  assert.match(
    appRouterExecution,
    /server_data_request_contract\(\s*request_value_mode\.label\(\),\s*route_params,\s*search_params,\s*request_value_mode\.is_build_time_contract_inputs\(\),\s*request_value_mode\.is_runtime\(\),/,
  );
  assert.doesNotMatch(appRouterExecution, /"build_time_contract_inputs": request_value_mode\.is_build_time_contract_inputs\(\)/);
  assert.doesNotMatch(appRouterExecution, /"runtime_request_values": request_value_mode\.is_runtime\(\)/);
  assert.match(appRouterExecution, /object\.insert\("server_data"\.to_string\(\), server_data\.clone\(\)\)/);
  assert.match(appRouterExecution, /server_data_dom_attrs\(&server_data\)/);
  assert.match(appRouterExecution, /data-dx-server-data="source-owned-safe-interpreter"/);
  assert.match(appRouterExecution, /data-dx-server-data-schema/);
  assert.match(appRouterExecution, /data-dx-server-data-format/);
  assert.match(appRouterExecution, /data-dx-server-data-revision/);
  assert.match(appRouterExecution, /data-dx-server-data-request-mode/);
  assert.match(appRouterExecution, /data-dx-server-data-runtime-values/);
  assert.match(appRouterExecution, /data-dx-server-data-build-contract-inputs/);
  assert.match(appRouterExecution, /data-dx-server-data-source-owned-contract/);
  assert.match(appRouterExecution, /data-dx-server-data-external-runtime-required/);
  assert.match(appRouterExecution, /data-dx-server-data-external-runtime-executed/);
  assert.doesNotMatch(appRouterExecution, /data-dx-server-data-next-parity/);
  assert.match(appRouterExecution, /data-dx-server-data-node-modules-required/);
  assert.match(appRouterExecution, /data-dx-server-data-lifecycle-scripts-executed/);
  assert.match(appRouterExecution, /data-dx-server-data-adapter-boundary/);
  assert.match(appRouterExecution, /data-dx-server-data-adapter-boundary-kind/);
  assert.match(appRouterExecution, /data-dx-server-data-adapter-boundary-reason/);
  assert.match(appRouterExecution, /data-dx-server-data-adapter-boundary-build-output/);
  assert.match(appRouterExecution, /data-dx-server-data-adapter-boundary-runtime-values/);
  assert.match(appRouterExecution, /json_attr_value\(server_data\.get\("schema"\)\)/);
  assert.match(appRouterExecution, /json_attr_value\(server_data\.get\("format"\)\)/);
  assert.match(appRouterExecution, /json_attr_value\(server_data\.get\("schema_revision"\)\)/);
  assert.match(appRouterExecution, /json_attr_value\(server_data\.get\("node_modules_required"\)\)/);
  assert.match(appRouterExecution, /json_attr_value\(server_data\.get\("lifecycle_scripts_executed"\)\)/);
  assert.match(appRouterExecution, /server_data_request_contract\(/);
  assert.match(appRouterExecution, /json_attr_value\(adapter_boundary_details\.get\("reason"\)\)/);
  assert.match(appRouterExecution, /json_attr_value\(adapter_boundary_details\.get\("runtime_request_values"\)\)/);
  assert.match(appRouterExecution, /json_attr_value\(request\.get\("mode"\)\)/);
  assert.match(appRouterExecution, /json_attr_value\(request\.get\("runtime_request_values"\)\)/);
  assert.match(appRouterExecution, /json_attr_value\(request\.get\("build_time_contract_inputs"\)\)/);
  assert.match(
    appRouterExecution,
    /request\s*\.get\("source_owned_contract"\)\s*\.or_else\(\|\| server_data\.get\("source_owned_contract"\)\)/,
  );
  assert.match(appRouterExecution, /data-dx-server-data-status/);
  assert.match(appRouterExecution, /data-dx-server-data-entries/);
  assert.match(appRouterExecution, /data-dx-route-count/);
  assert.match(appRouterExecution, /data-dx-package-count/);
  assert.doesNotMatch(appRouterExecution, /full_nextjs_runtime_parity|next-parity|next_parity/);
  assert.match(appRouterExecution, /source_owned_contract": true/);
  assert.match(appRouterExecution, /external_runtime_required": false/);
  assert.match(appRouterExecution, /external_runtime_executed": false/);
  assert.match(appRouterExecution, /server_data_surface_marks_runtime_request_values_as_source_owned/);
  assert.match(appRouterExecution, /server_data_dom_attrs_expose_schema_revision_for_dev_build_consistency/);
  assert.match(appRouterExecution, /server_data_dom_attrs_expose_numeric_format_for_stable_schema_name/);
  assert.match(appRouterExecution, /data-dx-server-data-source-owned-contract="true"/);
  assert.match(appRouterExecution, /data-dx-server-data-external-runtime-required="false"/);
  assert.match(appRouterExecution, /data-dx-server-data-external-runtime-executed="false"/);
  assert.match(appRouterExecution, /server_data_dom_attrs_expose_request_mode_as_source_owned/);
  assert.match(appRouterExecution, /server_data_dom_attrs_expose_source_safe_loader_flags/);
  assert.match(appRouterExecution, /server_data_dom_attrs_expose_adapter_boundary_without_fake_runtime/);
  assert.match(appRouterExecution, /server_data_surface_marks_adapter_boundary_contract_without_fake_runtime/);
  assert.match(appRouterExecution, /server_data_dom_attrs_expose_adapter_boundary_reason/);
  assert.match(appRouterExecution, /server_data_dom_attrs_expose_no_loader_status_without_fake_runtime/);
  assert.match(buildOutputModule, /build_app_router_execution_contract/);
  assert.match(buildOutputModule, /route_params: &request_props\.route_params/);
  assert.match(buildOutputModule, /search_params: &request_props\.search_params/);
  assert.match(buildOutputModule, /static_request_props_cover_destructured_search_params/);
  assert.match(buildOutputModule, /static_request_props_cover_awaited_search_param_aliases/);
  assert.match(buildOutputModule, /tab: activeTab/);
  assert.match(buildOutputModule, /resolvedSearchParams\.tab/);
  assert.doesNotMatch(buildOutputModule, /let empty_route_params = BTreeMap::new\(\);/);
  assert.doesNotMatch(buildOutputModule, /let empty_search_params = BTreeMap::new\(\);/);
  assert.match(buildOutputModule, /app_output_dir\.join\("app-router-execution\.json"\)/);
  assert.match(buildOutputModule, /pub\(super\) struct DxAppClientIslandsOutputInput/);
  assert.match(buildOutputModule, /pub\(super\) fn write_app_client_islands_contract/);
  assert.match(buildOutputModule, /compile_react_client_islands/);
  assert.match(buildOutputModule, /react_client_island_micro_js_bundle/);
  assert.match(buildOutputModule, /app_output_dir\.join\("client-islands\.json"\)/);
  assert.match(buildOutputModule, /app_output_dir\.join\("client-islands\.js"\)/);
  assert.match(buildOutputModule, /pub\(super\) struct DxAppStreamingPlanOutputInput/);
  assert.match(buildOutputModule, /pub\(super\) fn write_app_streaming_plan/);
  assert.match(buildOutputModule, /proof\.streaming\.enabled/);
  assert.match(buildOutputModule, /app_output_dir\.join\("streaming-plan\.json"\)/);
  assert.match(buildOutputModule, /pub\(super\) struct DxAppGeneratedStyleAssetsOutputInput/);
  assert.match(buildOutputModule, /pub\(super\) fn write_app_generated_style_assets/);
  assert.match(buildOutputModule, /fn safe_generated_asset_path/);
  assert.match(buildOutputModule, /Generated asset path must be relative/);
  assert.match(buildOutputModule, /Generated asset path cannot escape build output/);
  assert.doesNotMatch(cli, /compile_react_server_data_manifest/);
  assert.match(appRouterBuildCommand, /write_app_server_data_contract\(\s*DxAppServerDataOutputInput/);
  assert.doesNotMatch(cli, /write_app_server_data_contract\(\s*DxAppServerDataOutputInput/);
  assert.match(buildOutputModule, /pub\(super\) struct DxAppServerDataOutputInput/);
  assert.match(buildOutputModule, /pub\(super\) fn write_app_server_data_contract/);
  assert.match(buildOutputModule, /compile_react_server_data_manifest/);
  assert.match(buildOutputModule, /fn server_data_manifest_contract_json/);
  assert.match(buildOutputModule, /fn server_data_adapter_boundary_contract_json/);
  assert.match(buildOutputModule, /fn insert_build_time_request_props/);
  assert.match(buildOutputModule, /insert_build_time_server_data_request_contracts/);
  assert.doesNotMatch(
    buildOutputModule,
    /"request"\.to_string\(\),\s*json!\(\{\s*"mode": "static-route-contract-inputs"/,
  );
  assert.doesNotMatch(
    buildOutputModule,
    /"build_time_request_props"\.to_string\(\),\s*server_data_request_contract\(/,
  );
  assert.doesNotMatch(appRouterExecution, /"status": if entry_count == 0/);
  assert.doesNotMatch(appRouterExecution, /"execution_model": if entry_count == 0/);
  assert.doesNotMatch(buildOutputModule, /fn insert_server_data_surface_metadata/);
  assert.doesNotMatch(
    buildOutputModule,
    /compile_react_server_data_manifest\(\s*input\.route,[\s\S]{0,300}\)\s*\.map_err\(\|message\| DxError::ConfigValidationError/,
  );
  assert.doesNotMatch(buildOutputModule, /full_nextjs_runtime_parity|next-parity|next_parity/);
  assert.match(buildOutputModule, /source_owned_contract/);
  assert.match(buildOutputModule, /external_runtime_required/);
  assert.match(buildOutputModule, /external_runtime_executed/);
  assert.match(buildOutputModule, /server_data_contracts_can_exist_without_loader_entries_for_dynamic_request_props/);
  assert.match(buildOutputModule, /server_data_contract_marks_compile_errors_as_adapter_boundary_without_fake_runtime/);
  assert.match(buildOutputModule, /DxServerDataSurfaceStatus::AdapterBoundary/);
  assert.match(buildOutputModule, /"unsupported-safe-loader-shape"/);
  assert.match(buildOutputModule, /"adapter_boundary"/);
  assert.match(buildOutputModule, /insert_server_data_adapter_boundary\(object, error, true, false\)/);
  assert.match(buildOutputModule, /manifest\.entries\.is_empty\(\)\s*&&\s*request_props\.route_params\.is_empty\(\)\s*&&\s*request_props\.search_params\.is_empty\(\)/);
  assert.match(buildOutputModule, /app\/dashboard\/\[team\]\/page\.tsx/);
  assert.match(buildOutputModule, /app_output_dir\.join\("server-data\.json"\)/);
  assert.match(buildCommandModule, /"server_data_entries_compiled": input\.server_data_entries_compiled/);
  assert.match(cli, /mod app_server_data_manifest;/);
  assert.match(
    cliCoreImpl,
    /collect_app_server_data_manifest\(\s*&self\.cwd,\s*&output_dir,\s*&source_build_report\.server_data_routes,\s*\)/,
  );
  assert.match(buildCommandModule, /"server_data_routes": input\.server_data_routes/);
  assert.match(buildCommandModule, /"server_data_routes_compiled": input\.server_data_routes\.len\(\)/);
  assert.match(
    cliCoreImpl,
    /let server_data_route_manifest = summarize_app_server_data_manifest_routes/,
  );
  assert.match(buildCommandModule, /"server_data_route_manifest": input\.server_data_route_manifest/);
  assert.match(serverDataBuildManifestModule, /pub\(super\) fn collect_app_server_data_manifest/);
  assert.match(serverDataBuildManifestModule, /pub\(super\) fn summarize_app_server_data_manifest_routes/);
  assert.match(serverDataBuildManifestModule, /use crate::build::SourceBuildServerDataRoute;/);
  assert.match(serverDataBuildManifestModule, /fn collect_source_build_server_data_manifest/);
  assert.match(serverDataBuildManifestModule, /fn validate_source_build_server_data_contract/);
  assert.match(serverDataBuildManifestModule, /source-build server-data artifact route mismatch/);
  assert.match(serverDataBuildManifestModule, /source-build server-data artifact route_source_path mismatch/);
  assert.match(serverDataBuildManifestModule, /source-build server-data artifact must declare source_owned_contract=true/);
  assert.match(serverDataBuildManifestModule, /fn missing_source_build_server_data_routes/);
  assert.match(serverDataBuildManifestModule, /source_build_route_keys/);
  assert.match(serverDataBuildManifestModule, /source_build_route\.output/);
  assert.match(serverDataBuildManifestModule, /"manifest_includes_source_build_routes"/);
  assert.match(serverDataBuildManifestModule, /"missing_source_build_routes"/);
  assert.match(serverDataBuildManifestModule, /walkdir::WalkDir::new\(&app_output_dir\)/);
  assert.match(serverDataBuildManifestModule, /"output":/);
  assert.match(serverDataBuildManifestModule, /"route_source_path":/);
  assert.match(serverDataBuildManifestModule, /"source_owned_contract":/);
  assert.match(serverDataBuildManifestModule, /"external_runtime_required":/);
  assert.match(serverDataBuildManifestModule, /"external_runtime_executed":/);
  assert.doesNotMatch(serverDataBuildManifestModule, /full_nextjs_runtime_parity|next-parity|next_parity/);
  assert.match(serverDataBuildManifestModule, /let mut route_entry = json!\(\{/);
  assert.match(serverDataBuildManifestModule, /if let Some\(object\) = route_entry\.as_object_mut\(\)/);
  assert.match(serverDataBuildManifestModule, /object\.insert\(key\.to_string\(\), value\.clone\(\)\)/);

  assert.match(serverContract, /compile_react_server_data_manifest/);
  assert.match(serverContract, /source\.source_path\.replace\('\\\\', "\/"\) == import\.source_path/);
  assert.match(serverContract, /execution_model: "source-owned-safe-interpreter"\.to_string\(\)/);
  assert.match(serverContract, /node_modules_required: false/);
  assert.match(serverContract, /lifecycle_scripts_executed: false/);

  assert.ok(fs.existsSync(serverDataProofTestPath), "server-data build proof should live in a focused integration test");
  assert.match(serverDataProofTest, /fn dx_build_emits_server_data_for_loader_and_dynamic_request_props/);
  assert.match(serverDataProofTest, /project_cli\.cmd_build\(\)\.expect\("dx build"\)/);
  assert.match(serverDataProofTest, /\.dx\/build\/app\/server-data\.json/);
  assert.match(serverDataProofTest, /\.dx\/build\/app\/dashboard\/\[team\]\/server-data\.json/);
  assert.match(serverDataProofTest, /build_time_request_props/);
  assert.match(serverDataProofTest, /root_server_data\["status"\]/);
  assert.match(serverDataProofTest, /root_server_data\["format"\]/);
  assert.match(serverDataProofTest, /root_server_data\["entry_count"\]/);
  assert.match(serverDataProofTest, /root_server_data\["execution_model"\]/);
  assert.match(serverDataProofTest, /server_data_route_manifest/);
  assert.match(serverDataProofTest, /manifest_includes_source_build_routes/);
  assert.match(serverDataProofTest, /dynamic_server_data\["status"\]/);
  assert.match(serverDataProofTest, /dynamic_server_data\["entry_count"\]/);
  assert.match(serverDataProofTest, /dynamic_server_data\["execution_model"\]/);
  assert.match(serverDataProofTest, /root_server_data\["source_owned_contract"\]/);
  assert.match(serverDataProofTest, /root_server_data\["external_runtime_required"\]/);
  assert.match(serverDataProofTest, /root_server_data\["external_runtime_executed"\]/);
  assert.doesNotMatch(serverDataProofTest, /full_nextjs_runtime_parity|next-parity|next_parity/);
  assert.match(serverDataProofTest, /root_server_data\["request"\]\["mode"\]/);
  assert.match(serverDataProofTest, /dynamic_server_data\["request"\]\["mode"\]/);
  assert.match(serverDataProofTest, /root_server_data\["request"\]\["build_time_contract_inputs"\]/);
  assert.match(serverDataProofTest, /root_server_data\["build_time_request_props"\]\["build_time_contract_inputs"\]/);
  assert.match(serverDataProofTest, /dynamic_server_data\["build_time_request_props"\]\["build_time_contract_inputs"\]/);
  assert.match(serverDataProofTest, /sample-team/);
  assert.match(serverDataProofTest, /sample-tab/);
  assert.match(serverDataProofTest, /manifest\["server_data_routes"\]/);
  assert.match(serverDataProofTest, /app\/dashboard\/\[team\]\/server-data\.json/);
  assert.match(serverDataProofTest, /source-routes\/dashboard--team\/server-data\.json/);
  assert.match(serverDataProofTest, /!root\.join\("node_modules"\)\.exists\(\)/);

  assert.ok(
    fs.existsSync(sourceBuildServerDataProofPath),
    "source-build server-data behavior should have a focused integration proof",
  );
  assert.match(sourceBuildServerData, /pub struct SourceBuildServerDataRoute/);
  assert.match(sourceBuildServerData, /pub source_owned_contract: bool/);
  assert.match(sourceBuildServerData, /pub external_runtime_required: bool/);
  assert.match(sourceBuildServerData, /pub external_runtime_executed: bool/);
  assert.doesNotMatch(sourceBuildServerData, /full_nextjs_runtime_parity|next-parity|next_parity/);
  assert.match(sourceBuildGraph, /pub server_data_routes: Vec<SourceBuildServerDataRoute>/);
  assert.match(sourceBuildGraph, /server_data_routes: server_data_routes\.to_vec\(\)/);
  assert.match(sourceBuildGraph, /pub server_data_output: Option<String>/);
  assert.match(sourceBuildRouteOutput, /server_data_output: None/);
  assert.match(sourceBuildServerData, /compile_react_server_data_manifest/);
  assert.match(sourceBuildServerData, /attach_server_data_outputs/);
  assert.match(sourceBuildServerData, /source-routes.+server-data\.json/s);
  assert.match(sourceBuildReceipt, /pub server_data_routes: usize/);
  assert.match(sourceBuildReceipt, /pub server_data_entries: usize/);
  assert.match(sourceBuildServerDataProof, /source_build_manifest_records_route_server_data_outputs/);
  assert.match(sourceBuildServerDataProof, /report\.server_data_routes\.len\(\), 5/);
  assert.match(sourceBuildServerDataProof, /summary\.server_data_entries, 1/);
  assert.match(sourceBuildServerDataProof, /source-routes\/root\/server-data\.json/);
  assert.match(sourceBuildServerDataProof, /source-routes\/dashboard--team\/server-data\.json/);
});
