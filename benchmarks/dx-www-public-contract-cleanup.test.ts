const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("public framework contract is TSX-only and does not advertise legacy page/component extensions", () => {
  const readme = read("README.md");
  const frameworkDocs = read("docs/DX_WWW_FRAMEWORK_STRUCTURE.md");
  const completeness = read("examples/template/framework-completeness.ts");
  const publicContract = `${readme}\n${frameworkDocs}\n${completeness}`;

  assert.doesNotMatch(publicContract, new RegExp("\\." + "pg\\b"));
  assert.doesNotMatch(publicContract, new RegExp("\\." + "cp\\b"));
  assert.doesNotMatch(publicContract, /\.dxob\b/);
  assert.doesNotMatch(publicContract, /legacy_page_formats|internalLegacyFormats/);
  assert.match(completeness, /publicAuthoring: "tsx-app-router"/);
  assert.match(completeness, /packagePolicy: "forge-source-owned-visible-files"/);
});

test("Authentication owns provider/plugin surfaces instead of standalone Google OAuth package lane", () => {
  const catalog = read("examples/template/package-catalog.ts");
  const registry = read("core/src/ecosystem/forge_registry.rs");
  const cli = read("dx-www/src/cli/mod.rs");
  const standaloneAuthProviderPackage = "auth/" + "google";
  const standaloneAuthProviderPattern = new RegExp(`"${standaloneAuthProviderPackage}"`);

  assert.doesNotMatch(catalog, new RegExp(`packageId: "${standaloneAuthProviderPackage}"`));
  assert.doesNotMatch(registry, standaloneAuthProviderPattern);
  assert.doesNotMatch(cli, standaloneAuthProviderPattern);
  assert.match(catalog, /packageId: "auth\/better-auth"[\s\S]*providerSurfaces: \[/);
  assert.match(catalog, /"google-oauth"/);
});

test("framework completeness uses official DX package names, not upstream implementation names", () => {
  const completeness = read("examples/template/framework-completeness.ts");

  const requiredLabels = [
    "Authentication",
    "Validation & Schemas",
    "Forms",
    "State Management",
    "Data Fetching & Cache",
    "Database ORM",
    "Backend Platform Client",
    "Payments",
    "Internationalization",
    "Markdown & MDX Content",
    "Motion & Animation",
    "UI Components and Icons",
  ];

  for (const label of requiredLabels) {
    assert.match(completeness, new RegExp(`"${label.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}"`));
  }

  for (const staleLabel of [
    '"better-auth"',
    '"zod"',
    '"zustand"',
    '"Query"',
    '"drizzle"',
    '"supabase"',
    '"stripe"',
    '"next-intl"',
    '"markdown and MDX"',
    '"motion"',
    '"shadcn, radix, and icons"',
  ]) {
    assert.doesNotMatch(completeness, new RegExp(staleLabel.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }
});

test("Icon is the front-facing component and icon is the DX-owned source tag", () => {
  const cli = [
    read("dx-www/src/cli/mod.rs"),
    read("dx-www/src/cli/new_command.rs"),
    read("dx-www/src/cli/public_framework_tools.rs"),
    read("examples/template/dx"),
  ].join("\n");
  const coreIcons = read("core/src/ecosystem/icons.rs");
  const publicContract = read("benchmarks/public-framework-contract.test.ts");

  assert.match(cli, /icons\(\s*component=Icon/s);
  assert.match(cli, /source_tag=icon/);
  assert.match(cli, /runtime_tag=dx-icon/);
  assert.match(coreIcons, /<\(\?:icon\|dx-icon\|Icon\|DxIcon\)/);
  assert.match(publicContract, /source_tag=icon/);
  assert.doesNotMatch(cli, /components\/icons\/dx-icons\.tsx/);
});
