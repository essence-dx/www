import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = process.cwd();

function read(relativePath: string): string {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("App Router static metadata and viewport extraction require exported surfaces", () => {
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");

  assert.match(metadata, /fn static_metadata_requires_export_const_surface/);
  assert.match(metadata, /fn has_export_const_declaration_prefix/);
  assert.match(metadata, /declaration_tokens\.ends_with\(&\["export", "const"\]\)/);
  assert.match(metadata, /static_object_literal\(source, "metadata"\)/);
  assert.match(metadata, /static_object_literal\(source, "viewport"\)/);
  assert.match(metadata, /Local implementation detail/);
  assert.match(metadata, /safe-static-metadata-literal-object/);
});

test("App Router generate metadata and viewport extraction require exported surfaces", () => {
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");

  assert.match(metadata, /fn generate_metadata_requires_exported_surface/);
  assert.match(metadata, /fn has_export_function_declaration_prefix/);
  assert.match(metadata, /has_export_const_declaration_prefix\(declaration_prefix\)/);
  assert.match(metadata, /Local generateMetadata helper/);
  assert.match(metadata, /Local generateViewport helper/);
  assert.match(metadata, /Exported generateMetadata/);
  assert.match(metadata, /Exported generateViewport/);
});

test("App Router client metadata exports are diagnostic-only", () => {
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");
  const directives = read("dx-www/src/cli/app_router_execution/directives.rs");
  const conflicts = read(
    "dx-www/src/cli/app_router_execution/next_custom_transforms/conflicts.rs",
  );
  const rscBoundaries = read(
    "dx-www/src/cli/app_router_execution/next_custom_transforms/rsc_boundaries.rs",
  );

  assert.match(metadata, /fn client_metadata_exports_are_diagnostic_only/);
  assert.match(directives, /fn has_use_client_directive/);
  assert.match(
    metadata,
    /if has_use_client_directive\(source\) \{\s*return None;\s*\}/s,
  );
  assert.match(
    rscBoundaries,
    /metadata_export_in_client: use_client && metadata_export/,
  );
  assert.match(conflicts, /"kind": "client-metadata-export"/);
  assert.match(conflicts, /"diagnostic_status": "dx-check-receipt-only"/);
});

test("App Router metadata and RSC receipts share directive parsing", () => {
  const appRouterExecution = read("dx-www/src/cli/app_router_execution.rs");
  const directives = read("dx-www/src/cli/app_router_execution/directives.rs");
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");
  const rscBoundaries = read(
    "dx-www/src/cli/app_router_execution/next_custom_transforms/rsc_boundaries.rs",
  );

  assert.match(appRouterExecution, /mod directives;/);
  assert.match(directives, /pub\(super\) fn collect_top_level_directives/);
  assert.match(directives, /pub\(super\) fn has_use_client_directive/);
  assert.match(metadata, /use super::directives::has_use_client_directive;/);
  assert.match(
    rscBoundaries,
    /use super::super::directives::collect_top_level_directives;/,
  );
  assert.doesNotMatch(metadata, /fn skip_directive_whitespace_and_comments/);
  assert.doesNotMatch(rscBoundaries, /fn quoted_statement_value/);
});
