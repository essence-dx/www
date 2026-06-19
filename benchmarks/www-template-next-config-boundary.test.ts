import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const nextConfigPath = path.join(root, "examples/template/next.config.mjs");
const sourceConfigPath = path.join(root, "examples/template/source.config.ts");
const fumadocsReadmePath = path.join(root, "examples/template/lib/fumadocs/README.md");
const fumadocsMetadataPath = path.join(root, "examples/template/lib/fumadocs/metadata.ts");
const fumadocsRouteContractPath = path.join(
  root,
  "examples/template/lib/fumadocs/route-contract.ts",
);
const packageCatalogPath = path.join(root, "examples/template/package-catalog.ts");
const packageStatusPath = path.join(root, "examples/template/.dx/forge/package-status.json");
const dxConfigPath = path.join(root, "examples/template/dx");

function readForgeFumadocsSource() {
  return fs.readFileSync(path.join(root, "core/src/ecosystem/forge_fumadocs.rs"), "utf8");
}

function extractRawRustStringConstant(source: string, name: string) {
  const match = source.match(new RegExp(`const ${name}: &str = r#"([\\s\\S]*?)"#;`));
  assert.ok(match, `${name} raw string constant should exist`);
  return match[1];
}

test("launch template keeps framework config in dx, not Next config files", () => {
  const source = readForgeFumadocsSource();

  assert.equal(fs.existsSync(nextConfigPath), false);
  assert.equal(fs.existsSync(sourceConfigPath), false);
  assert.doesNotMatch(source, /NEXT_CONFIG_MJS|SOURCE_CONFIG_TS/);
  assert.doesNotMatch(source, /js\/root\/next\.config\.mjs|js\/root\/source\.config\.ts/);
});

test("Fumadocs Forge README documents dx-owned config", () => {
  const source = readForgeFumadocsSource();
  const readmeTemplate = extractRawRustStringConstant(source, "FUMADOCS_README_MD");
  const materializedReadme = fs.readFileSync(fumadocsReadmePath, "utf8");

  for (const readme of [readmeTemplate, materializedReadme]) {
    assert.match(readme, /`dx` is the only generated project config file/);
    assert.doesNotMatch(readme, /dxWwwNextBoundary|createMDX\(\)|wraps the project config/i);
  }
});

test("Fumadocs route and metadata point at dx as the config owner", () => {
  const source = readForgeFumadocsSource();
  const metadataTemplate = extractRawRustStringConstant(source, "FUMADOCS_METADATA_TS");
  const materializedMetadata = fs.readFileSync(fumadocsMetadataPath, "utf8");
  const routeContract = fs.readFileSync(fumadocsRouteContractPath, "utf8");
  const packageCatalog = fs.readFileSync(packageCatalogPath, "utf8");
  const dxConfig = fs.readFileSync(dxConfigPath, "utf8");

  for (const metadata of [metadataTemplate, materializedMetadata]) {
    assert.match(metadata, /extensionless dx config owns WWW\/Fumadocs adapter settings/);
    assert.match(metadata, /typed DxConfig extracts framework\.www\.\* and framework\.fumadocs\.\* values/);
    assert.doesNotMatch(metadata, /next\.config\.mjs|source\.config\.ts|dxWwwNextBoundary/);
  }
  assert.match(routeContract, /configOwnerFile: "dx"/);
  assert.match(routeContract, /dxConfigKeys: \{/);
  assert.match(routeContract, /wwwConfigOwner: "framework\.www\.config_owner_file"/);
  assert.match(routeContract, /docsRoute: "framework\.fumadocs\.docs_route"/);
  assert.match(routeContract, /generatedRoutes: "framework\.fumadocs\.generated_routes"/);
  assert.doesNotMatch(routeContract, /nextConfigFile|sourceConfigFile/);
  assert.match(packageCatalog, /"dx config boundary"/);
  assert.doesNotMatch(packageCatalog, /"next\.config\.mjs"|"source\.config\.ts"|"dxWwwNextBoundary"/);
  assert.match(dxConfig, /www\(\s*app_dir=app\s+output_dir=\.dx\/www\/output\s*\)/s);
  assert.match(dxConfig, /docs\(\s*route=\/docs\s+content=content\/docs\s+openapi=openapi\/dx-www\.yaml\s*\)/s);
  assert.doesNotMatch(dxConfig, /fumadocs\(/);
  assert.doesNotMatch(dxConfig, /fumadocs_routes\[route\]\(/);
  assert.doesNotMatch(dxConfig, /config_owner_file=dx|config_files=\[\]/);
  assert.doesNotMatch(dxConfig, /\[project\]|\[package\]|\[forge\]$/m);
});

test("Fumadocs Forge source plugin template uses DX Icons instead of Lucide", () => {
  const source = readForgeFumadocsSource();
  const sourcePluginTemplate = extractRawRustStringConstant(source, "FUMADOCS_SOURCE_PLUGINS_TSX");
  const readmeTemplate = extractRawRustStringConstant(source, "FUMADOCS_README_MD");
  const materializedReadme = fs.readFileSync(fumadocsReadmePath, "utf8");

  for (const contents of [sourcePluginTemplate, readmeTemplate, materializedReadme]) {
    assert.doesNotMatch(contents, /lucide-react|lucideIconsPlugin|fumadocs-core\/source\/lucide-icons/i);
  }

  assert.match(sourcePluginTemplate, /from "\@\/components\/icons\/icon"/);
  assert.match(sourcePluginTemplate, /iconPlugin from fumadocs-core\/source\/icon/);
  assert.match(sourcePluginTemplate, /data-dx-docs-icon-source="dx-icons"/);
  assert.match(readmeTemplate, /DX Icon page-tree icons/);
});

test("Fumadocs Forge generator version matches the active locked package", () => {
  const source = readForgeFumadocsSource();
  const status = JSON.parse(fs.readFileSync(packageStatusPath, "utf8"));
  const fumadocsPackage = status.packages.find(
    (entry: { name: string }) => entry.name === "content/fumadocs-next",
  );

  assert.ok(fumadocsPackage, "content/fumadocs-next should stay in package-status");
  assert.match(
    source,
    new RegExp(`FUMADOCS_NEXT_VERSION: &str = "${fumadocsPackage.version}"`),
  );
  assert.doesNotMatch(source, /16\.8\.12-dx\.9/);
});

test("launch template TypeScript boundary does not require Next-generated artifacts", () => {
  const tsconfigPath = path.join(root, "examples/template/tsconfig.json");
  const tsconfig = JSON.parse(fs.readFileSync(tsconfigPath, "utf8"));

  assert.deepEqual(tsconfig.include, ["**/*.ts", "**/*.tsx"]);
  assert.ok(tsconfig.exclude.includes(".next"));
  assert.ok(tsconfig.exclude.includes("node_modules"));
  assert.ok(tsconfig.exclude.includes(".dx/forge/cache"));
  assert.ok(tsconfig.exclude.includes(".dx/forge/cache-archive"));
});
