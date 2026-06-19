const assert = require("assert");
const fs = require("fs");
const os = require("os");
const path = require("path");
const test = require("node:test");

const { buildReport, renderMarkdown } = require("./compare-forge-live-frameworks.ts");

function staticReport() {
  return {
    generated_at: "2026-05-17T00:00:00.000Z",
    source_route_comparison: "benchmarks/reports/forge-public-route-comparison.json",
    required_routes: ["/forge", "/forge/scorecard"],
    scope: {
      competitor_builds_not_run: true,
      no_package_install: true,
      no_node_modules_created: true,
    },
    frameworks: [
      {
        framework: "DX-WWW",
        total_brotli_bytes: 2000,
      },
      {
        framework: "Astro",
        total_brotli_bytes: 1200,
      },
      {
        framework: "Svelte",
        total_brotli_bytes: 1300,
      },
      {
        framework: "Next.js",
        total_brotli_bytes: 1400,
      },
    ],
  };
}

test("live framework harness keeps live builds opt-in and separate", () => {
  const report = buildReport({
    generatedAt: "2026-05-17T00:00:00.000Z",
    staticReport: staticReport(),
    liveEnabled: false,
    profiles: [
      {
        framework: "Astro",
        project_dir: "missing",
        build_command: ["npm", ["run", "build"]],
        output_dirs: ["dist"],
        static_floor_id: "astro-static-floor",
      },
    ],
  });
  const markdown = renderMarkdown(report);

  assert.equal(report.static_floor_report.scope.competitor_builds_not_run, true);
  assert.equal(report.live_frameworks.enabled, false);
  assert.equal(report.live_frameworks.builds_run, false);
  assert.equal(report.live_frameworks.package_installs_run, false);
  assert.equal(report.live_frameworks.rows[0].status, "skipped");
  assert.match(markdown, /separates deterministic static-floor evidence from opt-in live framework builds/);
  assert.match(markdown, /DX_FORGE_LIVE_FRAMEWORKS=1/);
});

test("live framework harness runs only already-installed local builds", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forge-live-harness-"));
  fs.writeFileSync(
    path.join(dir, "package.json"),
    JSON.stringify({ name: "fake-astro", scripts: { build: "node build.js" } })
  );
  fs.mkdirSync(path.join(dir, "node_modules"));
  fs.mkdirSync(path.join(dir, "dist"));
  fs.writeFileSync(path.join(dir, "dist", "index.html"), "<h1>ok</h1>");
  let called = false;

  const report = buildReport({
    generatedAt: "2026-05-17T00:00:00.000Z",
    staticReport: staticReport(),
    liveEnabled: true,
    profiles: [
      {
        framework: "Astro",
        project_dir: dir,
        build_command: ["npm", ["run", "build"]],
        output_dirs: ["dist"],
        static_floor_id: "astro-static-floor",
      },
    ],
    runCommand(command, args, options) {
      called = true;
      assert.equal(command, "npm");
      assert.deepEqual(args, ["run", "build"]);
      assert.equal(options.cwd, dir);
      return { status: 0, stdout: "built", stderr: "" };
    },
  });

  assert.equal(called, true);
  assert.equal(report.live_frameworks.enabled, true);
  assert.equal(report.live_frameworks.builds_run, true);
  assert.equal(report.live_frameworks.package_installs_run, false);
  assert.equal(report.live_frameworks.rows[0].status, "passed");
  assert.equal(report.live_frameworks.rows[0].output_summary[0].exists, true);
  assert.ok(report.live_frameworks.rows[0].output_summary[0].total_bytes > 0);
});
