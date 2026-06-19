import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");

function read(relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function expectAll(source: string, markers: string[]): void {
  for (const marker of markers) {
    assert.match(source, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
}

test("release-readiness primitive proof matrix is honest and source-owned", () => {
  const readiness = read("dx-www/src/cli/readiness.rs");
  const sourceRender = read("dx-www/src/cli/app_router_execution/source_render.rs");
  const staticMarkup = read(
    "dx-www/src/cli/app_router_execution/source_render_parts/static_markup.rs",
  );
  const staticExpression = read(
    "dx-www/src/cli/app_router_execution/source_render_parts/static_expression.rs",
  );
  const fontTransforms = read(
    "dx-www/src/cli/app_router_execution/next_custom_transforms/font_loaders.rs",
  );
  const clientIsland = read("core/src/delivery/client_island.rs");
  const deploy = read("dx-www/src/cli/deploy_adapter_contract.rs");

  expectAll(readiness, [
    "dx.www.readiness.primitive_proof",
    "dx.www.readiness.primitive_proof_receipt_contract",
    "source-owned-primitive-foundation-current",
    "primitive-proof",
    "source-owned-primitive-foundation",
    "static-safe next/image lowers to <img>",
    "module-scope font loader detection",
    "static-safe next/script lowers to <script>",
    ".wasm and .wasm.gz are immutable runtime assets",
    "not_yet_claimed",
    "Primitive claims must stay tied to source-owned receipts",
  ]);

  expectAll(sourceRender, [
    "next_image_component_names",
    "next_script_component_names",
    "next-image-static-img",
    "next-script-static-script",
  ]);

  expectAll(staticMarkup, [
    "data-dx-framework-component",
  ]);

  expectAll(staticExpression, [
    "data-dx-next-script-strategy",
  ]);

  expectAll(fontTransforms, [
    "next/font/google",
    "next/font/local",
    "css_variable_receipt",
    "generated_css_import",
    "font-loader-call-outside-module-scope",
  ]);

  expectAll(clientIsland, [
    "clientLoad",
    "clientVisible",
    "clientIdle",
    "clientOnly",
    "unsupported_client_island_directive_syntax",
    "full_react_hydration: false",
    "no_js_fallback_required: true",
    "readiness_release_ready: false",
    "foundation-not-release-proof",
  ]);

  expectAll(deploy, [
    "wasm",
    "content_type",
    "content_encoding",
    "encoded_from",
    "Content-Encoding",
    "Content-Type",
    "application/wasm",
  ]);
});
