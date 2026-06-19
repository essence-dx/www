const assert = require("assert");
const fs = require("fs");
const os = require("os");
const path = require("path");
const test = require("node:test");

const { buildReport, renderMarkdown, routeDefinitions } = require("./measure-forge-adoption-browser-smoke.ts");

function writeFixtureProject() {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forge-adoption-browser-"));
  const publicDir = path.join(dir, "public");
  for (const route of routeDefinitions) {
    writeFile(
      path.join(publicDir, route.html),
      `<html><head><title>${route.route}</title></head><body><h1>${route.route}</h1><a href="/forge">Forge</a></body></html>`
    );
    writeFile(
      path.join(publicDir, route.clean_html),
      `<html><body><h1>${route.route}</h1><a href="/forge">Forge</a></body></html>`
    );
    for (const artifact of route.artifacts) {
      writeFile(path.join(publicDir, artifact), artifact.endsWith(".json") ? "{}" : "DXPK");
    }
  }
  return dir;
}

function writeFile(filePath, content) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, content);
}

test("adoption browser smoke inspects static routes and artifacts without node_modules", async () => {
  const projectDir = writeFixtureProject();
  const report = await buildReport({
    projectDir,
    generatedAt: "2026-05-17T00:00:00.000Z",
    prepare: false,
    measure: false,
  });
  const markdown = renderMarkdown(report);

  assert.equal(report.score, 96);
  assert.equal(report.passed, true);
  assert.equal(report.no_node_modules, true);
  assert.equal(report.route_count, routeDefinitions.length);
  assert.equal(report.routes.every((route) => route.passed), true);
  assert.match(markdown, /Forge Adoption Browser Smoke/);
  assert.match(markdown, /Static\/no-runtime/);
});

test("adoption browser smoke reports node_modules and missing route artifacts", async () => {
  const projectDir = writeFixtureProject();
  fs.mkdirSync(path.join(projectDir, "node_modules"));
  fs.rmSync(path.join(projectDir, "public", "forge", "scorecard.dxp"), { force: true });

  const report = await buildReport({
    projectDir,
    generatedAt: "2026-05-17T00:00:00.000Z",
    prepare: false,
    measure: false,
  });

  assert.equal(report.passed, false);
  assert.equal(report.no_node_modules, false);
  assert.ok(report.findings.some((finding) => finding.includes("node_modules")));
  assert.ok(report.findings.some((finding) => finding.includes("scorecard.dxp")));
});
