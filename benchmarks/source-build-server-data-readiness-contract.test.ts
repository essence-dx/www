import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("source-build readiness graph exposes server-data output counts", () => {
  const readiness = read("dx-www/src/build/source_engine/readiness.rs");
  const sourceBuildGraph = read("dx-www/src/build/source_engine/graph.rs");

  assert.match(readiness, /"server_data_routes": manifest\.server_data_routes\.len\(\)/);
  assert.match(
    readiness,
    /let server_data_entries = manifest\s*\.\s*server_data_routes\s*\.\s*iter\(\)\s*\.\s*map\(\|route\| route\.entry_count\)\s*\.\s*sum::<usize>\(\);/s,
  );
  assert.match(readiness, /"server_data_entries": server_data_entries/);
  assert.match(sourceBuildGraph, /pub server_data_route_manifest: SourceBuildServerDataRouteManifest/);
  assert.match(sourceBuildGraph, /pub struct SourceBuildServerDataRouteManifest/);
  assert.match(sourceBuildGraph, /source_build_server_data_route_manifest\(server_data_routes\)/);
});

test("source-build server-data samples src/app dynamic params without Next runtime", () => {
  const serverData = read("dx-www/src/build/source_engine/server_data.rs");
  const serverDataProof = read("dx-www/tests/source_build_server_data.rs");

  assert.match(
    serverData,
    /#\[serde\(rename = "route_source_path", alias = "source_path"\)\]\s*pub source_path: String/s,
  );
  assert.match(serverDataProof, /route\["route_source_path"\] == "app\/page\.tsx"/);
  assert.match(serverDataProof, /route\.get\("source_path"\)\.is_none\(\)/);
  assert.match(serverData, /fn app_route_source_without_app_root/);
  assert.match(serverData, /strip_prefix\("src\/app\/"\)/);
  assert.match(serverData, /strip_prefix\("app\/"\)/);
  assert.match(serverData, /strip_app_page_suffix/);
  assert.match(serverData, /src_app_dynamic_params_are_sampled/);
  assert.match(serverData, /"src\/app\/dashboard\/\[team\]\/page\.tsx"/);
  assert.match(serverData, /"sample-team"/);
  assert.doesNotMatch(serverData, /Turbopack\s+powers|Next\s+DevTools|next-devtools/);
});

test("source-build request props include optional searchParams reads", () => {
  const serverData = read("dx-www/src/build/source_engine/server_data.rs");
  const appRouterBuildOutput = read("dx-www/src/cli/app_router_build_output.rs");
  const serverDataProof = read("dx-www/tests/source_build_server_data.rs");

  for (const source of [serverData, appRouterBuildOutput]) {
    assert.match(
      source,
      /collect_static_dot_accesses\(route_source, "searchParams\?\.", &mut names\);/,
    );
    assert.match(
      source,
      /collect_static_dot_accesses\(route_source, "\(await searchParams\)\?\.", &mut names\);/,
    );
    assert.match(
      source,
      /collect_static_bracket_accesses\(route_source, "searchParams\?\.\[", &mut names\);/,
    );
    assert.match(
      source,
      /collect_static_bracket_accesses\(route_source, "\(await searchParams\)\?\.\[", &mut names\);/,
    );
    assert.match(source, /collect_static_identifier_optional_dot_accesses\(route_source, &alias, &mut names\);/);
    assert.match(source, /collect_static_identifier_optional_bracket_accesses\(route_source, &alias, &mut names\);/);
  }

  assert.match(serverDataProof, /resolvedSearchParams\?\.tab/);
  assert.match(serverDataProof, /searchParams\?\.\["view"\]/);
  assert.match(serverDataProof, /\(await searchParams\)\?\.mode/);
  assert.match(serverDataProof, /search_param_keys"\]\[2\],\s*"view"/s);
});

test("source-build graph exposes server-data route artifacts as source-owned nodes", () => {
  const serverData = read("dx-www/src/build/source_engine/server_data.rs");
  const graph = read("dx-www/src/build/source_engine/ecosystem_graph.rs");
  const invalidation = read("dx-www/src/build/source_engine/ecosystem_invalidation.rs");
  const serverDataProof = read("dx-www/tests/source_build_server_data.rs");

  assert.match(graph, /SourceBuildServerDataRoute/);
  assert.match(graph, /&manifest\.server_data_routes/);
  assert.match(graph, /"kind": "server-data-route"/);
  assert.match(graph, /"route_source_path": server_data\.source_path/);
  assert.match(graph, /"output": server_data\.output/);
  assert.match(graph, /"entry_source_paths": server_data\.entry_source_paths/);
  assert.match(graph, /"request": server_data\.request/);
  assert.match(graph, /"node_modules_required": server_data\.node_modules_required/);
  assert.match(graph, /"source_owned_contract": server_data\.source_owned_contract/);
  assert.match(graph, /"external_runtime_required": server_data\.external_runtime_required/);
  assert.match(graph, /"external_runtime_executed": server_data\.external_runtime_executed/);
  assert.match(graph, /"kind": "emits-server-data"/);
  assert.match(graph, /"kind": "links-server-data"/);
  assert.match(graph, /"kind": "uses-server-loader"/);
  assert.match(graph, /server_data_summary\(&nodes, &edges\)/);
  assert.match(graph, /"serverData": server_data/);
  assert.match(invalidation, /"server-data-route"/);

  assert.match(serverData, /pub entry_source_paths: Vec<String>/);
  assert.match(serverData, /pub request: SourceBuildServerDataRequest/);
  assert.match(serverData, /pub struct SourceBuildServerDataRequest/);
  assert.match(serverData, /source_build_server_data_request\(&request_props\)/);
  assert.match(serverData, /fn entry_source_paths\(manifest: &DxReactServerDataManifest\)/);
  assert.match(serverDataProof, /graph_receipt\["graph"\]\["nodes"\]/);
  assert.match(serverDataProof, /node\["kind"\] == "server-data-route"/);
  assert.match(serverDataProof, /entry_source_paths/);
  assert.match(serverDataProof, /server_data_node\["route_source_path"\]/);
  assert.match(serverDataProof, /server_data_node\["request"\]\["route_params"\]\["team"\]/);
  assert.match(serverDataProof, /server_data_node\["source_owned_contract"\]/);
  assert.match(serverDataProof, /edge\["kind"\] == "emits-server-data"/);
  assert.match(serverDataProof, /edge\["kind"\] == "uses-server-loader"/);
  assert.match(serverDataProof, /rebuildNodeIds/);
  assert.match(serverDataProof, /rebuildRoutes/);
  assert.match(serverDataProof, /graph_snapshot\["graph"\]\["serverData"\]\["routeCount"\]/);
});

test("source-build emits no-loader server-data contracts for static App Router routes", () => {
  const serverData = read("dx-www/src/build/source_engine/server_data.rs");
  const serverDataProof = read("dx-www/tests/source_build_server_data.rs");

  assert.match(serverDataProof, /app\/about\/page\.tsx/);
  assert.match(serverDataProof, /source-routes\/about\/server-data\.json/);
  assert.match(serverDataProof, /route\["route"\] == "\/about"[\s\S]{0,160}route\["server_data_output"\]/);
  assert.match(serverDataProof, /report\.server_data_routes\.len\(\), 5/);
  assert.match(serverDataProof, /static_server_data\["status"\], "no-loader-bindings"/);
  assert.doesNotMatch(
    serverData,
    /if manifest\.entries\.is_empty\(\)\s*&&\s*request_props\.route_params\.is_empty\(\)\s*&&\s*request_props\.search_params\.is_empty\(\)\s*\{\s*continue;\s*\}/s,
  );
});

test("source-build server-data route paths are collision-safe without renaming stable routes", () => {
  const serverData = read("dx-www/src/build/source_engine/server_data.rs");
  const routePaths = read("dx-www/src/build/source_engine/route_paths.rs");
  const serverDataProof = read("dx-www/tests/source_build_server_data.rs");

  assert.match(routePaths, /fn source_route_output_slugs\(routes: &\[SourceBuildRoute\]\)/);
  assert.match(routePaths, /hash_bytes\(route\.path\.as_bytes\(\)\)/);
  assert.match(serverData, /source_route_output_slugs\(routes\)/);
  assert.match(serverDataProof, /app\/docs\/\[slug\]\/page\.tsx/);
  assert.match(serverDataProof, /app\/docs\/-slug\/page\.tsx/);
  assert.match(serverDataProof, /assert_ne!\(\s*dynamic_docs_output,\s*literal_docs_output/s);
  assert.match(serverDataProof, /source-routes\/root\/server-data\.json/);
  assert.match(serverDataProof, /source-routes\/about\/server-data\.json/);
  assert.match(serverDataProof, /source-routes\/dashboard--team\/server-data\.json/);
});

test("source-build route outputs and server-data share collision-safe route path planning", () => {
  const sourceEngineMod = read("dx-www/src/build/source_engine/mod.rs");
  const routePaths = read("dx-www/src/build/source_engine/route_paths.rs");
  const routeOutput = read("dx-www/src/build/source_engine/route_output.rs");
  const serverData = read("dx-www/src/build/source_engine/server_data.rs");
  const serverDataProof = read("dx-www/tests/source_build_server_data.rs");

  assert.match(sourceEngineMod, /mod route_paths;/);
  assert.match(routePaths, /pub\(super\) fn source_route_output_slugs\(routes: &\[SourceBuildRoute\]\)/);
  assert.match(routePaths, /pub\(super\) fn source_route_key\(route: &SourceBuildRoute\)/);
  assert.match(routePaths, /hash_bytes\(route\.path\.as_bytes\(\)\)/);
  assert.match(routeOutput, /source_route_output_slugs\(routes\)/);
  assert.match(routeOutput, /source_route_key\(route\)/);
  assert.doesNotMatch(routeOutput, /fn route_slug\(route: &str\)/);
  assert.match(serverData, /source_route_output_slugs\(routes\)/);
  assert.match(serverData, /source_route_key\(route\)/);
  assert.doesNotMatch(serverData, /fn server_data_route_slugs\(routes: &\[SourceBuildRoute\]\)/);
  assert.doesNotMatch(serverData, /fn route_slug\(route: &str\)/);
  assert.match(serverDataProof, /dynamic_docs_html_output/);
  assert.match(serverDataProof, /literal_docs_html_output/);
  assert.match(serverDataProof, /assert_ne!\(\s*dynamic_docs_html_output,\s*literal_docs_html_output/s);
  assert.match(serverDataProof, /dynamic_docs_route_output\["server_data_output"\]/);
  assert.match(serverDataProof, /literal_docs_route_output\["server_data_output"\]/);
});

test("dx build public manifest prefers source-build server-data route artifacts", () => {
  const cli = read("dx-www/src/cli/mod.rs");
  const publicManifest = read("dx-www/src/cli/app_server_data_manifest.rs");

  assert.match(
    cli,
    /collect_app_server_data_manifest\(\s*&self\.cwd,\s*&output_dir,\s*&source_build_report\.server_data_routes,\s*\)/,
  );
  assert.match(publicManifest, /use crate::build::SourceBuildServerDataRoute;/);
  assert.match(publicManifest, /source_build_server_data_routes: &\[SourceBuildServerDataRoute\]/);
  assert.match(publicManifest, /fn collect_source_build_server_data_manifest/);
  assert.match(publicManifest, /source_build_route\.output/);
  assert.match(publicManifest, /source_build_route_keys/);
  assert.match(publicManifest, /fn server_data_route_source_key/);
  assert.match(publicManifest, /project_relative_output_path\(project_dir, &source_build_route\.output\)/);
  assert.doesNotMatch(publicManifest, /full_nextjs_runtime_parity|next-parity|next_parity/);
});

test("dx build public manifest reports source-build server-data consistency", () => {
  const cli = read("dx-www/src/cli/mod.rs");
  const buildCommand = read("dx-www/src/cli/build_command.rs");
  const publicManifest = read("dx-www/src/cli/app_server_data_manifest.rs");

  assert.match(
    cli,
    /let server_data_route_manifest = summarize_app_server_data_manifest_routes\(\s*&server_data_routes,\s*&source_build_report\.server_data_routes,\s*\);/,
  );
  assert.match(buildCommand, /pub\(super\) server_data_route_manifest: &'a Value/);
  assert.match(buildCommand, /"server_data_route_manifest": input\.server_data_route_manifest/);
  assert.match(
    publicManifest,
    /pub\(super\) fn summarize_app_server_data_manifest_routes\(\s*routes: &\[Value\],\s*source_build_server_data_routes: &\[SourceBuildServerDataRoute\],\s*\) -> Value/s,
  );
  assert.match(publicManifest, /let missing_source_build_routes\s*=\s*missing_source_build_server_data_routes/);
  assert.match(publicManifest, /"source_build_routes": source_build_server_data_routes\.len\(\)/);
  assert.match(publicManifest, /"manifest_routes": routes\.len\(\)/);
  assert.match(publicManifest, /"source_build_entries": source_build_server_data_entry_count/);
  assert.match(publicManifest, /"manifest_entries": manifest_server_data_entry_count\(routes\)/);
  assert.match(publicManifest, /"routes_with_route_params": request_prop_route_count\(routes, "route_params"\)/);
  assert.match(publicManifest, /"routes_with_search_params": request_prop_route_count\(routes, "search_params"\)/);
  assert.match(publicManifest, /"route_param_keys": request_prop_keys\(routes, "route_params"\)/);
  assert.match(publicManifest, /"search_param_keys": request_prop_keys\(routes, "search_params"\)/);
  assert.match(publicManifest, /fn request_prop_keys\(routes: &\[Value\], field: &str\) -> Vec<String>/);
  assert.match(publicManifest, /fn request_prop_route_count\(routes: &\[Value\], field: &str\) -> usize/);
  assert.match(publicManifest, /"manifest_includes_source_build_routes": missing_source_build_routes\.is_empty\(\)/);
  assert.match(publicManifest, /"missing_source_build_routes": missing_source_build_routes/);
});

test("dx build rejects stale source-build server-data artifacts before public manifest emission", () => {
  const publicManifest = read("dx-www/src/cli/app_server_data_manifest.rs");

  assert.match(
    publicManifest,
    /validate_source_build_server_data_contract\(source_build_route, &contract\)\?/,
  );
  assert.match(
    publicManifest,
    /fn validate_source_build_server_data_contract\(\s*source_build_route: &SourceBuildServerDataRoute,\s*contract: &Value,\s*\) -> DxResult<\(\)>/s,
  );
  assert.match(publicManifest, /server-data artifact route mismatch/);
  assert.match(publicManifest, /server-data artifact route_source_path mismatch/);
  assert.match(publicManifest, /server-data artifact must declare node_modules_required=false/);
  assert.match(publicManifest, /server-data artifact must declare lifecycle_scripts_executed=false/);
  assert.match(publicManifest, /server-data artifact must declare source_owned_contract=true/);
  assert.match(publicManifest, /server-data artifact must not require external runtime/);
  assert.match(publicManifest, /server-data artifact must not execute external runtime/);
  assert.match(publicManifest, /validate_source_build_server_data_request/);
  assert.match(publicManifest, /server-data artifact request props mismatch/);
});

test("dx build public manifest normalizes route_source_path before consistency checks", () => {
  const publicManifest = read("dx-www/src/cli/app_server_data_manifest.rs");

  assert.match(
    publicManifest,
    /"route_source_path": normalized_server_data_route_source_path\(contract\)/,
  );
  assert.match(
    publicManifest,
    /fn normalized_server_data_route_source_path\(contract: &Value\) -> Value/,
  );
  assert.match(publicManifest, /fn manifest_route_source_path\(route: &Value\) -> Option<String>/);
  assert.match(
    publicManifest,
    /manifest_route_source_path\(route\)\.as_deref\(\)\s*==\s*Some\(source_build_source_path\.as_str\(\)\)/s,
  );
});

test("dx build public manifest annotates entries that came from source-build server-data routes", () => {
  const publicManifest = read("dx-www/src/cli/app_server_data_manifest.rs");

  assert.match(publicManifest, /source_build_server_data_route_json\(source_build_route\)/);
  assert.match(publicManifest, /object\.insert\("source_build_route"\.to_string\(\), source_build_route\);/);
  assert.match(publicManifest, /fn source_build_server_data_route_json\(route: &SourceBuildServerDataRoute\) -> Value/);
  assert.match(publicManifest, /"route": route\.route\.as_str\(\)/);
  assert.match(publicManifest, /"route_source_path": normalized_manifest_path\(&route\.source_path\)/);
  assert.match(publicManifest, /"output": normalized_manifest_path\(&route\.output\)/);
  assert.match(publicManifest, /"entry_source_paths": normalized_manifest_paths\(&route\.entry_source_paths\)/);
  assert.match(publicManifest, /"source_owned_contract": route\.source_owned_contract/);
  assert.match(publicManifest, /fn normalized_manifest_paths\(paths: &\[String\]\) -> Vec<String>/);
});
