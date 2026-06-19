import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("lane 9 resolver keeps package imports source owned and node_modules free", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const linkerPaths = read("dx-www/src/build/source_engine/module_linker_paths.rs");
  const graph = read("dx-www/src/build/source_engine/ecosystem_graph.rs");
  const receipt = read("dx-www/src/build/source_engine/receipt.rs");

  assert.match(resolver, /package_imports:/);
  assert.match(resolver, /fn package_import_aliases/);
  assert.match(resolver, /fn package_self_reference_aliases/);
  assert.match(resolver, /fn source_owned_package_import_targets/);
  assert.match(resolver, /fn source_owned_package_import_target_or_boundary/);
  assert.match(resolver, /fn clean_package_import_target_path/);
  assert.match(resolver, /resolver_source:/);
  assert.match(resolver, /pub\(super\) const RESOLVER_SOURCE_PACKAGE_IMPORT/);
  assert.match(resolver, /pub\(super\) const RESOLVER_SOURCE_PACKAGE_SELF_REFERENCE/);
  assert.match(resolver, /pub\(super\) const RESOLVER_SOURCE_ADAPTER_BOUNDARY/);
  assert.match(resolver, /RESOLVER_DETAIL_PACKAGE_IMPORT_TARGET_NODE_MODULES_BOUNDARY/);
  assert.match(resolver, /RESOLVER_DETAIL_PACKAGE_IMPORT_TARGET_OUTSIDE_PACKAGE_BOUNDARY/);
  assert.match(resolver, /pub\(super\) const RESOLVER_SOURCE_JS_CONFIG_PATH/);
  assert.match(resolver, /pub\(super\) const RESOLVER_SOURCE_TS_CONFIG_PATH/);
  assert.match(resolver, /package-import/);
  assert.match(resolver, /package-self-reference/);
  assert.match(resolver, /package_self_references:/);
  assert.match(resolver, /adapter_boundary_detail: targets\.adapter_boundary_detail/);
  assert.match(resolver, /clean_package_import_target_path\(target\)/);
  assert.ok(resolver.includes('starts_with("./")'));
  assert.ok(resolver.includes('segment == "node_modules"'));
  assert.match(linkerPaths, /!contains_node_modules\(&candidate\.path\)/);
  assert.match(linkerPaths, /RESOLVER_SOURCE_PROJECT_ROOT_ALIAS/);
  assert.match(linkerPaths, /RESOLVER_SOURCE_RELATIVE/);
  assert.match(linkerPaths, /ResolvedSourceImport/);
  assert.match(graph, /RESOLVER_SOURCE_ADAPTER_BOUNDARY/);
  assert.match(graph, /adapter-boundary-import/);
  assert.match(graph, /fn is_adapter_boundary_dependency/);
  assert.match(graph, /"public_architecture": false/);
  assert.match(receipt, /package\.json imports/);
  assert.match(receipt, /adapter-boundary import graph nodes/);
  assert.match(receipt, /node_modules package resolution/);
});

test("lane 9 resolver normalizes safe package import parent segments while preserving unsafe boundaries", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(resolver, /source_owned_package_import_target_or_boundary/);
  assert.match(resolver, /clean_package_import_target_path/);
  assert.match(
    resolver,
    /RESOLVER_DETAIL_PACKAGE_IMPORT_TARGET_NODE_MODULES_BOUNDARY/,
  );
  assert.match(
    resolver,
    /RESOLVER_DETAIL_PACKAGE_IMPORT_TARGET_OUTSIDE_PACKAGE_BOUNDARY/,
  );
  assert.match(
    compatTest,
    /source_build_engine_resolves_package_json_imports_parent_segments_inside_project/,
  );
  assert.match(compatTest, /"#shared\/\*": "\.\/src\/\.\.\/components\/\*"/);
  assert.match(compatTest, /"#blocked\/\*": "\.\/src\/\.\.\/node_modules\/trap\/\*"/);
  assert.match(compatTest, /dependency\["resolved_path"\] == "components\/Hero\.tsx"/);
  assert.match(
    compatTest,
    /dependency\["resolver_detail"\] == "package-import-target-node-modules-boundary"/,
  );
  assert.match(compatTest, /chunk\["source_path"\] != "node_modules\/trap\/Widget\.ts"/);
});

test("lane 9 resolver falls back to src source modules for src/app project-root aliases", () => {
  const linkerPaths = read("dx-www/src/build/source_engine/module_linker_paths.rs");
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(resolver, /RESOLVER_SOURCE_SRC_PROJECT_ROOT_ALIAS/);
  assert.match(linkerPaths, /fn src_project_root_alias_base/);
  assert.match(linkerPaths, /project_root\.join\("src"\)\.join\(project_path\)/);
  assert.match(
    compatTest,
    /source_build_engine_resolves_project_root_alias_from_src_app_to_src_modules_without_config/,
  );
  assert.match(compatTest, /src\/app\/page\.tsx/);
  assert.match(compatTest, /"@\/components\/Hero"/);
  assert.match(compatTest, /dependency\["resolved_path"\] == "src\/components\/Hero\.tsx"/);
  assert.match(compatTest, /dependency\["resolver_source"\] == "src-project-root-alias"/);
  assert.match(compatTest, /chunk\["source_path"\] == "src\/app\/page\.tsx"/);
  assert.match(compatTest, /dependency\["node_modules_required"\] == false/);
});

test("lane 9 resolver falls through missing safe config aliases before src/app fallback", () => {
  const linkerPaths = read("dx-www/src/build/source_engine/module_linker_paths.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(linkerPaths, /fn append_project_root_alias_fallback_bases/);
  assert.doesNotMatch(
    linkerPaths,
    /&& !resolver_config\.matches_source_alias\(specifier\)/,
  );
  assert.match(
    compatTest,
    /source_build_engine_falls_back_to_src_project_root_alias_after_missing_safe_config_aliases/,
  );
  assert.match(compatTest, /"@\/\*": \["generated\/\*", "missing\/\*"\]/);
  assert.match(compatTest, /dependency\["resolved_path"\] == "src\/components\/Hero\.tsx"/);
  assert.match(compatTest, /dependency\["resolver_source"\] == "src-project-root-alias"/);
  assert.match(
    compatTest,
    /dependency\["resolver_source"\] != "source-alias-unresolved"/,
  );
});

test("lane 6 resolver keeps package self-reference roots source owned", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(resolver, /fn package_self_reference_aliases/);
  assert.match(resolver, /fn source_owned_package_name_segment/);
  assert.match(
    resolver,
    /push_package_self_reference_alias\(&mut aliases,\s*name\.to_string\(\),\s*vec!\["\."\.to_string\(\)\]\)/,
  );
  assert.match(
    resolver,
    /push_package_self_reference_alias\(&mut aliases,\s*format!\("\{name\}\/\*"\),\s*vec!\["\*"\.to_string\(\)\]\)/,
  );
  assert.match(resolver, /resolver_source: RESOLVER_SOURCE_PACKAGE_SELF_REFERENCE/);
  assert.match(resolver, /fn compiler_reserved_package_name/);
  assert.match(
    resolver,
    /package_self_reference_aliases_skip_compiler_reserved_names/,
  );
  assert.match(
    resolver,
    /package_self_reference_aliases_skip_invalid_package_names/,
  );
  assert.match(resolver, /!segment\.contains\('@'\)/);
  assert.match(resolver, /"@@scope\/pkg"/);
  assert.match(resolver, /"@scope\/@pkg"/);

  assert.match(
    compatTest,
    /source_build_engine_resolves_package_self_reference_root_without_node_modules/,
  );
  assert.match(compatTest, /import \{ packageLabel \} from "dx-source-resolver-fixture"/);
  assert.match(compatTest, /dependency\["resolved_path"\] == "index\.ts"/);
  assert.match(compatTest, /dependency\["resolver_source"\] == "package-self-reference"/);
  assert.match(compatTest, /edge\["specifier"\] == "dx-source-resolver-fixture"/);
  assert.match(
    compatTest,
    /source_build_engine_keeps_invalid_package_names_out_of_self_reference_namespace/,
  );
  assert.match(compatTest, /"name": "plain\/pkg"/);
  assert.match(compatTest, /dependency\["specifier"\] == specifier/);
  assert.match(compatTest, /dependency\["resolver_source"\] == "external-package-boundary"/);
  assert.match(
    compatTest,
    /dependency\["resolver_source"\] != "package-self-reference"/,
  );
});

test("lane 6 resolver honors safe package self-reference exports without node_modules", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(resolver, /fn package_self_reference_export_aliases/);
  assert.match(resolver, /fn push_package_self_reference_alias/);
  assert.match(resolver, /fn package_export_key_to_self_reference_pattern/);
  assert.match(
    compatTest,
    /source_build_engine_resolves_package_self_reference_exports_without_node_modules/,
  );
  assert.match(compatTest, /"exports": \{/);
  assert.match(compatTest, /"\.": "\.\/src\/public\.ts"/);
  assert.match(compatTest, /"\.\/feature\/\*"/);
  assert.match(compatTest, /"\.\/blocked\/\*": "\.\/node_modules\/not-source\/\*"/);
  assert.match(compatTest, /dependency\["resolved_path"\] == "src\/public\.ts"/);
  assert.match(compatTest, /dependency\["resolved_path"\] == "src\/features\/Hero\.tsx"/);
  assert.match(compatTest, /dependency\["kind"\] == "package-export-adapter-boundary"/);
});

test("lane 6 resolver keeps unsafe package imports at an explicit boundary", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const linker = read("dx-www/src/build/source_engine/module_linker.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(resolver, /RESOLVER_SOURCE_PACKAGE_IMPORT_BOUNDARY/);
  assert.match(resolver, /fn matches_package_import_boundary/);
  assert.match(linker, /package-import-adapter-boundary/);
  assert.match(linker, /RESOLVER_SOURCE_PACKAGE_IMPORT_BOUNDARY/);
  assert.match(compatTest, /"#blocked\/\*": "left-pad\/\*"/);
  assert.match(compatTest, /dependency\["specifier"\] == "#blocked\/Widget"/);
  assert.match(compatTest, /dependency\["kind"\] == "package-import-adapter-boundary"/);
  assert.match(compatTest, /dependency\["resolver_source"\] == "package-import-boundary"/);
  assert.match(compatTest, /dependency\["node_modules_required"\] == false/);
});

test("lane 6 linker gives unknown external packages a precise no-node_modules boundary", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const linker = read("dx-www/src/build/source_engine/module_linker.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(resolver, /RESOLVER_SOURCE_EXTERNAL_PACKAGE_BOUNDARY/);
  assert.match(linker, /fn external_package_import/);
  assert.match(linker, /external-package-adapter-boundary/);
  assert.match(linker, /RESOLVER_SOURCE_EXTERNAL_PACKAGE_BOUNDARY/);
  assert.match(compatTest, /import scopedWidget from "@scope\/widget"/);
  assert.match(compatTest, /dependency\["specifier"\] == "left-pad"/);
  assert.match(compatTest, /dependency\["kind"\] == "external-package-adapter-boundary"/);
  assert.match(compatTest, /dependency\["resolver_source"\] == "external-package-boundary"/);
  assert.match(compatTest, /dependency\["specifier"\] == "@scope\/widget"/);
  assert.match(compatTest, /dependency\["node_modules_required"\] == false/);
});

test("lane 6 resolver keeps mixed missing-source and external package imports at a package boundary", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(resolver, /adapter_boundary:/);
  assert.match(resolver, /fn source_owned_package_import_targets_with_boundary/);
  assert.match(resolver, /alias\.adapter_boundary/);
  assert.match(
    compatTest,
    /source_build_engine_keeps_mixed_missing_and_external_package_imports_at_adapter_boundary_without_node_modules/,
  );
  assert.match(compatTest, /"#mixed\/\*": \["\.\/missing\/\*", "left-pad\/\*"\]/);
  assert.match(compatTest, /dependency\["specifier"\] == "#mixed\/Widget"/);
  assert.match(compatTest, /dependency\["kind"\] == "package-import-adapter-boundary"/);
  assert.match(compatTest, /dependency\["resolver_source"\] == "package-import-boundary"/);
  assert.match(compatTest, /dependency\["node_modules_required"\] == false/);
});

test("lane 6 resolver treats node_modules tsconfig paths as explicit adapter boundaries", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const linker = read("dx-www/src/build/source_engine/module_linker.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(resolver, /RESOLVER_SOURCE_SOURCE_ALIAS_BOUNDARY/);
  assert.match(resolver, /fn matches_source_alias_boundary/);
  assert.match(resolver, /fn path_alias_target_requires_adapter_boundary/);
  assert.match(
    resolver,
    /path_alias_target_has_node_modules_boundary\(base_url, target\)/,
  );
  assert.match(
    resolver,
    /path_alias_target_has_outside_boundary\(project_root, base_url, target\)/,
  );
  assert.match(resolver, /segment == "node_modules"/);
  assert.match(linker, /source-alias-adapter-boundary/);
  assert.match(linker, /RESOLVER_SOURCE_SOURCE_ALIAS_BOUNDARY/);
  assert.match(
    compatTest,
    /source_build_engine_resolves_safe_parent_segments_in_tsconfig_path_aliases_without_boundary/,
  );
  assert.match(compatTest, /"@shared\/\*": \["src\/\.\.\/components\/\*"\]/);
  assert.match(compatTest, /"@blocked\/\*": \["src\/\.\.\/node_modules\/trap\/\*"\]/);
  assert.match(compatTest, /dependency\["resolved_path"\] == "components\/Hero\.tsx"/);
  assert.match(
    compatTest,
    /dependency\["resolver_detail"\] == "source-alias-target-node-modules-boundary"/,
  );
  assert.match(compatTest, /chunk\["source_path"\] != "node_modules\/trap\/Widget\.ts"/);
  assert.match(
    compatTest,
    /source_build_engine_keeps_tsconfig_node_modules_path_aliases_at_adapter_boundary_without_node_modules/,
  );
  assert.match(compatTest, /"@vendor\/\*": \["node_modules\/vendor\/\*"\]/);
  assert.match(compatTest, /dependency\["specifier"\] == "@vendor\/Button"/);
  assert.match(compatTest, /dependency\["kind"\] == "source-alias-adapter-boundary"/);
  assert.match(compatTest, /dependency\["resolver_source"\] == "source-alias-boundary"/);
  assert.match(compatTest, /dependency\["node_modules_required"\] == false/);
});

test("lane 6 resolver treats mixed missing-source and node_modules path aliases as adapter boundaries", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(resolver, /fn matches_source_alias_boundary/);
  assert.match(
    resolver,
    /path_alias_target_requires_adapter_boundary\(\s*project_root,\s*&path_alias_base_url,\s*target,\s*\)/,
  );
  assert.match(
    compatTest,
    /source_build_engine_keeps_mixed_missing_and_node_modules_path_aliases_at_adapter_boundary_without_node_modules/,
  );
  assert.match(compatTest, /"@mixed\/\*": \["missing\/\*", "node_modules\/vendor\/\*"\]/);
  assert.match(compatTest, /dependency\["specifier"\] == "@mixed\/Button"/);
  assert.match(compatTest, /dependency\["kind"\] == "source-alias-adapter-boundary"/);
  assert.match(compatTest, /dependency\["resolver_source"\] == "source-alias-boundary"/);
  assert.match(compatTest, /dependency\["node_modules_required"\] == false/);
});

test("lane 6 resolver treats outside-project path aliases as explicit adapter boundaries", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(
    resolver,
    /fn path_alias_target_requires_adapter_boundary\([\s\S]*path_alias_target_has_outside_boundary/,
  );
  assert.match(
    resolver,
    /fn path_alias_target_has_outside_boundary/,
  );
  assert.match(
    compatTest,
    /source_build_engine_keeps_outside_project_path_aliases_at_adapter_boundary_without_node_modules/,
  );
  assert.match(compatTest, /"@outside\/\*": \["\.\.\/outside\/\*"\]/);
  assert.match(compatTest, /dependency\["specifier"\] == "@outside\/Secret"/);
  assert.match(compatTest, /dependency\["kind"\] == "source-alias-adapter-boundary"/);
  assert.match(compatTest, /dependency\["resolver_source"\] == "source-alias-boundary"/);
  assert.match(compatTest, /dependency\["node_modules_required"\] == false/);
});

test("lane 6 resolver treats project-root node_modules aliases as explicit adapter boundaries", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const linker = read("dx-www/src/build/source_engine/module_linker.rs");
  const linkerPaths = read("dx-www/src/build/source_engine/module_linker_paths.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(resolver, /RESOLVER_SOURCE_PROJECT_ROOT_ALIAS_BOUNDARY/);
  assert.match(linkerPaths, /pub\(super\) fn project_root_alias_adapter_boundary_detail/);
  assert.doesNotMatch(linker, /fn project_root_alias_adapter_boundary_detail/);
  assert.match(linker, /project-root-alias-adapter-boundary/);
  assert.match(linker, /RESOLVER_SOURCE_PROJECT_ROOT_ALIAS_BOUNDARY/);
  assert.match(
    compatTest,
    /source_build_engine_keeps_project_root_node_modules_aliases_at_adapter_boundary_without_node_modules/,
  );
  assert.match(compatTest, /import trap from "@\/node_modules\/trap"/);
  assert.match(compatTest, /dependency\["specifier"\] == "@\/node_modules\/trap"/);
  assert.match(compatTest, /dependency\["kind"\] == "project-root-alias-adapter-boundary"/);
  assert.match(compatTest, /dependency\["resolver_source"\] == "project-root-alias-boundary"/);
  assert.match(compatTest, /dependency\["node_modules_required"\] == false/);
});

test("lane 9 resolver treats project-root alias escapes as explicit adapter boundaries", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const linker = read("dx-www/src/build/source_engine/module_linker.rs");
  const linkerPaths = read("dx-www/src/build/source_engine/module_linker_paths.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(
    resolver,
    /RESOLVER_DETAIL_PROJECT_ROOT_ALIAS_OUTSIDE_PROJECT_BOUNDARY/,
  );
  assert.match(linkerPaths, /fn project_root_alias_path_boundary_detail/);
  assert.match(linkerPaths, /RESOLVER_DETAIL_PROJECT_ROOT_ALIAS_OUTSIDE_PROJECT_BOUNDARY/);
  assert.doesNotMatch(linker, /fn project_root_alias_adapter_boundary_detail/);
  assert.match(
    compatTest,
    /source_build_engine_keeps_project_root_alias_parent_segments_at_adapter_boundary_without_node_modules/,
  );
  assert.match(compatTest, /import secret from "@\/\.\.\/outside\/Secret"/);
  assert.match(compatTest, /dependency\["specifier"\] == "@\/\.\.\/outside\/Secret"/);
  assert.match(
    compatTest,
    /dependency\["resolver_detail"\] == "project-root-alias-outside-project-boundary"/,
  );
  assert.match(compatTest, /dependency\["node_modules_required"\] == false/);
});

test("lane 6 resolver treats relative node_modules imports as explicit adapter boundaries", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const linker = read("dx-www/src/build/source_engine/module_linker.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(resolver, /RESOLVER_SOURCE_LOCAL_NODE_MODULES_BOUNDARY/);
  assert.match(linker, /fn local_import_requires_adapter_boundary/);
  assert.match(linker, /local-node-modules-adapter-boundary/);
  assert.match(linker, /RESOLVER_SOURCE_LOCAL_NODE_MODULES_BOUNDARY/);
  assert.match(
    compatTest,
    /source_build_engine_keeps_relative_node_modules_imports_at_adapter_boundary_without_node_modules/,
  );
  assert.match(compatTest, /import trap from "\.\.\/node_modules\/trap"/);
  assert.match(compatTest, /dependency\["specifier"\] == "\.\.\/node_modules\/trap"/);
  assert.match(compatTest, /dependency\["kind"\] == "local-node-modules-adapter-boundary"/);
  assert.match(compatTest, /dependency\["resolver_source"\] == "local-node-modules-boundary"/);
  assert.match(compatTest, /dependency\["node_modules_required"\] == false/);
});

test("lane 6 resolver treats baseUrl node_modules imports as explicit adapter boundaries", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const linker = read("dx-www/src/build/source_engine/module_linker.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(resolver, /RESOLVER_SOURCE_BASE_URL_NODE_MODULES_BOUNDARY/);
  assert.match(resolver, /fn matches_base_url_node_modules_boundary/);
  assert.match(linker, /base-url-node-modules-adapter-boundary/);
  assert.match(linker, /RESOLVER_SOURCE_BASE_URL_NODE_MODULES_BOUNDARY/);
  assert.match(
    compatTest,
    /source_build_engine_keeps_base_url_node_modules_imports_at_adapter_boundary_without_node_modules/,
  );
  assert.match(compatTest, /"baseUrl": "\."/);
  assert.match(compatTest, /import trap from "node_modules\/trap"/);
  assert.match(compatTest, /dependency\["specifier"\] == "node_modules\/trap"/);
  assert.match(compatTest, /dependency\["kind"\] == "base-url-node-modules-adapter-boundary"/);
  assert.match(compatTest, /dependency\["resolver_source"\] == "base-url-node-modules-boundary"/);
  assert.match(compatTest, /dependency\["node_modules_required"\] == false/);
});

test("lane 6 resolver treats outside-project baseUrl as an explicit adapter boundary", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const linker = read("dx-www/src/build/source_engine/module_linker.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(resolver, /RESOLVER_SOURCE_BASE_URL_BOUNDARY/);
  assert.match(resolver, /adapter_boundary:/);
  assert.match(resolver, /fn source_path_requires_adapter_boundary/);
  assert.match(resolver, /fn matches_base_url_boundary/);
  assert.match(linker, /base-url-adapter-boundary/);
  assert.match(linker, /RESOLVER_SOURCE_BASE_URL_BOUNDARY/);
  assert.match(
    compatTest,
    /source_build_engine_keeps_outside_project_base_url_at_adapter_boundary_without_node_modules/,
  );
  assert.match(compatTest, /"baseUrl": "\.\."/);
  assert.match(compatTest, /import \{ secretLabel \} from "outside\/Secret"/);
  assert.match(compatTest, /dependency\["specifier"\] == "outside\/Secret"/);
  assert.match(compatTest, /dependency\["kind"\] == "base-url-adapter-boundary"/);
  assert.match(compatTest, /dependency\["resolver_source"\] == "base-url-boundary"/);
  assert.match(compatTest, /dependency\["node_modules_required"\] == false/);
});

test("lane 6 resolver rejects project-prefix sibling baseUrl escapes", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const compatTest = read("dx-www/tests/source_resolver_compat.rs");

  assert.match(resolver, /fn normalize_source_path_for_boundary/);
  assert.match(resolver, /fn source_path_is_inside_project/);
  assert.match(resolver, /source_path_boundary_rejects_project_prefix_siblings/);
  assert.match(resolver, /G:\/Dx\/www-other\/src/);
  assert.match(
    compatTest,
    /source_build_engine_keeps_sibling_prefix_base_url_at_adapter_boundary_without_node_modules/,
  );
  assert.match(compatTest, /www-sibling/);
  assert.match(compatTest, /dependency\["kind"\] == "base-url-adapter-boundary"/);
  assert.match(compatTest, /dependency\["resolver_source"\] == "base-url-boundary"/);
  assert.match(compatTest, /dependency\["node_modules_required"\] == false/);
});

test("lane 6 linker skips type-only imports while preserving value re-export edges", () => {
  const linkerPaths = read("dx-www/src/build/source_engine/module_linker_paths.rs");

  assert.match(linkerPaths, /fn fallback_import_statement_has_value/);
  assert.match(linkerPaths, /fn named_specifier_list_has_value/);
  assert.match(
    linkerPaths,
    /import_specifiers_keep_value_edges_and_skip_type_only_edges/,
  );
  assert.match(linkerPaths, /import type \{ Config \} from "\.\/types"/);
  assert.match(linkerPaths, /import \{ type Shape \} from "\.\/shape"/);
  assert.match(linkerPaths, /export \{ createStore \} from "\.\/store"/);
  assert.match(
    linkerPaths,
    /vec!\["\.\/all", "\.\/side-effect", "\.\/store", "\.\/value"\]/,
  );
});

test("lane 6 compiler intrinsics cannot be shadowed by source aliases", () => {
  const resolver = read("dx-www/src/build/source_engine/module_resolver_config.rs");
  const linkerPaths = read("dx-www/src/build/source_engine/module_linker_paths.rs");

  assert.match(resolver, /matches!\(name, "dx-www" \| "next" \| "react"\)/);
  assert.match(resolver, /name\.starts_with\("node:"\)/);
  assert.match(linkerPaths, /else if compiler_intrinsic\(specifier\) \{\s+Vec::new\(\)/);
  assert.match(linkerPaths, /"next"\s+\| "react"/);
  assert.match(linkerPaths, /specifier\.starts_with\("node:"\)/);
});
