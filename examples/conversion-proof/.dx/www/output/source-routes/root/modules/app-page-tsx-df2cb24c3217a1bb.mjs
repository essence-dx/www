import { dxSourceModule as dep0, dxRuntimeExports as dep0Runtime } from "./lib-forge-www-landing-page-tsx-85cb17e3b5359ab6.mjs";
export const dxSourceText = "const dxCreateElement = (tag, props = {}, ...children) => Object.freeze({ kind: \"dx.element\", tag, props: Object.freeze(props), children: Object.freeze(children) });\nexport function LandingPage() {\n  return LandingPageContent({});\n}\nexport default LandingPage;\n";
export const dxSourceModule = Object.freeze({
  "source_path": "app/page.tsx",
  "chunk_output": ".dx/www/output/source-routes/root/modules/app-page-tsx-df2cb24c3217a1bb.mjs",
  "kind": "tsx",
  "hash": "df2cb24c3217a1bb",
  "dependencies": [
    {
      "specifier": "@/lib/forge/www/landing-page",
      "resolved_path": "lib/forge/www/landing-page.tsx",
      "chunk_output": ".dx/www/output/source-routes/root/modules/lib-forge-www-landing-page-tsx-85cb17e3b5359ab6.mjs",
      "kind": "tsx",
      "resolver_source": "project-root-alias",
      "node_modules_required": false
    }
  ],
  "browser_executable": true,
  "source_transformed": true,
  "transform_kind": "tsx-component-runtime",
  "runtime_exports": [
    "LandingPage"
  ],
  "ecmascript_analysis": {
    "schema": "dx.ecmascript.analysis",
    "schema_revision": 1,
    "source_path": "app/page.tsx",
    "source_kind": "tsx",
    "parser_backend": "oxc-parser",
    "diagnostics": 0,
    "compatibility_reference": {
      "upstream_crates": [
        "turbopack-ecmascript"
      ],
      "reference_only": true,
      "runtime_build_adoption": false,
      "public_runtime_dependency": false,
      "vendor_root": "vendor/next-rust",
      "vendor_commit": "f3f56ecec2f3f8cefa0f0a1323ea406740251d5c",
      "next_transform_references": [
        "next-custom-transforms::track_dynamic_imports",
        "next-custom-transforms::react_server_components"
      ],
      "copied_code": false
    },
    "output_model": {
      "contract": "dx.www.moduleGraph",
      "compiler_owns_output": true,
      "public_architecture": "DX-owned source graph analysis"
    },
    "runtime_boundaries": {
      "next_runtime_required": false,
      "react_runtime_required": false,
      "rsc_required": false,
      "node_modules_required": false
    },
    "directives": [],
    "static_imports": [
      {
        "specifier": "@/lib/forge/www/landing-page",
        "side_effect_only": false,
        "type_only": false
      }
    ],
    "dynamic_imports": [],
    "unresolved_dynamic_imports": [],
    "unsupported_dynamic_imports": [],
    "dynamic_import_analysis": {
      "status": "none-observed",
      "static_count": 0,
      "unresolved_count": 0,
      "unsupported_count": 0,
      "boundary": "source-owned dynamic import analysis; static specifiers become evidence, expressions remain unresolved, and unsupported call forms stay as adapter-boundary receipts"
    },
    "export_names": [
      "LandingPage"
    ],
    "jsx": true,
    "top_level_await": false,
    "full_nextjs_parity": false,
    "analysis_boundary": "Uses vendored Turbopack ECMAScript and selected Next transform behavior as compatibility references while emitting DX-owned source graph receipts."
  },
  "node_modules_required": false
});
export const dxRuntimeModule = Object.freeze({
  transformed: true,
  transformKind: "tsx-component-runtime",
  exportNames: ["LandingPage"]
});
const dxCreateElement = (tag, props = {}, ...children) => Object.freeze({ kind: "dx.element", tag, props: Object.freeze(props), children: Object.freeze(children) });
export function LandingPage() {
  return LandingPageContent({});
}
export default LandingPage;
export const dxRuntimeExports = Object.freeze({ LandingPage });
export const dxLinkedDependencies = Object.freeze([dep0]);
export default dxSourceModule;
