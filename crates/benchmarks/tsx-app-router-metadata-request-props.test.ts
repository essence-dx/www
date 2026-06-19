import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = process.cwd();

function read(relativePath: string): string {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("safe generateMetadata request-bound reads support optional chaining without runtime takeover", () => {
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");
  const compatibilityMap = read("docs/NEXTJS_COMPATIBILITY_MAP.md");

  assert.match(metadata, /params\?\.slug/);
  assert.match(metadata, /params\?\.\["slug"\]/);
  assert.match(metadata, /\(await searchParams\)\?\.preview/);
  assert.match(metadata, /parse_optional_request_prop_access/);
  assert.match(metadata, /parse_request_prop_access_rest/);
  assert.match(metadata, /metadata_request_props_resolve_safe_optional_chaining_reads/);
  assert.match(metadata, /node_modules_required": false/);
  assert.match(metadata, /source_owned_metadata": true/);
  assert.match(metadata, /external_runtime_required": false/);
  assert.match(metadata, /external_runtime_executed": false/);
  assert.match(metadata, /Does not execute dynamic metadata code/);

  assert.match(compatibilityMap, /generateMetadata\(\).*safe optional chaining reads/s);
  assert.match(compatibilityMap, /params\?\.slug/);
  assert.match(compatibilityMap, /\(await searchParams\)\?\.preview/);
  assert.match(compatibilityMap, /source-owned metadata extraction/i);
});

test("safe generateMetadata request-bound reads support destructured quoted defaults", () => {
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");
  const compatibilityMap = read("docs/NEXTJS_COMPATIBILITY_MAP.md");

  assert.match(metadata, /struct MetadataRequestPropAlias/);
  assert.match(metadata, /collect_generate_metadata_request_aliases/);
  assert.match(metadata, /metadata_request_prop_alias_defaults_resolve_quoted_strings/);
  assert.match(metadata, /const \{ slug = "latest" \} = params/);
  assert.match(metadata, /const \{ preview: previewMode = "off" \} = searchParams/);
  assert.match(metadata, /request_prop_alias_bindings/);
  assert.match(metadata, /arbitrary_request_code_execution": false/);
  assert.match(metadata, /source_owned_metadata": true/);

  assert.match(compatibilityMap, /const \{ slug = "latest" \} = params/);
  assert.match(compatibilityMap, /const \{ preview: previewMode = "off" \} = searchParams/);
  assert.match(compatibilityMap, /without executing destructuring defaults/);
});

test("safe generateMetadata request-bound reads support parameter aliases", () => {
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");
  const compatibilityMap = read("docs/NEXTJS_COMPATIBILITY_MAP.md");

  assert.match(metadata, /struct MetadataRequestRootAlias/);
  assert.match(metadata, /collect_generate_metadata_request_root_aliases/);
  assert.match(metadata, /metadata_request_parameter_aliases_resolve_known_values/);
  assert.match(metadata, /generateMetadata\(\{ params: routeParams, searchParams: queryParams \}\)/);
  assert.match(metadata, /routeParams\.slug/);
  assert.match(metadata, /queryParams\?\.preview/);
  assert.match(metadata, /request_prop_root_alias_bindings/);
  assert.match(metadata, /node_modules_required": false/);
  assert.match(metadata, /arbitrary_request_code_execution": false/);
  assert.match(metadata, /source_owned_metadata": true/);

  assert.match(compatibilityMap, /params: routeParams/);
  assert.match(compatibilityMap, /searchParams: queryParams/);
  assert.match(compatibilityMap, /routeParams\.slug/);
});

test("safe generateMetadata request-bound reads support const arrow exports", () => {
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");
  const compatibilityMap = read("docs/NEXTJS_COMPATIBILITY_MAP.md");

  assert.match(metadata, /generate_metadata_const_arrow_parts/);
  assert.match(metadata, /metadata_request_const_arrow_exports_resolve_known_values/);
  assert.match(metadata, /export const generateMetadata = async \(\{ params, searchParams \}\) =>/);
  assert.match(metadata, /=>/);
  assert.match(metadata, /node_modules_required": false/);
  assert.match(metadata, /arbitrary_request_code_execution": false/);
  assert.match(metadata, /external_runtime_required": false/);
  assert.match(metadata, /external_runtime_executed": false/);

  assert.match(compatibilityMap, /export const generateMetadata = async/);
  assert.match(compatibilityMap, /safe const-arrow generateMetadata/i);
  assert.match(compatibilityMap, /without importing Next runtime/i);
});

test("safe generateMetadata const-arrow exports tolerate TypeScript annotations", () => {
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");

  assert.match(metadata, /skip_const_arrow_return_type_annotation/);
  assert.match(metadata, /simple_parameter_binding_name/);
  assert.match(
    metadata,
    /metadata_request_const_arrow_typed_props_exports_resolve_known_values/,
  );
  assert.match(
    metadata,
    /export const generateMetadata = async \(props: MetadataProps\): Promise<Metadata> =>/,
  );
  assert.match(
    metadata,
    /export const generateMetadata = \(\{ params, searchParams \}: MetadataProps\): Metadata => \(\{/,
  );
  assert.match(metadata, /node_modules_required": false/);
  assert.match(metadata, /arbitrary_request_code_execution": false/);
  assert.match(metadata, /external_runtime_required": false/);
  assert.match(metadata, /external_runtime_executed": false/);
});

test("safe generateMetadata const-arrow exports tolerate bare props parameter", () => {
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");

  assert.match(metadata, /const_arrow_bare_parameter_parts/);
  assert.match(
    metadata,
    /metadata_request_const_arrow_bare_props_exports_resolve_known_values/,
  );
  assert.match(metadata, /export const generateMetadata = async props =>/);
  assert.match(metadata, /const \{ params: routeParams, searchParams: queryParams \} = props/);
  assert.match(metadata, /node_modules_required": false/);
  assert.match(metadata, /arbitrary_request_code_execution": false/);
  assert.match(metadata, /external_runtime_required": false/);
  assert.match(metadata, /external_runtime_executed": false/);
});

test("safe generateMetadata const-arrow exports tolerate generic type parameters", () => {
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");

  assert.match(metadata, /skip_const_arrow_type_parameters/);
  assert.match(metadata, /find_type_parameter_list_end/);
  assert.match(
    metadata,
    /metadata_request_const_arrow_generic_props_exports_resolve_known_values/,
  );
  assert.match(
    metadata,
    /export const generateMetadata = async <T extends MetadataProps>\(props: T\): Promise<Metadata> =>/,
  );
  assert.match(metadata, /node_modules_required": false/);
  assert.match(metadata, /arbitrary_request_code_execution": false/);
  assert.match(metadata, /external_runtime_required": false/);
  assert.match(metadata, /external_runtime_executed": false/);
});

test("safe generateMetadata request-bound reads support props object destructuring", () => {
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");
  const compatibilityMap = read("docs/NEXTJS_COMPATIBILITY_MAP.md");

  assert.match(metadata, /collect_generate_metadata_request_object_aliases/);
  assert.match(metadata, /metadata_request_object_parameter_aliases_resolve_known_values/);
  assert.match(metadata, /export const generateMetadata = async \(props\) =>/);
  assert.match(metadata, /const \{ params: routeParams, searchParams: queryParams \} = props/);
  assert.match(metadata, /routeParams\.slug/);
  assert.match(metadata, /queryParams\?\.preview/);
  assert.match(metadata, /node_modules_required": false/);
  assert.match(metadata, /arbitrary_request_code_execution": false/);
  assert.match(metadata, /external_runtime_required": false/);
  assert.match(metadata, /external_runtime_executed": false/);

  assert.match(compatibilityMap, /generateMetadata\(props\)/);
  assert.match(compatibilityMap, /props object destructuring/i);
  assert.match(compatibilityMap, /without importing Next runtime/i);
});
