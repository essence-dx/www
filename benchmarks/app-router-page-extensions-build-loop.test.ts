import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const cliModPath = path.join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const appSegmentFilesPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "app_segment_files.rs",
);
const appRouterRuntimeCommandPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "app_router_runtime_command.rs",
);
const appRouterBuildCommandPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "app_router_build_command.rs",
);
const appRouterPathsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "app_router_paths.rs",
);
const appRouterStyleAssetsPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "app_router_style_assets.rs",
);
const sourceDiscoveryPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "build",
  "source_engine",
  "discovery.rs",
);
const sourceGraphPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "build",
  "source_engine",
  "graph.rs",
);
const appPageRoutesPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "app_page_routes.rs",
);
const nextMigrationPlanPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "next_migration_plan.rs",
);

test("dx build App Router page discovery accepts Next-familiar page extensions and src/app roots", () => {
  const cliMod = fs.readFileSync(cliModPath, "utf8");
  const appSegmentFiles = fs.readFileSync(appSegmentFilesPath, "utf8");
  const appRouterRuntimeCommand = fs.readFileSync(appRouterRuntimeCommandPath, "utf8");
  const appRouterBuildCommand = fs.readFileSync(appRouterBuildCommandPath, "utf8");
  const appRouterPaths = fs.readFileSync(appRouterPathsPath, "utf8");
  const appRouterStyleAssets = fs.readFileSync(appRouterStyleAssetsPath, "utf8");
  const appPageRoutes = fs.readFileSync(appPageRoutesPath, "utf8");
  const nextMigrationPlan = fs.readFileSync(nextMigrationPlanPath, "utf8");
  const sourceDiscovery = fs.readFileSync(sourceDiscoveryPath, "utf8");
  const sourceGraph = fs.readFileSync(sourceGraphPath, "utf8");

  for (const extension of ["tsx", "jsx", "ts", "js"]) {
    assert.match(appSegmentFiles, new RegExp(`"${extension}"`));
  }
  assert.match(appSegmentFiles, /APP_ROUTER_SOURCE_ROOTS/);
  assert.match(appSegmentFiles, /"src\/app"/);
  assert.match(appSegmentFiles, /app_route_roots/);
  assert.match(appSegmentFiles, /app_root_for_route/);
  assert.match(appSegmentFiles, /strip_app_root_prefix/);

  assert.match(appPageRoutes, /app_segment_files::app_route_roots\(cwd\)/);
  assert.match(appPageRoutes, /src\/app\/page\.tsx/);

  assert.match(appRouterBuildCommand, /app_segment_files::is_app_page_file_name/);
  assert.match(cliMod, /mod app_router_runtime_command;/);
  assert.match(cliMod, /mod app_router_build_command;/);
  assert.match(cliMod, /mod app_router_paths;/);
  assert.match(cliMod, /mod app_router_style_assets;/);
  assert.match(cliMod, /app_router_runtime_command::render_app_route\(cwd, &app_route_match\)/);
  assert.match(
    cliMod,
    /app_router_style_assets::render_generated_style_asset\(cwd, path\)/,
  );
  assert.doesNotMatch(cliMod, /fn render_app_route\(/);
  assert.doesNotMatch(cliMod, /fn render_generated_style_asset\(/);
  assert.doesNotMatch(cliMod, /fn compile_app_route_proof\(/);
  assert.doesNotMatch(cliMod, /fn react_app_segment_sources\(/);
  assert.doesNotMatch(cliMod, /fn route_from_app_path\(/);
  assert.doesNotMatch(cliMod, /fn app_build_output_dir\(/);
  assert.match(appRouterRuntimeCommand, /pub\(super\) fn render_app_route/);
  assert.match(appRouterRuntimeCommand, /pub\(super\) fn compile_app_route_proof/);
  assert.match(appRouterRuntimeCommand, /pub\(super\) fn react_app_segment_sources/);
  assert.match(appRouterRuntimeCommand, /fn app_segment_dirs/);
  assert.match(
    appRouterRuntimeCommand,
    /app_segment_files::app_root_for_route\(cwd, app_route_path\)/,
  );
  assert.match(appRouterRuntimeCommand, /render_app_router_runtime/);
  assert.match(appRouterRuntimeCommand, /DxAppRouterRequestValueMode::Runtime/);
  assert.match(appRouterRuntimeCommand, /tsx_launch_runtime::render_launch_tsx_route/);
  assert.match(appRouterRuntimeCommand, /app_router_paths::route_from_app_path/);
  assert.match(appRouterBuildCommand, /app_segment_files::app_route_roots\(input\.cwd\)/);
  assert.match(appRouterBuildCommand, /pub\(super\) fn is_app_router_entrypoint_compiled/);
  assert.match(appRouterBuildCommand, /entrypoint_compiled/);
  assert.match(appRouterBuildCommand, /app_router_paths::app_build_output_dir/);
  assert.match(appRouterBuildCommand, /app_router_paths::route_from_app_path/);
  assert.match(appRouterPaths, /pub\(super\) fn route_from_app_path/);
  assert.match(appRouterPaths, /pub\(super\) fn app_build_output_dir/);
  assert.match(appRouterPaths, /app_page_routes::route_path_from_page_source_path/);
  assert.match(appRouterPaths, /Cli::relative_cli_path/);
  assert.match(cliMod, /tsx_app_router_entrypoint:\s*app_router_build\.entrypoint_compiled/);
  assert.doesNotMatch(cliMod, /fn is_app_router_entrypoint_compiled/);
  assert.match(appRouterStyleAssets, /pub\(super\) fn render_generated_style_asset/);
  assert.match(appRouterStyleAssets, /app_segment_files::app_route_roots\(cwd\)/);
  assert.match(appRouterStyleAssets, /compile_app_route_proof\(cwd, entry\.path\(\)\)/);
  assert.match(appRouterStyleAssets, /route_sources_scanned/);
  assert.match(appRouterStyleAssets, /node_modules_required": false/);
  assert.match(appRouterStyleAssets, /lifecycle_scripts_executed": false/);
  assert.match(appRouterStyleAssets, /source_owned_contract": true/);
  assert.match(appRouterStyleAssets, /generated-style-not-found/);
  assert.match(appRouterStyleAssets, /No source-owned App Router generated style asset matched/);
  assert.match(appRouterStyleAssets, /route_path_from_page_source_path\(&relative\)\.is_some\(\)/);
  assert.doesNotMatch(cliMod, /e\.file_name\(\)\s*==\s*"page\.tsx"/);
  assert.doesNotMatch(cliMod, /entry\.file_name\(\)\s*==\s*"page\.tsx"/);

  assert.match(sourceDiscovery, /APP_ROUTE_ROOTS/);
  assert.match(sourceDiscovery, /APP_PAGE_FILENAMES/);
  assert.match(sourceDiscovery, /"src\/app"/);
  assert.match(sourceDiscovery, /for app_dir in app_route_roots\(project_root\)/);
  for (const filename of ["page.tsx", "page.jsx", "page.ts", "page.js"]) {
    assert.match(sourceDiscovery, new RegExp(`"${filename.replace(".", "\\.")}"`));
  }
  assert.match(sourceDiscovery, /is_app_page_file_name/);
  assert.match(sourceDiscovery, /has_unsupported_app_page_route_segments/);
  assert.match(sourceDiscovery, /unsupported_app_page_route_segment/);
  assert.doesNotMatch(sourceDiscovery, /entry\.file_name\(\)\s*==\s*"page\.tsx"/);

  assert.match(sourceGraph, /APP_ROUTE_ROOTS/);
  assert.match(sourceGraph, /"src\/app"/);
  assert.match(sourceGraph, /app_route_relative_dir/);
  assert.doesNotMatch(sourceGraph, /strip_prefix\(project_root\.join\("app"\)\)/);

  assert.match(nextMigrationPlan, /NEXT_APP_ROOTS: &\[&str\] = &\["app", "src\/app"\]/);
  assert.match(nextMigrationPlan, /NEXT_PAGE_FILE_NAMES/);
  assert.match(nextMigrationPlan, /"page\.jsx"/);
  assert.match(nextMigrationPlan, /"page\.ts"/);
  assert.match(nextMigrationPlan, /"page\.js"/);
  assert.match(nextMigrationPlan, /NEXT_ROUTE_HANDLER_FILE_NAMES/);
  assert.match(nextMigrationPlan, /"route\.tsx"/);
  assert.match(nextMigrationPlan, /"route\.jsx"/);
  assert.match(nextMigrationPlan, /scan_matching_files\(project_dir, NEXT_APP_ROOTS, NEXT_PAGE_FILE_NAMES\)/);
  assert.match(nextMigrationPlan, /scan_matching_files\(project_dir, NEXT_APP_ROOTS, NEXT_ROUTE_HANDLER_FILE_NAMES\)/);
  assert.match(nextMigrationPlan, /NEXT_SOURCE_ROOTS/);
  assert.match(nextMigrationPlan, /Next-familiar compatibility evidence/);
  assert.doesNotMatch(nextMigrationPlan, /claiming Next parity|parity evidence|full Next\.js parity/);
});
