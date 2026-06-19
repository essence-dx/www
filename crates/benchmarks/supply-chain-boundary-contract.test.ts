import assert from "node:assert/strict";
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

function walkFiles(relativePath: string): string[] {
  const absolutePath = path.join(root, relativePath);
  if (!fs.existsSync(absolutePath)) {
    return [];
  }
  const stat = fs.statSync(absolutePath);
  if (stat.isFile()) {
    return [relativePath.replaceAll("\\", "/")];
  }

  const ignoredDirectories = new Set([
    ".dx",
    ".git",
    ".next",
    "dist",
    "node_modules",
  ]);
  const files: string[] = [];
  for (const entry of fs.readdirSync(absolutePath, { withFileTypes: true })) {
    if (entry.isDirectory() && ignoredDirectories.has(entry.name)) {
      continue;
    }
    const child = path.join(relativePath, entry.name).replaceAll("\\", "/");
    if (entry.isDirectory()) {
      files.push(...walkFiles(child));
    } else if (entry.isFile()) {
      files.push(child);
    }
  }
  return files.sort();
}

test("schema noise guard covers public supply-chain boundary surfaces", () => {
  const schemaNoiseCheck = read("tools/next-rust-merge/schema-status-noise-check.cjs");

  assert.match(schemaNoiseCheck, /"dx-www\/src\/cli\/app_router_build_command\.rs"/);
  assert.match(schemaNoiseCheck, /"examples\/template\/package\.json"/);
});

test("generated default styles do not import remote font or CDN CSS", () => {
  const newCommand = read("dx-www/src/cli/new_command.rs");
  const defaultTemplateSources = read("dx-www/src/cli/default_template_sources.rs");

  for (const source of [newCommand, defaultTemplateSources]) {
    assert.doesNotMatch(
      source,
      /@import\s+url\(["']https?:\/\/(?:fonts\.googleapis\.com|cdn\.tailwindcss\.com|unpkg\.com|cdn\.jsdelivr\.net|esm\.sh|cdn\.skypack\.dev|cdnjs\.cloudflare\.com)/i,
    );
    assert.doesNotMatch(source, /https:\/\/fonts\.(?:googleapis|gstatic)\.com/i);
    assert.doesNotMatch(source, /<link[^>]+href="https:\/\/fonts\.(?:googleapis|gstatic)\.com/i);
  }
});

test("public DX package and route discovery schemas avoid .v1 suffixes", () => {
  const appRouterBuildCommand = read("dx-www/src/cli/app_router_build_command.rs");
  const wwwTemplatePackage = readJson("examples/template/package.json");

  assert.doesNotMatch(appRouterBuildCommand, /"schema":\s*"dx\.[^"]+\.v1"/);
  assert.match(appRouterBuildCommand, /"schema": "dx\.app-router\.route-discovery"/);
  assert.match(appRouterBuildCommand, /"format": 1/);

  assert.deepEqual(wwwTemplatePackage.dxWwwBoundary, {
    schema: "dx.www.template.package_boundary",
    format: 1,
    foundation: "dx-www-forge",
    nodeModulesRequired: false,
    packageManagerLifecycleRequired: false,
    runtimeAdapterRequired: false,
  });
  assert.doesNotMatch(JSON.stringify(wwwTemplatePackage.dxWwwBoundary), /\.v1/);
  assert.equal(wwwTemplatePackage.scripts, undefined);
  assert.equal(wwwTemplatePackage.dependencies, undefined);
  assert.equal(wwwTemplatePackage.devDependencies, undefined);
});

test("template auth boundary does not advertise live package-manager migration commands", () => {
  const authBoundary = read("examples/template/server/auth/better-auth.ts");

  assert.doesNotMatch(authBoundary, /\bnpx\s+auth@latest\b/);
  assert.doesNotMatch(authBoundary, /\bnpm\s+(?:install|exec|run)\b/);
  assert.match(authBoundary, /migrationBoundary:\s*\{/);
  assert.match(authBoundary, /owner:\s*"app"/);
  assert.match(authBoundary, /packageManagerLifecycleRequired:\s*false/);
  assert.match(authBoundary, /dxExecutesMigrationCli:\s*false/);
  assert.match(authBoundary, /reviewedAdapterPlan:\s*"dx forge import npm better-auth --plan"/);
});

test("launch template runtime surfaces do not execute remote package-manager or CDN paths", () => {
  const runtimeSurfaceRoots = ["examples/template"];
  const checkedExtensions = new Set([
    ".css",
    ".html",
    ".js",
    ".json",
    ".jsx",
    ".mjs",
    ".ts",
    ".tsx",
  ]);
  const forbiddenPatterns = [
    {
      label: "live package-manager command",
      pattern: /\b(?:npx|npm|pnpm|yarn|bun)\s+(?:install|add|exec|dlx|run|create|auth)\b/i,
    },
    {
      label: "remote runtime CDN",
      pattern:
        /\b(?:cdn\.tailwindcss\.com|unpkg\.com|cdn\.jsdelivr\.net|esm\.sh|cdn\.skypack\.dev|cdnjs\.cloudflare\.com)\b/i,
    },
    {
      label: "remote script tag",
      pattern: /<script\b[^>]+src=["']https?:\/\//i,
    },
    {
      label: "remote stylesheet tag",
      pattern: /<link\b(?=[^>]+\brel=["']stylesheet["'])(?=[^>]+\bhref=["']https?:\/\/)[^>]*>/i,
    },
  ];
  const remoteStylesheetPattern = forbiddenPatterns.find(
    (entry) => entry.label === "remote stylesheet tag",
  )?.pattern;

  assert.ok(remoteStylesheetPattern, "expected remote stylesheet tag pattern");
  assert.match(
    '<link href="https://cdn.jsdelivr.net/npm/pkg/style.css" rel="stylesheet">',
    remoteStylesheetPattern,
    "remote stylesheet detection should not depend on attribute order",
  );

  const files = runtimeSurfaceRoots
    .flatMap((runtimeRoot) => walkFiles(runtimeRoot))
    .filter((file) => checkedExtensions.has(path.extname(file)))
    .filter((file) => !file.endsWith("/README.md"));
  assert.ok(files.length > 20, "expected launch template runtime files to be scanned");
  assert.ok(
    files.includes("examples/template/template-shell.tsx"),
    "supply-chain scan should include the root launch shell surface",
  );
  assert.ok(
    files.includes("examples/template/package-catalog.ts"),
    "supply-chain scan should include root package catalog surfaces",
  );

  const offenders = [];
  for (const file of files) {
    const source = read(file);
    for (const { label, pattern } of forbiddenPatterns) {
      if (pattern.test(source)) {
        offenders.push(`${file}: ${label}`);
      }
    }
  }

  assert.deepEqual(offenders, []);
});
