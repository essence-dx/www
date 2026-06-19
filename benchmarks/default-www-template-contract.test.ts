import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const cliSourcePath = path.join(root, "dx-www", "src", "cli", "mod.rs");
const cliNewCommandPath = path.join(root, "dx-www", "src", "cli", "new_command.rs");
const templateContractPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "default_template_contract.rs",
);
const templateSourcesPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "default_template_sources.rs",
);
const templateMaterializerPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "default_template_materializer.rs",
);
const readinessBundleConsumerPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "launch_readiness_bundle.rs",
);

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function captureRustRawString(source: string, start: string) {
  const startIndex = source.indexOf(start);
  assert.notEqual(startIndex, -1, `missing ${start}`);
  const rawStart = source.indexOf('r#"', startIndex + start.length);
  assert.notEqual(rawStart, -1, `missing raw string start for ${start}`);
  const afterStart = source.slice(rawStart + 3);
  const endIndex = afterStart.indexOf('"#');
  assert.notEqual(endIndex, -1, `missing raw string close for ${start}`);
  return afterStart.slice(0, endIndex);
}

function captureRustFunction(source: string, signature: string) {
  const startIndex = source.indexOf(signature);
  assert.notEqual(startIndex, -1, `missing ${signature}`);
  const nextFunctions = [
    "\nfn ",
    "\npub(crate) fn ",
    "\npub fn ",
    "\n    fn ",
    "\n    pub(crate) fn ",
    "\n    pub fn ",
  ]
    .map((marker) => source.indexOf(marker, startIndex + signature.length))
    .filter((index) => index !== -1);
  const nextFunction = nextFunctions.length > 0 ? Math.min(...nextFunctions) : -1;
  assert.notEqual(nextFunction, -1, `missing following function after ${signature}`);
  return source.slice(startIndex, nextFunction);
}

function captureRustUseBlock(source: string, moduleName: string) {
  const startIndex = source.indexOf(`use ${moduleName}::`);
  assert.notEqual(startIndex, -1, `missing use ${moduleName}::`);
  const endIndex = source.indexOf(";", startIndex);
  assert.notEqual(endIndex, -1, `missing semicolon after use ${moduleName}::`);
  return source.slice(startIndex, endIndex + 1);
}

function readCliSource(): string {
  return [cliSourcePath, cliNewCommandPath]
    .map((sourcePath) => fs.readFileSync(sourcePath, "utf8"))
    .join("\n");
}

test("dx new default template contract keeps DX runtime authoritative", () => {
  const cliSource = readCliSource();
  assert.ok(fs.existsSync(templateContractPath), "default template contract module should exist");
  assert.ok(fs.existsSync(templateSourcesPath), "default template sources module should exist");
  assert.ok(
    fs.existsSync(templateMaterializerPath),
    "default template materializer module should exist",
  );
  const contractSource = fs.readFileSync(templateContractPath, "utf8");
  const templateSources = fs.readFileSync(templateSourcesPath, "utf8");
  const materializerSource = fs.readFileSync(templateMaterializerPath, "utf8");

  assert.match(cliSource, /mod default_template_contract;/);
  assert.match(cliSource, /mod default_template_materializer;/);
  assert.match(cliSource, /mod default_template_sources;/);
  const materializerUse = captureRustUseBlock(cliSource, "default_template_materializer");
  for (const importedName of [
    "DEFAULT_TEMPLATE_CORE_SOURCE_RECEIPT_FILE",
    "write_default_template_source_files",
  ]) {
    assert.match(materializerUse, new RegExp(`\\b${importedName}\\b`));
  }
  const templateSourcesUse = captureRustUseBlock(cliSource, "default_template_sources");
  for (const importedName of [
    "DEFAULT_TEMPLATE_APP_ROUTE_SOURCES",
    "NEXT_FAMILIAR_HOME_ROUTE_PAGE_TSX",
    "NEXT_FAMILIAR_UI_BUTTON_TSX",
    "NEXT_FAMILIAR_UI_CARD_TSX",
    "NEXT_FAMILIAR_UI_SCROLL_AREA_TSX",
  ]) {
    assert.match(templateSourcesUse, new RegExp(`\\b${importedName}\\b`));
  }
  assert.match(templateSources, /pub\(crate\) struct DefaultTemplateAppRouteSource/);
  for (const field of ["route", "aliases", "source_file", "materialized_file", "role"]) {
    assert.match(
      templateSources,
      new RegExp(`pub\\(crate\\) ${field}: &'static`),
      `expected typed route source field ${field}`,
    );
  }
  assert.match(templateSources, /pub\(crate\) struct DefaultTemplateSourceFile/);
  for (const field of ["source_file", "materialized_file", "role", "content"]) {
    assert.match(
      templateSources,
      new RegExp(`pub\\(crate\\) ${field}: &'static`),
      `expected typed materialized source field ${field}`,
    );
  }
  assert.match(
    templateSources,
    /pub\(crate\) const DEFAULT_TEMPLATE_CORE_SOURCE_FILES: &\[DefaultTemplateSourceFile\] = &\[/,
  );
  for (const materializedFile of [
    "app/layout.tsx",
    "app/loading.tsx",
    "app/error.tsx",
    "app/not-found.tsx",
    "components/ui/button.tsx",
    "components/ui/card.tsx",
    "components/ui/scroll-area.tsx",
    "public/d-logo.svg",
    "public/favicon.svg",
    "public/og-image.svg",
    "public/robots.txt",
    ".gitignore",
    ".dx/forge/docs/dx-www-starter-ui.md",
    "components/local/WelcomeCard.tsx",
    "components/icons/icon.tsx",
    "components/marketing/Hero.tsx",
    "components/dashboard/TemplateDashboard.tsx",
    "components/forms/SettingsForm.tsx",
    "app/dashboard/page.tsx",
    "app/settings/page.tsx",
    "app/auth/page.tsx",
    "app/billing/page.tsx",
    "server/actions.ts",
    "server/loaders.ts",
    "app/api/health/route.ts",
  ]) {
    assert.match(
      templateSources,
      new RegExp(`materialized_file: "${materializedFile.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}"`),
      `expected extracted source for ${materializedFile}`,
    );
  }
  assert.doesNotMatch(
    cliSource,
    /let app_layout_content = r#"/,
    "root layout source should live in the default template source module, not inline in cli/mod.rs",
  );
  assert.doesNotMatch(
    cliSource,
    /let app_(?:loading|error|not_found)_content = r#"/,
    "App Router boundary files should live in the default template source module, not inline in cli/mod.rs",
  );
  assert.doesNotMatch(
    cliSource,
    /let ui_(?:button|card|scroll_area)_content = r#"/,
    "starter UI primitives should live in the default template source module, not inline in cli/mod.rs",
  );
  assert.doesNotMatch(
    cliSource,
    /let (?:d_logo|og_image|robots)_content = r#/,
    "starter public assets should live in the default template source module, not inline in cli/mod.rs",
  );
  assert.doesNotMatch(
    templateSources,
    /role: "[^"]*(?:proof|fixture|benchmark|receipt|status)[^"]*"/,
    "core starter source roles should describe product files, not proof fixtures or status artifacts",
  );
  assert.doesNotMatch(
    templateSources,
    /\bproofCards\b|dashboard-proof|proof-card|Open launch proof/,
    "core starter UI should use product dashboard language instead of proof-fixture language",
  );
  for (const sourceConst of [
    "NEXT_FAMILIAR_ROOT_LAYOUT_TSX",
    "NEXT_FAMILIAR_LOADING_TSX",
    "NEXT_FAMILIAR_ERROR_TSX",
    "NEXT_FAMILIAR_NOT_FOUND_TSX",
    "NEXT_FAMILIAR_UI_BUTTON_TSX",
    "NEXT_FAMILIAR_UI_CARD_TSX",
    "NEXT_FAMILIAR_UI_SCROLL_AREA_TSX",
    "NEXT_FAMILIAR_D_LOGO_SVG",
    "NEXT_FAMILIAR_FAVICON_SVG",
    "NEXT_FAMILIAR_OG_IMAGE_SVG",
    "NEXT_FAMILIAR_ROBOTS_TXT",
    "NEXT_FAMILIAR_GITIGNORE",
    "NEXT_FAMILIAR_STARTER_UI_DOC_MD",
    "NEXT_FAMILIAR_WELCOME_CARD_TSX",
    "NEXT_FAMILIAR_ICON_COMPONENT_TSX",
    "NEXT_FAMILIAR_MARKETING_HERO_TSX",
    "NEXT_FAMILIAR_DASHBOARD_COMPONENT_TSX",
    "NEXT_FAMILIAR_SETTINGS_FORM_TSX",
    "NEXT_FAMILIAR_DASHBOARD_ROUTE_PAGE_TSX",
    "NEXT_FAMILIAR_SETTINGS_ROUTE_PAGE_TSX",
    "NEXT_FAMILIAR_AUTH_ROUTE_PAGE_TSX",
    "NEXT_FAMILIAR_BILLING_ROUTE_PAGE_TSX",
    "NEXT_FAMILIAR_SERVER_ACTIONS_TS",
    "NEXT_FAMILIAR_SERVER_LOADERS_TS",
    "NEXT_FAMILIAR_HEALTH_ROUTE_TS",
  ]) {
    assert.match(templateSources, new RegExp(`pub\\(crate\\) const ${sourceConst}: &str =`));
  }
  assert.doesNotMatch(cliSource, /fn write_default_template_source_files\(/);
  assert.match(cliSource, /write_default_template_source_files\(&project_dir\)\?/);
  assert.match(cliSource, /"forge_artifacts": \[[\s\S]*DEFAULT_TEMPLATE_CORE_SOURCE_RECEIPT_FILE/);
  assert.match(materializerSource, /pub\(crate\) const DEFAULT_TEMPLATE_CORE_SOURCE_RECEIPT_FILE: &str =\s*"\.dx\/forge\/template-core-sources\.json";/);
  assert.match(materializerSource, /pub\(crate\) fn write_default_template_source_files\(/);
  assert.match(materializerSource, /use super::default_template_sources::\{[\s\S]*\};/);
  assert.match(materializerSource, /DefaultTemplateSourceFile/);
  assert.match(materializerSource, /DEFAULT_TEMPLATE_CORE_SOURCE_FILES/);
  assert.match(materializerSource, /for source_file in DEFAULT_TEMPLATE_CORE_SOURCE_FILES/);
  assert.match(materializerSource, /content_hash/);
  assert.match(materializerSource, /blake3::hash\(source_file\.content\.as_bytes\(\)\)/);
  assert.match(
    materializerSource,
    /const DEFAULT_TEMPLATE_CONTENT_HASH_ALGORITHM: &str = "blake3";/,
  );
  assert.match(materializerSource, /content_hash_algorithm: DEFAULT_TEMPLATE_CONTENT_HASH_ALGORITHM/);
  assert.match(
    materializerSource,
    /struct DefaultTemplateCoreSourceReceiptFile[\s\S]*content_hash_algorithm: &'static str,[\s\S]*content_hash: String,/,
  );
  assert.match(
    materializerSource,
    /content_hash_algorithm: DEFAULT_TEMPLATE_CONTENT_HASH_ALGORITHM,\s*content_hash,/,
  );
  assert.match(materializerSource, /path_policy: "relative-no-traversal"/);
  assert.match(materializerSource, /aggregate_content_hash/);
  assert.match(materializerSource, /let mut aggregate_hasher = blake3::Hasher::new\(\);/);
  assert.match(materializerSource, /aggregate_hasher\.update/);
  assert.match(materializerSource, /aggregate_hasher\.finalize\(\)\.to_hex\(\)\.to_string\(\)/);
  assert.match(materializerSource, /"dx\.www\.default_template\.core_sources"/);
  assert.match(materializerSource, /node_modules_required: false/);
  assert.match(materializerSource, /serde_json::to_string_pretty/);
  assert.match(
    materializerSource,
    /write_default_template_source_file_set\(project_dir, DEFAULT_TEMPLATE_CORE_SOURCE_FILES\)/,
  );
  assert.match(materializerSource, /fn write_default_template_source_file_set\(/);
  assert.match(materializerSource, /fn validate_default_template_source_files\(/);
  assert.match(
    materializerSource,
    /let validated_targets = validate_default_template_source_files\(project_dir, source_files\)\?/,
  );
  assert.match(
    materializerSource,
    /fn default_template_materializer_rejects_invalid_source_set_before_writing\(/,
  );
  assert.match(
    materializerSource,
    /fn default_template_materializer_rejects_template_local_tooling_artifacts\(/,
  );
  assert.match(
    materializerSource,
    /assert!\([\s\S]*!project[\s\S]*\.path\(\)[\s\S]*\.join\("components\/local\/WelcomeCard\.tsx"\)[\s\S]*\.exists\(\)[\s\S]*\);/,
  );
  assert.match(materializerSource, /fn reject_template_local_artifact_path\(/);
  assert.match(materializerSource, /reject_template_local_artifact_path\(materialized_file\)\?/);
  assert.match(materializerSource, /TEMPLATE_LOCAL_TOOLING_FILE_NAMES/);
  assert.match(materializerSource, /TEMPLATE_LOCAL_TOOLING_DIR_NAMES/);
  for (const blockedTemplateArtifact of [
    "package.json",
    "package-lock.json",
    "npm-shrinkwrap.json",
    "pnpm-lock.yaml",
    "yarn.lock",
    "bun.lock",
    "bun.lockb",
    "next.config.js",
    "next.config.cjs",
    "next.config.mjs",
    "next.config.ts",
    "source.config.ts",
    "next-env.d.ts",
    "tsconfig.json",
    "jsconfig.json",
    "node_modules",
    ".next",
  ]) {
    assert.match(
      materializerSource,
      new RegExp(`"${blockedTemplateArtifact.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}"`),
      `materializer should block ${blockedTemplateArtifact}`,
    );
  }
  for (const removedInlineBinding of [
    "local_component_content",
    "icon_content",
    "marketing_hero_content",
    "dashboard_content",
    "settings_form_content",
    "dashboard_page_content",
    "settings_page_content",
    "auth_page_content",
    "billing_page_content",
    "server_action_content",
    "server_loader_content",
    "health_route_content",
    "ui_button_content",
    "ui_card_content",
    "ui_scroll_area_content",
    "d_logo_content",
    "og_image_content",
    "robots_content",
    "gitignore_content",
    "starter_ui_doc",
  ]) {
    assert.doesNotMatch(
      cliSource,
      new RegExp(`let ${removedInlineBinding} = r#`),
      `${removedInlineBinding} should live in default_template_sources.rs`,
    );
  }
  assert.match(
    templateSources,
    /pub\(crate\) const DEFAULT_TEMPLATE_APP_ROUTE_SOURCES: &\[DefaultTemplateAppRouteSource\]\s*=\s*&\[/,
  );
  assert.match(
    templateSources,
    /DefaultTemplateAppRouteSource \{\s*route: "\/",\s*aliases: &\[\],\s*source_file: "examples\/template\/app\/page\.tsx",\s*materialized_file: "app\/page\.tsx",\s*role: "primary-www-dashboard",\s*\}/,
  );
  assert.match(
    templateSources,
    /pub\(crate\) const NEXT_FAMILIAR_HOME_ROUTE_PAGE_TSX: &str =\s*include_str!\("..\/..\/..\/examples\/template\/app\/page\.tsx"\);/,
  );
  assert.doesNotMatch(
    templateSources,
    /NEXT_FAMILIAR_LAUNCH_ROUTE_PAGE_TSX/,
  );
  assert.doesNotMatch(
    cliSource,
    /include_str!\("..\/..\/..\/examples\/template\/app\/(?:launch\/)?page\.tsx"\)/,
  );
  assert.match(cliSource, /default_www_template_architecture_contract/);
  assert.match(cliSource, /"architecture_contract": default_www_template_architecture_contract\(\)/);
  const zedHandoffSource = captureRustFunction(
    cliSource,
    "fn launch_zed_template_handoff_contract() -> serde_json::Value",
  );
  const readinessBundleSource = captureRustFunction(
    cliSource,
    "fn launch_readiness_bundle_contract() -> serde_json::Value",
  );
  assert.match(zedHandoffSource, /"architecture_contract": default_www_template_architecture_contract\(\)/);
  assert.match(zedHandoffSource, /"file": NEXT_FAMILIAR_LAUNCH_ZED_TEMPLATE_HANDOFF_FILE/);
  assert.match(zedHandoffSource, /let primary_route = DEFAULT_TEMPLATE_APP_ROUTE_SOURCES\[0\];/);
  assert.doesNotMatch(zedHandoffSource, /let secondary_route = DEFAULT_TEMPLATE_APP_ROUTE_SOURCES\[1\];/);
  assert.match(zedHandoffSource, /"route": primary_route\.route/);
  assert.match(zedHandoffSource, /"route_aliases": primary_route\.aliases/);
  assert.match(zedHandoffSource, /"entrypoint_file": primary_route\.materialized_file/);
  assert.match(zedHandoffSource, /"source_entrypoint_file": primary_route\.source_file/);
  assert.match(zedHandoffSource, /"entrypoint_role": primary_route\.role/);
  assert.doesNotMatch(zedHandoffSource, /"secondary_entrypoint_file"/);
  assert.doesNotMatch(zedHandoffSource, /"secondary_source_entrypoint_file"/);
  assert.doesNotMatch(zedHandoffSource, /"secondary_entrypoint_role"/);
  assert.match(
    readinessBundleSource,
    /"architecture_contract": default_www_template_architecture_contract\(\)/,
  );
  assert.match(cliSource, /const NEXT_FAMILIAR_LAUNCH_ZED_TEMPLATE_HANDOFF_FILE: &str =/);
  assert.match(cliSource, /\.dx\/forge\/template-readiness\/zed-template-handoff\.json/);
  const zedHandoffWriter = captureRustFunction(
    cliSource,
    "fn write_next_familiar_launch_zed_template_handoff(",
  );
  assert.match(zedHandoffWriter, /let mut handoff = launch_zed_template_handoff_contract\(\)/);
  assert.match(zedHandoffWriter, /NEXT_FAMILIAR_LAUNCH_ZED_TEMPLATE_HANDOFF_FILE/);
  assert.match(cliSource, /Self::write_next_familiar_launch_zed_template_handoff\(/);
  assert.doesNotMatch(
    cliSource,
    /"route": "\/",\s*"route_aliases": \["\/"\],\s*"source_file": "examples\/template\/app\/page\.tsx",\s*"materialized_file": "app\/page\.tsx"/,
  );

  const readinessConsumerSource = fs.readFileSync(readinessBundleConsumerPath, "utf8");
  assert.match(
    readinessConsumerSource,
    /const ZED_TEMPLATE_HANDOFF_PATH: &str =\s*"\.dx\/forge\/template-readiness\/zed-template-handoff\.json";/,
  );
  assert.match(
    readinessConsumerSource,
    /read_json_file\(&project\.join\(ZED_TEMPLATE_HANDOFF_PATH\)\)/,
  );
  assert.match(readinessConsumerSource, /contract_schema: Option<String>/);
  assert.match(readinessConsumerSource, /route_aliases: Vec<String>/);
  assert.match(readinessConsumerSource, /runtime_foundation: Option<String>/);
  assert.match(readinessConsumerSource, /secondary_entrypoint_file: Option<String>/);
  assert.match(readinessConsumerSource, /react_required: Option<bool>/);
  assert.match(readinessConsumerSource, /rsc_required: Option<bool>/);
  assert.match(readinessConsumerSource, /node_required: Option<bool>/);
  assert.match(readinessConsumerSource, /napi_required: Option<bool>/);
  assert.match(readinessConsumerSource, /node_modules_required: Option<bool>/);
  assert.match(readinessConsumerSource, /next_familiar_authoring: Option<bool>/);
  assert.match(readinessConsumerSource, /dx_source_build: Option<bool>/);
  assert.match(readinessConsumerSource, /external_bundler_runtime_executed: Option<bool>/);
  assert.match(readinessConsumerSource, /external_bundler_runtime_required: Option<bool>/);
  assert.doesNotMatch(readinessConsumerSource, /full_next_parity/);
  const readinessTerminalSource = captureRustFunction(
    readinessConsumerSource,
    "pub(crate) fn launch_readiness_bundle_terminal(",
  );
  const readinessMarkdownSource = captureRustFunction(
    readinessConsumerSource,
    "pub(crate) fn launch_readiness_bundle_markdown(",
  );
  for (const outputSource of [readinessTerminalSource, readinessMarkdownSource]) {
    assert.match(outputSource, /Zed handoff architecture contract/i);
    assert.match(outputSource, /reads_standalone_handoff/);
    assert.match(outputSource, /route_aliases/);
    assert.match(outputSource, /entrypoint_file/);
    assert.match(outputSource, /secondary_entrypoint_file/);
    assert.match(outputSource, /runtime_foundation/);
    assert.match(outputSource, /react_required/);
    assert.match(outputSource, /rsc_required/);
    assert.match(outputSource, /node_required/);
    assert.match(outputSource, /napi_required/);
    assert.match(outputSource, /node_modules_required/);
    assert.match(outputSource, /next_familiar_authoring/);
    assert.match(outputSource, /dx_source_build/);
    assert.match(outputSource, /external_bundler_runtime_executed/);
    assert.match(outputSource, /external_bundler_runtime_required/);
    assert.doesNotMatch(outputSource, /full_next_parity/);
  }
  assert.match(readinessConsumerSource, /"zed-handoff-entrypoint"/);
  assert.match(readinessConsumerSource, /zed_summary\.route_aliases\.is_empty\(\)/);
  assert.match(readinessConsumerSource, /zed_summary\.entrypoint_file\.as_deref\(\) == Some\("app\/page\.tsx"\)/);
  assert.match(
    readinessConsumerSource,
    /zed_summary\.secondary_entrypoint_file\.is_none\(\)/,
  );

  const configSource = captureRustRawString(cliSource, "fn default_dx_project_config(project_name: &str) -> String");
  assert.match(configSource, /project\(name=.*version=0\.1\.0 kind=www-app/);
  assert.match(configSource, /www\([\s\S]*app_dir=app[\s\S]*output_dir=\.dx\/www\/output/);
  assert.doesNotMatch(configSource, /runtime\(/);
  assert.doesNotMatch(configSource, /dx-www-html|dx-www-js|dx-www-wasm|dx-www-protocol/);
  assert.match(configSource, /style\([\s\S]*tokens=styles\/theme\.css[\s\S]*generated_css=styles\/generated\.css/);
  assert.match(configSource, /icons\(component=Icon generated_dir=components\/icons\)/);
  assert.match(configSource, /forge\(policy=forge-first-no-node-modules\)/);
  assert.match(configSource, /check\(score_scale=500 lighthouse=true\)/);
  assert.doesNotMatch(configSource, /contract\(/);
  assert.doesNotMatch(configSource, /client=/);
  assert.doesNotMatch(configSource, /tools\[name command enabled output\]/);
  assert.doesNotMatch(configSource, /shadcn\(/);
  assert.doesNotMatch(configSource, /react-shaped/i);

  const readmeSource = captureRustRawString(cliSource, "let readme_content = format!(");
  assert.match(readmeSource, /DX-owned runtime/);
  assert.match(readmeSource, /React-compatible authoring is optional/);
  assert.match(readmeSource, /dx-check/);
  assert.match(readmeSource, /Zed and Studio/);
  assert.doesNotMatch(readmeSource, /React-shaped|source-owned React/i);

  for (const protectedCrate of [
    "dx-www-browser-micro",
    "dx-www-browser",
    "dx-www-packet",
    "dx-www-binary",
    "dx-www-morph",
    "dx-serializer",
    "dx-style",
    "dx-www-server",
  ]) {
    assert.match(contractSource, new RegExp(protectedCrate));
  }
  for (const boundary of [
    /"react_required": false/,
    /"rsc_required": false/,
    /"node_required": false/,
    /"napi_required": false/,
    /"node_resolver_default": false/,
    /"public_runtime_layers"/,
    /"html"/,
    /"javascript"/,
    /"wasm"/,
    /"browser"/,
    /"dx_source_build": true/,
    /"external_bundler_runtime_executed": false/,
    /"external_bundler_runtime_required": false/,
    /"next_rust_reference_scope": "reference-provenance-only"/,
    /"next_familiar_authoring": true/,
    /"forge_receipts": true/,
    /"dx_check": true/,
    /"zed_template_handoff": true/,
    /"studio_edit_contract": true/,
  ]) {
    assert.match(contractSource, boundary);
  }
  assert.doesNotMatch(contractSource, /"turbopack_runtime_build_adoption"/);
  assert.doesNotMatch(contractSource, /"turbopack_powers_dx_build"/);
  assert.doesNotMatch(contractSource, /"turbopack_powers_dx_dev"/);
  assert.doesNotMatch(contractSource, /"full_next_parity"/);
  assert.doesNotMatch(contractSource, /"selected_next_rust_groups"/);
  assert.doesNotMatch(contractSource, /"turbopack-dev-server"/);
  assert.match(contractSource, /"Keep Next\/Turbopack materials as reference\/provenance only\."/);
});

test("launch template source remains node_modules free", () => {
  assert.equal(fs.existsSync(path.join(root, "examples", "template", "node_modules")), false);
  assert.doesNotMatch(read("examples/template/dx"), /node_modules\s*=\s*true/);
});

test("browser preview route surfaces use dx-style generated CSS without Tailwind runtime", () => {
  const previewGlobals = read(".dx/template-app-browser-preview/styles/globals.css");
  assert.match(previewGlobals, /Generated by dx style build/);
  assert.doesNotMatch(previewGlobals, /#[0-9a-fA-F]{3,8}|\brgba?\(/);

  const routeSurfaces = [
    ["/", ".dx/template-app-browser-preview/pages/index.html"],
    ["/dashboard", ".dx/template-app-browser-preview/pages/dashboard.html"],
    ["/login", ".dx/template-app-browser-preview/pages/login.html"],
  ] as const;

  for (const [route, surfacePath] of routeSurfaces) {
    const surface = read(surfacePath);
    assert.match(
      surface,
      /<link rel="stylesheet" href="\/styles\/globals\.css"\s*\/?>/,
      `${route} must load the dx-style generated stylesheet`,
    );
    assert.doesNotMatch(surface, /cdn\.tailwindcss\.com|<script[^>]+tailwind|@tailwind/i);
    assert.doesNotMatch(surface, /Binary-First Web Framework|fake fallback/i);
  }
});

test("starter, dev shell, and template style path use dx-style without Tailwind runtime leaks", () => {
  const cliSource = readCliSource();
  const templateSources = fs.readFileSync(templateSourcesPath, "utf8");
  const wwwTemplatePackage = read("examples/template/package.json");
  const wwwTemplateGlobals = read("examples/template/styles/globals.css");
  const previewGlobals = read(".dx/template-app-browser-preview/styles/globals.css");
  const appRouteSource = read("core/src/delivery/app_route.rs");
  const appRouterBuildOutputSource = read("dx-www/src/cli/app_router_build_output.rs");
  const buildGraphContract = read("benchmarks/dx-build-graph-receipt.test.ts");
  const tsxLaunchRuntimeSource = read("dx-www/src/cli/tsx_launch_runtime.rs");
  const defaultLayout = captureRustFunction(cliSource, "fn default_layout() -> String");
  const htmlShell = captureRustFunction(cliSource, "fn wrap_with_html_shell(content: &str) -> String");
  const errorPage = captureRustFunction(
    cliSource,
    "fn render_error_page(code: &str, message: &str) -> String",
  );
  const shellSources = [defaultLayout, htmlShell, errorPage].join("\n");

  assert.doesNotMatch(cliSource, /cdn\.tailwindcss\.com/);
  assert.doesNotMatch(cliSource, /tooling\.dx_style\.mode="tailwind-compatible"/);
  assert.match(cliSource, /tokens=styles\/theme\.css/);
  assert.match(cliSource, /generated_css=styles\/generated\.css/);
  assert.match(cliSource, /www\([\s\S]*app_dir=app[\s\S]*output_dir=\.dx\/www\/output/);
  assert.match(cliSource, /docs\([\s\S]*route=\/docs[\s\S]*content=content\/docs[\s\S]*openapi=openapi\/dx-www\.yaml/);
  assert.doesNotMatch(cliSource, /fumadocs\(/);
  assert.doesNotMatch(cliSource, /config_owner_file=dx|config_files=\[\]|turbopack_runtime=false/);
  assert.match(templateSources, /import "\.\.\/styles\/globals\.css";/);
  assert.match(cliSource, /project_dir\.join\("styles\/theme\.css"\)/);
  assert.match(cliSource, /project_dir\.join\("styles\/generated\.css"\)/);
  assert.match(cliSource, /project_dir\.join\("styles\/globals\.css"\)/);
  assert.match(cliSource, /@import \\"\.\/generated\.css\\";/);

  assert.doesNotMatch(shellSources, /cdn\.tailwindcss\.com/);
  assert.doesNotMatch(shellSources, /Binary-First Web Framework/);
  assert.doesNotMatch(shellSources, /\b(?:bg|text|border)-(?:slate|emerald|teal)-\d{2,3}/);
  for (const shellSource of [defaultLayout, htmlShell, errorPage]) {
    assert.match(shellSource, /\/styles\/globals\.css/);
    assert.match(shellSource, /data-dx-renderer="dx-www-shell"/);
  }

  assert.doesNotMatch(`${templateSources}\n${wwwTemplatePackage}`, /cdn\.tailwindcss\.com/);
  assert.doesNotMatch(wwwTemplatePackage, /"dependencies"|"devDependencies"|tailwindcss|postcss/);
  assert.match(wwwTemplateGlobals, /Canonical global stylesheet/);
  assert.match(previewGlobals, /Generated by dx style build/);
  assert.doesNotMatch(wwwTemplateGlobals, /#[0-9a-fA-F]{3,8}|\brgba?\(/);
  assert.doesNotMatch(previewGlobals, /#[0-9a-fA-F]{3,8}|\brgba?\(/);

  assert.match(appRouteSource, /DxStyleDelivery::GeneratedCss/);
  assert.match(appRouteSource, /DxReactGeneratedStyleAsset/);
  assert.match(appRouteSource, /data-dx-generated="true"/);
  assert.match(appRouterBuildOutputSource, /write_app_generated_style_assets/);
  assert.match(appRouterBuildOutputSource, /proof\.generated_styles/);
  assert.match(buildGraphContract, /styles\/generated\.css/);
  assert.match(buildGraphContract, /dx-style-css/);
  assert.match(tsxLaunchRuntimeSource, /\/styles\/globals\.css/);
  assert.doesNotMatch(tsxLaunchRuntimeSource, /#[0-9a-fA-F]{3,8}|\brgba?\(/);
  assert.doesNotMatch(
    `${appRouteSource}\n${appRouterBuildOutputSource}\n${tsxLaunchRuntimeSource}`,
    /cdn\.tailwindcss\.com|<script[^>]+tailwind|tailwind-compatible/,
  );
});
