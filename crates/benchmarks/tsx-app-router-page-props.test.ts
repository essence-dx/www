const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const executionPath = path.join(root, "dx-www", "src", "cli", "app_router_execution.rs");
const sourceRenderPath = path.join(root, "dx-www", "src", "cli", "app_router_execution", "source_render.rs");
const requestPropsPath = path.join(root, "dx-www", "src", "cli", "app_router_execution", "request_props.rs");
const compatibilityMapPath = path.join(root, "docs", "NEXTJS_COMPATIBILITY_MAP.md");

function read(filePath) {
  assert.ok(fs.existsSync(filePath), `missing ${path.relative(root, filePath)}`);
  return fs.readFileSync(filePath, "utf8");
}

test("generic TSX App Router renderer resolves safe params and searchParams reads", () => {
  const execution = read(executionPath);
  const sourceRender = read(sourceRenderPath);
  const requestProps = read(requestPropsPath);

  assert.match(execution, /build_tsx_source_render_surface\([\s\S]*input\.route_params,[\s\S]*input\.search_params,/);
  assert.match(execution, /data-dx-tsx-page-prop-bindings/);

  assert.match(sourceRender, /use std::collections::\{BTreeMap, BTreeSet, VecDeque\};/);
  assert.match(sourceRender, /route_params: &BTreeMap<String, String>/);
  assert.match(sourceRender, /search_params: &BTreeMap<String, String>/);
  assert.match(sourceRender, /request_prop_bindings/);
  assert.match(requestProps, /dx\.tsx\.requestPropBindings/);
  assert.match(requestProps, /route_param_bindings/);
  assert.match(requestProps, /search_param_bindings/);
  assert.match(requestProps, /request_prop_alias_bindings/);
  assert.match(requestProps, /collect_next_app_router_page_prop_aliases/);
  assert.match(requestProps, /resolve_next_app_router_page_prop_identifier/);
  assert.match(requestProps, /params\.slug/);
  assert.match(requestProps, /const \{ slug \} = params/);
  assert.match(requestProps, /const preview = searchParams\.query/);
  assert.match(requestProps, /searchParams\.query/);
  assert.match(requestProps, /next-app-router-page-prop-bindings/);
  assert.match(sourceRender, /static_document_snapshot\(document, state_graph, request_prop_bindings\)/);
  assert.match(sourceRender, /build_composed_static_dom_snapshot\([\s\S]*request_prop_bindings,/);
  assert.match(requestProps, /node_modules_required": false/);
  assert.match(requestProps, /next_runtime_required": false/);
  assert.match(requestProps, /react_rsc_required": false/);
  assert.match(requestProps, /arbitrary_request_code_execution": false/);
  assert.match(requestProps, /source_owned_request_props": true/);
  assert.match(requestProps, /external_runtime_required": false/);
  assert.match(requestProps, /external_runtime_executed": false/);
});

test("generic TSX App Router renderer records safe async params and searchParams aliases", () => {
  const sourceRender = read(sourceRenderPath);
  const requestProps = read(requestPropsPath);
  const docs = read(compatibilityMapPath);

  assert.match(requestProps, /const \{ slug \} = await params/);
  assert.match(requestProps, /const preview = \(await searchParams\)\.query/);
  assert.match(requestProps, /async_request_prop_member_access_name/);
  assert.match(requestProps, /async page params and searchParams aliases/);
  assert.match(requestProps, /request_prop_unresolved_alias_bindings/);
  assert.match(requestProps, /unresolved_alias_binding_count/);
  assert.match(sourceRender, /page_prop_unresolved_alias_bindings/);
  assert.match(requestProps, /missing-request-prop-value/);
  assert.match(requestProps, /adapter_boundary": "source-owned-next-app-router-request-props"/);
  assert.match(requestProps, /node_modules_required": false/);
  assert.match(requestProps, /next_runtime_required": false/);
  assert.match(requestProps, /react_rsc_required": false/);
  assert.match(requestProps, /arbitrary_request_code_execution": false/);
  assert.match(requestProps, /source_owned_request_props": true/);
  assert.match(requestProps, /external_runtime_required": false/);
  assert.match(requestProps, /external_runtime_executed": false/);

  assert.match(docs, /async page params and searchParams aliases/);
  assert.match(docs, /unresolved page-prop aliases/);
  assert.match(docs, /adapter_boundary=source-owned-next-app-router-request-props/);
  assert.match(docs, /next_runtime_required=false/);
  assert.match(docs, /react_rsc_required=false/);
  assert.match(docs, /arbitrary_request_code_execution=false/);
  assert.match(docs, /not full React Server Component execution/);
});

test("generic TSX App Router renderer resolves safe optional chaining request prop reads", () => {
  const requestProps = read(requestPropsPath);
  const docs = read(compatibilityMapPath);

  assert.match(requestProps, /params\?\.\["slug"\]/);
  assert.match(requestProps, /\(await searchParams\)\?\.query/);
  assert.match(requestProps, /optional_request_prop_member_access_name/);
  assert.match(requestProps, /optional_quoted_bracket_member_access_name/);
  assert.match(requestProps, /request_prop_identifier_resolves_optional_chaining_reads/);
  assert.match(requestProps, /safe optional chaining page params and searchParams reads/);
  assert.match(requestProps, /node_modules_required": false/);
  assert.match(requestProps, /next_runtime_required": false/);
  assert.match(requestProps, /react_rsc_required": false/);
  assert.match(requestProps, /arbitrary_request_code_execution": false/);
  assert.match(requestProps, /source_owned_request_props": true/);
  assert.match(requestProps, /external_runtime_required": false/);
  assert.match(requestProps, /external_runtime_executed": false/);

  assert.match(docs, /safe optional chaining reads/);
  assert.match(docs, /params\?\.slug/);
  assert.match(docs, /\(await searchParams\)\?\.preview/);
});

test("generic TSX App Router renderer resolves safe destructured request prop defaults", () => {
  const requestProps = read(requestPropsPath);

  assert.match(requestProps, /struct DestructuredRequestPropAlias/);
  assert.match(requestProps, /fallback_literal: Option<String>/);
  assert.match(requestProps, /quoted_request_prop_default_literal/);
  assert.match(requestProps, /const \{ slug = \\"latest\\" \} = params/);
  assert.match(requestProps, /const \{ preview: previewMode = \\"off\\" \} = searchParams/);
  assert.match(requestProps, /destructured_request_prop_alias_defaults_to_quoted_strings/);
  assert.match(requestProps, /quoted string destructuring defaults/);
  assert.match(requestProps, /node_modules_required": false/);
  assert.match(requestProps, /next_runtime_required": false/);
  assert.match(requestProps, /arbitrary_request_code_execution": false/);
  assert.match(requestProps, /source_owned_request_props": true/);
  assert.match(requestProps, /external_runtime_required": false/);
  assert.match(requestProps, /external_runtime_executed": false/);
});

test("generic TSX App Router keeps request prop receipts in a small source-owned module", () => {
  const execution = read(executionPath);
  const sourceRender = read(sourceRenderPath);
  const requestProps = read(requestPropsPath);

  assert.match(execution, /mod request_props;/);
  assert.match(sourceRender, /super::request_props::/);
  assert.doesNotMatch(sourceRender, /struct RequestPropAliasCollection/);
  assert.match(requestProps, /pub\(super\) struct RequestPropAliasCollection/);
  assert.match(requestProps, /pub\(super\) fn request_prop_bindings_manifest/);
  assert.match(requestProps, /pub\(super\) fn resolve_next_app_router_page_prop_identifier/);
  assert.match(requestProps, /adapter_boundary": "source-owned-next-app-router-request-props"/);
  assert.match(requestProps, /node_modules_required": false/);
  assert.match(requestProps, /next_runtime_required": false/);
  assert.match(requestProps, /react_rsc_required": false/);
  assert.match(requestProps, /arbitrary_request_code_execution": false/);
  assert.match(requestProps, /source_owned_request_props": true/);
  assert.match(requestProps, /external_runtime_required": false/);
  assert.match(requestProps, /external_runtime_executed": false/);
  assert.doesNotMatch(requestProps, /next\/server.*runtime/i);
});
