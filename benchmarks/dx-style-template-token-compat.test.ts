import assert from "node:assert/strict";
import { readdirSync, readFileSync, statSync } from "node:fs";
import { extname, join } from "node:path";
import test from "node:test";

function readRequiredFile(path: string): string {
  return readFileSync(path, "utf8");
}

function escaped(marker: string): RegExp {
  return new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"));
}

const SOURCE_EXTENSIONS = new Set([".css", ".js", ".jsx", ".md", ".mdx", ".ts", ".tsx"]);
const SKIPPED_DIRECTORIES = new Set([
  ".dx",
  ".git",
  ".next",
  "dist",
  "node_modules",
  "out",
  "target",
]);
const SEMANTIC_COLOR_PREFIXES = [
  "placeholder-",
  "border-x-",
  "border-y-",
  "border-t-",
  "border-r-",
  "border-b-",
  "border-l-",
  "divide-",
  "border-",
  "stroke-",
  "ring-",
  "text-",
  "fill-",
  "bg-",
];
const REQUIRED_THEME_VARIABLES = [
  "--background:",
  "--foreground:",
  "--card:",
  "--card-foreground:",
  "--primary:",
  "--primary-foreground:",
  "--muted:",
  "--muted-foreground:",
  "--border:",
  "--input:",
  "--ring:",
  "--chart-1:",
  "--chart-2:",
  "--chart-3:",
  "--chart-4:",
  "--chart-5:",
  "--sidebar:",
  "--sidebar-foreground:",
  "--sidebar-primary:",
  "--sidebar-primary-foreground:",
  "--sidebar-accent:",
  "--sidebar-accent-foreground:",
  "--sidebar-border:",
  "--sidebar-ring:",
];

function sourceFiles(root: string): string[] {
  const files: string[] = [];
  for (const entry of readdirSync(root)) {
    if (SKIPPED_DIRECTORIES.has(entry)) {
      continue;
    }
    const path = join(root, entry);
    const stat = statSync(path);
    if (stat.isDirectory()) {
      files.push(...sourceFiles(path));
    } else if (SOURCE_EXTENSIONS.has(extname(path))) {
      files.push(path);
    }
  }
  return files;
}

function themeTokenNames(themeTokens: string): Set<string> {
  return new Set(
    [...themeTokens.matchAll(/"([^"]+)"\s*=>\s*"[^"]+"/g)].map((match) => match[1]),
  );
}

function semanticTokenForClassName(className: string, tokens: Set<string>): string | null {
  const utilityName = className.includes(":") ? className.split(":").at(-1)! : className;
  const [baseName] = utilityName.split("/");
  for (const prefix of SEMANTIC_COLOR_PREFIXES) {
    if (!baseName.startsWith(prefix)) {
      continue;
    }
    const token = baseName.slice(prefix.length);
    if (tokens.has(token)) {
      return token;
    }
  }
  return null;
}

function wwwTemplateSemanticTokenClasses(root: string, tokens: Set<string>): Set<string> {
  const classes = new Set<string>();
  const classCandidatePattern = /(?:^|[\s"'`{(])([A-Za-z][A-Za-z0-9_:/.[\]()%,-]*)(?=$|[\s"'`})])/g;

  for (const file of sourceFiles(root)) {
    const source = readRequiredFile(file);
    for (const match of source.matchAll(classCandidatePattern)) {
      const className = match[1];
      if (semanticTokenForClassName(className, tokens)) {
        classes.add(className);
      }
    }
  }

  return classes;
}

test("dx-style supports launch template semantic theme tokens without Tailwind runtime", () => {
  const utility = readRequiredFile("related-crates/style/src/core/engine/utility/mod.rs");
  const themeTokens = readRequiredFile(
    "related-crates/style/src/core/engine/utility/theme_tokens.rs",
  );
  const dxStyleSupport = readRequiredFile("dx-www/src/cli/dx_style_support.rs");
  const cargo = readRequiredFile("related-crates/style/Cargo.toml");
  const globals = readRequiredFile("examples/template/styles/globals.css");
  const previewGlobals = readRequiredFile(".dx/template-app-browser-preview/styles/globals.css");
  const tokens = themeTokenNames(themeTokens);

  for (const marker of [
    "mod theme_tokens;",
    "theme_tokens::theme_color_token_name",
    "theme_tokens::is_color_like_custom_property_reference",
    'generate_utility_css("bg-background").unwrap()',
    'generate_utility_css("text-muted-foreground").unwrap()',
    'generate_utility_css("border-border").unwrap()',
    'generate_utility_css("border-input").unwrap()',
    'generate_utility_css("ring-ring").unwrap()',
    'generate_utility_css("bg-primary/90").unwrap()',
    'generate_utility_css("placeholder-muted-foreground").unwrap()',
    'generate_utility_css("text-current/70").unwrap()',
    'generate_utility_css("border-current/20").unwrap()',
    'generate_utility_css("bg-[var(--dx-scene-chip)]").unwrap()',
    'generate_utility_css("text-[var(--dx-scene-muted)]").unwrap()',
    'generate_utility_css("border-[var(--dx-scene-border)]").unwrap()',
    'generate_utility_css("shadow-[var(--dx-scene-shadow)]").unwrap()',
    'generate_utility_css("bg-chart-1").unwrap()',
    'generate_utility_css("stroke-chart-5/50").unwrap()',
    'generate_utility_css("bg-sidebar").unwrap()',
    'generate_utility_css("text-sidebar-foreground").unwrap()',
    'generate_utility_css("border-sidebar-border").unwrap()',
    'generate_utility_css("ring-sidebar-ring").unwrap()',
  ]) {
    assert.match(utility, escaped(marker));
  }

  for (const marker of [
    "pub(super) fn theme_color_token_name",
    "pub(super) fn is_color_like_custom_property_reference",
    '"background" => "background"',
    '"foreground" => "foreground"',
    '"card" => "card"',
    '"card-foreground" => "card-foreground"',
    '"primary" => "primary"',
    '"primary-foreground" => "primary-foreground"',
    '"muted" => "muted"',
    '"muted-foreground" => "muted-foreground"',
    '"border" => "border"',
    '"input" => "input"',
    '"ring" => "ring"',
    '"chart-1" => "chart-1"',
    '"chart-5" => "chart-5"',
    '"sidebar" => "sidebar"',
    '"sidebar-foreground" => "sidebar-foreground"',
    '"sidebar-primary" => "sidebar-primary"',
    '"sidebar-primary-foreground" => "sidebar-primary-foreground"',
    '"sidebar-accent" => "sidebar-accent"',
    '"sidebar-accent-foreground" => "sidebar-accent-foreground"',
    '"sidebar-border" => "sidebar-border"',
    '"sidebar-ring" => "sidebar-ring"',
    "is_color_like_custom_property_reference",
  ]) {
    assert.match(themeTokens, escaped(marker));
  }

  const forgePanels = [
    "examples/template/forge-golden-path-panel.tsx",
    "examples/template/forge-remote-head-health-panel.tsx",
    "examples/template/forge-safety-archive-panel.tsx",
  ]
    .map(readRequiredFile)
    .join("\n");

  for (const marker of [
    "text-current/60",
    "text-current/70",
    "text-current/80",
    "border-current/20",
  ]) {
    assert.match(forgePanels, escaped(marker));
  }

  const launchScene =
    readRequiredFile("examples/template/components/scene/launch-scene.tsx") +
    readRequiredFile("examples/template/launch-scene.tsx");

  for (const marker of [
    "bg-[var(--dx-scene-chip)]",
    "text-[var(--dx-scene-muted)]",
    "border-[var(--dx-scene-border)]",
    "border-[var(--dx-scene-control-border)]",
    "shadow-[var(--dx-scene-shadow)]",
  ]) {
    assert.match(launchScene, escaped(marker));
  }

  assert.match(dxStyleSupport, /style::core::css_for_class\(class_name\)\.is_some\(\)/);

  for (const [label, stylesheet] of [
    ["www-template globals.css", globals],
    ["materialized browser preview globals.css", previewGlobals],
  ] as const) {
    for (const token of REQUIRED_THEME_VARIABLES) {
      assert.match(stylesheet, escaped(token), `${label} must define ${token}`);
    }
  }

  const launchSemanticClasses = wwwTemplateSemanticTokenClasses(
    "examples/template",
    tokens,
  );

  for (const className of [
    "bg-background",
    "text-muted-foreground",
    "border-border",
    "border-input",
    "hover:bg-primary/90",
    "placeholder:text-muted-foreground",
  ]) {
    assert(
      launchSemanticClasses.has(className),
      `${className} should be present in www-template sources`,
    );
  }

  for (const className of launchSemanticClasses) {
    const token = semanticTokenForClassName(className, tokens);
    assert(token, `${className} should map to a known dx-style theme token`);
    assert.match(
      globals,
      escaped(`--${token}:`),
      `${className} uses --${token}, so www-template globals.css should define it`,
    );
  }

  assert.doesNotMatch(
    `${utility}\n${themeTokens}\n${cargo}`,
    /(?:cdn\.tailwindcss\.com|@tailwind\s|tailwindcss\s*=|"tailwindcss")/i,
  );
});
