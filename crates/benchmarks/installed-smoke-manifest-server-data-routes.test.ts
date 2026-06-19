import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const {
  manifestServerDataRouteFailures,
  summarizeManifestServerDataRoutes,
} = require("../tools/build/installed-smoke/manifest-server-data-routes.ts");

function createInput({
  dynamicOutput,
  dynamicRoutePath = "/blog/:slug",
  dynamicRouteSourcePath = "src/app/blog/[slug]/page.tsx",
  dynamicRouteParams = { slug: "sample-slug" },
  dynamicSearchParams = { tab: "sample-tab" },
  sourceOwned = true,
  externalRuntimeRequired = false,
} = {}) {
  const projectRoot = fs.mkdtempSync(path.join(os.tmpdir(), "dx-server-data-routes-"));
  fs.mkdirSync(path.join(projectRoot, ".dx", "build", "source-routes", "root"), { recursive: true });
  fs.mkdirSync(path.join(projectRoot, ".dx", "build", "source-routes", "blog--slug"), { recursive: true });

  const rootOutput = ".dx/build/source-routes/root/server-data.json";
  const expectedDynamicOutput = ".dx/build/source-routes/blog--slug/server-data.json";
  const rootRoute = {
    route: "/",
    route_source_path: "app/page.tsx",
    output: rootOutput,
    status: "source-owned-safe-loader-data",
    entry_count: 1,
    execution_model: "source-owned-safe-interpreter",
    source_owned_contract: true,
    external_runtime_required: false,
    external_runtime_executed: false,
    node_modules_required: false,
    lifecycle_scripts_executed: false,
  };
  const dynamicRoute = {
    route: dynamicRoutePath,
    route_source_path: dynamicRouteSourcePath,
    output: dynamicOutput || expectedDynamicOutput,
    status: "no-loader-bindings",
    entry_count: 0,
    execution_model: "not-required",
    source_owned_contract: sourceOwned,
    external_runtime_required: externalRuntimeRequired,
    external_runtime_executed: false,
    node_modules_required: false,
    lifecycle_scripts_executed: false,
    request: {
      mode: "static-route-contract-inputs",
      route_params: dynamicRouteParams,
      search_params: dynamicSearchParams,
    },
  };

  writeServerDataArtifact(projectRoot, rootOutput, rootRoute);
  writeServerDataArtifact(projectRoot, expectedDynamicOutput, dynamicRoute);

  return {
    projectRoot,
    manifest: {
      value: {
        server_data_routes_compiled: 2,
        server_data_routes: [rootRoute, dynamicRoute],
        server_data_route_manifest: {
          source_build_routes: 2,
          manifest_routes: 2,
          source_build_entries: 1,
          manifest_entries: 1,
          manifest_includes_source_build_routes: true,
          missing_source_build_routes: [],
          routes_with_route_params: 1,
          routes_with_search_params: 1,
          route_param_keys: ["slug"],
          search_param_keys: ["tab"],
        },
      },
    },
    sourceBuildManifest: {
      value: {
        server_data_routes: [
          {
            route: "/",
            route_source_path: "app/page.tsx",
            output: rootOutput,
            entry_count: 1,
            request: {
              mode: "static-route-contract-inputs",
              route_params: {},
              search_params: {},
            },
          },
          {
            route: dynamicRoutePath,
            route_source_path: dynamicRouteSourcePath,
            output: expectedDynamicOutput,
            entry_count: 0,
            request: dynamicRoute.request,
          },
        ],
        route_outputs: [
          {
            route: "/",
            source_path: "app/page.tsx",
            server_data_output: ".dx/build/source-routes/root/server-data.json",
          },
          {
            route: dynamicRoutePath,
            source_path: dynamicRouteSourcePath,
            server_data_output: expectedDynamicOutput,
          },
        ],
      },
    },
  };
}

function writeServerDataArtifact(projectRoot, outputPath, route) {
  fs.writeFileSync(
    path.join(projectRoot, outputPath),
    JSON.stringify({
      route: route.route,
      route_source_path: route.route_source_path,
      status: route.status,
      entry_count: route.entry_count,
      execution_model: route.execution_model,
      source_owned_contract: route.source_owned_contract,
      external_runtime_required: route.external_runtime_required,
      external_runtime_executed: route.external_runtime_executed,
      node_modules_required: route.node_modules_required,
      lifecycle_scripts_executed: route.lifecycle_scripts_executed,
      request: route.request || { mode: "static-route-contract-inputs" },
    }),
  );
}

test("manifest server-data smoke validates every emitted App Router route", () => {
  const summary = summarizeManifestServerDataRoutes(createInput());

  assert.equal(summary.present, true);
  assert.equal(summary.compiledCount, 2);
  assert.equal(summary.routeCount, 2);
  assert.equal(summary.emittedCount, 2);
  assert.equal(summary.sourceBuildMatchCount, 2);
  assert.equal(summary.allOutputsPresent, true);
  assert.equal(summary.allRoutesMatchSourceBuildOutputs, true);
  assert.equal(summary.routeManifest.present, true);
  assert.equal(summary.routeManifest.sourceBuildRoutes, 2);
  assert.equal(summary.routeManifest.manifestRoutes, 2);
  assert.equal(summary.routeManifest.sourceBuildEntries, 1);
  assert.equal(summary.routeManifest.manifestEntries, 1);
  assert.equal(summary.routeManifest.manifestIncludesSourceBuildRoutes, true);
  assert.equal(summary.routeManifest.consistentWithComputedRoutes, true);
  assert.equal(summary.routeManifest.routesWithRouteParams, 1);
  assert.equal(summary.routeManifest.routesWithSearchParams, 1);
  assert.deepEqual(summary.routeManifest.routeParamKeys, ["slug"]);
  assert.deepEqual(summary.routeManifest.searchParamKeys, ["tab"]);
  assert.equal(summary.routeManifest.requestPropsMatchComputed, true);
  assert.deepEqual(summary.routeManifest.sourceBuildRouteParamKeys, ["slug"]);
  assert.deepEqual(summary.routeManifest.sourceBuildSearchParamKeys, ["tab"]);
  assert.equal(summary.routeManifest.sourceBuildRequestPropsMatchManifest, true);
  assert.equal(summary.routeManifest.sourceBuildRequestPropMismatchCount, 0);
  assert.deepEqual(summary.routeManifest.sourceBuildRequestPropMismatchRoutes, []);

  const dynamicRoute = summary.routes.find((route) => route.route === "/blog/:slug");
  assert.equal(dynamicRoute.routeSourcePath, "src/app/blog/[slug]/page.tsx");
  assert.equal(dynamicRoute.output.present, true);
  assert.equal(dynamicRoute.matchesSourceBuildRouteOutput, true);
  assert.equal(dynamicRoute.matchesSourceBuildServerDataRoute, true);
  assert.equal(dynamicRoute.artifact.route, "/blog/:slug");
  assert.equal(dynamicRoute.artifact.routeSourcePath, "src/app/blog/[slug]/page.tsx");
  assert.equal(dynamicRoute.artifactMatchesManifest, true);
  assert.deepEqual(dynamicRoute.routeParams, { slug: "sample-slug" });
  assert.deepEqual(dynamicRoute.searchParams, { tab: "sample-tab" });
  assert.deepEqual(dynamicRoute.routeParamKeys, ["slug"]);
  assert.deepEqual(dynamicRoute.searchParamKeys, ["tab"]);
  assert.deepEqual(dynamicRoute.artifact.routeParams, { slug: "sample-slug" });
  assert.deepEqual(dynamicRoute.artifact.searchParams, { tab: "sample-tab" });
  assert.deepEqual(dynamicRoute.artifact.routeParamKeys, ["slug"]);
  assert.deepEqual(dynamicRoute.artifact.searchParamKeys, ["tab"]);
  assert.equal(dynamicRoute.requestPropsMatchArtifact, true);
  assert.equal(dynamicRoute.sourceOwnedContract, true);
  assert.equal(dynamicRoute.externalRuntimeRequired, false);
  assert.equal(dynamicRoute.externalRuntimeExecuted, false);

  const failures = manifestServerDataRouteFailures({
    build: { manifest: { serverDataRoutes: summary } },
  });
  assert.deepEqual(failures, []);
});

test("manifest server-data smoke preserves catch-all route param value shape", () => {
  const summary = summarizeManifestServerDataRoutes(
    createInput({
      dynamicRoutePath: "/blog/+slug",
      dynamicRouteSourcePath: "src/app/blog/[...slug]/page.tsx",
      dynamicRouteParams: { slug: ["sample-slug"] },
    }),
  );

  const dynamicRoute = summary.routes.find((route) => route.route === "/blog/+slug");
  assert.equal(dynamicRoute.routeSourcePath, "src/app/blog/[...slug]/page.tsx");
  assert.deepEqual(dynamicRoute.routeParams, { slug: ["sample-slug"] });
  assert.deepEqual(dynamicRoute.artifact.routeParams, { slug: ["sample-slug"] });
  assert.deepEqual(dynamicRoute.routeParamKeys, ["slug"]);
  assert.deepEqual(dynamicRoute.artifact.routeParamKeys, ["slug"]);
  assert.equal(dynamicRoute.requestPropsMatchArtifact, true);
  assert.equal(dynamicRoute.artifactMatchesManifest, true);

  const failures = manifestServerDataRouteFailures({
    build: { manifest: { serverDataRoutes: summary } },
  });
  assert.deepEqual(failures, []);
});

test("manifest server-data smoke rejects stale catch-all route param value shapes", () => {
  const input = createInput({
    dynamicRoutePath: "/blog/+slug",
    dynamicRouteSourcePath: "src/app/blog/[...slug]/page.tsx",
    dynamicRouteParams: { slug: "sample-slug" },
  });
  fs.writeFileSync(
    path.join(input.projectRoot, ".dx", "build", "source-routes", "blog--slug", "server-data.json"),
    JSON.stringify({
      route: "/blog/+slug",
      route_source_path: "src/app/blog/[...slug]/page.tsx",
      status: "no-loader-bindings",
      entry_count: 0,
      execution_model: "not-required",
      source_owned_contract: true,
      external_runtime_required: false,
      external_runtime_executed: false,
      node_modules_required: false,
      lifecycle_scripts_executed: false,
      request: {
        mode: "static-route-contract-inputs",
        route_params: { slug: ["sample-slug"] },
        search_params: { tab: "sample-tab" },
      },
    }),
  );

  const summary = summarizeManifestServerDataRoutes(input);
  const dynamicRoute = summary.routes.find((route) => route.route === "/blog/+slug");
  assert.deepEqual(dynamicRoute.routeParamKeys, ["slug"]);
  assert.deepEqual(dynamicRoute.artifact.routeParamKeys, ["slug"]);
  assert.equal(dynamicRoute.requestPropsMatchArtifact, false);

  const failures = manifestServerDataRouteFailures({
    build: { manifest: { serverDataRoutes: summary } },
  });
  assert.ok(
    failures.includes(
      "manifest server_data_routes /blog/+slug src/app/blog/[...slug]/page.tsx request props do not match emitted artifact",
    ),
    JSON.stringify(failures, null, 2),
  );
});

test("manifest server-data smoke rejects stale dx build route-manifest summary", () => {
  const input = createInput();
  input.manifest.value.server_data_route_manifest = {
    source_build_routes: 1,
    manifest_routes: 1,
    source_build_entries: 0,
    manifest_entries: 0,
    manifest_includes_source_build_routes: false,
    missing_source_build_routes: [
      {
        route: "/stale",
        route_source_path: "app/stale/page.tsx",
        output: ".dx/build/source-routes/stale/server-data.json",
      },
    ],
  };

  const summary = summarizeManifestServerDataRoutes(input);
  const failures = manifestServerDataRouteFailures({
    build: { manifest: { serverDataRoutes: summary } },
  });

  assert.equal(summary.routeManifest.present, true);
  assert.equal(summary.routeManifest.consistentWithComputedRoutes, false);
  assert.ok(
    failures.includes(
      "manifest server_data_route_manifest source_build_routes does not match source-build server-data routes",
    ),
    JSON.stringify(failures, null, 2),
  );
  assert.ok(
    failures.includes(
      "manifest server_data_route_manifest missing_source_build_routes disagrees with source-build server-data routes",
    ),
    JSON.stringify(failures, null, 2),
  );
});

test("manifest server-data smoke rejects stale request prop summaries and artifacts", () => {
  const input = createInput();
  input.manifest.value.server_data_route_manifest.routes_with_route_params = 0;
  input.manifest.value.server_data_route_manifest.route_param_keys = [];
  fs.writeFileSync(
    path.join(input.projectRoot, ".dx", "build", "source-routes", "blog--slug", "server-data.json"),
    JSON.stringify({
      route: "/blog/:slug",
      route_source_path: "src/app/blog/[slug]/page.tsx",
      status: "no-loader-bindings",
      entry_count: 0,
      execution_model: "not-required",
      source_owned_contract: true,
      external_runtime_required: false,
      external_runtime_executed: false,
      node_modules_required: false,
      lifecycle_scripts_executed: false,
      request: {
        mode: "static-route-contract-inputs",
      },
    }),
  );

  const summary = summarizeManifestServerDataRoutes(input);
  const failures = manifestServerDataRouteFailures({
    build: { manifest: { serverDataRoutes: summary } },
  });

  assert.equal(summary.routeManifest.requestPropsMatchComputed, false);
  assert.ok(
    failures.includes(
      "manifest server_data_route_manifest route_param_keys disagree with server_data_routes requests",
    ),
    JSON.stringify(failures, null, 2),
  );
  assert.ok(
    failures.includes(
      "manifest server_data_routes /blog/:slug src/app/blog/[slug]/page.tsx request props do not match emitted artifact",
    ),
    JSON.stringify(failures, null, 2),
  );
});

test("manifest server-data smoke rejects request props that drift from source-build routes", () => {
  const input = createInput();
  input.sourceBuildManifest.value.server_data_routes[1].request = {
    mode: "static-route-contract-inputs",
    route_params: { slug: "sample-slug" },
    search_params: { preview: "sample-preview", tab: "sample-tab" },
  };

  const summary = summarizeManifestServerDataRoutes(input);
  const dynamicRoute = summary.routes.find((route) => route.route === "/blog/:slug");
  assert.deepEqual(dynamicRoute.sourceBuildRouteParamKeys, ["slug"]);
  assert.deepEqual(dynamicRoute.sourceBuildSearchParamKeys, ["preview", "tab"]);
  assert.equal(dynamicRoute.requestPropsMatchSourceBuildRoute, false);
  assert.deepEqual(summary.routeManifest.sourceBuildRouteParamKeys, ["slug"]);
  assert.deepEqual(summary.routeManifest.sourceBuildSearchParamKeys, ["preview", "tab"]);
  assert.equal(summary.routeManifest.sourceBuildRequestPropsMatchManifest, false);
  assert.equal(summary.routeManifest.sourceBuildRequestPropMismatchCount, 1);
  assert.deepEqual(summary.routeManifest.sourceBuildRequestPropMismatchRoutes, [
    "/blog/:slug src/app/blog/[slug]/page.tsx",
  ]);
  assert.equal(summary.routeManifest.consistentWithComputedRoutes, false);

  const failures = manifestServerDataRouteFailures({
    build: { manifest: { serverDataRoutes: summary } },
  });
  assert.ok(
    failures.includes(
      "manifest server_data_route_manifest source-build request props do not match server_data_routes requests",
    ),
    JSON.stringify(failures, null, 2),
  );
  assert.ok(
    failures.includes(
      "manifest server_data_routes /blog/:slug src/app/blog/[slug]/page.tsx request props do not match source-build server-data route",
    ),
    JSON.stringify(failures, null, 2),
  );
});

test("manifest server-data smoke rejects source-build server-data route output drift", () => {
  const input = createInput();
  input.sourceBuildManifest.value.server_data_routes[1].output =
    ".dx/build/source-routes/blog--slug/source-build-stale-server-data.json";

  const summary = summarizeManifestServerDataRoutes(input);
  const failures = manifestServerDataRouteFailures({
    build: { manifest: { serverDataRoutes: summary } },
  });

  assert.ok(
    failures.includes(
      "manifest server_data_routes /blog/:slug src/app/blog/[slug]/page.tsx output does not match source-build server-data route",
    ),
    JSON.stringify(failures, null, 2),
  );
});

test("manifest server-data smoke rejects stale dynamic outputs and external runtime claims", () => {
  const summary = summarizeManifestServerDataRoutes(
    createInput({
      dynamicOutput: ".dx/build/source-routes/blog--slug/stale-server-data.json",
      sourceOwned: false,
      externalRuntimeRequired: true,
    }),
  );

  const failures = manifestServerDataRouteFailures({
    build: { manifest: { serverDataRoutes: summary } },
  });

  assert.ok(
    failures.includes(
      "manifest server_data_routes /blog/:slug src/app/blog/[slug]/page.tsx output was not emitted",
    ),
    JSON.stringify(failures, null, 2),
  );
  assert.ok(
    failures.includes(
      "manifest server_data_routes /blog/:slug src/app/blog/[slug]/page.tsx output does not match source-build route output",
    ),
    JSON.stringify(failures, null, 2),
  );
  assert.ok(
    failures.includes(
      "manifest server_data_routes /blog/:slug src/app/blog/[slug]/page.tsx does not declare source_owned_contract=true",
    ),
    JSON.stringify(failures, null, 2),
  );
  assert.ok(
    failures.includes(
      "manifest server_data_routes /blog/:slug src/app/blog/[slug]/page.tsx requires an external runtime",
    ),
    JSON.stringify(failures, null, 2),
  );
});

test("manifest server-data smoke failures name the route source path", () => {
  const summary = summarizeManifestServerDataRoutes(
    createInput({
      dynamicOutput: ".dx/build/source-routes/blog--slug/stale-server-data.json",
    }),
  );

  const failures = manifestServerDataRouteFailures({
    build: { manifest: { serverDataRoutes: summary } },
  });

  assert.ok(
    failures.includes(
      "manifest server_data_routes /blog/:slug src/app/blog/[slug]/page.tsx output was not emitted",
    ),
    JSON.stringify(failures, null, 2),
  );
  assert.ok(
    !failures.includes("manifest server_data_routes /blog/:slug output was not emitted"),
    JSON.stringify(failures, null, 2),
  );
});

test("manifest server-data smoke rejects source-build route outputs missing from the public manifest", () => {
  const input = createInput();
  input.manifest.value.server_data_routes = input.manifest.value.server_data_routes.filter(
    (route) => route.route === "/",
  );
  input.manifest.value.server_data_routes_compiled = input.manifest.value.server_data_routes.length;

  const summary = summarizeManifestServerDataRoutes(input);
  const failures = manifestServerDataRouteFailures({
    build: { manifest: { serverDataRoutes: summary } },
  });

  assert.ok(
    failures.includes(
      "manifest server_data_routes is missing source-build route output /blog/:slug src/app/blog/[slug]/page.tsx",
    ),
    JSON.stringify(failures, null, 2),
  );
});

test("manifest server-data smoke rejects duplicate public manifest routes", () => {
  const input = createInput();
  input.manifest.value.server_data_routes.push({ ...input.manifest.value.server_data_routes[1] });
  input.manifest.value.server_data_routes_compiled = input.manifest.value.server_data_routes.length;

  const summary = summarizeManifestServerDataRoutes(input);
  const failures = manifestServerDataRouteFailures({
    build: { manifest: { serverDataRoutes: summary } },
  });

  assert.equal(summary.duplicateRouteCount, 1);
  assert.ok(
    failures.includes(
      "manifest server_data_routes has duplicate route /blog/:slug src/app/blog/[slug]/page.tsx",
    ),
    JSON.stringify(failures, null, 2),
  );
});

test("manifest server-data smoke rejects stale emitted artifact payloads", () => {
  const input = createInput();
  fs.writeFileSync(
    path.join(input.projectRoot, ".dx", "build", "source-routes", "blog--slug", "server-data.json"),
    JSON.stringify({
      route: "/stale",
      route_source_path: "app/stale/page.tsx",
      status: "no-loader-bindings",
      entry_count: 0,
      source_owned_contract: true,
      external_runtime_required: false,
      external_runtime_executed: false,
      node_modules_required: false,
      lifecycle_scripts_executed: false,
    }),
  );

  const summary = summarizeManifestServerDataRoutes(input);
  const failures = manifestServerDataRouteFailures({
    build: { manifest: { serverDataRoutes: summary } },
  });

  assert.ok(
    failures.includes(
      "manifest server_data_routes /blog/:slug src/app/blog/[slug]/page.tsx output payload does not match manifest route",
    ),
    JSON.stringify(failures, null, 2),
  );
});
