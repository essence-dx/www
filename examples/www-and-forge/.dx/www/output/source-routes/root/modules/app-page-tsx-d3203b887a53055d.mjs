
export const dxSourceText = "import { startCase } from \"lodash\";\n\nexport const metadata = {\n  title: \"Enhanced Development Experience\",\n  description: \"Orchestrate your code, don't just own it.\",\n} as const;\n\nexport default function HomePage() {\n  return (\n    <main\n      className=\"starter-shell forge-backed-shell\"\n      data-dx-route=\"/\"\n      data-forge-package=\"npm/lodash\"\n    >\n      <section\n        className=\"starter-card source-owned-forge-package\"\n        aria-labelledby=\"starter-title\"\n      >\n        <img className=\"starter-logo\" src=\"/logo.svg\" alt=\"Dx WWW\" />\n        <p className=\"starter-kicker\">Dx WWW</p>\n        <h1 id=\"starter-title\">Enhanced Development Experience</h1>\n        <p className=\"starter-copy\">\n          Orchestrate your code, don't just own it.\n        </p>\n        <p className=\"starter-proof\" data-forge-helper=\"startCase\">\n          Source-owned lodash proof.\n        </p>\n      </section>\n    </main>\n  );\n}\n";
export const dxSourceModule = Object.freeze({
  "source_path": "app/page.tsx",
  "chunk_output": ".dx/www/output/source-routes/root/modules/app-page-tsx-d3203b887a53055d.mjs",
  "kind": "tsx",
  "hash": "d3203b887a53055d",
  "dependencies": [
    {
      "specifier": "lodash",
      "resolved_path": null,
      "chunk_output": null,
      "kind": "external-package-adapter-boundary",
      "resolver_source": "external-package-boundary",
      "resolver_detail": "external-package-adapter-boundary",
      "node_modules_required": false
    }
  ],
  "browser_executable": true,
  "source_transformed": false,
  "transform_kind": "metadata-only",
  "runtime_exports": [],
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
        "specifier": "lodash",
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
      "HomePage",
      "metadata"
    ],
    "jsx": true,
    "top_level_await": false,
    "full_nextjs_parity": false,
    "analysis_boundary": "Uses vendored Turbopack ECMAScript and selected Next transform behavior as compatibility references while emitting DX-owned source graph receipts."
  },
  "node_modules_required": false
});
export const dxRuntimeModule = Object.freeze({
  transformed: false,
  transformKind: "metadata-only",
  exportNames: []
});
export const dxRuntimeExports = Object.freeze({});
export const dxLinkedDependencies = Object.freeze([]);
export default dxSourceModule;
