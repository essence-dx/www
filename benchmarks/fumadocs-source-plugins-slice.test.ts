const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "fumadocs");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

const forge = read(path.join(root, "core", "src", "ecosystem", "forge_fumadocs.rs"));
const docsStatus = read(path.join(root, "examples", "template", "docs-status.tsx"));
const packageCatalog = read(path.join(root, "examples", "template", "package-catalog.ts"));
const cli = read(path.join(root, "dx-www", "src", "cli", "mod.rs"));
const scorecard = read(path.join(root, "core", "src", "ecosystem", "forge_scorecard.rs"));
const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));
const trustPolicy = read(path.join(root, "core", "src", "ecosystem", "forge_trust_policy.rs"));

test("upstream fumadocs exposes real source plugin APIs", () => {
  const packageJson = read(path.join(mirror, "packages", "core", "package.json"));
  const statusBadges = read(
    path.join(mirror, "packages", "core", "src", "source", "plugins", "status-badges.tsx"),
  );
  const lucideIcons = read(
    path.join(mirror, "packages", "core", "src", "source", "plugins", "lucide-icons.ts"),
  );
  const slugs = read(path.join(mirror, "packages", "core", "src", "source", "plugins", "slugs.ts"));
  const pluginDocs = read(
    path.join(
      mirror,
      "apps",
      "docs",
      "content",
      "docs",
      "headless",
      "source-api",
      "plugins.mdx",
    ),
  );

  assert.match(packageJson, /"\.\/source\/plugins\/lucide-icons"/);
  assert.match(packageJson, /"\.\/source\/plugins\/status-badges"/);
  assert.match(packageJson, /"\.\/source\/plugins\/slugs"/);
  assert.match(statusBadges, /export function statusBadgesPlugin/);
  assert.match(statusBadges, /renderBadge\?: \(status: string\) => ReactNode/);
  assert.match(statusBadges, /\(node as Item\)\.status = status/);
  assert.match(lucideIcons, /export function lucideIconsPlugin/);
  assert.match(lucideIcons, /defaultIcon\?: keyof typeof icons/);
  assert.match(slugs, /export function slugsPlugin/);
  assert.match(slugs, /export function slugsFromData/);
  assert.match(pluginDocs, /plugins: \[lucideIconsPlugin\(\)\]/);
});

test("fumadocs slice materializes source plugins and schema fields", () => {
  assert.match(forge, /js\/lib\/fumadocs\/source-plugins\.tsx/);
  assert.match(forge, /import \{ lucideIconsPlugin \} from "fumadocs-core\/source\/lucide-icons"/);
  assert.match(forge, /import \{ statusBadgesPlugin \} from "fumadocs-core\/source\/status-badges"/);
  assert.match(forge, /import \{ slugsFromData \} from "fumadocs-core\/source\/slugs"/);
  assert.match(forge, /dxFumadocsSourcePluginContract/);
  assert.match(forge, /dxFumadocsSourcePlugins/);
  assert.match(forge, /dxFumadocsSlugFn/);
  assert.match(forge, /statusBadgesPlugin\(\{/);
  assert.match(forge, /lucideIconsPlugin\(\{ defaultIcon: dxFumadocsSourcePluginContract\.defaultIcon \}\)/);
  assert.match(forge, /slugsFromData\(dxFumadocsSourcePluginContract\.slugField\)/);
  assert.match(forge, /\.\.\.dxFumadocsSourcePlugins/);
  assert.match(forge, /slugs: dxFumadocsSlugFn/);
  assert.match(forge, /pageSchema\.extend/);
  assert.match(forge, /status: z\.enum/);
  assert.match(forge, /slug: z\.string\(\)\.optional\(\)/);
  assert.match(forge, /status: beta/);
  assert.match(forge, /icon: action:rocket/);
  assert.match(forge, /slug: ""/);
});

test("fumadocs source plugin metadata is discoverable and honest", () => {
  assert.match(forge, /sourcePluginFile: "lib\/fumadocs\/source-plugins\.tsx"/);
  assert.match(forge, /sourcePluginFrontmatterFields: \["icon", "status", "slug"\]/);
  assert.match(packageCatalog, /source plugin taxonomy/);
  assert.match(scorecard, /source plugin frontmatter/);
  assert.match(security, /lucideIconsPlugin/);
  assert.match(security, /statusBadgesPlugin/);
  assert.match(trustPolicy, /source plugin taxonomy/);
  assert.match(cli, /dxFumadocsSourcePluginContract/);
});

test("launch template consumes the fumadocs source plugin contract", () => {
  assert.match(docsStatus, /dxFumadocsSourcePluginContract/);
  assert.match(docsStatus, /data-dx-docs-source-plugins/);
  assert.match(docsStatus, /source plugin fields/);
});
