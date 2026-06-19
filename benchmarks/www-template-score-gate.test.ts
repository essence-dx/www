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

const expectedProviderGatePackages = [
  "ai/vercel-ai",
  "auth/better-auth",
  "automations/n8n",
  "instantdb/react",
  "payments/stripe-js",
  "supabase/client",
];

test("launch score gate blocks 90+ until real browser or provider evidence exists", async () => {
  const reality = await import(
    "../examples/template/components/template-app/package-reality.ts"
  );
  const summary = reality.forgeRealitySummary;
  const gate = summary.scoreGate;
  const evidenceRows = reality.launchEvidenceSummaryRows;

  assert.equal(summary.unboundedSourceScore, 93);
  assert.ok(
    summary.unboundedSourceScore >= gate.targetScore,
    "source-owned proof can exceed the target before live-proof capping",
  );
  assert.equal(summary.score, 89);
  assert.equal(gate.currentScore, 89);
  assert.equal(gate.targetScore, 90);
  assert.equal(gate.ceilingWithoutLiveProof, 89);
  assert.equal(gate.canExceedCeiling, false);
  assert.equal(gate.browserRuntimeProof, false);
  assert.equal(gate.liveProviderProof, false);
  assert.match(gate.honesty, /capped at 89/);

  const requiredProofs = new Map(
    gate.requiredProofs.map((proof) => [proof.id, proof]),
  );
  assert.deepEqual(requiredProofs.get("browser-route-proof"), {
    id: "browser-route-proof",
    label: "Browser route proof",
    status: "not-attached",
    routes: ["/", "/dashboard", "/login"],
    requiredEvidence:
      "Attach a running preview and verify nonblank rendering, native wheel scrolling, responsive shell behavior, and no custom scrollbar runtime.",
  });
  const liveProviderProof = requiredProofs.get("live-provider-proof");
  assert.ok(liveProviderProof);
  assert.equal(liveProviderProof.label, "Live provider proof");
  assert.equal(liveProviderProof.status, "provider-gated");
  assert.deepEqual(
    [...liveProviderProof.packageIds].sort(),
    expectedProviderGatePackages,
  );
  assert.equal(
    liveProviderProof.requiredEvidence,
    "Configure real app-owned credentials and capture live-provider proof without exposing secret values.",
  );

  assert.deepEqual(
    evidenceRows.map((row) => ({
      id: row.id,
      label: row.label,
      status: row.status,
      value: row.value,
    })),
    [
      {
        id: "package-receipts",
        label: "Package health",
        status: "ready",
        value: "20/20",
      },
      {
        id: "browser-route-proof",
        label: "Browser check",
        status: "not-attached",
        value: "Pending",
      },
      {
        id: "live-provider-proof",
        label: "Live integrations",
        status: "provider-gated",
        value: "6 gated",
      },
    ],
  );
});

test("generated preview manifests and dashboard expose score gate without overclaiming", () => {
  const manifestPaths = [
    "examples/template/public/preview-manifest.json",
    ".dx/template-app-browser-preview/public/preview-manifest.json",
  ];

  for (const manifestPath of manifestPaths) {
    const manifest = readJson(manifestPath);
    const gate = manifest.forgePackageReality.scoreGate;
    assert.equal(manifest.forgePackageReality.score, 89);
    assert.equal(manifest.forgePackageReality.unboundedSourceScore, 93);
    assert.equal(gate.currentScore, 89);
    assert.equal(gate.targetScore, 90);
    assert.equal(gate.ceilingWithoutLiveProof, 89);
    assert.equal(gate.browserRuntimeProof, false);
    assert.equal(gate.liveProviderProof, false);
    assert.equal(gate.canExceedCeiling, false);
    assert.deepEqual(
      manifest.forgePackageReality.launchEvidenceSummaryRows.map(
        (row: {
          id: string;
          label: string;
          status: string;
          value: string;
          routes?: readonly string[];
          packageIds?: readonly string[];
        }) => ({
          id: row.id,
          label: row.label,
          status: row.status,
          value: row.value,
          routes: row.routes ?? [],
          packageIds: [...(row.packageIds ?? [])].sort(),
        }),
      ),
      [
        {
          id: "package-receipts",
          label: "Package health",
          status: "ready",
          value: "20/20",
          routes: [],
          packageIds: [],
        },
        {
          id: "browser-route-proof",
          label: "Browser check",
          status: "not-attached",
          value: "Pending",
          routes: ["/", "/dashboard", "/login"],
          packageIds: [],
        },
        {
          id: "live-provider-proof",
          label: "Live integrations",
          status: "provider-gated",
          value: "6 gated",
          routes: [],
          packageIds: expectedProviderGatePackages,
        },
      ],
    );
  }

  const dashboard = read("tools/launch/runtime-template/pages/index.html");
  assert.match(dashboard, /data-dx-forge-reality-score="89"/);
  assert.match(dashboard, /data-dx-forge-score-ceiling="89"/);
  assert.match(dashboard, /data-dx-forge-unbounded-source-score="93"/);
  assert.match(dashboard, /data-dx-forge-score-can-exceed-ceiling="false"/);
  assert.match(dashboard, /data-dx-forge-score-gate-current="89"/);
  assert.match(dashboard, /data-dx-forge-score-gate-target="90"/);
  assert.match(dashboard, /data-dx-forge-score-gate-ceiling="89"/);
  assert.match(dashboard, /data-dx-forge-score-gate-browser-proof="false"/);
  assert.match(dashboard, /data-dx-forge-score-gate-provider-proof="false"/);
  assert.match(dashboard, /data-dx-forge-score-gate-proof="browser-route-proof"/);
  assert.match(dashboard, /data-dx-forge-score-gate-proof="live-provider-proof"/);
  assert.match(dashboard, /data-dx-component="launch-evidence-summary"/);
  assert.match(dashboard, /data-dx-launch-evidence-id="package-receipts"/);
  assert.match(dashboard, /data-dx-launch-evidence-id="browser-route-proof"/);
  assert.match(dashboard, /data-dx-launch-evidence-status="not-attached"/);
  assert.match(dashboard, /data-dx-launch-evidence-score-ceiling="89"/);
  assert.match(dashboard, /data-dx-launch-evidence-browser-proof="false"/);
  assert.match(dashboard, /data-dx-launch-evidence-routes="\/ \/dashboard \/login"/);
  assert.match(dashboard, /data-dx-launch-evidence-id="live-provider-proof"/);
  assert.match(dashboard, /data-dx-launch-evidence-provider-proof="false"/);
  assert.match(dashboard, /<details class="forge-reality-audit-details"[\s\S]*Launch score gate/);

  const packageRealityPanel = read(
    "examples/template/components/template-app/package-reality-panel.tsx",
  );
  assert.match(
    packageRealityPanel,
    /<details[\s\S]*className="forge-reality-audit-details"[\s\S]*data-dx-forge-audit-details-default="collapsed"[\s\S]*>/,
    "source-owned package readiness details must carry the same collapsed audit marker as generated dashboard HTML",
  );

  assert.match(dashboard, />Package health</);
  assert.match(dashboard, />Browser check</);
  assert.match(dashboard, />Route checks</);
  assert.doesNotMatch(
    dashboard,
    />Package evidence<|>Browser proof<|>Route proof<|Rows without provider, adapter, or limited-proof caveats|Files and receipts are present; runtime proof is still limited\./,
  );
  assert.doesNotMatch(dashboard, /data-dx-forge-reality-score="9[0-9]"/);
  assert.doesNotMatch(dashboard, /Launch readiness<\/span><strong>9[0-9]\/100/);
});

test("launch route copy does not carry stale score or green-proof claims", () => {
  const launchSources = [
    "tools/launch/runtime-template/pages/index.html",
    ".dx/template-app-browser-preview/pages/index.html",
  ];
  const generatedDashboardSources = [
    "tools/launch/runtime-template/pages/index.html",
    ".dx/template-app-browser-preview/pages/dashboard.html",
  ];
  const launchCopySources = [
    "examples/template/template-shell.tsx",
    "examples/template/docs-status.tsx",
    "examples/template/drizzle-query-proof.tsx",
    "examples/template/wasm-interop-status.tsx",
    "examples/template/zod-validation-status.tsx",
    ...launchSources,
  ];
  const runtimeSources = [
    "tools/launch/runtime-template/assets/launch-runtime.ts",
    ".dx/template-app-browser-preview/public/launch-runtime.js",
    "examples/template/query-cache-status.tsx",
  ];
  const staleDemoCopy =
    /Runtime dashboard for the demo flow|Launch demo checkout|Local WebAssembly demo|Local WebAssembly interop demo|Local demo remains available|Offline demo|layout reorder demo|launch demo|The demo path uses|Local validation demo|waiting for local demo/i;

  for (const launchSource of launchSources) {
    const launch = read(launchSource);
    assert.doesNotMatch(
      launch,
      /data-dx-forge-reality-score="87"|Launch readiness<\/span><strong>87\/100|green no-op|dx-check is green|dx-check smoke score is 87\/100 green/i,
      `${launchSource} should not expose stale score or green-proof copy`,
    );
    assert.match(
      launch,
      /dx-check source guard is capped at 89\/100 until browser\s+or live-provider proof is attached/i,
      `${launchSource} should point users at the current honest score gate`,
    );
  }

  for (const dashboardSource of generatedDashboardSources) {
    const dashboard = read(dashboardSource);
    assert.match(
      dashboard,
      /data-dx-forge-reality-score="89"/,
      `${dashboardSource} should expose the current launch score in the dashboard contract`,
    );
    assert.match(
      dashboard,
      /Launch readiness<\/span><strong>89\/100/,
      `${dashboardSource} should expose the current launch score in the public dashboard summary`,
    );
    assert.match(
      dashboard,
      /class="forge-reality-score" data-dx-score-scope="package-readiness-row" data-dx-package-score="87">87\/100/,
      `${dashboardSource} should scope package-row scores so they cannot be confused with the launch score`,
    );
    assert.doesNotMatch(
      dashboard,
      /class="forge-reality-score">87\/100/,
      `${dashboardSource} should not expose unscoped package-row scores that look like stale launch scores`,
    );
    assert.doesNotMatch(
      dashboard,
      /data-dx-forge-reality-score="87"|Launch readiness<\/span><strong>87\/100|green no-op|dx-check is green|dx-check smoke score is 87\/100 green/i,
      `${dashboardSource} should not expose stale top-level score or green-proof copy`,
    );
  }

  for (const launchSource of launchCopySources) {
    assert.doesNotMatch(
      read(launchSource),
      staleDemoCopy,
      `${launchSource} should use readiness/workflow language instead of demo copy`,
    );
  }

  assert.doesNotMatch(
    read("examples/template/zod-validation-status.tsx"),
    /\bdemoResult\b/,
    "validation source should use readiness/result naming instead of demo naming",
  );

  for (const runtimeSource of runtimeSources) {
    assert.doesNotMatch(
      read(runtimeSource),
      staleDemoCopy,
      `${runtimeSource} should use readiness/workflow language instead of demo copy`,
    );
  }
});
