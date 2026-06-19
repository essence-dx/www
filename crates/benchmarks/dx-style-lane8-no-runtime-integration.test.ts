import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("lane 8 dx style check blocks Tailwind runtime and toolchain package leaks", () => {
  const tools = read("dx-www/src/cli/public_framework_tools.rs");

  for (const marker of [
    "postcss.config.cjs",
    "postcss.config.cts",
    "tailwind.config.cjs",
    "tailwind.config.mts",
    "@tailwindcss/cli",
    "@tailwindcss/vite",
    "@tailwindcss/webpack",
    "@tailwindcss/browser",
    "LEGACY_CSS_TOOLING_PACKAGE_PREFIXES",
    "collect_named_files",
    "legacy_css_tooling_blocks_cjs_mjs_tailwind_and_postcss_configs",
    "legacy_css_tooling_scan_blocks_nested_configs_without_scanning_node_modules",
    "legacy_css_dependency_scan_blocks_tailwind_first_party_packages",
    "legacy_css_dependency_scan_blocks_nested_workspace_manifests",
    "legacy_css_lockfile_scan_blocks_nested_workspace_lockfiles",
    "tailwindcss_import_findings",
    "tailwindcss_import_count",
    "dx-style-tailwindcss-import-migration-boundary",
    "normal_starter_css_allowed",
    "tailwindcss_migration_imports_are_receipted_without_runtime_dependency",
    "tailwindcss_reference_findings",
    "tailwindcss_reference_count",
    "dx-style-tailwindcss-reference-default-theme-boundary",
    "tailwindcss_references_are_receipted_without_runtime_dependency",
    "tailwind_directive_findings",
    "tailwind_directive_count",
    "dx-style-tailwind-directive-runtime-boundary",
    "tailwind_directives_are_receipted_as_runtime_leakage",
    "tailwind_runtime_directive_findings",
    "tailwind_runtime_directive_count",
    "dx-style-tailwind-runtime-directive-boundary",
    "tailwind_runtime_directives_are_receipted_as_toolchain_leakage",
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});

test("lane 8 starter and template CSS remain dx-style owned", () => {
  const templateSources = read("dx-www/src/cli/default_template_sources.rs");
  const wwwTemplatePackage = read("examples/template/package.json");
  const defaultTemplateGuard = read("benchmarks/default-www-template-contract.test.ts");

  assert.doesNotMatch(templateSources, /@import\s+["']tailwindcss["']|@tailwind|cdn\.tailwindcss\.com/);
  assert.doesNotMatch(wwwTemplatePackage, /"tailwindcss"|"@tailwindcss\/[^"]+"|"postcss"|"autoprefixer"/);
  assert.match(defaultTemplateGuard, /starter, dev shell, and template style path use dx-style without Tailwind runtime leaks/);
});
