const assert = require("node:assert");
const { spawnSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function readRequiredFile(relativePath) {
  const filePath = path.join(root, relativePath);
  assert.ok(fs.existsSync(filePath), `expected ${relativePath} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

function readDxStyleTools() {
  const publicTools = readRequiredFile("dx-www/src/cli/public_framework_tools.rs");
  const dxStyleTools = readRequiredFile("dx-www/src/cli/public_framework_tools/dx_style.rs");
  assert.match(publicTools, /mod dx_style;/);
  return dxStyleTools;
}

function writeFixtureFile(filePath, content) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, content);
}

function parseJsonOutput(output) {
  const jsonStart = output.indexOf("{");
  assert.notEqual(jsonStart, -1, `expected JSON output, got: ${output}`);
  return JSON.parse(output.slice(jsonStart));
}

test("dx-style launch docs carry a precise Tailwind compatibility matrix", () => {
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const category of [
    "Layout",
    "Flex",
    "Grid",
    "Spacing",
    "Sizing",
    "Typography",
    "Colors",
    "Borders",
    "Radius",
    "Shadows",
    "Opacity",
    "Transforms",
    "Transitions",
    "Animation",
    "Filters",
    "Responsive variants",
    "Dark mode",
    "Hover/focus/active",
    "Data/aria/group/peer variants",
    "Arbitrary values",
    "Negative values",
    "Important modifiers",
    "Container queries",
  ]) {
    assert.match(matrix, new RegExp(`\\| ${category} \\|`));
  }

  assert.match(matrix, /Important modifiers \| Supported subset/);
  assert.match(matrix, /Data\/aria\/group\/peer variants \| Partial/);
  assert.match(matrix, /Does dx-style support every Tailwind class\? \| No/);
  assert.match(matrix, /Do not claim full Tailwind classname parity/);
});

test("dx-style public launch story is normal CSS and not binary-first marketing", () => {
  const readme = readRequiredFile("related-crates/style/README.md");
  const dx = readRequiredFile("DX.md");
  const rootReadme = readRequiredFile("README.md");

  for (const source of [readme, dx, rootReadme]) {
    assert.match(source, /normal generated CSS|generated CSS|normal CSS/i);
    assert.match(source, /no hidden dependency surface/i);
  }

  for (const source of [readme, dx]) {
    assert.match(source, /matrix-gated|not.*full Tailwind|not.*all Tailwind|Full Tailwind classname parity remains gated/i);
  }

  assert.doesNotMatch(readme, /fastest CSS utility generator in the world/i);
  assert.doesNotMatch(readme, /replaces traditional CSS frameworks like Tailwind/i);
  assert.match(readme, /Binary style output is internal and deprecated for the public launch path/);
});

test("dx-style evidence answers rkyv and memmap2 honestly", () => {
  const cargo = readRequiredFile("related-crates/style/Cargo.toml");
  const snapshot = readRequiredFile("related-crates/style/src/cache/snapshot.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const output = readRequiredFile("related-crates/style/src/core/output.rs");
  const readme = readRequiredFile("related-crates/style/README.md");

  assert.match(cargo, /memmap2 = "0\.9\.5"/);
  assert.match(snapshot, /use memmap2::Mmap/);
  assert.match(engine, /use memmap2::\{Mmap, MmapOptions\}/);
  assert.match(output, /use memmap2::MmapMut/);
  assert.doesNotMatch(cargo, /\brkyv\b/);
  assert.match(readme, /rkyv is not a dx-style dependency/);
});

test("dx-style registry snapshot cache plan is isolated and evidence-backed", () => {
  const cacheMod = readRequiredFile("related-crates/style/src/cache/mod.rs");
  const plan = readRequiredFile("related-crates/style/src/cache/registry_snapshot_plan.rs");
  const readme = readRequiredFile("related-crates/style/README.md");

  for (const marker of [
    "pub mod registry_snapshot_plan;",
    "pub const REGISTRY_SNAPSHOT_FORMAT_VERSION",
    "pub const REGISTRY_SNAPSHOT_PATH",
    "pub struct RegistrySnapshotPlan",
    "pub fn registry_snapshot_plan() -> RegistrySnapshotPlan",
    "class registry",
    "theme token",
    "memmap2-backed",
    "rkyv is not wired into dx-style",
  ]) {
    assert.match(`${cacheMod}\n${plan}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(plan, /\.dx\/style\/registry\.snapshot/);
  assert.match(plan, /read-only memory map/);
  assert.match(plan, /normal generated CSS/);
  assert.match(readme, /Registry snapshot plan/);
  assert.match(readme, /rkyv remains unwired/);
});

test("dx-style public CLI owns the PostCSS replacement scan contract", () => {
  const tools = readDxStyleTools();
  const dx = readRequiredFile("DX.md");
  const readme = readRequiredFile("README.md");

  for (const marker of [
    "DX_STYLE_POSTCSS_REPLACEMENT_SCAN_ROOTS",
    "DX_STYLE_POSTCSS_REPLACEMENT_EXTENSIONS",
    "DX_STYLE_CLASS_ATTRIBUTE_PATTERNS",
    "DX_STYLE_PUBLIC_THEME_FILE",
    "DX_STYLE_PUBLIC_GENERATED_CSS",
    "postcss_replacement_contract",
    "postcss_adapter_dependency",
    "none",
    "className=\\\"",
    "styles/globals.css",
    "styles/theme.css",
    "styles/generated.css",
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(tools, /"app", "components", "styles", "forge"/);
  assert.match(tools, /"tsx", "jsx", "ts", "js", "css"/);
  assert.match(dx, /PostCSS replacement status/);
  assert.match(readme, /dx-style tokens plus normal CSS/);
});

test("dx-style scanner separates style entry files from HTML and MDX class sources", () => {
  const tools = readDxStyleTools();
  const dx = readRequiredFile("DX.md");

  for (const marker of [
    "fn is_dx_style_static_class_source(ext: &str) -> bool",
    'matches!(ext, "tsx" | "jsx" | "ts" | "js" | "html" | "mdx")',
    ".is_some_and(is_dx_style_static_class_source)",
    "fn style_entry_files(&self) -> Vec<String>",
    "files.insert(self.theme_file.clone())",
    "files.insert(self.app_import.clone())",
    '"app/**/*.mdx"',
    '"components/**/*.html"',
    '"forge/**/*.mdx"',
    '"forge/**/*.html"',
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(dx, /HTML and MDX scanner fixture/);
  assert.match(dx, /CSS remains a style entry input, not a class-token source/);
});

test("dx style build emits utility CSS from HTML and MDX source files", (t) => {
  const cli = path.join(root, "target", "debug", "dx-www.exe");
  if (!fs.existsSync(cli)) {
    t.skip("target/debug/dx-www.exe is required for this lightweight CLI smoke");
    return;
  }

  const project = fs.mkdtempSync(path.join(os.tmpdir(), "dx-style-html-mdx-"));
  try {
    writeFixtureFile(
      path.join(project, "app", "index.html"),
      '<main class="p-4 text-red-500 md:hover:bg-blue-500">HTML source</main>',
    );
    writeFixtureFile(
      path.join(project, "components", "card.mdx"),
      '<section className="mt-2 text-blue-500 hover:bg-red-500">MDX source</section>',
    );
    writeFixtureFile(
      path.join(project, "styles", "globals.css"),
      ".fixture { color: hsl(var(--foreground)); }\n",
    );

    const result = spawnSync(cli, ["style", "build", "--json"], {
      cwd: project,
      encoding: "utf8",
      timeout: 15000,
    });

    assert.equal(result.status, 0, result.stderr || result.stdout);
    const report = parseJsonOutput(result.stdout);
    const generated = fs.readFileSync(path.join(project, "styles", "generated.css"), "utf8");

    assert.ok(report.scanned_static_class_count >= 5, JSON.stringify(report));
    assert.ok(report.generated_utility_class_count >= 5, JSON.stringify(report));
    assert.equal(report.unsupported_scanned_class_count, 0, JSON.stringify(report));
    assert.match(generated, /\.p-4\s*\{/);
    assert.match(generated, /--spacing:\s*0\.25rem/);
    assert.match(generated, /padding:\s*calc\(var\(--spacing\)\s*\*\s*4\)/);
    assert.match(generated, /\.mt-2\s*\{/);
    assert.match(generated, /\.text-red-500\s*\{/);
    assert.match(generated, /\.text-blue-500\s*\{/);
    assert.match(generated, /\.hover\\:bg-red-500:hover\s*\{/);
    assert.match(generated, /@media\s*\(min-width:\s*768px\)/);
    assert.match(generated, /\.md\\:hover\\:bg-blue-500:hover\s*\{/);
  } finally {
    fs.rmSync(project, { recursive: true, force: true });
  }
});

test("launch runtime CSS uses dx-style tokens instead of hardcoded colors", () => {
  const runtimeCss = readRequiredFile("examples/template/styles/globals.css");
  const docs = readRequiredFile("DX.md");

  assert.match(runtimeCss, /--dx-bg: hsl\(var\(--background\)\)/);
  assert.match(runtimeCss, /--dx-text: hsl\(var\(--foreground\)\)/);
  assert.match(runtimeCss, /--dx-green: hsl\(var\(--success\)\)/);
  assert.match(runtimeCss, /--dx-red: hsl\(var\(--destructive\)\)/);
  assert.doesNotMatch(runtimeCss, /#[0-9a-fA-F]{3,8}/);
  assert.doesNotMatch(runtimeCss, /\brgba?\(/);
  assert.match(docs, /launch runtime CSS bridge now aliases DX theme tokens/);
});

test("dx-www style compiler exposes normal CSS as public output path", () => {
  const compiler = readRequiredFile("dx-www/src/build/style.rs");
  const dx = readRequiredFile("DX.md");

  for (const marker of [
    "pub const STYLE_PUBLIC_OUTPUT_MODE",
    "normal-generated-css",
    "pub const STYLE_BINARY_OUTPUT_STATUS",
    "deprecated-internal",
    "pub fn compile_normal_css(&self, style: &ParsedStyle) -> DxResult<String>",
    "fn render_css_rule",
    "fn render_css_declarations",
    "DXC1 remains available only for internal compatibility",
  ]) {
    assert.match(compiler, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(dx, /dx-www style compiler now exposes a normal CSS renderer/);
});

test("dx-www style parser exposes TSX class attribute token extraction", () => {
  const parser = readRequiredFile("dx-www/src/parser/style.rs");
  const dx = readRequiredFile("DX.md");

  for (const marker of [
    "pub const STYLE_CLASS_ATTRIBUTE_PATTERNS",
    "className=\\\"",
    "className='",
    "class=\\\"",
    "class='",
    "pub fn extract_class_attribute_tokens(source: &str) -> Vec<String>",
    "fn clean_class_attribute_token",
    "extract_class_attribute_tokens_reads_class_and_class_name_attributes",
    "dx-style @apply-compatible directive",
  ]) {
    assert.match(parser, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.doesNotMatch(parser, /Tailwind-style @apply directive/);
  assert.match(dx, /dx-www style parser now exposes TSX class attribute extraction/);
});

test("dx-style class scanner covers static TSX expression class strings without claiming dynamic JS parity", () => {
  const parser = readRequiredFile("dx-www/src/parser/style.rs");
  const cli = readDxStyleTools();
  const dx = readRequiredFile("DX.md");

  const sharedScannerMarkers = [
    'className={`',
    'className={\\"',
    "className={'",
    'class={`',
    'class={\\"',
    "class={'",
  ];

  for (const marker of sharedScannerMarkers) {
    assert.match(parser, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
    assert.match(cli, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(parser, /extract_class_attribute_tokens_reads_static_tsx_expression_strings/);
  assert.match(dx, /Static TSX expression class strings/);
  assert.match(dx, /does not claim arbitrary JavaScript class expression parity/);
});

test("dx-style class scanner covers static class helper strings without claiming arbitrary JS parsing", () => {
  const parser = readRequiredFile("dx-www/src/parser/style.rs");
  const cli = readDxStyleTools();
  const dx = readRequiredFile("DX.md");

  for (const marker of [
    "STYLE_STATIC_CLASS_FUNCTIONS",
    '"classes("',
    '"dxClass("',
    '"cn("',
    '"cx("',
    '"clsx("',
    '"classNames("',
    '"cva("',
    "extract_static_function_class_tokens",
    "extract_quoted_tokens_until_call_end",
    "extract_class_attribute_tokens_reads_static_helper_function_strings",
  ]) {
    assert.match(parser, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "DX_STYLE_STATIC_CLASS_FUNCTIONS",
    '"classes("',
    '"dxClass("',
    '"cn("',
    '"cx("',
    '"clsx("',
    '"classNames("',
    '"cva("',
    "extract_static_function_class_tokens",
    "extract_quoted_tokens_until_call_end",
  ]) {
    assert.match(cli, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(dx, /Static `classes\(\)`, `dxClass\(\)`, `cn\(\)`, `cx\(\)`, `clsx\(\)`, `classNames\(\)`, and `cva\(\)` class strings/);
  assert.match(dx, /`classes\(\)` is the preferred DX-owned helper/);
  assert.match(dx, /does not claim arbitrary JavaScript parsing/);
});

test("dx-style class scanner rejects dynamic template class placeholders", () => {
  const parser = readRequiredFile("dx-www/src/parser/style.rs");
  const cli = readDxStyleTools();
  const dx = readRequiredFile("DX.md");

  for (const marker of [
    "fn static_class_token(token: &str) -> Option<String>",
    "cleaned.contains(\"${\")",
    "cleaned.contains('$')",
    "extract_class_attribute_tokens_skips_dynamic_template_placeholders",
  ]) {
    assert.match(parser, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "fn static_class_token(token: &str) -> Option<String>",
    "cleaned.contains(\"${\")",
    "cleaned.contains('$')",
  ]) {
    assert.match(cli, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(dx, /Dynamic template class placeholders/);
  assert.match(dx, /skipped instead of emitted as generated CSS selectors/);
});

test("dx style receipts distinguish scanned static class tokens from generated style class tokens", () => {
  const tools = readDxStyleTools();
  const dx = readRequiredFile("DX.md");

  for (const marker of [
    "fn collect_scanned_class_tokens(",
    "fn collect_generated_style_class_tokens(",
    "fn class_source_extension_counts(",
    "fn style_entry_file_count(",
    "class_source_extension_counts",
    "style_entry_file_count",
    "scanned_static_class_count",
    "generated_dx_class_count",
    "generated_utility_class_count",
    "generated_class_count",
    "scanned_class_inventory_policy",
    "only concrete static class tokens are scanned; supported tokens are materialized through the dx-style engine plus default template CSS",
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(dx, /scanned static class tokens from generated style class tokens/);
});

test("dx-style check reports unsupported scanned utility classes from project source", () => {
  const tools = readDxStyleTools();
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const mod = readRequiredFile("dx-www/src/cli/mod.rs");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    "mod dx_style_support;",
    "unsupported_scanned_classes_with_resolver",
    "pub(super) fn class_has_generated_css",
    "pub(super) fn unsupported_scanned_classes(",
    "unsupported_scanned_class_count",
    "unsupported_scanned_class_findings",
    "dx-style-unsupported-scanned-class",
    "tailwind_like_class_requires_generation",
    "exact_tailwind_utility_names",
    '"prose"',
    '"text-shadow-sm"',
    '"field-sizing-content"',
  ]) {
    assert.match(`${tools}\n${support}\n${mod}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(
    tools,
    /let unsupported_scanned_classes =\s+unsupported_scanned_classes_with_resolver\(&scanned_class_tokens, \|class_name\|/,
  );
  assert.match(tools, /&& unsupported_scanned_classes\.is_empty\(\)/);
  assert.match(dx, /unsupported scanned utility classes/i);
  assert.match(todo, /dx-check unsupported scanned utility summary/i);
  assert.match(changelog, /Reported unsupported dx-style scanned utility classes/i);
});

test("dx-www public style build feeds scanned utilities through the dx-style engine", () => {
  const tools = readDxStyleTools();
  const core = readRequiredFile("related-crates/style/src/core/mod.rs");
  const dx = readRequiredFile("DX.md");

  for (const marker of [
    "style::core::StyleEngine::from_theme_css(&theme_css)",
    "style_engine.css_for_class(class_name).is_some()",
    "style::core::format_css_pretty(&css)",
    "browser_compatible_generated_css",
    "css_browser_normalizer",
    "lightningcss-parse-print",
    "generated_utility_class_count",
    "generated_app_css(&classes, &source_hash, &theme_css)",
    "if let Some(rule) = engine.css_for_class(class_name)",
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "pub fn css_for_class(class_name: &str) -> Option<String>",
    "AppState::engine().css_for_class(class_name)",
    "pub fn format_css_pretty(input: &str) -> Option<String>",
    "formatter::format_css_pretty(input)",
  ]) {
    assert.match(core, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(dx, /dx style build now feeds scanned class tokens through the dx-style engine/);
  assert.match(dx, /Lightning CSS parse\/print compatibility pass/);
});

test("dx-style high-usage Tailwind utility families have targeted Rust guards", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const tools = readDxStyleTools();
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn aspect_ratio_utility(class_name: &str) -> Option<String>",
    "fn background_utility(class_name: &str) -> Option<String>",
    "fn flex_basis_utility(class_name: &str) -> Option<String>",
    "fn grid_auto_utility(class_name: &str) -> Option<String>",
    "fn list_utility(class_name: &str) -> Option<String>",
    "fn scroll_utility(class_name: &str) -> Option<String>",
    "fn divide_utility(class_name: &str) -> Option<String>",
    "fn filter_numeric_value(raw_value: &str) -> Option<String>",
    "const TRANSFORM_GPU_VALUE",
    "high_usage_tailwind_layout_utilities_generate_css",
    "high_usage_tailwind_background_and_list_utilities_generate_css",
    "high_usage_tailwind_filter_utilities_generate_css",
    'generate_utility_css("aspect-video").unwrap()',
    'generate_utility_css("basis-1/2").unwrap()',
    'generate_utility_css("auto-cols-fr").unwrap()',
    'generate_utility_css("bg-cover").unwrap()',
    'generate_utility_css("overscroll-y-contain").unwrap()',
    'generate_utility_css("divide-red-500").unwrap()',
    'generate_utility_css("brightness-125").unwrap()',
    'generate_utility_css("backdrop-blur-md").unwrap()',
    'generate_utility_css("transform-gpu")',
  ]) {
    assert.match(utility, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "browser_compatible_generated_css",
    "/* Generated by dx style build. */",
    "/* dx-style source-hash: {source_hash} */",
    "style::core::format_css_pretty(&css)",
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(matrix, /high-usage Tailwind utility families now have targeted guards/);
  assert.match(matrix, /`transform-gpu`, `transform-cpu`/);
});

test("dx-style logical sizing utilities track Tailwind v4.3 generated output", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    "enum SizeAxis",
    '("min-inline-", "min-inline-size", SizeAxis::Width, true)',
    '("max-inline-", "max-inline-size", SizeAxis::Width, true)',
    '("inline-", "inline-size", SizeAxis::Width, true)',
    '("min-block-", "min-block-size", SizeAxis::Height, false)',
    '("max-block-", "max-block-size", SizeAxis::Height, false)',
    '("block-", "block-size", SizeAxis::Height, false)',
    "fn size_value(raw_value: &str, axis: SizeAxis, allow_container_scale: bool) -> Option<String>",
    "fn custom_property_size_value(raw_value: &str) -> Option<String>",
    'generate_utility_css("inline-1/2").unwrap()',
    'generate_utility_css("inline-(--dx-inline-size)").unwrap()',
    'generate_utility_css("block-screen").unwrap()',
    'generate_utility_css("w-screen").unwrap()',
    'generate_utility_css("h-screen").unwrap()',
  ]) {
    assert.match(utility, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    'class_name: "inline-1/2"',
    'class_name: "inline-(--dx-inline-size)"',
    'class_name: "block-screen"',
    'area: "logical-sizing"',
  ]) {
    assert.match(parity, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(matrix, /logical sizing utilities now have targeted guards/i);
  assert.match(`${dx}\n${todo}\n${changelog}`, /Tailwind v4\.3 logical sizing/i);
});

test("dx-style Tailwind v4.3 interactivity additions generate CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    '"scheme-light-dark" => Some("color-scheme: light dark")',
    '"scheme-only-dark" => Some("color-scheme: only dark")',
    "fn scrollbar_utility(class_name: &str) -> Option<String>",
    "fn scrollbar_color_declaration(role: &str, raw_value: &str) -> Option<String>",
    "fn zoom_utility(class_name: &str) -> Option<String>",
    "fn tab_size_utility(class_name: &str) -> Option<String>",
    'generate_utility_css("scheme-light-dark").unwrap()',
    'generate_utility_css("scrollbar-thin").unwrap()',
    'generate_utility_css("scrollbar-gutter-both").unwrap()',
    'generate_utility_css("scrollbar-thumb-red-500").unwrap()',
    'generate_utility_css("scrollbar-track-slate-200").unwrap()',
    'generate_utility_css("zoom-125").unwrap()',
    'generate_utility_css("zoom-(--dx-zoom)").unwrap()',
    'generate_utility_css("tab-4").unwrap()',
    'generate_utility_css("tab-(--dx-tab-size)").unwrap()',
  ]) {
    assert.match(utility, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    'class_name: "scheme-light-dark"',
    'class_name: "scrollbar-thin"',
    'class_name: "scrollbar-gutter-both"',
    'class_name: "scrollbar-thumb-red-500"',
    'class_name: "zoom-125"',
    'class_name: "tab-4"',
    'area: "tailwind-v4.3-interactivity"',
  ]) {
    assert.match(parity, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(matrix, /Tailwind v4\.3 interactivity additions now have targeted guards/i);
  assert.match(`${dx}\n${todo}\n${changelog}`, /Tailwind v4\.3 interactivity additions/i);
});

test("dx-style text flow and line clamp utilities have targeted Rust guards", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn line_clamp_utility(class_name: &str) -> Option<String>",
    "text_flow_and_line_clamp_utilities_generate_css",
    'generate_utility_css("line-clamp-3").unwrap()',
    'generate_utility_css("line-clamp-none").unwrap()',
    'generate_utility_css("line-clamp-[7]").unwrap()',
    'generate_utility_css("hyphens-auto").unwrap()',
    'generate_utility_css("text-balance").unwrap()',
    'generate_utility_css("text-clip").unwrap()',
  ]) {
    assert.match(utility, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(matrix, /text flow and line-clamp utilities now have targeted guards/);
});

test("dx style check blocks legacy Tailwind and PostCSS config files", () => {
  const tools = readDxStyleTools();
  const dx = readRequiredFile("DX.md");

  for (const marker of [
    "const LEGACY_CSS_TOOLING_CONFIG_FILES",
    "collect_legacy_css_tooling_findings",
    "legacy_css_tooling_findings",
    "dx-style-replaces-postcss-and-tailwind-config",
    "postcss.config.js",
    "postcss.config.ts",
    "tailwind.config.js",
    "tailwind.config.ts",
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(dx, /legacy Tailwind\/PostCSS config files/);
});

test("dx style check blocks legacy Tailwind and PostCSS package dependencies", () => {
  const tools = readDxStyleTools();
  const dx = readRequiredFile("DX.md");

  for (const marker of [
    "const LEGACY_CSS_TOOLING_PACKAGES",
    "collect_legacy_css_dependency_findings",
    "legacy_css_dependency_findings",
    "dx-style-replaces-tailwind-and-postcss-dependencies",
    '"tailwindcss"',
    '"postcss"',
    '"autoprefixer"',
    '"@tailwindcss/postcss"',
    '"dependencies"',
    '"devDependencies"',
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(dx, /legacy Tailwind\/PostCSS package dependencies/);
});

test("dx style check blocks legacy Tailwind and PostCSS lockfile entries", () => {
  const tools = readDxStyleTools();
  const dx = readRequiredFile("DX.md");

  for (const marker of [
    "const LEGACY_CSS_TOOLING_LOCKFILES",
    "collect_legacy_css_lockfile_findings",
    "legacy_css_lockfile_findings",
    "dx-style-replaces-tailwind-and-postcss-lockfile-entries",
    "package-lock.json",
    "pnpm-lock.yaml",
    "yarn.lock",
    "bun.lock",
    "bun.lockb",
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(dx, /legacy Tailwind\/PostCSS lockfile entries/);
});

test("dx-style important modifiers are implemented with targeted Rust guards", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");

  for (const marker of [
    "fn split_important_modifier(class_name: &str) -> (&str, bool)",
    "fn mark_declarations_important(input: &str) -> String",
    "important_modifier_applies_to_plain_utility",
    "important_modifier_applies_after_variant_prefix",
    "important_modifier_preserves_child_encoded_css",
    'let (class_name, important) = split_important_modifier(class_name);',
    'let important_css = if important {',
  ]) {
    assert.match(engine, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("!p-4"\)[\s\S]*padding: calc\(var\(--spacing\) \* 4\) !important;/);
  assert.match(engine, /css_for_class\("hover:!bg-blue-500"\)[\s\S]*background-color: rgb\(59 130 246\) !important;/);
  assert.match(engine, /css_for_class\("!space-x-4"\)[\s\S]*margin-inline-end: calc\(calc\(var\(--spacing\) \* 4\) \* calc\(1 - var\(--tw-space-x-reverse\)\)\) !important;/);
});

test("dx-style data and aria variants are implemented with targeted Rust guards", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn data_attribute_selector(part: &str) -> Option<String>",
    "fn aria_attribute_selector(part: &str) -> Option<String>",
    "data_variant_maps_to_attribute_selector",
    "aria_variant_maps_to_true_attribute_selector",
    "arbitrary_data_variant_maps_to_attribute_value_selector",
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("data-open:bg-blue-500"\)[\s\S]*\[data-open\]/);
  assert.match(engine, /css_for_class\("aria-expanded:text-slate-900"\)[\s\S]*\[aria-expanded=\\?"true\\?"\]/);
  assert.match(engine, /css_for_class\("data-\[state=open\]:opacity-100"\)[\s\S]*\[data-state=\\?"open\\?"\]/);
  assert.match(matrix, /data-\* and aria-\* attribute variants now have targeted guards/);
});

test("dx-style named group and peer variants are implemented with targeted Rust guards", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn named_group_peer_selector(part: &str) -> Option<String>",
    "fn named_variant_fragment(name: &str) -> Option<String>",
    "named_group_hover_variant_maps_to_named_group_selector",
    "named_group_focus_variant_maps_to_named_group_selector",
    "named_peer_checked_variant_maps_to_named_peer_selector",
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("group-hover\/card:bg-blue-500"\)[\s\S]*:is\(:where\(\.group\\\\\/card\):hover \*\)/);
  assert.match(engine, /css_for_class\("group-focus\/nav:text-slate-900"\)[\s\S]*:is\(:where\(\.group\\\\\/nav\):focus \*\)/);
  assert.match(engine, /css_for_class\("peer-checked\/published:opacity-100"\)[\s\S]*:is\(:where\(\.peer\\\\\/published\):checked ~ \*\)/);
  assert.match(matrix, /named group\/peer variants now have targeted guards/);
});

test("dx-style arbitrary group and peer variants are implemented with targeted Rust guards", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn arbitrary_group_peer_selector(part: &str) -> Option<String>",
    "fn sanitize_arbitrary_variant_selector(raw: &str) -> Option<String>",
    "arbitrary_group_selector_variant_maps_to_group_selector",
    "arbitrary_peer_selector_variant_maps_to_peer_selector",
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("group-\[\.is-open\]:bg-blue-500"\)[\s\S]*:is\(:where\(\.group\):is\(\.is-open\) \*\)/);
  assert.match(engine, /css_for_class\("peer-\[\.is-dirty\]:opacity-100"\)[\s\S]*:is\(:where\(\.peer\):is\(\.is-dirty\) ~ \*\)/);
  assert.match(matrix, /arbitrary group\/peer selector variants now have targeted guards/);
});

test("dx-style named arbitrary group and peer variants are implemented with targeted Rust guards", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn arbitrary_variant_selector_and_name(raw: &str) -> Option<(&str, Option<&str>)>",
    "named_arbitrary_group_selector_variant_maps_to_named_group_selector",
    "named_arbitrary_peer_selector_variant_maps_to_named_peer_selector",
    'css_for_class("group-[.is-open]/card:bg-blue-500")',
    'css_for_class("peer-[.is-dirty]/published:opacity-100")',
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("group-\[\.is-open\]\/card:bg-blue-500"\)[\s\S]*:is\(:where\(\.group\\\\\/card\):is\(\.is-open\) \*\)/);
  assert.match(engine, /css_for_class\("peer-\[\.is-dirty\]\/published:opacity-100"\)[\s\S]*:is\(:where\(\.peer\\\\\/published\):is\(\.is-dirty\) ~ \*\)/);
  assert.match(matrix, /named arbitrary group\/peer selector variants now have targeted guards/);
});

test("dx-style has selector variants are implemented with targeted Rust guards", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn has_variant_selector(part: &str) -> Option<String>",
    "fn has_condition_selector_and_name(raw: &str) -> Option<(String, Option<&str>)>",
    "has_checked_variant_maps_to_has_pseudo_selector",
    "arbitrary_has_variant_maps_to_has_selector",
    "named_group_has_variant_maps_to_named_group_selector",
    "named_peer_has_variant_maps_to_named_peer_selector",
    'css_for_class("has-checked:bg-blue-500")',
    'css_for_class("has-[img]:p-4")',
    'css_for_class("group-has-[a]/card:text-slate-900")',
    'css_for_class("peer-has-[input:checked]/published:opacity-100")',
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("has-checked:bg-blue-500"\)[\s\S]*:has\(\*:checked\)/);
  assert.match(engine, /css_for_class\("has-\[img\]:p-4"\)[\s\S]*:has\(img\)/);
  assert.match(engine, /css_for_class\("group-has-\[a\]\/card:text-slate-900"\)[\s\S]*\.group\\\\\/card\):has\(a\)/);
  assert.match(engine, /css_for_class\("peer-has-\[input:checked\]\/published:opacity-100"\)[\s\S]*\.peer\\\\\/published\):has\(input:checked\) ~ \*/);
  assert.match(matrix, /has selector variants now have targeted guards/);
});

test("dx-style not selector variants are implemented with targeted Rust guards", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn not_variant_selector(part: &str) -> Option<String>",
    "fn not_condition_selector(raw: &str) -> Option<String>",
    "not_hover_variant_maps_to_not_pseudo_selector",
    "arbitrary_not_variant_maps_to_not_selector",
    "named_group_not_variant_maps_to_named_group_selector",
    "named_peer_not_variant_maps_to_named_peer_selector",
    'css_for_class("not-hover:bg-blue-500")',
    'css_for_class("not-[.is-open]:p-4")',
    'css_for_class("group-not-hover/card:text-slate-900")',
    'css_for_class("peer-not-checked/published:opacity-100")',
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("not-hover:bg-blue-500"\)[\s\S]*:not\(\*:hover\)/);
  assert.match(engine, /css_for_class\("not-\[\.is-open\]:p-4"\)[\s\S]*:not\(\.is-open\)/);
  assert.match(engine, /css_for_class\("group-not-hover\/card:text-slate-900"\)[\s\S]*\.group\\\\\/card\):not\(\*:hover\)/);
  assert.match(engine, /css_for_class\("peer-not-checked\/published:opacity-100"\)[\s\S]*\.peer\\\\\/published\):not\(\*:checked\) ~ \*/);
  assert.match(matrix, /not selector variants now have targeted guards/);
});

test("dx-style safe generic arbitrary selector variants are implemented with targeted Rust guards", () => {
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn arbitrary_selector_variant(part: &str) -> Option<String>",
    "generic_arbitrary_child_selector_variant_maps_selector",
    "generic_arbitrary_state_selector_variant_maps_selector",
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("\[&>svg\]:bg-blue-500"\)[\s\S]*>svg/);
  assert.match(engine, /css_for_class\("\[&:nth-child\(3\)\]:opacity-100"\)[\s\S]*:nth-child\(3\)/);
  assert.match(matrix, /safe generic arbitrary selector variants now have targeted guards/);
});

test("dx-style arbitrary descendant and sibling selector variants are implemented with targeted Rust guards", () => {
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn decode_arbitrary_variant_selector(raw: &str) -> String",
    "arbitrary_descendant_variant_decodes_underscore_to_space",
    "arbitrary_adjacent_sibling_variant_maps_selector",
    "arbitrary_general_sibling_variant_maps_selector",
    "arbitrary_variant_preserves_escaped_underscore",
    'css_for_class("[&_p]:mt-4")',
    'css_for_class("[&+*]:mt-4")',
    'css_for_class("[&~*]:opacity-100")',
    'css_for_class("[&_.file\\\\_name]:mt-4")',
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("\[&_p\]:mt-4"\)[\s\S]*" p"/);
  assert.match(engine, /css_for_class\("\[&\+\*\]:mt-4"\)[\s\S]*"\+\*"/);
  assert.match(engine, /css_for_class\("\[&~\*\]:opacity-100"\)[\s\S]*"~\*"/);
  assert.match(engine, /css_for_class\("\[&_\.file\\\\_name\]:mt-4"\)[\s\S]*" \.file_name"/);
  assert.match(matrix, /arbitrary descendant and sibling variants now have targeted guards/);
});

test("dx-style arbitrary at-rule variants are implemented with targeted Rust guards", () => {
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const integration = readRequiredFile("related-crates/style/tests/arbitrary_variant_css.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn arbitrary_at_rule_variant(part: &str) -> Option<String>",
    "fn arbitrary_at_rule_selector_variant(part: &str) -> Option<(String, String)>",
    "arbitrary_media_at_rule_variant_maps_to_media_query",
    "arbitrary_supports_at_rule_variant_maps_to_supports_query",
    "arbitrary_container_at_rule_variant_maps_to_container_query",
    "arbitrary_starting_style_at_rule_variant_maps_to_starting_style",
    "arbitrary_layer_at_rule_variant_maps_to_cascade_layer",
    "safe_unknown_arbitrary_at_rule_variant_generates_wrapped_css",
    "arbitrary_tailwind_runtime_directive_variants_fail_closed",
    '"@media"',
    '"@supports"',
    '"@container"',
    '"@starting-style"',
    '"@layer "',
    'css_for_class("[@media_(any-hover:hover)]:opacity-100")',
    'css_for_class("[@supports_(display:grid)]:grid")',
    'css_for_class("[@container_main_(min-width:_32rem)]:p-4")',
    'css_for_class("[@starting-style]:opacity-0")',
    'css_for_class("[@layer_components]:p-4")',
    'css_for_class("[@media_(any-hover:hover){&:hover}]:opacity-100")',
    'css_for_class("[@unknown_rule]:p-4")',
  ]) {
    assert.match(`${engine}\n${states}\n${integration}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("\[@media_\(any-hover:hover\)\]:opacity-100"\)[\s\S]*@media \(any-hover:hover\)/);
  assert.match(integration, /css_for_class\("\[\@media_\(any-hover:hover\)\{&:hover\}\]:opacity-100"\)[\s\S]*@media \(any-hover:hover\)[\s\S]*:hover/);
  assert.match(engine, /css_for_class\("\[@supports_\(display:grid\)\]:grid"\)[\s\S]*@supports \(display:grid\)/);
  assert.match(engine, /css_for_class\("\[@container_main_\(min-width:_32rem\)\]:p-4"\)[\s\S]*@container main \(min-width: 32rem\)/);
  assert.match(engine, /css_for_class\("\[@starting-style\]:opacity-0"\)[\s\S]*@starting-style/);
  assert.match(engine, /css_for_class\("\[@layer_components\]:p-4"\)[\s\S]*@layer components/);
  assert.match(engine, /css_for_class\("\[\@unknown_rule\]:p-4"\)[\s\S]*@unknown rule/);
  assert.match(matrix, /arbitrary at-rule variants now have targeted guards/);
  assert.match(matrix, /nested-selector arbitrary at-rule variants/);
  assert.match(matrix, /`@starting-style`, `@layer`, nested-selector arbitrary at-rule variants/);
  assert.match(matrix, /safe unknown at-rule variants emit normal CSS/);
});

test("dx-style child selector variants are implemented with targeted Rust guards", () => {
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn child_selector_variant(part: &str) -> Option<&'static str>",
    "direct_child_variant_maps_to_child_selector",
    "descendant_child_variant_maps_to_descendant_selector",
    'css_for_class("*:p-4")',
    'css_for_class("**:text-slate-900")',
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("\*:p-4"\)[\s\S]*> \*/);
  assert.match(engine, /css_for_class\("\*\*:text-slate-900"\)[\s\S]* \*/);
  assert.match(matrix, /child selector variants now have targeted guards/);
});

test("dx-style in ancestor variants are implemented with targeted Rust guards", () => {
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn in_condition_selector(raw: &str) -> Option<String>",
    "fn in_variant_selector(part: &str) -> Option<String>",
    "in_hover_variant_maps_to_ancestor_state_selector",
    "arbitrary_in_variant_maps_to_ancestor_selector",
    'css_for_class("in-hover:bg-blue-500")',
    'css_for_class("in-[.is-open]:p-4")',
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("in-hover:bg-blue-500"\)[\s\S]*:where\(\*:hover\)/);
  assert.match(engine, /css_for_class\("in-\[\.is-open\]:p-4"\)[\s\S]*:where\(\.is-open\)/);
  assert.match(matrix, /in ancestor variants now have targeted guards/);
});

test("dx-style has/not/in state aliases cover common Tailwind selectors", () => {
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    '"even" => Some(":nth-child(even)")',
    '"visited" => Some(":visited")',
    '"target" => Some(":target")',
    '"autofill" => Some(":autofill")',
    '"read-only" => Some(":read-only")',
    '"read-write" => Some(":read-write")',
    '"indeterminate" => Some(":indeterminate")',
    '"default" => Some(":default")',
    '"in-range" => Some(":in-range")',
    '"out-of-range" => Some(":out-of-range")',
    '("target", ":target")',
    '("read-only", ":read-only")',
    '("indeterminate", ":indeterminate")',
    "empty_engine_supports_common_form_and_location_state_variants",
    "has_even_variant_maps_to_even_selector",
    "not_visited_variant_maps_to_visited_selector",
    "in_read_only_variant_maps_to_read_only_ancestor_selector",
    'css_for_class("target:p-4")',
    'css_for_class("read-only:bg-blue-500")',
    'css_for_class("indeterminate:opacity-100")',
    'css_for_class("has-even:bg-blue-500")',
    'css_for_class("not-visited:text-slate-900")',
    'css_for_class("in-read-only:p-4")',
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("target:p-4"\)[\s\S]*:target/);
  assert.match(engine, /css_for_class\("read-only:bg-blue-500"\)[\s\S]*:read-only/);
  assert.match(engine, /css_for_class\("indeterminate:opacity-100"\)[\s\S]*:indeterminate/);
  assert.match(engine, /css_for_class\("has-even:bg-blue-500"\)[\s\S]*:has\(\*:nth-child\(even\)\)/);
  assert.match(engine, /css_for_class\("not-visited:text-slate-900"\)[\s\S]*:not\(\*:visited\)/);
  assert.match(engine, /css_for_class\("in-read-only:p-4"\)[\s\S]*:where\(\*:read-only\)/);
  assert.match(matrix, /common direct and has\/not\/in state aliases now have targeted guards/);
});

test("dx-style nth structural variants are implemented with targeted Rust guards", () => {
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn sanitize_nth_expression(raw: &str) -> Option<String>",
    "fn structural_nth_variant(part: &str) -> Option<String>",
    "numeric_nth_variant_maps_to_nth_child_selector",
    "arbitrary_nth_variant_maps_to_nth_child_selector",
    "nth_last_variant_maps_to_nth_last_child_selector",
    "arbitrary_nth_of_type_variant_maps_to_nth_of_type_selector",
    "nth_last_of_type_variant_maps_to_nth_last_of_type_selector",
    'css_for_class("nth-3:bg-blue-500")',
    'css_for_class("nth-[2n+1]:p-4")',
    'css_for_class("nth-last-2:bg-blue-500")',
    'css_for_class("nth-of-type-[3n+1]:p-4")',
    'css_for_class("nth-last-of-type-2:opacity-100")',
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("nth-3:bg-blue-500"\)[\s\S]*:nth-child\(3\)/);
  assert.match(engine, /css_for_class\("nth-\[2n\+1\]:p-4"\)[\s\S]*:nth-child\(2n\+1\)/);
  assert.match(engine, /css_for_class\("nth-last-2:bg-blue-500"\)[\s\S]*:nth-last-child\(2\)/);
  assert.match(engine, /css_for_class\("nth-of-type-\[3n\+1\]:p-4"\)[\s\S]*:nth-of-type\(3n\+1\)/);
  assert.match(engine, /css_for_class\("nth-last-of-type-2:opacity-100"\)[\s\S]*:nth-last-of-type\(2\)/);
  assert.match(matrix, /nth structural variants now have targeted guards/);
  assert.match(matrix, /nth-last structural variants now have targeted guards/);
});

test("dx-style structural pseudo-class variants are implemented with targeted Rust guards", () => {
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    '("only", ":only-child")',
    '("first-of-type", ":first-of-type")',
    '("last-of-type", ":last-of-type")',
    '("only-of-type", ":only-of-type")',
    '"only" => Some(":only-child")',
    '"first-of-type" => Some(":first-of-type")',
    "only_child_variant_maps_to_only_child_selector",
    "first_of_type_variant_maps_to_first_of_type_selector",
    "last_of_type_variant_maps_to_last_of_type_selector",
    "only_of_type_variant_maps_to_only_of_type_selector",
    'css_for_class("only:bg-blue-500")',
    'css_for_class("first-of-type:p-4")',
    'css_for_class("last-of-type:text-slate-900")',
    'css_for_class("only-of-type:opacity-100")',
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("only:bg-blue-500"\)[\s\S]*:only-child/);
  assert.match(engine, /css_for_class\("first-of-type:p-4"\)[\s\S]*:first-of-type/);
  assert.match(engine, /css_for_class\("last-of-type:text-slate-900"\)[\s\S]*:last-of-type/);
  assert.match(engine, /css_for_class\("only-of-type:opacity-100"\)[\s\S]*:only-of-type/);
  assert.match(matrix, /structural pseudo-class variants now have targeted guards/);
});

test("dx-style pseudo-element variants are implemented with targeted Rust guards", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    '("before", "::before")',
    '("after", "::after")',
    '("placeholder", "::placeholder")',
    '"file"',
    '"&::file-selector-button"',
    '("marker", "&::marker, & *::marker")',
    '("selection", "&::selection, & *::selection")',
    '("first-letter", "::first-letter")',
    '("first-line", "::first-line")',
    "before_variant_maps_to_before_pseudo_element",
    "after_variant_maps_to_after_pseudo_element",
    "placeholder_variant_maps_to_placeholder_pseudo_element",
    "file_variant_maps_to_file_selector_button_pseudo_element",
    "marker_variant_maps_to_marker_pseudo_element",
    "selection_variant_maps_to_selection_pseudo_element",
    "first_letter_variant_maps_to_first_letter_pseudo_element",
    "first_line_variant_maps_to_first_line_pseudo_element",
    'css_for_class("before:bg-blue-500")',
    'css_for_class("file:p-4")',
  ]) {
    assert.match(engine, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("before:bg-blue-500"\)[\s\S]*::before/);
  assert.match(engine, /css_for_class\("after:opacity-100"\)[\s\S]*::after/);
  assert.match(engine, /css_for_class\("placeholder:text-slate-900"\)[\s\S]*::placeholder/);
  assert.match(engine, /css_for_class\("file:p-4"\)[\s\S]*::file-selector-button/);
  assert.match(engine, /css_for_class\("file:p-4"\)[\s\S]*!css\.contains\("::-webkit-file-upload-button"\)/);
  assert.match(matrix, /pseudo-element variants now have targeted guards/);
});

test("dx-style content utilities are implemented with targeted Rust guards", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn content_utility(class_name: &str) -> Option<String>",
    "fn content_value(raw_value: &str) -> Option<String>",
    "fn decode_content_arbitrary_value(value: &str) -> String",
    "generate_utility_css(\"content-none\")",
    "generate_utility_css(\"content-['hello_world']\")",
    "generate_utility_css(\"content-[attr(data-label)]\")",
    "generate_utility_css(\"content-(--dx-label)\")",
    "content_utilities_generate_css",
    "before_content_variant_maps_to_before_pseudo_element",
    "after_content_variable_variant_maps_to_after_pseudo_element",
    'css_for_class("before:content-[\'New\']")',
    'css_for_class("after:content-(--dx-suffix)")',
  ]) {
    assert.match(`${utility}\n${engine}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("before:content-\['New'\]"\)[\s\S]*::before[\s\S]*content: 'New';/);
  assert.match(engine, /css_for_class\("after:content-\(--dx-suffix\)"\)[\s\S]*::after[\s\S]*content: var\(--dx-suffix\);/);
  assert.match(matrix, /content utilities now have targeted guards/);
});

test("dx-style text-shadow and field-sizing utilities generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn text_shadow_value(raw_value: &str, alpha: Option<&TextShadowAlpha>) -> Option<String>",
    'generate_utility_css("text-shadow-sm")',
    'generate_utility_css("text-shadow-none")',
    'generate_utility_css("text-shadow-(--dx-copy-shadow)")',
    'generate_utility_css("field-sizing-content")',
    'generate_utility_css("field-sizing-fixed")',
    'class_name: "text-shadow-sm"',
    'class_name: "field-sizing-content"',
    "dx-style emits normal CSS for text-shadow utility classes",
    "dx-style emits normal CSS for field-sizing utility classes",
  ]) {
    assert.match(`${utility}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(utility, /"field-sizing-content"\s*=>\s*Some\("field-sizing: content"\)/);
  assert.match(utility, /"field-sizing-fixed"\s*=>\s*Some\("field-sizing: fixed"\)/);
  assert.match(utility, /"sm"\s*=>[\s\S]*text-shadow/);
  assert.match(parity, /class_name: "text-shadow-sm"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "field-sizing-content"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /assert!\(\s*unsupported\.is_empty\(\),\s*"\{unsupported:\?\}"\s*\);/);
  assert.match(matrix, /text-shadow[\s\S]*targeted guards/);
  assert.match(matrix, /field-sizing utility classes now have targeted guards/);
});

test("dx-style typography plugin baseline generates prose CSS", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const typography = readRequiredFile("related-crates/style/src/core/engine/typography.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "pub mod typography;",
    "typography::generate_typography_css(base_class)",
    'css_for_class("prose")',
    'css_for_class("md:prose")',
    'css_for_class("prose-invert")',
    'css_for_class("prose-lg")',
    "fn typography_nested_rule(selector_suffix: &str, declarations: &str) -> String",
    "pub fn generate_typography_css(class_name: &str) -> Option<String>",
    "fn prose_baseline() -> String",
    "fn prose_size(class_name: &str) -> Option<String>",
    "typography_prose_generates_nested_rules",
    "typography_prose_variants_generate_css",
    'class_name: "prose"',
    'class_name: "prose-a:text-blue-600"',
    'class_name: "mask-l-from-50%"',
  ]) {
    assert.match(`${engine}\n${typography}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /"BASE\|",[\s\S]*"STATE\|",[\s\S]*"CHILD\|",[\s\S]*"COND\|",[\s\S]*"DATA\|",[\s\S]*"RAW\|",[\s\S]*"NEST\|",/);
  assert.match(typography, /typography_nested_rule\(\s*" :where\(h1\)"/);
  assert.match(typography, /typography_nested_rule\(\s*" :where\(a\)"/);
  assert.match(typography, /typography_nested_rule\(\s*" :where\(pre\)"/);
  assert.match(engine, /css_for_class\("prose"\)[\s\S]*\.prose :where\(h1\)[\s\S]*font-size/);
  assert.match(engine, /css_for_class\("md:prose"\)[\s\S]*@media \(min-width: 768px\)/);
  assert.match(engine, /responsive_css\.contains\("\.md\\\\:prose :where\(p\)"\)/);
  assert.match(parity, /class_name: "prose"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "prose-a:text-blue-600"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "mask-linear-\[70deg,transparent_10%,black,transparent_80%\]"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "mask-radial-\[100%_100%\]"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "mask-alpha"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "mask-origin-content"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "mask-type-alpha"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "font-stretch-condensed"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "forced-color-adjust-auto"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "columns-3"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "break-before-page"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "box-decoration-clone"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-blend-multiply"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-origin-border"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-none"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /assert!\(\s*unsupported\.is_empty\(\),\s*"\{unsupported:\?\}"\s*\);/);
  assert.match(matrix, /baseline `prose` typography classes and prose element variants now have targeted guards/);
});

test("dx-style font-stretch utilities generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn font_stretch_value(raw_value: &str) -> Option<String>",
    "fn font_stretch_keyword(raw_value: &str) -> Option<&'static str>",
    "fn percentage_value(raw_value: &str) -> Option<String>",
    'generate_utility_css("font-stretch-condensed").unwrap()',
    'generate_utility_css("font-stretch-semi-expanded").unwrap()',
    'generate_utility_css("font-stretch-50%").unwrap()',
    'generate_utility_css("font-stretch-[62.5%]").unwrap()',
    'generate_utility_css("font-stretch-(--dx-font-stretch)").unwrap()',
    'css_for_class("font-stretch-condensed")',
    'css_for_class("font-stretch-50%")',
    'css_for_class("font-stretch-[62.5%]")',
    'css_for_class("font-stretch-(--dx-font-stretch)")',
    'class_name: "font-stretch-condensed"',
    'class_name: "forced-color-adjust-auto"',
    'class_name: "columns-3"',
    'class_name: "break-before-page"',
    'class_name: "box-decoration-clone"',
    'class_name: "bg-blend-multiply"',
    'class_name: "bg-origin-border"',
    'class_name: "bg-none"',
    '"forced-color-adjust-"',
  ]) {
    assert.match(`${utility}\n${engine}\n${support}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(utility, /class_name\.strip_prefix\("font-stretch-"\)/);
  assert.match(utility, /format!\("font-stretch: \{\}", value\)/);
  assert.match(engine, /css_for_class\("font-stretch-condensed"\)[\s\S]*font-stretch: condensed/);
  assert.match(engine, /css_for_class\("font-stretch-50%"\)[\s\S]*font-stretch: 50%/);
  assert.match(engine, /css_for_class\("font-stretch-\[62\.5%\]"\)[\s\S]*font-stretch: 62\.5%/);
  assert.match(parity, /class_name: "font-stretch-condensed"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "forced-color-adjust-auto"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "columns-3"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "break-before-page"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "box-decoration-clone"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-blend-multiply"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-origin-border"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-none"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /assert!\(\s*unsupported\.is_empty\(\),\s*"\{unsupported:\?\}"\s*\);/);
  assert.match(matrix, /font-stretch utilities now have targeted guards/);
});

test("dx-style numeric font variant utilities generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn numeric_font_variant_utility(class_name: &str) -> Option<String>",
    '"normal-nums" => Some("font-variant-numeric: normal".to_string())',
    '"ordinal" => Some(format!("--tw-ordinal: ordinal; {NUMERIC_VALUE}"))',
    '"tabular-nums" => Some(format!',
    '"diagonal-fractions" => Some(format!',
    'generate_utility_css("normal-nums").unwrap()',
    'generate_utility_css("ordinal")',
    'generate_utility_css("tabular-nums")',
    'generate_utility_css("diagonal-fractions")',
    'css_for_class("ordinal")',
    'css_for_class("tabular-nums")',
    'class_name: "tabular-nums"',
    'class_name: "ordinal"',
  ]) {
    assert.match(`${utility}\n${engine}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(utility, /font-variant-numeric: var\(--tw-ordinal,\) var\(--tw-slashed-zero,\) var\(--tw-numeric-figure,\) var\(--tw-numeric-spacing,\) var\(--tw-numeric-fraction,\)/);
  assert.match(engine, /css_for_class\("ordinal"\)[\s\S]*--tw-ordinal: ordinal[\s\S]*font-variant-numeric:/);
  assert.match(engine, /css_for_class\("tabular-nums"\)[\s\S]*--tw-numeric-spacing: tabular-nums[\s\S]*font-variant-numeric:/);
  assert.match(parity, /class_name: "tabular-nums"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "ordinal"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(matrix, /numeric font variant utilities now have targeted guards/);
});

test("dx-style forced-color-adjust utilities generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    '"forced-color-adjust-auto" => Some("forced-color-adjust: auto")',
    '"forced-color-adjust-none" => Some("forced-color-adjust: none")',
    'generate_utility_css("forced-color-adjust-auto").unwrap()',
    'generate_utility_css("forced-color-adjust-none").unwrap()',
    'css_for_class("forced-color-adjust-auto")',
    'css_for_class("forced-color-adjust-none")',
    'class_name: "forced-color-adjust-auto"',
    'class_name: "columns-3"',
    'class_name: "break-before-page"',
    'class_name: "box-decoration-clone"',
    'class_name: "bg-blend-multiply"',
    'class_name: "bg-origin-border"',
    'class_name: "bg-none"',
    '"bg-none".to_string()',
  ]) {
    assert.match(`${utility}\n${engine}\n${support}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("forced-color-adjust-auto"\)[\s\S]*forced-color-adjust: auto/);
  assert.match(engine, /css_for_class\("forced-color-adjust-none"\)[\s\S]*forced-color-adjust: none/);
  assert.match(parity, /class_name: "forced-color-adjust-auto"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "columns-3"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "break-before-page"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "box-decoration-clone"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-blend-multiply"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-origin-border"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-none"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /assert!\(\s*unsupported\.is_empty\(\),\s*"\{unsupported:\?\}"\s*\);/);
  assert.match(matrix, /forced-color-adjust utilities now have targeted guards/);
});

test("dx-style outline and ring edge utilities generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    '"outline-dashed" => Some("outline-style: dashed")',
    '"outline-hidden" => Some("outline-style: hidden")',
    'class_name.strip_prefix("outline-")',
    'format!("outline-width: {}", value)',
    '"ring-inset" => Some("--tw-ring-inset: inset")',
    'class_name.strip_prefix("ring-offset-")',
    'format!("--tw-ring-offset-width: {}", width)',
    'var(--tw-ring-inset,)',
    'generate_utility_css("outline-2").unwrap()',
    'generate_utility_css("outline-dashed").unwrap()',
    'generate_utility_css("ring-inset").unwrap()',
    'generate_utility_css("ring-offset-2").unwrap()',
    'css_for_class("outline-2")',
    'css_for_class("outline-dashed")',
    'css_for_class("ring-inset")',
    'css_for_class("ring-offset-2")',
    'class_name: "outline-2"',
    'class_name: "ring-inset"',
    'class_name: "ring-offset-2"',
  ]) {
    assert.match(`${utility}\n${engine}\n${support}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("outline-2"\)[\s\S]*outline-width: 2px/);
  assert.match(engine, /css_for_class\("outline-dashed"\)[\s\S]*outline-style: dashed/);
  assert.match(engine, /css_for_class\("ring-inset"\)[\s\S]*--tw-ring-inset: inset/);
  assert.match(engine, /css_for_class\("ring-offset-2"\)[\s\S]*--tw-ring-offset-width: 2px/);
  assert.match(engine, /css_for_class\("ring-2"\)[\s\S]*var\(--tw-ring-inset,\)/);
  assert.match(parity, /class_name: "outline-2"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "ring-inset"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "ring-offset-2"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /"outline"/);
  assert.match(support, /"ring"/);
  assert.match(matrix, /outline[\s\S]*ring[\s\S]*edge/);
});

test("dx-style touch-action edge utilities generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    '"touch-pan-left" => Some(',
    '"--tw-pan-x: pan-left; touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)"',
    '"touch-pan-right" => Some(',
    '"--tw-pan-x: pan-right; touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)"',
    '"touch-pan-up" => Some(',
    '"--tw-pan-y: pan-up; touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)"',
    '"touch-pan-down" => Some(',
    '"--tw-pan-y: pan-down; touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)"',
    '"touch-pinch-zoom" => Some(',
    '"--tw-pinch-zoom: pinch-zoom; touch-action: var(--tw-pan-x,) var(--tw-pan-y,) var(--tw-pinch-zoom,)"',
    'generate_utility_css("touch-pan-left").unwrap()',
    'generate_utility_css("touch-pinch-zoom").unwrap()',
    'css_for_class("touch-pan-left")',
    'css_for_class("touch-pinch-zoom")',
    'class_name: "touch-pan-left"',
    'class_name: "touch-pinch-zoom"',
  ]) {
    assert.match(`${utility}\n${engine}\n${support}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("touch-pan-left"\)[\s\S]*--tw-pan-x: pan-left/);
  assert.match(engine, /css_for_class\("touch-pinch-zoom"\)[\s\S]*--tw-pinch-zoom: pinch-zoom/);
  assert.match(parity, /class_name: "touch-pan-left"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "touch-pinch-zoom"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /"touch-"/);
  assert.match(matrix, /touch-action edge utilities now have targeted guards/);
});

test("dx-style token-aware classnames preserve scanner tokens and generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const parser = readRequiredFile("dx-www/src/parser/style.rs");
  const publicTools = readDxStyleTools();
  const theme = readRequiredFile("examples/template/styles/theme.css");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");
  const readme = readRequiredFile("related-crates/style/README.md");

  for (const marker of [
    "fn token_color_value(raw_value: &str) -> Option<String>",
    'format!("hsl(var(--{}))", token)',
    'generate_utility_css("bg-token(surface)").unwrap()',
    'generate_utility_css("text-token(foreground)").unwrap()',
    'generate_utility_css("border-token(border)").unwrap()',
    'generate_utility_css("ring-token(ring)").unwrap()',
    'css_for_class("bg-token(surface)")',
    'css_for_class("text-token(foreground)")',
    'css_for_class("border-token(border)")',
    'css_for_class("ring-token(ring)")',
    'class_name: "bg-token(surface)"',
    'class_name: "text-token(foreground)"',
    'class_name: "border-token(border)"',
    'class_name: "ring-token(ring)"',
    "extract_class_attribute_tokens_preserves_parenthesized_dx_token_classes",
    "static_class_token_preserves_parenthesized_dx_token_classes",
  ]) {
    assert.match(
      `${utility}\n${engine}\n${parity}\n${parser}\n${publicTools}`,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }

  assert.match(engine, /css_for_class\("bg-token\(surface\)"\)[\s\S]*background-color: hsl\(var\(--surface\)\)/);
  assert.match(engine, /css_for_class\("text-token\(foreground\)"\)[\s\S]*color: hsl\(var\(--foreground\)\)/);
  assert.match(engine, /css_for_class\("border-token\(border\)"\)[\s\S]*border-color: hsl\(var\(--border\)\)/);
  assert.match(engine, /css_for_class\("ring-token\(ring\)"\)[\s\S]*--tw-ring-color: hsl\(var\(--ring\)\)/);
  assert.match(parser, /bg-token\(surface\) text-token\(foreground\) border-token\(border\) ring-token\(ring\)/);
  assert.match(publicTools, /bg-token\(surface\) text-token\(foreground\) border-token\(border\) ring-token\(ring\)/);
  assert.match(theme, /--surface:/);
  assert.match(matrix, /token-aware classnames now have targeted guards/);
  assert.match(readme, /bg-token\(surface\)/);
});

test("dx-style grouped classnames expand before generation and report invalid groups", () => {
  const parser = readRequiredFile("dx-www/src/parser/style.rs");
  const publicTools = readDxStyleTools();
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");
  const readme = readRequiredFile("related-crates/style/README.md");

  for (const marker of [
    "pub fn expand_grouped_class_tokens(value: &str) -> Vec<String>",
    "fn push_grouped_class_token",
    "fn grouped_class_prefix",
    "dx-grouping-error:",
    "extract_class_attribute_tokens_expands_grouped_classnames",
    "extract_class_attribute_tokens_reports_invalid_grouped_classnames",
    "extract_class_tokens_expands_grouped_classnames",
    "grouped classname syntax is invalid",
    "hover:(bg-accent text-accent-foreground shadow-sm)",
    "hover:bg-accent",
    "hover:text-accent-foreground",
    "hover:shadow-sm",
    "md:grid",
    "md:grid-cols-2",
    "md:gap-4",
    "dark:hover:bg-card",
    "dark:hover:text-foreground",
    "group-hover:opacity-100",
    "group-hover:translate-y-0",
  ]) {
    assert.match(
      `${parser}\n${publicTools}\n${support}`,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"))
    );
  }

  assert.match(matrix, /grouped classnames now have targeted guards/);
  assert.match(readme, /hover:\(bg-accent text-accent-foreground shadow-sm\)/);
});

test("dx-style columns utilities generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn columns_utility(class_name: &str) -> Option<String>",
    "fn columns_value(raw_value: &str) -> Option<String>",
    "fn columns_width_value(raw_value: &str) -> Option<&'static str>",
    'generate_utility_css("columns-3").unwrap()',
    'generate_utility_css("columns-auto").unwrap()',
    'generate_utility_css("columns-lg").unwrap()',
    'generate_utility_css("columns-[14rem]").unwrap()',
    'generate_utility_css("columns-(--dx-column-width)").unwrap()',
    'css_for_class("columns-3")',
    'css_for_class("columns-auto")',
    'css_for_class("columns-lg")',
    'css_for_class("columns-[14rem]")',
    'css_for_class("columns-(--dx-column-width)")',
    'class_name: "columns-3"',
    'class_name: "break-before-page"',
    'class_name: "box-decoration-clone"',
    'class_name: "bg-blend-multiply"',
    'class_name: "bg-origin-border"',
    'class_name: "bg-none"',
    '"break-"',
    '"bg-none".to_string()',
  ]) {
    assert.match(`${utility}\n${engine}\n${support}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(utility, /columns_width_value\(raw_value\)[\s\S]*"lg"\s*=>\s*Some\("32rem"\)/);
  assert.match(engine, /css_for_class\("columns-3"\)[\s\S]*columns: 3/);
  assert.match(engine, /css_for_class\("columns-auto"\)[\s\S]*columns: auto/);
  assert.match(engine, /css_for_class\("columns-lg"\)[\s\S]*columns: 32rem/);
  assert.match(engine, /css_for_class\("columns-\[14rem\]"\)[\s\S]*columns: 14rem/);
  assert.match(parity, /class_name: "columns-3"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "break-before-page"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "box-decoration-clone"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-blend-multiply"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-origin-border"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-none"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /assert!\(\s*unsupported\.is_empty\(\),\s*"\{unsupported:\?\}"\s*\);/);
  assert.match(matrix, /columns utilities now have targeted guards/);
});

test("dx-style break utilities generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn break_utility(class_name: &str) -> Option<String>",
    "fn break_before_after_value(raw_value: &str) -> Option<&'static str>",
    "fn break_inside_value(raw_value: &str) -> Option<&'static str>",
    'generate_utility_css("break-before-page").unwrap()',
    'generate_utility_css("break-after-avoid-page").unwrap()',
    'generate_utility_css("break-inside-avoid-column").unwrap()',
    'css_for_class("break-before-page")',
    'css_for_class("break-after-avoid-page")',
    'css_for_class("break-inside-avoid-column")',
    'class_name: "break-before-page"',
    'class_name: "box-decoration-clone"',
    'class_name: "bg-blend-multiply"',
    'class_name: "bg-origin-border"',
    'class_name: "bg-none"',
    '"box-decoration-"',
    '"bg-none".to_string()',
  ]) {
    assert.match(`${utility}\n${engine}\n${support}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(utility, /strip_prefix\("break-before-"\)[\s\S]*format!\("break-before: \{value\}"\)/);
  assert.match(utility, /strip_prefix\("break-after-"\)[\s\S]*format!\("break-after: \{value\}"\)/);
  assert.match(utility, /strip_prefix\("break-inside-"\)[\s\S]*format!\("break-inside: \{value\}"\)/);
  assert.match(engine, /css_for_class\("break-before-page"\)[\s\S]*break-before: page/);
  assert.match(engine, /css_for_class\("break-after-avoid-page"\)[\s\S]*break-after: avoid-page/);
  assert.match(engine, /css_for_class\("break-inside-avoid-column"\)[\s\S]*break-inside: avoid-column/);
  assert.match(parity, /class_name: "break-before-page"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "box-decoration-clone"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-blend-multiply"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-origin-border"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-none"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /assert!\(\s*unsupported\.is_empty\(\),\s*"\{unsupported:\?\}"\s*\);/);
  assert.match(matrix, /break utilities now have targeted guards/);
});

test("dx-style box-decoration utilities generate prefixed normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    '"box-decoration-clone" =>',
    '"box-decoration-slice" =>',
    'generate_utility_css("box-decoration-clone").unwrap()',
    'generate_utility_css("box-decoration-slice").unwrap()',
    'css_for_class("box-decoration-clone")',
    'css_for_class("box-decoration-slice")',
    'class_name: "box-decoration-clone"',
    'class_name: "bg-blend-multiply"',
    'class_name: "bg-origin-border"',
    'class_name: "bg-none"',
    '"bg-none".to_string()',
  ]) {
    assert.match(`${utility}\n${engine}\n${support}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(utility, /-webkit-box-decoration-break: clone; box-decoration-break: clone/);
  assert.match(utility, /-webkit-box-decoration-break: slice; box-decoration-break: slice/);
  assert.match(engine, /css_for_class\("box-decoration-clone"\)[\s\S]*-webkit-box-decoration-break: clone[\s\S]*box-decoration-break: clone/);
  assert.match(engine, /css_for_class\("box-decoration-slice"\)[\s\S]*-webkit-box-decoration-break: slice[\s\S]*box-decoration-break: slice/);
  assert.match(parity, /class_name: "box-decoration-clone"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-blend-multiply"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-origin-border"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-none"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /assert!\(\s*unsupported\.is_empty\(\),\s*"\{unsupported:\?\}"\s*\);/);
  assert.match(matrix, /box-decoration-break utilities now have targeted guards/);
});

test("dx-style browser prefix utilities generate PostCSS-free compatibility CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");
  const readme = readRequiredFile("related-crates/style/README.md");

  for (const marker of [
    '"appearance-none" => Some("-webkit-appearance: none; appearance: none")',
    '"select-none" => Some("-webkit-user-select: none; user-select: none")',
    '"select-text" => Some("-webkit-user-select: text; user-select: text")',
    '"select-all" => Some("-webkit-user-select: all; user-select: all")',
    '"select-auto" => Some("-webkit-user-select: auto; user-select: auto")',
    '"backface-hidden" => {',
    "-webkit-backface-visibility: hidden; backface-visibility: hidden",
    '"backface-visible" => {',
    "-webkit-backface-visibility: visible; backface-visibility: visible",
    "page-break-inside: avoid; break-inside: avoid",
    'fn backdrop_filter_declaration(value: &str) -> String',
    'generate_utility_css("appearance-none").unwrap()',
    'generate_utility_css("select-none").unwrap()',
    'generate_utility_css("backface-hidden").unwrap()',
    'generate_utility_css("break-inside-avoid").unwrap()',
    'generate_utility_css("backdrop-filter-none").unwrap()',
    'generate_utility_css("backdrop-blur-md").unwrap()',
    'generate_utility_css("hyphens-auto").unwrap()',
    'class_name: "appearance-none"',
    'class_name: "select-none"',
    'class_name: "backface-hidden"',
    'class_name: "break-inside-avoid"',
    'class_name: "backdrop-blur-md"',
    'class_name: "hyphens-auto"',
    'class_name: "file:p-4"',
  ]) {
    assert.match(`${utility}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(utility, /-webkit-backdrop-filter: \{\}; backdrop-filter: \{\}/);
  assert.match(engine, /css_for_class\("appearance-none"\)[\s\S]*-webkit-appearance: none[\s\S]*appearance: none/);
  assert.match(engine, /css_for_class\("select-none"\)[\s\S]*-webkit-user-select: none[\s\S]*user-select: none/);
  assert.match(engine, /css_for_class\("backface-hidden"\)[\s\S]*-webkit-backface-visibility: hidden[\s\S]*backface-visibility: hidden/);
  assert.match(engine, /css_for_class\("break-inside-avoid"\)[\s\S]*page-break-inside: avoid[\s\S]*break-inside: avoid/);
  assert.match(engine, /css_for_class\("backdrop-blur-md"\)[\s\S]*-webkit-backdrop-filter:[\s\S]*backdrop-filter:/);
  assert.match(utility, /-webkit-hyphens: auto; hyphens: auto/);
  assert.match(parity, /class_name: "appearance-none"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "select-none"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "backface-hidden"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "break-inside-avoid"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "backdrop-blur-md"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "hyphens-auto"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "file:p-4"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(engine, /css_for_class\("file:p-4"\)[\s\S]*::file-selector-button[\s\S]*!css\.contains\("::-webkit-file-upload-button"\)/);
  assert.match(matrix, /browser-prefix utilities now have targeted guards/);
  assert.match(readme, /browser-prefix/i);
});

test("dx-style browser compat fixture records declaration-level Tailwind PostCSS parity", () => {
  const fixture = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-postcss-browser-compat.json"),
  );
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");
  const readme = readRequiredFile("related-crates/style/README.md");

  assert.equal(fixture.schema, "dx.style.tailwindPostcssBrowserCompatFixture");
  assert.equal(fixture.schemaVersion, 1);
  assert.equal(fixture.comparisonScope, "declaration-fragment-equality");
  assert.match(fixture.baseline, /tailwindcss-4\.3\.0/);
  assert.match(fixture.runPolicy, /no package install/i);
  assert.ok(Array.isArray(fixture.classes));
  assert.ok(fixture.classes.length >= 6);
  assert.ok(fixture.classes.some((entry) => entry.className === "backface-hidden"));
  assert.ok(fixture.classes.some((entry) => entry.className === "break-inside-avoid"));

  for (const entry of fixture.classes) {
    assert.deepEqual(
      entry.dxStyleRequiredDeclarations,
      entry.tailwindPostcssRequiredDeclarations,
      `${entry.className} must keep the same declaration-fragment contract`,
    );
    assert.match(parity, new RegExp(`class_name: "${entry.className.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}"`));

    for (const declaration of entry.dxStyleRequiredDeclarations) {
      assert.match(utility, new RegExp(declaration.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
    }
  }

  assert.match(matrix, /tailwind-postcss-browser-compat\.json/);
  assert.match(readme, /tailwind-postcss-browser-compat\.json/);
});

test("dx-style exposes browser compat fixture as a first-class source contract", () => {
  const browserCompat = readRequiredFile("related-crates/style/src/core/engine/browser_compat.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const core = readRequiredFile("related-crates/style/src/core/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");
  const readme = readRequiredFile("related-crates/style/README.md");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    "pub mod browser_compat;",
    "pub struct TailwindPostcssBrowserCompatContract",
    "pub const TAILWIND_POSTCSS_BROWSER_COMPAT_SCHEMA",
    "pub const TAILWIND_POSTCSS_BROWSER_COMPAT_FIXTURE_PATH",
    "pub const TAILWIND_POSTCSS_BROWSER_COMPAT_BASELINE",
    "pub const TAILWIND_POSTCSS_BROWSER_COMPAT_COMPARISON_SCOPE",
    "pub const TAILWIND_POSTCSS_BROWSER_COMPAT_CLASSES: &[&str]",
    "pub fn tailwind_postcss_browser_compat_contract() -> TailwindPostcssBrowserCompatContract",
    "tailwind-postcss-browser-compat.json",
    "appearance-none",
    "select-none",
    "backface-hidden",
    "break-inside-avoid",
    "backdrop-blur-md",
    "hyphens-auto",
  ]) {
    assert.match(
      `${browserCompat}\n${engine}`,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }

  assert.match(core, /pub use engine::browser_compat::\{/);
  assert.match(readme, /src\/core\/engine\/browser_compat\.rs/);
  assert.match(matrix, /browser_compat\.rs/);
  assert.match(dx, /browser-compat source contract/i);
  assert.match(todo, /browser-compat source contract/i);
  assert.match(changelog, /browser-compat source contract/i);
});

test("dx-style equal-output canary blocks unfair Tailwind speed claims", () => {
  const fixture = JSON.parse(
    readRequiredFile("related-crates/style/fixtures/tailwind-equal-output-canary.json"),
  );
  const equalOutput = readRequiredFile("related-crates/style/src/core/engine/equal_output.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const core = readRequiredFile("related-crates/style/src/core/mod.rs");
  const tools = readDxStyleTools();
  const receipts = readRequiredFile("core/src/ecosystem/dx_style_receipts.rs");
  const projectCheck = [
    readRequiredFile("core/src/ecosystem/project_check.rs"),
    readRequiredFile("core/src/ecosystem/project_check/readiness_parts/dx_style.rs"),
    readRequiredFile("core/src/ecosystem/project_check/readiness_parts/tests.rs"),
  ].join("\n");
  const checkPanel = [
    readRequiredFile("core/src/ecosystem/dx_check_receipt.rs"),
    readRequiredFile("core/src/ecosystem/dx_check_receipt/panel.rs"),
    readRequiredFile("core/src/ecosystem/dx_check_receipt/panel_parts/style_evidence.rs"),
  ].join("\n");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  assert.equal(fixture.schema, "dx.style.tailwindEqualOutputCanary");
  assert.equal(fixture.schemaVersion, 1);
  assert.equal(fixture.comparisonScope, "generated-css-declaration-equality");
  assert.match(fixture.baseline, /tailwindcss-4\.3\.0/);
  assert.equal(fixture.liveTailwindExecution, false);
  assert.equal(fixture.fullTailwindParity, false);
  assert.equal(fixture.fairSpeedBenchmark, false);
  assert.ok(Array.isArray(fixture.classes));
  assert.ok(fixture.classes.length >= 5);

  for (const entry of fixture.classes) {
    assert.deepEqual(
      entry.dxStyleRequiredDeclarations,
      entry.tailwindRequiredDeclarations,
      `${entry.className} must keep the same checked declaration baseline`,
    );
    assert.match(
      equalOutput,
      new RegExp(entry.className.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")),
    );
  }

  for (const marker of [
    "pub mod equal_output;",
    "pub const TAILWIND_EQUAL_OUTPUT_CANARY_SCHEMA",
    "pub const TAILWIND_EQUAL_OUTPUT_CANARY_FIXTURE_PATH",
    "pub struct TailwindEqualOutputCanaryContract",
    "pub fn tailwind_equal_output_canary_contract() -> TailwindEqualOutputCanaryContract",
    "live_tailwind_execution: false",
    "full_tailwind_parity: false",
    "fair_speed_benchmark: false",
    "tailwind-equal-output-canary.json",
  ]) {
    assert.match(`${equalOutput}\n${engine}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(core, /pub use engine::equal_output::\{/);

  for (const marker of [
    "fn dx_style_tailwind_equal_output_canary_contract() -> Value",
    "style::core::tailwind_equal_output_canary_contract()",
    '"tailwind_equal_output_canary_contract": dx_style_tailwind_equal_output_canary_contract()',
    '"live_tailwind_execution": contract.live_tailwind_execution',
    '"fair_speed_benchmark": contract.fair_speed_benchmark',
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "DX_STYLE_TAILWIND_EQUAL_OUTPUT_SCHEMA",
    "DxStyleTailwindEqualOutputSummary",
    "dx_style_tailwind_equal_output_summary",
    "tailwind_equal_output_canary_contract",
    "fair_speed_benchmark",
  ]) {
    assert.match(receipts, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "let tailwind_equal_output = dx_style_tailwind_equal_output_summary(root);",
    "dx_style_tailwind_equal_output_receipt_present",
    "dx_style_tailwind_equal_output_contract_present",
    "dx_style_tailwind_equal_output_equal_class_count",
    "dx_style_tailwind_equal_output_live_tailwind_execution",
    "dx-style-tailwind-equal-output-fair-speed-overclaim",
  ]) {
    assert.match(projectCheck, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "DX_STYLE_TAILWIND_EQUAL_OUTPUT_ROW_ID",
    "DX_STYLE_TAILWIND_EQUAL_OUTPUT_FIXTURE_PATH",
    "dx_style_tailwind_equal_output_panel_row",
    "dx-style:tailwind-equal-output",
  ]) {
    assert.match(checkPanel, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(dx, /equal-output canary/i);
  assert.match(todo, /equal-output canary/i);
  assert.match(changelog, /equal-output canary/i);
});

test("dx-www style receipts expose class-to-rule visual metadata for Zed and Studio", () => {
  const tools = readDxStyleTools();
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    "fn style_rule_metadata(",
    "fn class_source_file_index(",
    "fn css_rule_metadata(",
    "fn css_declarations(",
    "fn declaration_visual_property(",
    "fn css_token_references(",
    '"style_rule_metadata_count": style_rule_metadata.len()',
    '"style_rule_metadata": style_rule_metadata',
    '"schema": "dx.style.rule_metadata"',
    '"visual_properties": visual_properties',
    '"source_files": source_files',
    '"token_references": token_references',
    '"zed_studio_editable": true',
    "engine.css_for_class(class_name)",
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(dx, /style rule metadata/i);
  assert.match(todo, /style rule metadata/i);
  assert.match(changelog, /style rule metadata/i);
});

test("dx-www style receipts expose browser compat evidence to Check and Zed", () => {
  const tools = readDxStyleTools();
  const browserCompat = readRequiredFile("related-crates/style/src/core/engine/browser_compat.rs");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    "fn dx_style_browser_compat_receipt_contract() -> Value",
    "style::core::tailwind_postcss_browser_compat_contract()",
    "style::core::TAILWIND_POSTCSS_BROWSER_COMPAT_SCHEMA",
    "style::core::TAILWIND_POSTCSS_BROWSER_COMPAT_FIXTURE_PATH",
    "style::core::TAILWIND_POSTCSS_BROWSER_COMPAT_SELECTOR_CLASSES",
    "browser_compat_receipt_contract",
    '"selector_classes": contract.selector_classes',
    '"selector_class_count": contract.selector_classes.len()',
    '"selector_comparison_scope": "selector-fragment-presence"',
    '"receipt_source": "related-crates/style/src/core/engine/browser_compat.rs"',
    '"generated_by": "style::core::tailwind_postcss_browser_compat_contract"',
    '"full_autoprefixer_parity": false',
    '"full_tailwind_postcss_output_parity": false',
    '"tool_consumers": ["dx-style", "dx-check", "Forge", "Zed", "Friday"]',
  ]) {
    assert.match(`${tools}\n${browserCompat}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(tools, /"browser_compat_receipt_contract": dx_style_browser_compat_receipt_contract\(\)/g);
  assert.match(dx, /browser-compat receipt visibility/i);
  assert.match(todo, /browser-compat receipt visibility/i);
  assert.match(changelog, /browser-compat receipt visibility/i);
});

test("dx-check consumes dx-style browser compat receipt metrics", () => {
  const projectCheck = [
    readRequiredFile("core/src/ecosystem/project_check.rs"),
    readRequiredFile("core/src/ecosystem/project_check/readiness_parts/dx_style.rs"),
    readRequiredFile("core/src/ecosystem/project_check/readiness_parts/tests.rs"),
  ].join("\n");
  const receipts = readRequiredFile("core/src/ecosystem/dx_style_receipts.rs");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    "DxStyleBrowserCompatSummary",
    "DX_STYLE_BROWSER_COMPAT_SCHEMA",
    "dx_style_browser_compat_summary",
    "browser_compat_receipt_contract",
    "dx.style.tailwindPostcssBrowserCompatFixture",
  ]) {
    assert.match(`${projectCheck}\n${receipts}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "let browser_compat = dx_style_browser_compat_summary(root);",
    "dx_style_browser_compat_receipt_present",
    "dx_style_browser_compat_contract_present",
    "dx_style_browser_compat_schema_supported",
    "dx_style_browser_compat_class_count",
    "dx_style_browser_compat_selector_class_count",
    "dx_style_browser_compat_full_autoprefixer_parity",
    "dx_style_browser_compat_full_tailwind_postcss_output_parity",
    "dx-style-browser-compat-contract-missing",
    "dx-style-browser-compat-schema-unsupported",
    "dx-style-browser-compat-autoprefixer-parity-overclaim",
    "dx-style-browser-compat-tailwind-postcss-output-overclaim",
    "dx_style_section_summarizes_browser_compat_receipt",
  ]) {
    assert.match(projectCheck, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(dx, /browser-compat dx-check metrics/i);
  assert.match(todo, /browser-compat dx-check metrics/i);
  assert.match(changelog, /browser-compat dx-check metrics/i);
});

test("dx-style blend mode utilities generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn blend_utility(class_name: &str) -> Option<String>",
    "fn blend_mode_value(raw_value: &str, allow_plus_modes: bool) -> Option<String>",
    'generate_utility_css("bg-blend-multiply").unwrap()',
    'generate_utility_css("mix-blend-plus-lighter").unwrap()',
    'generate_utility_css("mix-blend-[screen]").unwrap()',
    'generate_utility_css("bg-blend-(--dx-bg-blend)").unwrap()',
    'css_for_class("bg-blend-multiply")',
    'css_for_class("mix-blend-plus-lighter")',
    'css_for_class("mix-blend-[screen]")',
    'css_for_class("bg-blend-(--dx-bg-blend)")',
    'class_name: "bg-blend-multiply"',
    'class_name: "bg-origin-border"',
    'class_name: "bg-none"',
    '"bg-none".to_string()',
  ]) {
    assert.match(`${utility}\n${engine}\n${support}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(utility, /strip_prefix\("bg-blend-"\)[\s\S]*background-blend-mode/);
  assert.match(utility, /strip_prefix\("mix-blend-"\)[\s\S]*mix-blend-mode/);
  assert.match(engine, /css_for_class\("bg-blend-multiply"\)[\s\S]*background-blend-mode: multiply/);
  assert.match(engine, /css_for_class\("mix-blend-plus-lighter"\)[\s\S]*mix-blend-mode: plus-lighter/);
  assert.match(engine, /css_for_class\("mix-blend-\[screen\]"\)[\s\S]*mix-blend-mode: screen/);
  assert.match(parity, /class_name: "bg-blend-multiply"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-origin-border"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-none"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /assert!\(\s*unsupported\.is_empty\(\),\s*"\{unsupported:\?\}"\s*\);/);
  assert.match(matrix, /blend mode utilities now have targeted guards/);
});

test("dx-style background-origin utilities generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    '"bg-origin-border" => Some("background-origin: border-box")',
    '"bg-origin-padding" => Some("background-origin: padding-box")',
    '"bg-origin-content" => Some("background-origin: content-box")',
    'generate_utility_css("bg-origin-border").unwrap()',
    'generate_utility_css("bg-origin-padding").unwrap()',
    'generate_utility_css("bg-origin-content").unwrap()',
    'css_for_class("bg-origin-border")',
    'css_for_class("bg-origin-padding")',
    'css_for_class("bg-origin-content")',
    'class_name: "bg-origin-border"',
    'class_name: "bg-none"',
    '"bg-none".to_string()',
  ]) {
    assert.match(`${utility}\n${engine}\n${support}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("bg-origin-border"\)[\s\S]*background-origin: border-box/);
  assert.match(engine, /css_for_class\("bg-origin-padding"\)[\s\S]*background-origin: padding-box/);
  assert.match(engine, /css_for_class\("bg-origin-content"\)[\s\S]*background-origin: content-box/);
  assert.match(parity, /class_name: "bg-origin-border"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-none"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /assert!\(\s*unsupported\.is_empty\(\),\s*"\{unsupported:\?\}"\s*\);/);
  assert.match(matrix, /background-origin utilities now have targeted guards/);
});

test("dx-style background image reset utilities generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    '"bg-none" => Some("background-image: none")',
    'generate_utility_css("bg-none").unwrap()',
    'css_for_class("bg-none")',
    'class_name: "bg-none"',
    'class_name: "bg-linear-to-r"',
    'class_name: "bg-radial"',
    'class_name: "bg-linear-45"',
    'class_name: "bg-size-(--dx-bg-size)"',
    'class_name: "[@starting-style]:opacity-0"',
    'class_name: "[@layer_components]:p-4"',
    '"bg-linear-to-r".to_string()',
    '"bg-radial".to_string()',
    '"[@starting-style]:opacity-0".to_string()',
    '"[@unknown_rule]:p-4".to_string()',
  ]) {
    assert.match(`${utility}\n${engine}\n${support}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("bg-none"\)[\s\S]*background-image: none/);
  assert.match(parity, /class_name: "bg-none"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-linear-to-r"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-radial"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-linear-45"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-size-\(--dx-bg-size\)"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "\[\@starting-style\]:opacity-0"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "\[\@layer_components\]:p-4"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /assert!\(\s*unsupported\.is_empty\(\),\s*"\{unsupported:\?\}"\s*\);/);
  assert.match(matrix, /background image reset utilities now have targeted guards/);
});

test("dx-style Tailwind v4 linear background gradient aliases generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn linear_gradient_direction(raw_value: &str) -> Option<&'static str>",
    'class_name.strip_prefix("bg-linear-to-")',
    'generate_utility_css("bg-linear-to-r").unwrap()',
    'generate_utility_css("bg-linear-to-tl").unwrap()',
    'css_for_class("bg-linear-to-r")',
    'class_name: "bg-linear-to-r"',
    'class_name: "bg-radial"',
    'class_name: "bg-linear-45"',
    'class_name: "bg-size-(--dx-bg-size)"',
    'class_name: "[@starting-style]:opacity-0"',
    'class_name: "[@layer_components]:p-4"',
    '"bg-radial".to_string()',
    '"[@starting-style]:opacity-0".to_string()',
    '"[@unknown_rule]:p-4".to_string()',
  ]) {
    assert.match(`${utility}\n${engine}\n${support}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(utility, /"r"\s*=>\s*Some\("to right"\)/);
  assert.match(engine, /css_for_class\("bg-linear-to-r"\)[\s\S]*background-image: linear-gradient\(to right, var\(--tw-gradient-stops\)\);/);
  assert.match(parity, /class_name: "bg-linear-to-r"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-radial"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-linear-45"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-size-\(--dx-bg-size\)"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "\[\@starting-style\]:opacity-0"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "\[\@layer_components\]:p-4"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /assert!\(\s*unsupported\.is_empty\(\),\s*"\{unsupported:\?\}"\s*\);/);
  assert.match(matrix, /linear background gradient aliases and interpolation modifiers now have targeted guards/);
});

test("dx-style radial and conic background gradient utilities generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn gradient_image_value(raw_value: &str) -> Option<String>",
    "fn conic_gradient_image(raw_value: &str, negative: bool) -> Option<String>",
    "fn gradient_angle_value(raw_value: &str, negative: bool) -> Option<String>",
    'generate_utility_css("bg-radial").unwrap()',
    'generate_utility_css("bg-radial-[circle_at_center]").unwrap()',
    'generate_utility_css("bg-radial-(--dx-bg-radial)").unwrap()',
    'generate_utility_css("bg-conic").unwrap()',
    'generate_utility_css("bg-conic-180").unwrap()',
    'generate_utility_css("-bg-conic-45").unwrap()',
    'generate_utility_css("bg-conic-[from_45deg_at_center]").unwrap()',
    'generate_utility_css("bg-conic-(--dx-bg-conic)").unwrap()',
    'css_for_class("bg-radial")',
    'css_for_class("bg-radial-[circle_at_center]")',
    'css_for_class("bg-conic-180")',
    'css_for_class("bg-conic-(--dx-bg-conic)")',
    'class_name: "bg-radial"',
    'class_name: "bg-radial-[circle_at_center]"',
    'class_name: "bg-radial-(--dx-bg-radial)"',
    'class_name: "bg-conic"',
    'class_name: "bg-conic-180"',
    'class_name: "-bg-conic-45"',
    'class_name: "bg-conic-[from_45deg_at_center]"',
    'class_name: "bg-conic-(--dx-bg-conic)"',
    'class_name: "bg-linear-45"',
    'class_name: "bg-size-(--dx-bg-size)"',
    'class_name: "[@starting-style]:opacity-0"',
    'class_name: "[@layer_components]:p-4"',
    '"[@starting-style]:opacity-0".to_string()',
    '"[@unknown_rule]:p-4".to_string()',
  ]) {
    assert.match(`${utility}\n${engine}\n${support}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("bg-radial"\)[\s\S]*background-image: radial-gradient\(var\(--tw-gradient-stops\)\)/);
  assert.match(engine, /css_for_class\("bg-radial-\[circle_at_center\]"\)[\s\S]*background-image: radial-gradient\(circle at center, var\(--tw-gradient-stops\)\)/);
  assert.match(engine, /css_for_class\("bg-conic-180"\)[\s\S]*background-image: conic-gradient\(from 180deg, var\(--tw-gradient-stops\)\)/);
  assert.match(parity, /class_name: "bg-radial"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-radial-\[circle_at_center\]"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-radial-\(--dx-bg-radial\)"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-conic"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-conic-180"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "-bg-conic-45"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-conic-\[from_45deg_at_center\]"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-conic-\(--dx-bg-conic\)"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-linear-45"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-size-\(--dx-bg-size\)"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "\[\@starting-style\]:opacity-0"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "\[\@layer_components\]:p-4"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /assert!\(\s*unsupported\.is_empty\(\),\s*"\{unsupported:\?\}"\s*\);/);
  assert.match(matrix, /radial and conic background gradient utilities now have targeted guards/);
});

test("dx-style linear angle and arbitrary background image utilities generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn linear_gradient_image(raw_value: &str, negative: bool) -> Option<String>",
    "fn background_image_arbitrary_value(raw_value: &str) -> Option<String>",
    'generate_utility_css("bg-linear-45").unwrap()',
    'generate_utility_css("-bg-linear-45").unwrap()',
    'generate_utility_css("bg-linear-[25deg]").unwrap()',
    'generate_utility_css("bg-linear-(--dx-bg-linear)").unwrap()',
    "generate_utility_css(\"bg-[url('/hero.png')]\").unwrap()",
    'generate_utility_css("bg-(image:--dx-bg-image)").unwrap()',
    'css_for_class("bg-linear-45")',
    "css_for_class(\"bg-[url('/hero.png')]\")",
    'class_name: "bg-linear-45"',
    "class_name: \"bg-[url('/hero.png')]\"",
    'class_name: "bg-size-(--dx-bg-size)"',
    'class_name: "[@starting-style]:opacity-0"',
    'class_name: "[@layer_components]:p-4"',
    '"[@starting-style]:opacity-0".to_string()',
    '"[@unknown_rule]:p-4".to_string()',
  ]) {
    assert.match(`${utility}\n${engine}\n${support}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("bg-linear-45"\)[\s\S]*background-image: linear-gradient\(45deg, var\(--tw-gradient-stops\)\)/);
  assert.match(engine, /css_for_class\("bg-\[url\('\/hero\.png'\)\]"\)[\s\S]*background-image: url\('\/hero\.png'\)/);
  assert.match(parity, /class_name: "bg-linear-45"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-\[url\('\/hero\.png'\)\]"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-size-\(--dx-bg-size\)"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "\[\@starting-style\]:opacity-0"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "\[\@layer_components\]:p-4"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /assert!\(\s*unsupported\.is_empty\(\),\s*"\{unsupported:\?\}"\s*\);/);
  assert.match(matrix, /linear angle and arbitrary background image utilities now have targeted guards/);
});

test("dx-style custom-property background size and position aliases generate normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn arbitrary_or_custom_property_value(raw_value: &str) -> Option<String>",
    'generate_utility_css("bg-size-(--dx-bg-size)").unwrap()',
    'generate_utility_css("bg-position-(--dx-bg-position)").unwrap()',
    'css_for_class("bg-size-(--dx-bg-size)")',
    'css_for_class("bg-position-(--dx-bg-position)")',
    'class_name: "bg-size-(--dx-bg-size)"',
    'class_name: "[@starting-style]:opacity-0"',
    'class_name: "[@layer_components]:p-4"',
    '"[@starting-style]:opacity-0".to_string()',
    '"[@unknown_rule]:p-4".to_string()',
  ]) {
    assert.match(`${utility}\n${engine}\n${support}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("bg-size-\(--dx-bg-size\)"\)[\s\S]*background-size: var\(--dx-bg-size\)/);
  assert.match(engine, /css_for_class\("bg-position-\(--dx-bg-position\)"\)[\s\S]*background-position: var\(--dx-bg-position\)/);
  assert.match(parity, /class_name: "bg-size-\(--dx-bg-size\)"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "\[\@starting-style\]:opacity-0"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "\[\@layer_components\]:p-4"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /assert!\(\s*unsupported\.is_empty\(\),\s*"\{unsupported:\?\}"\s*\);/);
  assert.match(matrix, /custom-property background size and position aliases now have targeted guards/);
});

test("dx-style mask utilities generate prefixed normal CSS", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const mask = readRequiredFile("related-crates/style/src/core/engine/utility/mask.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const support = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "mod mask;",
    "mask::mask_utility(class_name)",
    "pub(super) fn mask_utility(class_name: &str) -> Option<String>",
    "fn prefixed_mask_property(property: &str, value: &str) -> String",
    "fn prefixed_mask_composite_property(value: &str, webkit_value: &str) -> String",
    "fn mask_radial_stop_or_color(raw_value: &str) -> Option<MaskGradientValue>",
    "fn mask_radial_stop(raw_value: &str) -> Option<String>",
    "fn radial_mask_rule(stop: MaskStop, value: MaskGradientValue) -> String",
    "fn radial_size_rule(value: String) -> String",
    "fn radial_mask_image() -> String",
    "fn mask_conic_stop_or_color(raw_value: &str) -> Option<MaskGradientValue>",
    "fn linear_edge_mask_rule(edge: MaskEdge, stop: MaskStop, value: MaskGradientValue) -> String",
    "fn linear_axis_mask_rule(axis: MaskAxis, stop: MaskStop, value: MaskGradientValue) -> String",
    "fn linear_mask_image_rule(value: String) -> String",
    "fn linear_mask_image() -> String",
    "fn conic_mask_image() -> String",
    'css_for_class("mask-none")',
    'css_for_class("mask-alpha")',
    'css_for_class("mask-luminance")',
    'css_for_class("mask-match")',
    'css_for_class("mask-type-alpha")',
    'css_for_class("mask-type-luminance")',
    'css_for_class("mask-origin-content")',
    'css_for_class("mask-origin-view")',
    'css_for_class("mask-clip-padding")',
    'css_for_class("mask-no-clip")',
    'css_for_class("mask-radial-from-50%")',
    'css_for_class("mask-radial-to-90%")',
    'css_for_class("mask-radial-[100%_100%]")',
    'css_for_class("mask-radial-at-[35%_35%]")',
    'css_for_class("mask-circle")',
    'css_for_class("mask-radial-from-red-500")',
    'css_for_class("mask-conic-from-50%")',
    'css_for_class("mask-conic-to-75%")',
    'css_for_class("mask-conic-45")',
    'css_for_class("mask-l-from-50%")',
    'css_for_class("mask-l-to-90%")',
    'css_for_class("mask-x-from-70%")',
    'css_for_class("mask-y-to-90%")',
    'css_for_class("mask-linear-50")',
    'css_for_class("-mask-linear-50")',
    'css_for_class("mask-linear-from-60%")',
    'css_for_class("mask-linear-to-80%")',
    'css_for_class("mask-linear-[70deg,transparent_10%,black,transparent_80%]")',
    'css_for_class("mask-linear-(--launch-mask)")',
    'css_for_class("mask-add")',
    'css_for_class("mask-position-[center_top]")',
    'css_for_class("mask-radial-from-50%")',
    'class_name: "mask-radial-from-50%"',
    'class_name: "mask-conic-from-50%"',
    'class_name: "mask-l-from-50%"',
    'class_name: "mask-linear-from-60%"',
    'class_name: "mask-linear-[70deg,transparent_10%,black,transparent_80%]"',
    'class_name: "mask-radial-[100%_100%]"',
    'class_name: "mask-alpha"',
    'class_name: "mask-origin-content"',
    'class_name: "mask-type-alpha"',
    'class_name: "font-stretch-condensed"',
    'class_name: "forced-color-adjust-auto"',
    'class_name: "columns-3"',
    'class_name: "break-before-page"',
    'class_name: "box-decoration-clone"',
    'class_name: "bg-blend-multiply"',
    'class_name: "bg-origin-border"',
    'class_name: "bg-none"',
  ]) {
    assert.match(`${utility}\n${mask}\n${engine}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(mask, /"-webkit-mask-image: none; mask-image: none"/);
  assert.match(mask, /"mask-alpha" => Some\(prefixed_mask_property\("mask-mode", "alpha"\)\)/);
  assert.match(mask, /"mask-luminance" => Some\(prefixed_mask_property\("mask-mode", "luminance"\)\)/);
  assert.match(mask, /"mask-match" => Some\(prefixed_mask_property\("mask-mode", "match-source"\)\)/);
  assert.match(mask, /"mask-type-alpha" => Some\("mask-type: alpha"\.to_string\(\)\)/);
  assert.match(mask, /"mask-type-luminance" => Some\("mask-type: luminance"\.to_string\(\)\)/);
  assert.match(mask, /"mask-origin-content" => Some\(prefixed_mask_property\("mask-origin", "content-box"\)\)/);
  assert.match(mask, /"mask-clip-padding" => Some\(prefixed_mask_property\("mask-clip", "padding-box"\)\)/);
  assert.match(mask, /"mask-no-clip" => Some\(prefixed_mask_property\("mask-clip", "no-clip"\)\)/);
  assert.match(mask, /radial-gradient\(var\(--tw-mask-radial-shape, ellipse\) var\(--tw-mask-radial-size, farthest-corner\) at var\(--tw-mask-radial-position, center\),/);
  assert.match(mask, /fn linear_mask_image\(self\) -> String[\s\S]*--tw-mask-\{stem\}-from-color/);
  assert.match(mask, /linear-gradient\(var\(--tw-mask-linear-position, 180deg\), var\(--tw-mask-linear-from-color, #000\) var\(--tw-mask-linear-from, 0%\), var\(--tw-mask-linear-to-color, transparent\) var\(--tw-mask-linear-to, 100%\)\)/);
  assert.match(mask, /fn linear_mask_image_rule\(value: String\) -> String[\s\S]*linear-gradient\(\{value\}\)/);
  assert.match(mask, /conic-gradient\(from var\(--tw-mask-conic-angle, 0deg\), var\(--tw-mask-conic-from-color, #000\) var\(--tw-mask-conic-from, 0%\), var\(--tw-mask-conic-to-color, transparent\) var\(--tw-mask-conic-to, 100%\)\)/);
  assert.match(mask, /"mask-add" => Some\(prefixed_mask_composite_property\("add", "source-over"\)\)/);
  assert.match(mask, /format!\("-webkit-mask-composite: \{webkit_value\}; mask-composite: \{value\}"\)/);
  assert.match(mask, /prefixed_mask_property\("mask-position", &value\)/);
  assert.match(engine, /css_for_class\("mask-radial-from-50%"\)[\s\S]*-webkit-mask-image:[\s\S]*mask-image:/);
  assert.match(engine, /css_for_class\("mask-alpha"\)[\s\S]*-webkit-mask-mode: alpha[\s\S]*mask-mode: alpha/);
  assert.match(engine, /css_for_class\("mask-luminance"\)[\s\S]*mask-mode: luminance/);
  assert.match(engine, /css_for_class\("mask-match"\)[\s\S]*mask-mode: match-source/);
  assert.match(engine, /css_for_class\("mask-type-alpha"\)[\s\S]*mask-type: alpha/);
  assert.match(engine, /css_for_class\("mask-type-luminance"\)[\s\S]*mask-type: luminance/);
  assert.match(engine, /css_for_class\("mask-origin-content"\)[\s\S]*-webkit-mask-origin: content-box[\s\S]*mask-origin: content-box/);
  assert.match(engine, /css_for_class\("mask-origin-view"\)[\s\S]*mask-origin: view-box/);
  assert.match(engine, /css_for_class\("mask-clip-padding"\)[\s\S]*-webkit-mask-clip: padding-box[\s\S]*mask-clip: padding-box/);
  assert.match(engine, /css_for_class\("mask-no-clip"\)[\s\S]*mask-clip: no-clip/);
  assert.match(engine, /css_for_class\("mask-radial-\[100%_100%\]"\)[\s\S]*--tw-mask-radial-size: 100% 100%/);
  assert.match(engine, /css_for_class\("mask-radial-at-\[35%_35%\]"\)[\s\S]*--tw-mask-radial-position: 35% 35%/);
  assert.match(engine, /css_for_class\("mask-circle"\)[\s\S]*--tw-mask-radial-shape: circle/);
  assert.match(engine, /css_for_class\("mask-radial-from-red-500"\)[\s\S]*--tw-mask-radial-from-color: var\(--color-red-500\);/);
  assert.match(engine, /css_for_class\("mask-conic-from-50%"\)[\s\S]*--tw-mask-conic-from: 50%/);
  assert.match(engine, /css_for_class\("mask-conic-45"\)[\s\S]*--tw-mask-conic-angle: 45deg/);
  assert.match(engine, /css_for_class\("mask-l-from-50%"\)[\s\S]*--tw-mask-left-from: 50%/);
  assert.match(engine, /css_for_class\("mask-l-to-90%"\)[\s\S]*--tw-mask-left-to: 90%/);
  assert.match(engine, /css_for_class\("mask-x-from-70%"\)[\s\S]*--tw-mask-left-from: 70%/);
  assert.match(engine, /css_for_class\("mask-y-to-90%"\)[\s\S]*--tw-mask-bottom-to: 90%/);
  assert.match(engine, /css_for_class\("mask-linear-50"\)[\s\S]*--tw-mask-linear-position: 50deg/);
  assert.match(engine, /css_for_class\("-mask-linear-50"\)[\s\S]*--tw-mask-linear-position: calc\(50deg \* -1\)/);
  assert.match(engine, /css_for_class\("mask-linear-\[70deg,transparent_10%,black,transparent_80%\]"\)[\s\S]*linear-gradient\(70deg,transparent 10%,black,transparent 80%\)/);
  assert.match(engine, /css_for_class\("mask-linear-\(--launch-mask\)"\)[\s\S]*mask-image: var\(--launch-mask\)/);
  assert.match(engine, /css_for_class\("mask-add"\)[\s\S]*mask-composite: add/);
  assert.match(engine, /css_for_class\("mask-position-\[center_top\]"\)[\s\S]*-webkit-mask-position: center top/);
  assert.match(parity, /class_name: "mask-radial-from-50%"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "mask-conic-from-50%"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "mask-l-from-50%"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "mask-linear-\[70deg,transparent_10%,black,transparent_80%\]"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "mask-radial-\[100%_100%\]"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "mask-alpha"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "mask-origin-content"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "mask-type-alpha"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "font-stretch-condensed"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "forced-color-adjust-auto"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "columns-3"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "break-before-page"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "box-decoration-clone"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-blend-multiply"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-origin-border"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(parity, /class_name: "bg-none"[\s\S]*expected_status: TailwindParityStatus::Supported/);
  assert.match(support, /assert!\(\s*unsupported\.is_empty\(\),\s*"\{unsupported:\?\}"\s*\);/);
  assert.match(matrix, /mask utility classes now have targeted guards/);
});

test("dx-style Tailwind parity receipt records generated-output support honestly", () => {
  const core = readRequiredFile("related-crates/style/src/core/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");
  const dx = readRequiredFile("DX.md");

  for (const marker of [
    "pub mod parity;",
    "pub use engine::parity::",
    "pub const TAILWIND_PARITY_RECEIPT_SCHEMA",
    "pub const TAILWIND_PARITY_BASELINE",
    "pub enum TailwindParityStatus",
    "pub struct TailwindParityFixture",
    "pub struct TailwindParityEntry",
    "pub struct TailwindParityReceipt",
    "pub fn tailwind_parity_fixtures() -> &'static [TailwindParityFixture]",
    "pub fn build_tailwind_parity_receipt() -> TailwindParityReceipt",
    "let generated_css = engine.css_for_class(fixture.class_name);",
    "TailwindParityStatus::Supported",
    "TailwindParityStatus::Unsupported",
    "TailwindParityStatus::IntentionallyDifferent",
    'class_name: "p-4"',
    'class_name: "md:hover:bg-blue-500"',
    'class_name: "before:content-[\'New\']"',
    'class_name: "prose"',
    'class_name: "text-shadow-sm"',
    'class_name: "transform-gpu"',
    'class_name: "bg-radial-[circle_at_center]"',
    'class_name: "bg-conic-[from_45deg_at_center]"',
    'class_name: "file:p-4"',
    'class_name: "[@starting-style]:opacity-0"',
    'class_name: "[@layer_components]:p-4"',
    'class_name: "[@media_(any-hover:hover){&:hover}]:opacity-100"',
    'class_name: "[@unknown_rule]:p-4"',
    'class_name: "container"',
    "parity_receipt_records_supported_and_unsupported_classes",
    "parity_receipt_keeps_intentional_differences_visible",
  ]) {
    assert.match(`${core}\n${engine}\n${parity}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(parity, /let supported = entry_for\(&receipt, "p-4"\);[\s\S]*assert_eq!\(supported\.status, TailwindParityStatus::Supported\);[\s\S]*css\.contains\("padding: calc\(var\(--spacing\) \* 4\);"\)/);
  assert.match(parity, /let transform_gpu = entry_for\(&receipt, "transform-gpu"\);[\s\S]*assert_eq!\(transform_gpu\.status, TailwindParityStatus::Supported\);[\s\S]*css\.contains\("transform: translateZ\(0\)"\)/);
  assert.match(parity, /radial_position_background[\s\S]*radial-gradient\(circle at center, var\(--tw-gradient-stops\)\);/);
  assert.match(parity, /conic_arbitrary_background[\s\S]*conic-gradient\(from 45deg at center, var\(--tw-gradient-stops\)\);/);
  assert.match(parity, /let file_button = entry_for\(&receipt, "file:p-4"\);[\s\S]*assert_eq!\(file_button\.status, TailwindParityStatus::Supported\);[\s\S]*!css\.contains\("::-webkit-file-upload-button"\)/);
  assert.match(parity, /let starting_style = entry_for\(&receipt, "\[\@starting-style\]:opacity-0"\);[\s\S]*assert_eq!\(starting_style\.status, TailwindParityStatus::Supported\);[\s\S]*css\.contains\("\@starting-style"\)/);
  assert.match(parity, /let layer = entry_for\(&receipt, "\[\@layer_components\]:p-4"\);[\s\S]*assert_eq!\(layer\.status, TailwindParityStatus::Supported\);[\s\S]*css\.contains\("\@layer components"\)/);
  assert.match(parity, /let nested_at_rule = entry_for\(&receipt, "\[\@media_\(any-hover:hover\)\{&:hover\}\]:opacity-100"\);[\s\S]*assert_eq!\(nested_at_rule\.status, TailwindParityStatus::Supported\);[\s\S]*css\.contains\("\@media \(any-hover:hover\)"\)/);
  assert.match(parity, /let unknown_at_rule = entry_for\(&receipt, "\[\@unknown_rule\]:p-4"\);[\s\S]*assert_eq!\(unknown_at_rule\.status, TailwindParityStatus::Supported\)/);
  assert.match(parity, /let container = entry_for\(&receipt, "container"\);[\s\S]*container\.status,[\s\S]*TailwindParityStatus::IntentionallyDifferent/);
  assert.match(matrix, /Generated-output parity receipt/);
  assert.match(dx, /tailwind parity receipt pass/i);
});

test("dx-style Tailwind parity receipt records state alias generated CSS", () => {
  const parity = readRequiredFile("related-crates/style/src/core/engine/parity.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    'class_name: "target:p-4"',
    'class_name: "read-only:bg-blue-500"',
    'class_name: "indeterminate:opacity-100"',
    'class_name: "has-even:bg-blue-500"',
    'class_name: "not-visited:text-slate-900"',
    'class_name: "in-read-only:p-4"',
    'area: "state-variants"',
    'area: "conditional-state-variants"',
    "parity_receipt_records_common_state_alias_generated_css",
  ]) {
    assert.match(parity, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(parity, /let target = entry_for\(&receipt, "target:p-4"\);[\s\S]*assert_eq!\(target\.status, TailwindParityStatus::Supported\);[\s\S]*css\.contains\(":target"\)[\s\S]*css\.contains\("padding: calc\(var\(--spacing\) \* 4\);"\)/);
  assert.match(parity, /let read_only = entry_for\(&receipt, "read-only:bg-blue-500"\);[\s\S]*assert_eq!\(read_only\.status, TailwindParityStatus::Supported\);[\s\S]*css\.contains\(":read-only"\)[\s\S]*css\.contains\("background-color: rgb\(59 130 246\);"\)/);
  assert.match(parity, /let indeterminate = entry_for\(&receipt, "indeterminate:opacity-100"\);[\s\S]*assert_eq!\(indeterminate\.status, TailwindParityStatus::Supported\);[\s\S]*css\.contains\(":indeterminate"\)[\s\S]*css\.contains\("opacity: 100%;"\)/);
  assert.match(parity, /let has_even = entry_for\(&receipt, "has-even:bg-blue-500"\);[\s\S]*assert_eq!\(has_even\.status, TailwindParityStatus::Supported\);[\s\S]*css\.contains\(":has\(\*:nth-child\(even\)\)"\)/);
  assert.match(parity, /let not_visited = entry_for\(&receipt, "not-visited:text-slate-900"\);[\s\S]*assert_eq!\(not_visited\.status, TailwindParityStatus::Supported\);[\s\S]*css\.contains\(":not\(\*:visited\)"\)/);
  assert.match(parity, /let in_read_only = entry_for\(&receipt, "in-read-only:p-4"\);[\s\S]*assert_eq!\(in_read_only\.status, TailwindParityStatus::Supported\);[\s\S]*css\.contains\(":where\(\*:read-only\)"\)/);
  assert.match(matrix, /state alias generated-output parity receipt pass/i);
  assert.match(dx, /state alias generated-output parity receipt pass/i);
  assert.match(todo, /state alias generated-output parity receipt/i);
  assert.match(changelog, /state alias generated-output parity receipt/i);
});

test("dx-www style receipts expose Tailwind parity evidence to Check and Zed", () => {
  const tools = readDxStyleTools();
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn dx_style_tailwind_parity_receipt_contract() -> Value",
    "style::core::build_tailwind_parity_receipt()",
    "style::core::TAILWIND_PARITY_RECEIPT_SCHEMA",
    "tailwind_parity_receipt_contract",
    "supported_class_count",
    "unsupported_class_count",
    "intentionally_different_class_count",
    "unsupported_class_examples",
    "intentionally_different_examples",
    '"generated_by": "style::core::build_tailwind_parity_receipt"',
    '"receipt_source": "related-crates/style/src/core/engine/parity.rs"',
  ]) {
    assert.match(tools, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(tools, /"tool_consumers":\s*\[[\s\S]*"dx-check"[\s\S]*"Forge"[\s\S]*"Zed"[\s\S]*"Friday"/);
  assert.match(tools, /"tailwind_parity_receipt_contract": dx_style_tailwind_parity_receipt_contract\(\)/g);
  assert.match(`${tools}\n${matrix}`, /prose/);
  assert.match(`${tools}\n${matrix}`, /text-shadow-sm/);
  assert.match(`${tools}\n${matrix}`, /field-sizing-content/);
  assert.match(`${tools}\n${matrix}`, /container/);
  assert.match(dx, /parity receipt visibility pass/i);
  assert.match(todo, /Tailwind parity receipt visibility/i);
  assert.match(changelog, /Exposed dx-style Tailwind parity receipts/i);
});

test("dx-style Tailwind-style container query variants are implemented with targeted Rust guards", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "const DEFAULT_CONTAINER_QUERY_TOKENS: &[(&str, &str)]",
    "struct ContainerQueryVariant",
    "fn container_query_range_condition(value: &str, axis: ContainerQueryAxis) -> Option<String>",
    "tailwind_container_query_variant_maps_default_token",
    "tailwind_container_query_variant_maps_arbitrary_min_width",
    "tailwind_container_query_variant_composes_with_state",
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("@sm:bg-blue-500"\)[\s\S]*@container \(width >= 24rem\)/);
  assert.match(engine, /css_for_class\("@\[475px\]:text-slate-900"\)[\s\S]*@container \(width >= 475px\)/);
  assert.match(engine, /css_for_class\("@lg:hover:opacity-100"\)[\s\S]*@container \(width >= 32rem\)[\s\S]*:hover/);
  assert.match(matrix, /Tailwind-style container query variants now emit Tailwind v4\.3 range syntax/);
});

test("dx-style named and max container query variants are implemented with targeted Rust guards", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn named_container_query_variant(part: &str) -> Option<(&str, &str)>",
    "fn container_query_range_condition(value: &str, axis: ContainerQueryAxis) -> Option<String>",
    "tailwind_named_container_query_variant_maps_token",
    "tailwind_max_container_query_variant_maps_default_token",
    "tailwind_arbitrary_max_container_query_variant_maps_width",
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("@lg\/main:bg-blue-500"\)[\s\S]*@container main \(width >= 32rem\)/);
  assert.match(engine, /css_for_class\("@max-md:text-slate-900"\)[\s\S]*@container \(width < 28rem\)/);
  assert.match(engine, /css_for_class\("@max-\[960px\]:opacity-100"\)[\s\S]*@container \(width < 960px\)/);
  assert.match(matrix, /named, max-width, arbitrary, reversed range[\s\S]*container[\s\S]*targeted guards/);
});

test("dx-style responsive media variants are implemented with targeted Rust guards", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn tailwind_media_query_variant_for_engine(engine: &StyleEngine, part: &str) -> Option<String>",
    "fn exclusive_max_screen_width(value: &str) -> Option<String>",
    "tailwind_max_screen_variant_maps_default_token",
    "tailwind_arbitrary_media_query_variants_map_widths",
    "tailwind_environment_media_variants_map_queries",
    'css_for_class("max-md:text-slate-900")',
    'css_for_class("min-[475px]:bg-blue-500")',
    'css_for_class("max-[960px]:opacity-100")',
    'css_for_class("motion-safe:animate-spin")',
    'css_for_class("print:hidden")',
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(matrix, /Tailwind-style `max-\*`[\s\S]*motion-safe[\s\S]*targeted guards/);
});

test("dx-style supports feature-query variants are implemented with targeted Rust guards", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const states = readRequiredFile("related-crates/style/src/core/engine/states/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn supports_query_variant(part: &str) -> Option<String>",
    "fn supports_condition(raw: &str) -> Option<String>",
    "tailwind_supports_feature_query_variant_maps_condition",
    "tailwind_not_supports_feature_query_variant_maps_condition",
    'css_for_class("supports-[display:grid]:grid")',
    'css_for_class("not-supports-[display:grid]:block")',
    'css_for_class("supports-[backdrop-filter:blur(0)]:backdrop-blur-md")',
  ]) {
    assert.match(`${engine}\n${states}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(matrix, /feature-query variants now have targeted guards/);
});

test("dx-style transition utilities are implemented with targeted Rust guards", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn transition_utility(class_name: &str) -> Option<String>",
    "fn transition_time_value(raw_value: &str) -> Option<String>",
    "transition_property_utility_maps_colors",
    "transition_duration_delay_and_easing_utilities_generate_css",
    "transition_arbitrary_values_generate_css",
    ".or_else(|| transition_utility(class_name))",
  ]) {
    assert.match(`${engine}\n${utility}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("transition-colors"\)[\s\S]*transition-property: color, background-color, border-color, text-decoration-color, fill, stroke/);
  assert.match(engine, /css_for_class\("duration-300"\)[\s\S]*transition-duration: 300ms/);
  assert.match(engine, /css_for_class\("delay-200"\)[\s\S]*transition-delay: 200ms/);
  assert.match(engine, /css_for_class\("ease-in-out"\)[\s\S]*transition-timing-function: cubic-bezier\(0\.4, 0, 0\.2, 1\)/);
  assert.match(engine, /css_for_class\("duration-\[375ms\]"\)[\s\S]*transition-duration: 375ms/);
  assert.match(matrix, /Transitions \| Supported subset/);
  assert.match(matrix, /transition property, duration, delay, easing, and arbitrary time\/easing utilities now have targeted guards/);
});

test("dx-style transition property aliases and behavior utilities are implemented with targeted Rust guards", () => {
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn transition_property_value(raw_value: &str) -> Option<String>",
    "transition_property_aliases_generate_css",
    "transition_behavior_utilities_generate_css",
  ]) {
    assert.match(`${engine}\n${utility}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("transition-\[height,opacity\]"\)[\s\S]*transition-property: height,opacity/);
  assert.match(engine, /css_for_class\("transition-\(--dx-transition-property\)"\)[\s\S]*transition-property: var\(--dx-transition-property\)/);
  assert.match(engine, /css_for_class\("transition-discrete"\)[\s\S]*transition-behavior: allow-discrete/);
  assert.match(engine, /css_for_class\("transition-normal"\)[\s\S]*transition-behavior: normal/);
  assert.match(matrix, /transition property aliases and transition-behavior utilities now have targeted guards/);
});

test("dx-style Tailwind-style animation utilities are implemented with targeted Rust guards", () => {
  const animation = readRequiredFile("related-crates/style/src/core/animation/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn tailwind_animation_css(class_name: &str) -> Option<String>",
    "fn keyframes_raw_rule(name: &str, frames: &[(&str, &str)]) -> String",
    "tailwind_animation_spin_generates_keyframes",
    "tailwind_animation_pulse_and_bounce_generate_keyframes",
    "tailwind_animation_none_and_arbitrary_values_generate_css",
  ]) {
    assert.match(`${animation}\n${engine}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("animate-spin"\)[\s\S]*@keyframes dx-spin/);
  assert.match(engine, /css_for_class\("animate-spin"\)[\s\S]*animation: dx-spin 1s linear infinite/);
  assert.match(engine, /css_for_class\("animate-pulse"\)[\s\S]*@keyframes dx-pulse/);
  assert.match(engine, /css_for_class\("animate-bounce"\)[\s\S]*@keyframes dx-bounce/);
  assert.match(engine, /css_for_class\("animate-none"\)[\s\S]*animation: none/);
  assert.match(engine, /css_for_class\("animate-\[wiggle_1s_ease-in-out_infinite\]"\)[\s\S]*animation: wiggle 1s ease-in-out infinite/);
  assert.match(matrix, /Animation \| Supported subset/);
  assert.match(matrix, /Tailwind-style `animate-spin`, `animate-pulse`, `animate-bounce`, `animate-none`, and arbitrary `animate-\[\.\.\.\]` values have targeted guards/);
});

test("dx-style animation CSS variable aliases are implemented with targeted Rust guards", () => {
  const animation = readRequiredFile("related-crates/style/src/core/animation/mod.rs");
  const engine = readRequiredFile("related-crates/style/src/core/engine/mod.rs");
  const matrix = readRequiredFile("related-crates/style/TAILWIND_COMPATIBILITY.md");

  for (const marker of [
    "fn animation_variable_value(class_name: &str) -> Option<String>",
    "tailwind_animation_css_variable_alias_generates_css",
  ]) {
    assert.match(`${animation}\n${engine}`, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(engine, /css_for_class\("animate-\(--dx-animation-enter\)"\)[\s\S]*animation: var\(--dx-animation-enter\)/);
  assert.match(matrix, /animation CSS variable aliases now have targeted guards/);
});

test("dx-check receipt panel exposes dx-style browser compat evidence row", () => {
  const panel = [
    readRequiredFile("core/src/ecosystem/dx_check_receipt.rs"),
    readRequiredFile("core/src/ecosystem/dx_check_receipt/panel.rs"),
    readRequiredFile("core/src/ecosystem/dx_check_receipt/panel_parts/style_evidence.rs"),
    readRequiredFile("core/src/ecosystem/dx_check_receipt/panel_parts/tests_b.rs"),
  ].join("\n");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    "DxCheckPanelStyleEvidenceRow",
    "style_evidence_rows",
    "dx_style_browser_compat_panel_rows",
    "dx-style-browser-compat",
    "dx_style_browser_compat_receipt_present",
    "dx_style_browser_compat_contract_present",
    "dx_style_browser_compat_schema_supported",
    "dx_style_browser_compat_class_count",
    "dx_style_browser_compat_selector_class_count",
    "dx_style_browser_compat_full_autoprefixer_parity",
    "dx_style_browser_compat_full_tailwind_postcss_output_parity",
    "dx_style_tailwind_parity_state_alias_supported_classes",
    "selector_class_count",
    "selector_class_examples",
    "tailwind_parity_state_alias_supported_class_count",
    "tailwind_parity_supported_state_alias_examples",
    "tailwind-postcss-browser-compat.json",
    "full_autoprefixer_parity",
    "full_tailwind_postcss_output_parity",
    "dx_check_panel_exposes_style_browser_compat_evidence_row",
  ]) {
    assert.match(panel, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(panel, /zed_visibility:\s*"dx-style:browser-compat"/);
  assert.match(panel, /full_autoprefixer_parity[\s\S]*full_tailwind_postcss_output_parity/);
  assert.match(panel, /tailwind_parity_supported_state_alias_examples[\s\S]*target:p-4[\s\S]*in-read-only:p-4/);
  assert.match(panel, /selector_class_examples[\s\S]*file:p-4/);
  assert.match(panel, /metric_value\("dx_style_browser_compat_selector_class_count"\)[\s\S]*1/);
  assert.match(panel, /metric_value\("dx_style_tailwind_parity_state_alias_supported_classes"\)[\s\S]*6/);
  assert.match(`${dx}\n${todo}\n${changelog}`, /browser-compat.*receipt panel/i);
  assert.match(`${dx}\n${todo}\n${changelog}`, /state-alias.*style evidence row/i);
});

test("dx-check consumes dx-style rule metadata for Zed and Studio panels", () => {
  const receipts = readRequiredFile("core/src/ecosystem/dx_style_receipts.rs");
  const lowerCheck = [
    readRequiredFile("core/src/ecosystem/project_check.rs"),
    readRequiredFile("core/src/ecosystem/project_check/readiness_parts/dx_style.rs"),
  ].join("\n");
  const panel = [
    readRequiredFile("core/src/ecosystem/dx_check_receipt.rs"),
    readRequiredFile("core/src/ecosystem/dx_check_receipt/panel.rs"),
    readRequiredFile("core/src/ecosystem/dx_check_receipt/panel_parts/style_evidence.rs"),
  ].join("\n");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    "DX_STYLE_RULE_METADATA_SCHEMA",
    "DxStyleRuleMetadataSummary",
    "DxStyleRuleMetadataRow",
    "dx_style_rule_metadata_summary",
    "style_rule_metadata",
    "styleRuleMetadata",
    "visual_properties",
    "source_files",
    "token_references",
    "zed_studio_editable",
  ]) {
    assert.match(receipts, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "dx_style_rule_metadata_summary(root)",
    "dx_style_rule_metadata_receipt_present",
    "dx_style_rule_metadata_class_count",
    "dx_style_rule_metadata_editable_class_count",
    "dx_style_rule_metadata_visual_property_count",
    "dx_style_rule_metadata_token_reference_count",
    "dx_style_rule_metadata_source_file_count",
  ]) {
    assert.match(lowerCheck, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  for (const marker of [
    "DX_STYLE_RULE_METADATA_ROW_ID",
    "DX_STYLE_RULE_METADATA_METRICS",
    "dx_style_rule_metadata_panel_row",
    "dx-style-rule-metadata",
    "dx-style:rule-metadata",
    "rule_metadata_visual_properties",
    "rule_metadata_source_files",
    "rule_metadata_token_references",
    "rule_metadata_editable_class_count",
    "rule_metadata_zed_studio_editable",
  ]) {
    assert.match(panel, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(panel, /zed_visibility:\s*"dx-style:rule-metadata"/);
  assert.match(
    `${dx}\n${todo}\n${changelog}`,
    /rule metadata.*dx-check.*Zed.*Studio/i,
  );
});

test("dx-style browser compat evidence is consumed through the dx-check panel read model", async () => {
  const readModelSource = readRequiredFile(
    "examples/onboard/dx-check-style-evidence-read-model.ts",
  );
  const shell = readRequiredFile("examples/onboard/template-shell.tsx");
  const studioManifest = readRequiredFile("dx-www/src/cli/studio_manifest.rs");
  const materializer = readRequiredFile("tools/launch/materialize-www-template.ts");
  const { pathToFileURL } = require("node:url");
  const readModel = await import(
    pathToFileURL(
      path.join(root, "examples/onboard/dx-check-style-evidence-read-model.ts"),
    ).href
  );

  const evidence = readModel.dxStyleBrowserCompatEvidenceFromCheckPanel({
    style_evidence_rows: [
      {
        row_id: "dx-style-browser-compat",
        title: "dx-style browser compatibility",
        status: "present",
        receipt_path: ".dx/receipts/style/check.json",
        fixture_path:
          "related-crates/style/fixtures/tailwind-postcss-browser-compat.json",
        receipt_present: true,
        contract_present: true,
        schema_supported: true,
        class_count: 6,
        selector_class_count: 1,
        selector_class_examples: ["file:p-4"],
        tailwind_parity_state_alias_supported_class_count: 6,
        tailwind_parity_supported_state_alias_examples: [
          "target:p-4",
          "read-only:bg-blue-500",
          "indeterminate:opacity-100",
          "has-even:bg-blue-500",
          "not-visited:text-slate-900",
          "in-read-only:p-4",
        ],
        full_autoprefixer_parity: false,
        full_tailwind_postcss_output_parity: false,
        zed_visibility: "dx-style:browser-compat",
        metrics: [
          { name: "dx_style_browser_compat_receipt_present", value: 1 },
          { name: "dx_style_browser_compat_contract_present", value: 1 },
          { name: "dx_style_browser_compat_schema_supported", value: 1 },
          { name: "dx_style_browser_compat_class_count", value: 6 },
          { name: "dx_style_browser_compat_selector_class_count", value: 1 },
          { name: "dx_style_browser_compat_full_autoprefixer_parity", value: 0 },
          {
            name: "dx_style_browser_compat_full_tailwind_postcss_output_parity",
            value: 0,
          },
          {
            name: "dx_style_tailwind_parity_state_alias_supported_classes",
            value: 6,
          },
        ],
        runtime_limitations: ["source-only"],
        next_action:
          "Keep expanding `tailwind-postcss-browser-compat.json` with measured output.",
      },
    ],
  });

  assert.equal(readModel.DX_STYLE_BROWSER_COMPAT_ROW_ID, "dx-style-browser-compat");
  assert.equal(evidence.source, "check_panel.view_model.style_evidence_rows");
  assert.equal(evidence.rowId, "dx-style-browser-compat");
  assert.equal(evidence.status, "present");
  assert.equal(evidence.fixturePath, "related-crates/style/fixtures/tailwind-postcss-browser-compat.json");
  assert.equal(evidence.zedVisibility, "dx-style:browser-compat");
  assert.equal(evidence.canaryClassCount, 6);
  assert.equal(evidence.selectorCanaryClassCount, 1);
  assert.deepEqual(evidence.selectorClassExamples, ["file:p-4"]);
  assert.equal(evidence.tailwindParityStateAliasSupportedClassCount, 6);
  assert.deepEqual(evidence.tailwindParitySupportedStateAliasExamples, [
    "target:p-4",
    "read-only:bg-blue-500",
    "indeterminate:opacity-100",
    "has-even:bg-blue-500",
    "not-visited:text-slate-900",
    "in-read-only:p-4",
  ]);
  assert.equal(evidence.fullAutoprefixerParity, false);
  assert.equal(evidence.fullTailwindPostcssOutputParity, false);
  assert.equal(evidence.readsRawStyleReceipt, false);
  assert.deepEqual(evidence.metrics.dxStyleBrowserCompatClassCount, 6);
  assert.deepEqual(evidence.metrics.dxStyleBrowserCompatSelectorClassCount, 1);
  assert.deepEqual(
    evidence.metrics.dxStyleTailwindParityStateAliasSupportedClasses,
    6,
  );

  const missing = readModel.dxStyleBrowserCompatEvidenceFromCheckPanel({
    style_evidence_rows: [],
  });
  assert.equal(missing.status, "missing");
  assert.equal(missing.receiptPresent, false);
  assert.equal(missing.selectorCanaryClassCount, 0);
  assert.deepEqual(missing.selectorClassExamples, []);
  assert.equal(missing.tailwindParityStateAliasSupportedClassCount, 0);
  assert.deepEqual(missing.tailwindParitySupportedStateAliasExamples, []);

  assert.doesNotMatch(readModelSource, /browser_compat_receipt_contract/);
  assert.doesNotMatch(readModelSource, /readFileSync|fs\./);
  assert.match(readModelSource, /selector_class_count/);
  assert.match(readModelSource, /selectorClassExamples/);
  assert.match(readModelSource, /dx_style_browser_compat_selector_class_count/);
  assert.match(readModelSource, /tailwind_parity_state_alias_supported_class_count/);
  assert.match(readModelSource, /tailwindParitySupportedStateAliasExamples/);
  assert.match(readModelSource, /dx_style_tailwind_parity_state_alias_supported_classes/);
  assert.match(shell, /style_evidence_rows/);
  assert.match(shell, /data-dx-check-style-evidence-count/);
  assert.match(shell, /data-dx-check-style-evidence-row/);
  assert.match(shell, /data-dx-check-style-evidence-zed/);
  assert.match(shell, /data-dx-check-style-evidence-selector-class-count/);
  assert.match(shell, /data-dx-check-style-evidence-selector-class-examples/);
  assert.match(shell, /data-dx-check-style-evidence-state-alias-count/);
  assert.match(shell, /data-dx-check-style-evidence-state-alias-examples/);
  assert.match(shell, /data-dx-check-style-evidence-full-autoprefixer-parity/);
  assert.match(shell, /data-dx-check-style-evidence-full-tailwind-postcss-output-parity/);
  assert.match(studioManifest, /"style_evidence_rows"/);
  assert.match(studioManifest, /data-dx-check-style-evidence-row/);
  assert.match(studioManifest, /data-dx-check-style-evidence-zed/);
  assert.match(studioManifest, /data-dx-check-style-evidence-selector-class-count/);
  assert.match(studioManifest, /data-dx-check-style-evidence-selector-class-examples/);
  assert.match(studioManifest, /data-dx-check-style-evidence-state-alias-count/);
  assert.match(studioManifest, /data-dx-check-style-evidence-state-alias-examples/);
  assert.match(materializer, /data-dx-check-style-evidence-row/);
  assert.match(materializer, /data-dx-check-style-evidence-selector-class-count/);
  assert.match(materializer, /data-dx-check-style-evidence-selector-class-examples/);
  assert.match(materializer, /data-dx-check-style-evidence-state-alias-count/);
  assert.match(materializer, /data-dx-check-style-evidence-state-alias-examples/);
  assert.match(materializer, /dx-style:browser-compat/);
});

test("static launch check panel exposes dx-style browser compat fallback evidence markers", () => {
  const shell = readRequiredFile("examples/onboard/template-shell.tsx");
  const staticLaunch = readRequiredFile("tools/launch/runtime-template/pages/index.html");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  assert.match(shell, /const dxCheckStyleEvidenceTemplates: DxCheckPanelStyleEvidenceRow\[\] = \[/);
  assert.match(
    shell,
    /const receiptStyleEvidenceRows = viewModel\.style_evidence_rows \?\? \[\];[\s\S]*receiptStyleEvidenceRows\.length > 0[\s\S]*dxCheckStyleEvidenceTemplates/,
  );

  for (const marker of [
    'data-dx-check-style-evidence-count="1"',
    'data-dx-check-style-evidence-rows="fallback"',
    'data-dx-check-style-evidence-row="dx-style-browser-compat"',
    'data-dx-check-style-evidence-status="missing"',
    'data-dx-check-style-evidence-receipt-path=".dx/receipts/style/check.json"',
    'data-dx-check-style-evidence-fixture-path="related-crates/style/fixtures/tailwind-postcss-browser-compat.json"',
    'data-dx-check-style-evidence-zed="dx-style:browser-compat"',
    'data-dx-check-style-evidence-class-count="0"',
    'data-dx-check-style-evidence-selector-class-count="0"',
    'data-dx-check-style-evidence-selector-class-examples=""',
    'data-dx-check-style-evidence-state-alias-count="0"',
    'data-dx-check-style-evidence-state-alias-examples=""',
    'data-dx-check-style-evidence-full-autoprefixer-parity="false"',
    'data-dx-check-style-evidence-full-tailwind-postcss-output-parity="false"',
  ]) {
    assert.match(staticLaunch, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(
    `${dx}\n${todo}\n${changelog}`,
    /static .*dx-style browser-compat fallback evidence/i,
  );
});

test("launch preview manifest materializer carries dx-style browser compat evidence metadata", () => {
  const materializer = readRequiredFile("tools/launch/materialize-www-template.ts");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    "const DX_STYLE_BROWSER_COMPAT_PREVIEW_EVIDENCE = {",
    'schema: "dx.style.browser_compat.preview_evidence"',
    'rowId: "dx-style-browser-compat"',
    'status: "missing"',
    'receiptPath: ".dx/receipts/style/check.json"',
    'fixturePath: "related-crates/style/fixtures/tailwind-postcss-browser-compat.json"',
    'zedVisibility: "dx-style:browser-compat"',
    "canaryClassCount: 0",
    "selectorCanaryClassCount: 0",
    "selectorClassExamples: []",
    "tailwindParityStateAliasSupportedClassCount: 0",
    "tailwindParitySupportedStateAliasExamples: []",
    "fullAutoprefixerParity: false",
    "fullTailwindPostcssOutputParity: false",
    "styleEvidenceRows: [DX_STYLE_BROWSER_COMPAT_PREVIEW_EVIDENCE]",
  ]) {
    assert.match(materializer, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(
    materializer,
    /route: "\/"[\s\S]*styleEvidenceRows: \[DX_STYLE_BROWSER_COMPAT_PREVIEW_EVIDENCE\]/,
  );
  assert.match(
    `${dx}\n${todo}\n${changelog}`,
    /preview[- ]manifest.*dx-style browser-compat/i,
  );
});

test("dx-style preview manifest evidence has a typed read model for Zed and Studio", async () => {
  const readModelSource = readRequiredFile(
    "examples/onboard/preview-style-evidence-read-model.ts",
  );
  const materializer = readRequiredFile("tools/launch/materialize-www-template.ts");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");
  const { pathToFileURL } = require("node:url");
  const readModel = await import(
    pathToFileURL(
      path.join(root, "examples/onboard/preview-style-evidence-read-model.ts"),
    ).href
  );

  const evidence = readModel.dxStyleBrowserCompatEvidenceFromPreviewManifest({
    styleEvidenceRows: [
      {
        rowId: "dx-style-browser-compat",
        title: "root evidence",
        status: "root-present",
        receiptPath: ".dx/receipts/style/check.json",
        fixturePath:
          "related-crates/style/fixtures/tailwind-postcss-browser-compat.json",
        zedVisibility: "dx-style:browser-compat",
        canaryClassCount: 1,
        selectorCanaryClassCount: 1,
        selectorClassExamples: ["file:p-4"],
        tailwindParityStateAliasSupportedClassCount: 1,
        tailwindParitySupportedStateAliasExamples: ["target:p-4"],
        fullAutoprefixerParity: false,
        fullTailwindPostcssOutputParity: false,
      },
    ],
    routes: [
      {
        route: "/",
        styleEvidenceRows: [
          {
            rowId: "dx-style-browser-compat",
            title: "route evidence",
            status: "missing",
            receiptPath: ".dx/receipts/style/check.json",
            fixturePath:
              "related-crates/style/fixtures/tailwind-postcss-browser-compat.json",
            zedVisibility: "dx-style:browser-compat",
            canaryClassCount: 0,
            selectorCanaryClassCount: 1,
            selectorClassExamples: ["file:p-4"],
            tailwindParityStateAliasSupportedClassCount: 6,
            tailwindParitySupportedStateAliasExamples: [
              "target:p-4",
              "read-only:bg-blue-500",
              "indeterminate:opacity-100",
              "has-even:bg-blue-500",
              "not-visited:text-slate-900",
              "in-read-only:p-4",
            ],
            fullAutoprefixerParity: false,
            fullTailwindPostcssOutputParity: false,
          },
        ],
      },
    ],
  });

  assert.equal(evidence.schema, "dx.www.template.preview_style_evidence_read_model");
  assert.equal(evidence.source, "preview_manifest.routes[/].styleEvidenceRows");
  assert.equal(evidence.rowId, "dx-style-browser-compat");
  assert.equal(evidence.status, "missing");
  assert.equal(evidence.canaryClassCount, 0);
  assert.equal(evidence.selectorCanaryClassCount, 1);
  assert.deepEqual(evidence.selectorClassExamples, ["file:p-4"]);
  assert.equal(evidence.tailwindParityStateAliasSupportedClassCount, 6);
  assert.deepEqual(evidence.tailwindParitySupportedStateAliasExamples, [
    "target:p-4",
    "read-only:bg-blue-500",
    "indeterminate:opacity-100",
    "has-even:bg-blue-500",
    "not-visited:text-slate-900",
    "in-read-only:p-4",
  ]);
  assert.equal(evidence.fullAutoprefixerParity, false);
  assert.equal(evidence.fullTailwindPostcssOutputParity, false);
  assert.equal(evidence.readsHtml, false);
  assert.equal(evidence.readsRawStyleReceipt, false);

  const missing = readModel.dxStyleBrowserCompatEvidenceFromPreviewManifest({
    routes: [],
  });
  assert.equal(missing.status, "missing");
  assert.equal(missing.source, "preview_manifest.styleEvidenceRows");
  assert.equal(missing.selectorCanaryClassCount, 0);
  assert.deepEqual(missing.selectorClassExamples, []);
  assert.equal(missing.tailwindParityStateAliasSupportedClassCount, 0);
  assert.deepEqual(missing.tailwindParitySupportedStateAliasExamples, []);

  assert.doesNotMatch(readModelSource, /readFileSync|fs\.|querySelector|document\./);
  assert.match(readModelSource, /dxStyleBrowserCompatEvidenceFromPreviewManifest/);
  assert.match(readModelSource, /selectorCanaryClassCount/);
  assert.match(readModelSource, /selectorClassExamples/);
  assert.match(readModelSource, /tailwindParityStateAliasSupportedClassCount/);
  assert.match(readModelSource, /tailwindParitySupportedStateAliasExamples/);
  assert.match(readModelSource, /preview_manifest\.routes\[/);
  assert.match(materializer, /styleEvidenceRows: \[DX_STYLE_BROWSER_COMPAT_PREVIEW_EVIDENCE\]/);
  assert.match(materializer, /selectorCanaryClassCount: 0/);
  assert.match(materializer, /selectorClassExamples: \[\]/);
  assert.match(materializer, /tailwindParityStateAliasSupportedClassCount: 0/);
  assert.match(materializer, /tailwindParitySupportedStateAliasExamples: \[\]/);
  assert.match(
    `${dx}\n${todo}\n${changelog}`,
    /preview[- ]manifest.*state-alias.*read model/i,
  );
});

test("dx-style preview manifest state alias evidence has a Zed Studio package panel consumer", async () => {
  const packagePanelSource = readRequiredFile(
    "examples/onboard/preview-style-package-panel-read-model.ts",
  );
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");
  const { pathToFileURL } = require("node:url");
  const packagePanel = await import(
    pathToFileURL(
      path.join(
        root,
        "examples/onboard/preview-style-package-panel-read-model.ts",
      ),
    ).href
  );

  const panel = packagePanel.dxStylePackagePanelFromPreviewManifest({
    routes: [
      {
        route: "/",
        styleEvidenceRows: [
          {
            rowId: "dx-style-browser-compat",
            title: "route evidence",
            status: "present",
            receiptPath: ".dx/receipts/style/check.json",
            fixturePath:
              "related-crates/style/fixtures/tailwind-postcss-browser-compat.json",
            zedVisibility: "dx-style:browser-compat",
            canaryClassCount: 6,
            selectorCanaryClassCount: 1,
            selectorClassExamples: ["file:p-4"],
            tailwindParityStateAliasSupportedClassCount: 6,
            tailwindParitySupportedStateAliasExamples: [
              "target:p-4",
              "read-only:bg-blue-500",
              "indeterminate:opacity-100",
              "has-even:bg-blue-500",
              "not-visited:text-slate-900",
              "in-read-only:p-4",
            ],
            fullAutoprefixerParity: false,
            fullTailwindPostcssOutputParity: false,
            nextAction:
              "Keep expanding `tailwind-postcss-browser-compat.json` with measured output.",
          },
        ],
      },
    ],
  });

  assert.equal(
    panel.schema,
    "dx.www.template.preview_style_package_panel_read_model",
  );
  assert.equal(panel.panelId, "dx-style-browser-compat-package-panel");
  assert.equal(panel.packageId, "dx-style");
  assert.equal(panel.route, "/");
  assert.equal(panel.source, "preview_manifest.routes[/].styleEvidenceRows");
  assert.equal(panel.status, "present");
  assert.equal(panel.zedVisibility, "dx-style:browser-compat");
  assert.equal(panel.canaryClassCount, 6);
  assert.equal(panel.selectorCanaryClassCount, 1);
  assert.deepEqual(panel.selectorClassExamples, ["file:p-4"]);
  assert.equal(panel.stateAliasSupportedClassCount, 6);
  assert.deepEqual(panel.stateAliasSupportedExamples, [
    "target:p-4",
    "read-only:bg-blue-500",
    "indeterminate:opacity-100",
    "has-even:bg-blue-500",
    "not-visited:text-slate-900",
    "in-read-only:p-4",
  ]);
  assert.equal(panel.fullAutoprefixerParity, false);
  assert.equal(panel.fullTailwindPostcssOutputParity, false);
  assert.equal(panel.readsHtml, false);
  assert.equal(panel.readsRawStyleReceipt, false);
  assert.equal(panel.readsCheckReceipt, false);

  const missing = packagePanel.dxStylePackagePanelFromPreviewManifest({
    routes: [],
  });
  assert.equal(missing.status, "missing");
  assert.equal(missing.selectorCanaryClassCount, 0);
  assert.deepEqual(missing.selectorClassExamples, []);
  assert.equal(missing.stateAliasSupportedClassCount, 0);
  assert.deepEqual(missing.stateAliasSupportedExamples, []);
  assert.equal(missing.readsHtml, false);

  assert.match(
    packagePanelSource,
    /dxStyleBrowserCompatEvidenceFromPreviewManifest/,
  );
  assert.match(packagePanelSource, /DxStylePreviewManifest/);
  assert.match(packagePanelSource, /selectorCanaryClassCount/);
  assert.match(packagePanelSource, /selectorClassExamples/);
  assert.doesNotMatch(
    packagePanelSource,
    /readFileSync|fs\.|querySelector|document\.|browser_compat_receipt_contract/,
  );
  assert.match(
    `${dx}\n${todo}\n${changelog}`,
    /package[- ]panel.*preview[- ]manifest.*state-alias/i,
  );
});

test("dx studio edit contract advertises dx-style style evidence markers", () => {
  const editContract = readRequiredFile(
    "examples/onboard/dx-studio-edit-contract.ts",
  );
  const shell = readRequiredFile("examples/onboard/template-shell.tsx");
  const dx = readRequiredFile("DX.md");
  const todo = readRequiredFile("TODO.md");
  const changelog = readRequiredFile("CHANGELOG.md");

  for (const marker of [
    "data-dx-check-style-evidence-count",
    "data-dx-check-style-evidence-row",
    "data-dx-check-style-evidence-status",
    "data-dx-check-style-evidence-receipt-path",
    "data-dx-check-style-evidence-fixture-path",
    "data-dx-check-style-evidence-zed",
    "data-dx-check-style-evidence-class-count",
    "data-dx-check-style-evidence-selector-class-count",
    "data-dx-check-style-evidence-selector-class-examples",
    "data-dx-check-style-evidence-state-alias-count",
    "data-dx-check-style-evidence-state-alias-examples",
    "data-dx-check-style-evidence-full-autoprefixer-parity",
    "data-dx-check-style-evidence-full-tailwind-postcss-output-parity",
  ]) {
    assert.match(editContract, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
    assert.match(shell, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(editContract, /stateMarkers: dxCheckPanelStateMarkers/);
  assert.match(
    `${dx}\n${todo}\n${changelog}`,
    /Studio.*dx-style.*style[- ]evidence markers/i,
  );
});
