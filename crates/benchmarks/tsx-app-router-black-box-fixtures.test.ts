const assert = require("node:assert/strict");
const { spawnSync } = require("node:child_process");
const { spawn } = require("node:child_process");
const fs = require("node:fs");
const net = require("node:net");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const routeReportSourceFiles = [
  path.join(repoRoot, "dx-www", "src", "cli", "studio_command.rs"),
  path.join(repoRoot, "dx-www", "src", "cli", "app_page_routes.rs"),
  path.join(repoRoot, "dx-www", "src", "cli", "app_segment_files.rs"),
  path.join(repoRoot, "dx-www", "src", "app_router_segments.rs"),
  path.join(repoRoot, "dx-www", "src", "project.rs"),
];
const appRouterRuntimeSourceFiles = [
  path.join(repoRoot, "dx-www", "src", "cli", "app_router_execution.rs"),
  path.join(repoRoot, "dx-www", "src", "cli", "app_router_execution", "metadata.rs"),
  path.join(repoRoot, "dx-www", "src", "cli", "app_router_execution", "next_navigation.rs"),
  path.join(repoRoot, "dx-www", "src", "cli", "app_router_execution", "request_props.rs"),
  path.join(repoRoot, "dx-www", "src", "cli", "app_router_execution", "source_render.rs"),
];
const blackBoxSourceFiles = [...routeReportSourceFiles, ...appRouterRuntimeSourceFiles];
const studioCommandSource = routeReportSourceFiles[0];
const dxWwwBinary = path.join(
  repoRoot,
  "target",
  "debug",
  process.platform === "win32" ? "dx-www.exe" : "dx-www",
);

function writeFixtureFile(projectRoot, relativePath, content) {
  const filePath = path.join(projectRoot, relativePath);
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, content);
}

function createNestedAppRouterFixture(port = 3988) {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-app-router-black-box-"));
  writeFixtureFile(
    projectRoot,
    "dx",
    `project.name="app-router-black-box"
dev.host="127.0.0.1"
dev.port=${port}
dev.hot_reload=false
`,
  );
  writeFixtureFile(
    projectRoot,
    "app/layout.tsx",
    `export default function RootLayout({ children }) {
  return <html><body>{children}</body></html>;
}
`,
  );
  writeFixtureFile(
    projectRoot,
    "app/(marketing)/blog/layout.tsx",
    `export default function BlogLayout({ children }) {
  return <section>{children}</section>;
}
`,
  );
  writeFixtureFile(
    projectRoot,
    "app/(marketing)/blog/template.tsx",
    `export default function BlogTemplate({ children }) {
  return <article>{children}</article>;
}
`,
  );
  writeFixtureFile(
    projectRoot,
    "app/(marketing)/blog/loading.tsx",
    `export default function Loading() {
  return <p>Loading blog</p>;
}
`,
  );
  writeFixtureFile(
    projectRoot,
    "app/(marketing)/blog/error.tsx",
    `"use client";

export default function Error({ error, reset }) {
  return <button onClick={reset}>{error.message}</button>;
}
`,
  );
  writeFixtureFile(
    projectRoot,
    "app/(marketing)/blog/not-found.tsx",
    `export default function NotFound() {
  return <p>Blog post missing</p>;
}
`,
  );
  writeFixtureFile(
    projectRoot,
    "app/(marketing)/blog/[slug]/page.tsx",
    `export const metadata = {
  title: "Blog detail",
  description: "Source-owned blog detail metadata",
};

export default function BlogPost({ params, searchParams }) {
  return <main>{params.slug} / {searchParams.preview}</main>;
}
`,
  );
  writeFixtureFile(
    projectRoot,
    "app/(marketing)/blog/missing/page.tsx",
    `import { notFound } from "next/navigation";

export default function MissingBlogPost() {
  notFound();
}
`,
  );
  writeFixtureFile(
    projectRoot,
    "app/(marketing)/blog/redirect/page.tsx",
    `import { redirect, RedirectType } from "next/navigation";

export default function RedirectBlogPost() {
  redirect("/blog/launch-notes?preview=redirected", RedirectType.replace);
}
`,
  );
  return projectRoot;
}

function runRoutesJson(projectRoot) {
  assert.ok(fs.existsSync(dxWwwBinary), `missing ${dxWwwBinary}`);
  const result = spawnSync(dxWwwBinary, ["www", "routes", "--json"], {
    cwd: projectRoot,
    encoding: "utf8",
    timeout: 30000,
  });
  assert.equal(result.status, 0, result.stderr || result.stdout);
  return JSON.parse(result.stdout);
}

function sourceFilesFor(route) {
  return (route.source_files ?? []).map((sourceFile) => sourceFile.replace(/\\/g, "/"));
}

async function freePort() {
  const server = net.createServer();
  await new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(0, "127.0.0.1", resolve);
  });
  const { port } = server.address();
  await new Promise((resolve, reject) => server.close((error) => (error ? reject(error) : resolve())));
  return port;
}

function freshRoutesBinaryAvailable(t) {
  if (!fs.existsSync(dxWwwBinary)) {
    t.skip("target/debug/dx-www executable is required for this black-box route proof");
    return false;
  }

  const binaryMtime = fs.statSync(dxWwwBinary).mtimeMs;
  const staleSourceFile = blackBoxSourceFiles.find(
    (sourceFile) => binaryMtime < fs.statSync(sourceFile).mtimeMs,
  );
  if (staleSourceFile) {
    const relativeSource = path.relative(repoRoot, staleSourceFile).replace(/\\/g, "/");
    t.skip(`target/debug/dx-www is older than ${relativeSource}`);
    return false;
  }
  return true;
}

function waitForDevServer(child) {
  return new Promise((resolve, reject) => {
    let output = "";
    const timeout = setTimeout(() => {
      reject(new Error(`timed out waiting for dx dev startup\n${output}`));
    }, 15000);
    const onData = (chunk) => {
      output += chunk.toString();
      const match = output.match(/Development server running at (http:\/\/[^\s]+)/);
      if (!match) {
        return;
      }
      clearTimeout(timeout);
      resolve(match[1]);
    };
    child.stdout.on("data", onData);
    child.stderr.on("data", onData);
    child.once("exit", (code, signal) => {
      clearTimeout(timeout);
      reject(new Error(`dx dev exited before startup: code=${code} signal=${signal}\n${output}`));
    });
  });
}

async function stopDevServer(child) {
  if (child.exitCode !== null) {
    return;
  }
  child.kill();
  await new Promise((resolve) => child.once("exit", resolve));
}

function attrValue(html, name) {
  const match = html.match(new RegExp(`${name}="([^"]*)"`));
  assert.ok(match, `missing ${name}`);
  return match[1].replace(/&quot;/g, '"').replace(/&amp;/g, "&");
}

function scriptJson(html, id) {
  const match = html.match(new RegExp(`<script type="application/json" id="${id}">([\\s\\S]*?)<\\/script>`));
  assert.ok(match, `missing ${id}`);
  return JSON.parse(match[1].replace(/<\\\/script/g, "</script"));
}

function assertSourceOwnedAppRouterExecution(execution) {
  assert.equal(execution.runtime, "source-owned-app-router");
  assert.equal(execution.runtime_boundary.source_owned, true);
  assert.equal(execution.runtime_boundary.external_runtime_required, false);
  assert.equal(execution.runtime_boundary.external_runtime_executed, false);
  assert.equal(execution.node_modules_required, false);
  assert.equal(execution.next_navigation_control_flow.node_modules_required, false);
  assert.equal(execution.next_navigation_control_flow.external_runtime_required, false);
  assert.equal(execution.next_navigation_control_flow.external_runtime_executed, false);
}

function assertSourceRenderArtifactMatches(html, execution) {
  const sourceRender = scriptJson(html, "__DX_TSX_SOURCE_RENDER__");
  assert.deepEqual(sourceRender, execution.tsx_source_render);
  assert.equal(sourceRender.full_jsx_execution, false);
  assert.equal(sourceRender.client_islands.node_modules_required, false);
  assert.equal(sourceRender.dom_action_binder.node_modules_required, false);
  assert.equal(sourceRender.dom_action_binder.state_runtime_bridge.node_modules_required, false);
  assert.equal(
    sourceRender.composed_static_dom_snapshot.app_router_shell.full_app_router_runtime,
    false,
  );
  return sourceRender;
}

function assertRenderedShellMatchesSourceRender(html, sourceRender) {
  const shell = sourceRender.composed_static_dom_snapshot.app_router_shell;
  assert.equal(attrValue(html, "data-dx-app-router-shell-status"), shell.status);
  assert.equal(attrValue(html, "data-dx-app-router-shell-wrappers"), String(shell.wrapper_count));
  assert.equal(
    attrValue(html, "data-dx-app-router-shell-child-insertions"),
    String(shell.child_insertions),
  );
  return shell;
}

test("www routes source wires App Router boundary discovery into local route reports", () => {
  const source = fs.readFileSync(studioCommandSource, "utf8");

  assert.match(source, /app_router_route_summaries_by_source\(&project\.root\)/);
  assert.match(source, /app_router_boundary_source_paths\(&app_router_summaries\)/);
  assert.match(source, /if app_router_boundary_sources\.contains\(&page_source_path\)/);
  assert.match(source, /route\["app_router"\] = app_router_summary_json\(summary\)/);
  assert.match(source, /"schema": "dx\.app-router\.route-report"/);
  assert.match(source, /"format": 1/);
  assert.match(source, /"source_owned_app_router": true/);
  assert.match(source, /"node_modules_required": false/);
  assert.match(source, /"full_next_runtime": false/);
});

test("www routes exposes nested App Router boundary files from a black-box fixture", (t) => {
  if (!freshRoutesBinaryAvailable(t)) {
    return;
  }

  const projectRoot = createNestedAppRouterFixture();
  try {
    const report = runRoutesJson(projectRoot);
    const localRoutes = report.routes.filter((route) => route.source === "dx-www-project-scan");
    const route = localRoutes.find((candidate) =>
      sourceFilesFor(candidate).includes("app/(marketing)/blog/[slug]/page.tsx"),
    );

    assert.ok(route, JSON.stringify(localRoutes, null, 2));
    assert.equal(route.route, "/blog/:slug");
    assert.equal(route.dynamic, true);
    assert.deepEqual(route.params, ["slug"]);

    const boundaryPseudoRoutes = localRoutes.filter((candidate) =>
      sourceFilesFor(candidate).some((sourceFile) =>
        [
          "app/(marketing)/blog/loading.tsx",
          "app/(marketing)/blog/error.tsx",
          "app/(marketing)/blog/not-found.tsx",
        ].includes(sourceFile),
      ),
    );
    assert.deepEqual(boundaryPseudoRoutes, [], JSON.stringify(boundaryPseudoRoutes, null, 2));

    assert.equal(route.app_router.schema, "dx.app-router.route-report");
    assert.equal(route.app_router.format, 1);
    assert.equal(route.app_router.source_owned_app_router, true);
    assert.equal(route.app_router.node_modules_required, false);
    assert.equal(route.app_router.full_next_runtime, false);
    assert.equal(route.app_router.route_path, "/blog/[slug]");
    assert.equal(route.app_router.route_shape, "/blog/[]");
    assert.equal(route.app_router.non_path_segment_count, 1);
    assert.equal(route.app_router.specificity.static_segment_count, 1);
    assert.equal(route.app_router.specificity.dynamic_segment_count, 1);
    assert.deepEqual(route.app_router.specificity.segment_kinds, ["static", "dynamic"]);

    const segmentFiles = new Map(
      route.app_router.segment_files.map((segmentFile) => [segmentFile.source_path, segmentFile]),
    );
    for (const [sourcePath, kind] of [
      ["app/layout.tsx", "layout"],
      ["app/(marketing)/blog/layout.tsx", "layout"],
      ["app/(marketing)/blog/template.tsx", "template"],
      ["app/(marketing)/blog/loading.tsx", "loading"],
      ["app/(marketing)/blog/error.tsx", "error"],
      ["app/(marketing)/blog/not-found.tsx", "not-found"],
    ]) {
      assert.equal(segmentFiles.get(sourcePath)?.kind, kind, sourcePath);
    }
    assert.equal(segmentFiles.get("app/(marketing)/blog/error.tsx").route_path, "/blog");
    assert.equal(segmentFiles.get("app/(marketing)/blog/not-found.tsx").route_path, "/blog");
  } finally {
    fs.rmSync(projectRoot, { recursive: true, force: true });
  }
});

test("dx dev renders App Router route groups, params, metadata, and boundaries from a black-box fixture", async (t) => {
  if (!freshRoutesBinaryAvailable(t)) {
    return;
  }

  const port = await freePort();
  const projectRoot = createNestedAppRouterFixture(port);
  const child = spawn(dxWwwBinary, ["dev", "--host", "127.0.0.1", "--port", String(port), "--no-hot-reload"], {
    cwd: projectRoot,
    stdio: ["ignore", "pipe", "pipe"],
    windowsHide: true,
  });

  t.after(async () => {
    await stopDevServer(child);
    fs.rmSync(projectRoot, { recursive: true, force: true });
  });

  const baseUrl = await waitForDevServer(child);
  const response = await fetch(`${baseUrl}/blog/launch-notes?preview=draft`);
  const html = await response.text();

  assert.equal(response.status, 200);
  assert.match(html, /data-dx-renderer="tsx-app-router-generic"/);
  assert.match(html, /data-dx-app-router-runtime="source-owned-app-router"/);
  assert.match(html, /data-dx-node-modules-required="false"/);
  assert.match(html, /<title>Blog detail<\/title>/);
  assert.match(html, /<meta name="description" content="Source-owned blog detail metadata">/);
  assert.equal(attrValue(html, "data-dx-layout-count"), "2");
  assert.equal(attrValue(html, "data-dx-template-count"), "1");
  assert.equal(attrValue(html, "data-dx-boundary-count"), "3");
  assert.equal(attrValue(html, "data-dx-route-params"), "slug=launch-notes");
  assert.equal(attrValue(html, "data-dx-search-params"), "preview=draft");

  const execution = scriptJson(html, "__DX_APP_ROUTER_EXECUTION__");
  assertSourceOwnedAppRouterExecution(execution);
  const sourceRender = assertSourceRenderArtifactMatches(html, execution);
  assertRenderedShellMatchesSourceRender(html, sourceRender);
  assert.deepEqual(execution.route_groups, ["(marketing)"]);
  assert.deepEqual(execution.visible_segments, ["blog", "[slug]"]);
  assert.deepEqual(execution.request.route_params, { slug: "launch-notes" });
  assert.deepEqual(execution.request.search_params, { preview: "draft" });
  assert.deepEqual(execution.page_props.params, ["slug"]);
  assert.deepEqual(execution.page_props.search_params, ["preview"]);
  assert.equal(execution.metadata_head.status, "source-owned-head-tags-ready");
  assert.equal(execution.metadata_head.source_owned_head_tags, true);
  assert.equal(execution.metadata_head.full_next_head_runtime, false);
  assert.equal(execution.proof.layout_count, 2);
  assert.equal(execution.proof.template_count, 1);

  assert.deepEqual(execution.boundaries.loading, ["app/(marketing)/blog/loading.tsx"]);
  assert.deepEqual(execution.boundaries.error, ["app/(marketing)/blog/error.tsx"]);
  assert.deepEqual(execution.boundaries.not_found, ["app/(marketing)/blog/not-found.tsx"]);
  assert.deepEqual(findNodeModulesDirs(projectRoot), []);
});

test("dx dev renders App Router notFound and redirect control flow from black-box pages", async (t) => {
  if (!freshRoutesBinaryAvailable(t)) {
    return;
  }

  const port = await freePort();
  const projectRoot = createNestedAppRouterFixture(port);
  const child = spawn(dxWwwBinary, ["dev", "--host", "127.0.0.1", "--port", String(port), "--no-hot-reload"], {
    cwd: projectRoot,
    stdio: ["ignore", "pipe", "pipe"],
    windowsHide: true,
  });

  t.after(async () => {
    await stopDevServer(child);
    fs.rmSync(projectRoot, { recursive: true, force: true });
  });

  const baseUrl = await waitForDevServer(child);
  const missingResponse = await fetch(`${baseUrl}/blog/missing`);
  const missingHtml = await missingResponse.text();

  assert.equal(missingResponse.status, 200);
  assert.match(missingHtml, /data-dx-next-not-found="true"/);
  assert.match(missingHtml, /<meta name="robots" content="noindex" data-dx-next-not-found="true"\s*\/?>/);
  assert.match(missingHtml, /Blog post missing/);

  const missingExecution = scriptJson(missingHtml, "__DX_APP_ROUTER_EXECUTION__");
  assertSourceOwnedAppRouterExecution(missingExecution);
  const missingSourceRender = assertSourceRenderArtifactMatches(missingHtml, missingExecution);
  assert.equal(missingExecution.next_navigation_control_flow.status, "not-found-ready");
  assert.equal(missingExecution.next_navigation_control_flow.not_found.detected, true);
  const missingShell = assertRenderedShellMatchesSourceRender(missingHtml, missingSourceRender);
  assert.equal(missingShell.selected_leaf.role, "not-found");
  assert.equal(
    missingShell.selected_leaf.source_path,
    "app/(marketing)/blog/not-found.tsx",
  );
  assert.equal(missingShell.selected_leaf.not_found_boundary_selected, true);

  const redirectResponse = await fetch(`${baseUrl}/blog/redirect`);
  const redirectHtml = await redirectResponse.text();

  assert.equal(redirectResponse.status, 200);
  assert.match(redirectHtml, /data-dx-next-redirect="\/blog\/launch-notes\?preview=redirected"/);
  assert.match(redirectHtml, /data-dx-app-router-redirect="true"/);
  assert.match(
    redirectHtml,
    /<meta http-equiv="refresh" content="0;url=\/blog\/launch-notes\?preview=redirected" data-dx-next-redirect="true"\s*\/?>/,
  );

  const redirectExecution = scriptJson(redirectHtml, "__DX_APP_ROUTER_EXECUTION__");
  assertSourceOwnedAppRouterExecution(redirectExecution);
  const redirectSourceRender = assertSourceRenderArtifactMatches(redirectHtml, redirectExecution);
  assert.equal(redirectExecution.next_navigation_control_flow.status, "redirect-ready");
  assert.equal(redirectExecution.next_navigation_control_flow.redirect.destination, "/blog/launch-notes?preview=redirected");
  assert.equal(redirectExecution.next_navigation_control_flow.redirect.type, "replace");
  const redirectShell = assertRenderedShellMatchesSourceRender(redirectHtml, redirectSourceRender);
  assert.equal(redirectShell.selected_leaf.role, "redirect");
  assert.equal(redirectShell.selected_leaf.redirect_selected, true);
  assert.equal(redirectShell.redirect.status_code, 307);
  assert.deepEqual(findNodeModulesDirs(projectRoot), []);
});

function findNodeModulesDirs(root) {
  const found = [];
  const stack = [root];
  while (stack.length > 0) {
    const current = stack.pop();
    for (const entry of fs.readdirSync(current, { withFileTypes: true })) {
      const fullPath = path.join(current, entry.name);
      if (!entry.isDirectory()) {
        continue;
      }
      if (entry.name === "node_modules") {
        found.push(path.relative(root, fullPath).replace(/\\/g, "/"));
        continue;
      }
      stack.push(fullPath);
    }
  }
  return found.sort();
}
