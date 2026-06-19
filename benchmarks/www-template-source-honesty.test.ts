import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath: string) {
  return JSON.parse(read(relativePath));
}

function listFiles(relativeDir: string, predicate: (relativePath: string) => boolean) {
  const dir = path.join(root, relativeDir);
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  const files: string[] = [];

  for (const entry of entries) {
    const relativePath = `${relativeDir}/${entry.name}`;

    if (entry.isDirectory()) {
      files.push(...listFiles(relativePath, predicate));
      continue;
    }

    if (predicate(relativePath)) {
      files.push(relativePath);
    }
  }

  return files;
}

function catalogPackageIds() {
  return [
    ...read("examples/template/package-catalog.ts").matchAll(
      /packageId:\s*"([^"]+)"/g,
    ),
  ].map((match) => match[1]);
}

function packageCatalogEntries() {
  const source = read("examples/template/package-catalog.ts");
  const catalogStart = source.indexOf("export const launchPackageCatalog = [");
  const catalogEnd = source.indexOf(
    "] as const satisfies readonly LaunchPackageCatalogItem[]",
    catalogStart,
  );
  assert.notEqual(catalogStart, -1, "launchPackageCatalog start should exist");
  assert.notEqual(catalogEnd, -1, "launchPackageCatalog end should exist");
  const catalogSource = source.slice(catalogStart, catalogEnd);
  const entries: {
    packageId: string;
    officialName?: string;
    officialPackageName?: string;
    maturity?: string;
    honestyLabel?: string;
  }[] = [];
  const packageMatches = [...catalogSource.matchAll(/^\s*\{\s*\n\s*packageId:\s*"([^"]+)"/gm)];

  for (let index = 0; index < packageMatches.length; index += 1) {
    const match = packageMatches[index];
    const start = match.index ?? 0;
    const end = packageMatches[index + 1]?.index ?? catalogSource.length;
    const body = catalogSource.slice(start, end);
    entries.push({
      packageId: match[1],
      officialName: body.match(/officialName:\s*"([^"]+)"/)?.[1],
      officialPackageName: body.match(/officialPackageName:\s*"([^"]+)"/)?.[1],
      maturity: body.match(/maturity:\s*"([^"]+)"/)?.[1],
      honestyLabel: body.match(/honestyLabel:\s*"([^"]+)"/)?.[1],
    });
  }

  return entries;
}

function launchPackageMaturityLiterals() {
  const source = read("examples/template/package-catalog.ts");
  const unionBody = source.match(
    /export type LaunchPackageMaturity =([\s\S]*?);/,
  )?.[1];

  assert.ok(unionBody, "LaunchPackageMaturity union should be exported");

  return new Set(
    [...unionBody.matchAll(/"([^"]+)"/g)].map((match) => match[1]),
  );
}

function launchPackageHonestyLiterals() {
  const source = read("examples/template/package-catalog.ts");
  const unionBody = source.match(/honestyLabel\?:([\s\S]*?);/)?.[1];

  assert.ok(unionBody, "LaunchPackageCatalogItem.honestyLabel union should be typed");

  return new Set(
    [...unionBody.matchAll(/"([^"]+)"/g)].map((match) => match[1]),
  );
}

const extraAiRouteNames = [
  "agent",
  "image",
  "object",
  "speech",
  "text-stream",
  "transcribe",
  "ui-stream",
  "upload-file",
  "video",
];

test("launch package catalog count matches the TypeScript catalog source", () => {
  const packageIds = catalogPackageIds();
  const uniquePackageIds = new Set(packageIds);
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const readModel = read("examples/template/forge-package-status-read-model.ts");

  assert.equal(packageIds.length, uniquePackageIds.size);
  assert.equal(status.catalog_package_count, packageIds.length);
  assert.match(readModel, new RegExp(`catalogPackageCount: ${packageIds.length}`));
  assert.doesNotMatch(readModel, /catalogPackageCount: 31/);
});

test("launch package catalog entries carry DX names and typed maturity labels", () => {
  const entries = packageCatalogEntries();
  const maturityLiterals = launchPackageMaturityLiterals();
  const missingNames = entries
    .filter((entry) => !entry.officialName && !entry.officialPackageName)
    .map((entry) => entry.packageId);
  const untypedMaturities = entries
    .filter((entry) => entry.maturity && !maturityLiterals.has(entry.maturity))
    .map((entry) => `${entry.packageId}:${entry.maturity}`);

  assert.deepEqual(missingNames, []);
  assert.deepEqual(untypedMaturities, []);
});

test("launch package catalog entries carry explicit honesty labels", () => {
  const entries = packageCatalogEntries();
  const honestyLiterals = launchPackageHonestyLiterals();
  const missingHonestyLabels = entries
    .filter((entry) => !entry.honestyLabel)
    .map((entry) => entry.packageId);
  const untypedHonestyLabels = entries
    .filter((entry) => entry.honestyLabel && !honestyLiterals.has(entry.honestyLabel))
    .map((entry) => `${entry.packageId}:${entry.honestyLabel}`);

  assert.deepEqual(missingHonestyLabels, []);
  assert.deepEqual(untypedHonestyLabels, []);
});

test("legacy backend package claims are removed from launch runtime surfaces", () => {
  const surfaces = [
    "tools/launch/runtime-template/pages/backend.html",
    ".dx/template-app-browser-preview/pages/backend.html",
    "examples/template/public/preview-manifest.json",
    ".dx/template-app-browser-preview/public/preview-manifest.json",
    "tools/launch/materialize-www-template.ts",
  ];

  for (const surface of surfaces) {
    const source = read(surface);
    assert.doesNotMatch(source, /backend\/convex-compatible/);
    assert.doesNotMatch(source, /convex-compatible-proof/);
    assert.doesNotMatch(source, /Convex-style/i);
  }
});

test("Instant cursor package uses DX theme tokens instead of public hardcoded colors", () => {
  const source = read("examples/template/components/instant/instant-cursors.tsx");

  assert.doesNotMatch(source, /#2563eb/i);
  assert.match(source, /var\(--dx-instant-cursor-color/);
});

test("launch template favicon avoids hardcoded black and white fills", () => {
  const source = read("tools/launch/runtime-template/assets/favicon.svg");

  assert.doesNotMatch(source, /fill="#(?:fff|ffffff|000|000000)"/i);
  assert.match(source, /fill="currentColor"/);
  assert.match(source, /color="CanvasText"/);
});

test("extra AI package routes are explicitly outside the proven default AI surface", () => {
  const metadata = read("examples/template/lib/ai/metadata.ts");
  const providerBoundary = read("examples/template/lib/ai/provider-boundary.ts");
  const packageReadme = read("examples/template/lib/ai/README.md");

  assert.match(metadata, /outsideDefaultSurfaceRoutes/);
  assert.match(providerBoundary, /DX_ENABLE_EXTENDED_AI_ROUTES/);
  assert.match(providerBoundary, /outside-default-ai-surface/);
  assert.match(packageReadme, /Default Launch Proof/);
  assert.match(packageReadme, /Extended Route Boundaries/);
  assert.match(packageReadme, /outside the proven default AI surface/);

  for (const routeName of extraAiRouteNames) {
    const routePath = `examples/template/app/api/ai/${routeName}/route.ts`;
    const source = read(routePath);

    assert.match(source, /createDxAiExtendedRouteDisabledResponse/);
    assert.match(source, /isDxAiExtendedRouteEnabled/);
    assert.match(source, /outside the default launch AI proof surface/);
    assert.match(metadata, new RegExp(`app/api/ai/${routeName}/route\\.ts`));
  }
});

test("launch template root carries honest operator docs and a TypeScript boundary", () => {
  const readme = read("examples/template/README.md");
  const todo = read("examples/template/TODO.md");
  const changelog = read("examples/template/CHANGELOG.md");
  const packageJson = readJson("examples/template/package.json");
  const tsconfig = readJson("examples/template/tsconfig.json");

  assert.match(readme, /98\/100/);
  assert.match(readme, /490\/500/);
  assert.match(readme, /21 Forge package lanes/);
  assert.match(readme, /0 live provider credentials/);
  assert.doesNotMatch(readme, /100\/100|production runtime proof|live provider proof/i);

  assert.match(todo, /browser route proof/i);
  assert.match(todo, /provider credentials/i);
  assert.match(todo, /large generated read model/i);

  assert.match(changelog, /2026-05-23/);
  assert.match(changelog, /not browser runtime proof/i);
  assert.match(changelog, /not live provider proof/i);
  assert.doesNotMatch(
    `${readme}\n${todo}\n${changelog}`,
    /private `typecheck` script|package script boundary/i,
  );

  assert.equal(packageJson.private, true);
  assert.equal(packageJson.type, "module");
  assert.equal(packageJson.scripts, undefined);
  assert.equal(packageJson.dependencies, undefined);
  assert.equal(packageJson.devDependencies, undefined);
  assert.deepEqual(packageJson.dxWwwBoundary, {
    schema: "dx.www.template.package_boundary",
    format: 1,
    foundation: "dx-www-forge",
    nodeModulesRequired: false,
    packageManagerLifecycleRequired: false,
    runtimeAdapterRequired: false,
  });
  assert.doesNotMatch(
    JSON.stringify(packageJson),
    /\b(tsc|next|npm|pnpm|yarn|bun|tailwindcss|postcss)\b/i,
  );

  assert.equal(tsconfig.compilerOptions.noEmit, true);
  assert.equal(tsconfig.compilerOptions.strict, true);
  assert.deepEqual(tsconfig.compilerOptions.paths["@/*"], ["./*"]);
  assert.ok(tsconfig.exclude.includes(".dx/forge/cache"));
  assert.ok(tsconfig.exclude.includes(".dx/forge/cache-archive"));
});

test("launch template markdown keeps package installs app-owned", () => {
  const markdownFiles = listFiles("examples/template", (relativePath) =>
    relativePath.endsWith(".md"),
  );

  assert.ok(markdownFiles.length > 0, "launch template markdown docs should be checked");

  for (const markdownFile of markdownFiles) {
    const source = read(markdownFile);

    assert.doesNotMatch(
      source,
      /^\s*(?:npm\s+(?:install|i)|pnpm\s+(?:install|add)|yarn\s+(?:install|add)|bun\s+(?:install|add))\b/im,
      `${markdownFile} must keep dependency installation app-owned, not a template-local default`,
    );
  }
});

test("launch runtime form validation contract does not overclaim browser runtime proof", () => {
  const runtimeSources = [
    "tools/launch/runtime-template/assets/launch-runtime.ts",
  ];

  for (const runtimePath of runtimeSources) {
    const runtime = read(runtimePath);

    assert.doesNotMatch(
      runtime,
      /runtimeProof:\s*"source-owned-browser-preview"/,
      `${runtimePath} must not claim browser runtime proof`,
    );
    assert.match(runtime, /runtimeProof:\s*false/);
    assert.match(runtime, /sourceProof:\s*"source-owned-static-preview"/);
  }
});

test("receipt hash helpers run through the DX helper runner under module templates", () => {
  const runnerPath = path.join(root, "tools/launch/run-template-receipt-helper.js");
  const helperPath = "examples/template/ai-sdk-receipt-hashes.ts";

  assert.ok(fs.existsSync(runnerPath), "DX receipt helper runner should exist");

  const result = spawnSync(process.execPath, [runnerPath, helperPath, "--help"], {
    cwd: root,
    encoding: "utf8",
  });

  assert.equal(result.status, 0, result.stdout + result.stderr);
  assert.match(result.stdout, /AI SDK launch assistant receipt/);
  assert.doesNotMatch(result.stderr, /require is not defined in ES module scope/);

  const checkResult = spawnSync(
    process.execPath,
    [runnerPath, helperPath, "--check", "--json"],
    {
      cwd: root,
      encoding: "utf8",
    },
  );
  assert.notEqual(checkResult.status, 2, checkResult.stdout + checkResult.stderr);
  assert.doesNotMatch(checkResult.stderr, /require is not defined in ES module scope/);

  const report = JSON.parse(checkResult.stdout);
  assert.equal(report.helper_path, helperPath);
  assert.equal(
    report.json_check_command,
    `node tools/launch/run-template-receipt-helper.js ${helperPath} --check --json`,
  );
  assert.equal(
    report.write_command,
    `node tools/launch/run-template-receipt-helper.js ${helperPath} --write`,
  );
  assert.equal(report.runtime_execution, false);

  const helperSources = fs
    .readdirSync(path.join(root, "examples/template"))
    .filter((name) => name.endsWith("-receipt-hashes.ts"))
    .map((name) => ({
      name,
      source: read(`examples/template/${name}`),
    }));

  for (const helper of helperSources) {
    assert.doesNotMatch(
      helper.source,
      /(?:check_command|write_command|json_check_command): `node \$\{HELPER_PATH\}/,
      `${helper.name} must not publish direct template-local node commands`,
    );
    assert.match(
      helper.source,
      /tools\/launch\/run-template-receipt-helper\.js/,
      `${helper.name} should publish the DX receipt helper runner boundary`,
    );
  }
});

test("launch package-lane surfaces publish runner-backed receipt commands", () => {
  const surfaces = [
    "examples/template/template-shell.tsx",
    "tools/launch/runtime-template/pages/index.html",
    ".dx/template-app-browser-preview/pages/index.html",
  ];

  for (const surface of surfaces) {
    const source = read(surface);
    const directCommands = [
      ...source.matchAll(
        /node examples\/template\/[^"\n]+-receipt-hashes\.ts --check --json/g,
      ),
    ].map((match) => match[0]);

    assert.deepEqual(
      directCommands,
      [],
      `${surface} must not publish direct template-local receipt helper commands`,
    );
    assert.match(
      source,
      /node tools\/launch\/run-template-receipt-helper\.js examples\/template\/[^"\n]+-receipt-hashes\.ts --check --json/,
      `${surface} should expose the DX receipt helper runner command`,
    );
  }
});
