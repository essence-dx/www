import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const runtimeTemplatePagesRoot = path.join(root, "tools", "launch", "runtime-template", "pages");
const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function routeFromPageFile(file) {
  const pageName = path.basename(file, ".html");
  return pageName === "index" ? "/" : `/${pageName}`;
}

function collectManifestRoutes(value, routes = new Set()) {
  if (Array.isArray(value)) {
    for (const item of value) {
      collectManifestRoutes(item, routes);
    }
    return routes;
  }

  if (!value || typeof value !== "object") {
    return routes;
  }

  for (const [key, child] of Object.entries(value)) {
    if (key === "route" && typeof child === "string" && child.startsWith("/")) {
      routes.add(normalizeRoute(child));
      continue;
    }

    if (key === "routes" && Array.isArray(child)) {
      for (const route of child) {
        if (typeof route === "string" && route.startsWith("/")) {
          routes.add(normalizeRoute(route));
        }
      }
      continue;
    }

    collectManifestRoutes(child, routes);
  }

  return routes;
}

function collectPageRoutes() {
  const routes = new Set();
  for (const entry of fs.readdirSync(runtimeTemplatePagesRoot, { withFileTypes: true })) {
    if (entry.isFile() && entry.name.endsWith(".html")) {
      routes.add(normalizeRoute(routeFromPageFile(entry.name)));
    }
  }
  return routes;
}

function normalizeRoute(route) {
  const url = new URL(route, "https://dx.local");
  let routePath = decodeURIComponent(url.pathname);
  if (routePath.length > 1 && routePath.endsWith("/")) {
    routePath = routePath.slice(0, -1);
  }
  return routePath;
}

function collectLocaleRouteReferences(source) {
  return [
    ...source.matchAll(/routePreview:\s*"([^"]+)"/g),
    ...source.matchAll(/href:\s*"([^"]+)"[^}]*locale:\s*"bn"/g),
    ...source.matchAll(/data-dx-intl-route-preview="([^"]+)"/g),
    ...source.matchAll(/data-dx-intl-alternate-href="([^"]+)"/g),
  ].map((match) => match[1]).filter((route) => route.startsWith("/"));
}

function assertLocaleReferencesStayMaterialized(entries, materializedRoutes) {
  for (const [label, source] of entries) {
    assert.doesNotMatch(source, /\/bn\/launch\b/, `${label} must not link to an unmaterialized locale route`);

    for (const reference of collectLocaleRouteReferences(source)) {
      assert.ok(
        materializedRoutes.has(normalizeRoute(reference)),
        `${label} references ${reference}, but that route is not materialized`,
      );
    }
  }
}

test("launch template locale route references stay inside materialized template routes", () => {
  const manifest = JSON.parse(read("examples/template/public/preview-manifest.json"));
  const materializedRoutes = new Set([
    ...collectManifestRoutes(manifest),
    ...collectPageRoutes(),
  ]);

  const files = [
    "tools/launch/runtime-template/assets/launch-runtime.ts",
    "tools/launch/runtime-template/pages/index.html",
  ];

  assertLocaleReferencesStayMaterialized(
    files.map((file) => [file, read(file)]),
    materializedRoutes,
  );

  const runtime = read("tools/launch/runtime-template/assets/launch-runtime.ts");
  assert.match(runtime, /routePreview:\s*"\/\?locale=bn"/);
  assert.match(runtime, /href:\s*"\/\?locale=bn"[^}]*locale:\s*"bn"/);
});

test("materialized launch template keeps locale previews on generated routes", () => {
  const outputDir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-launch-link-integrity-"));
  try {
    execFileSync(process.execPath, [materializer, outputDir], { cwd: root, stdio: "pipe" });

    const manifest = JSON.parse(fs.readFileSync(path.join(outputDir, "public", "preview-manifest.json"), "utf8"));
    const materializedRoutes = collectManifestRoutes(manifest);
    for (const entry of fs.readdirSync(path.join(outputDir, "pages"), { withFileTypes: true })) {
      if (entry.isFile() && entry.name.endsWith(".html")) {
        materializedRoutes.add(normalizeRoute(routeFromPageFile(entry.name)));
      }
    }

    assertLocaleReferencesStayMaterialized(
      [
        ["materialized pages/index.html", fs.readFileSync(path.join(outputDir, "pages", "index.html"), "utf8")],
        [
          "materialized public/launch-runtime.js",
          fs.readFileSync(path.join(outputDir, "public", "launch-runtime.js"), "utf8"),
        ],
      ],
      materializedRoutes,
    );
  } finally {
    fs.rmSync(outputDir, { recursive: true, force: true });
  }
});
