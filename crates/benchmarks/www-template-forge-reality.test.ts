import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");

const lockBackedPackageIds = [
  "shadcn/ui/button",
  "state/zustand",
  "tanstack/query",
  "validation/zod",
  "forms/react-hook-form",
  "db/drizzle-sqlite",
  "instantdb/react",
  "supabase/client",
  "api/trpc",
  "reactive/store",
  "content/react-markdown",
  "content/fumadocs-next",
  "i18n/next-intl",
  "auth/better-auth",
  "payments/stripe-js",
  "ai/vercel-ai",
  "automations/n8n",
  "wasm/bindgen",
  "animation/motion",
  "3d/launch-scene",
];

const lane4PackageIds = [
  "db/drizzle-sqlite",
  "instantdb/react",
  "supabase/client",
  "api/trpc",
];

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function readJson(relativePath: string) {
  return JSON.parse(read(relativePath));
}

function readTemplateReadinessReceiptRows() {
  const readinessRoot = path.join(
    root,
    "examples",
    "template",
    ".dx",
    "forge",
    "template-readiness",
  );

  return fs
    .readdirSync(readinessRoot)
    .filter((name) => name.endsWith(".json"))
    .sort()
    .map((name) => {
      const receipt = JSON.parse(
        fs.readFileSync(path.join(readinessRoot, name), "utf8"),
      );
      const packageIds = Array.isArray(receipt.package_ids)
        ? receipt.package_ids
        : [receipt.package_id].filter(Boolean);

      return {
        packageId: receipt.package_id ?? packageIds.join(","),
        packageIds,
        readinessReceipt: `.dx/forge/template-readiness/${name}`,
        endpoint: receipt.readiness_route ?? null,
        classification: receipt.classification,
        runtimeProof: receipt.runtime_proof === true,
        secretValues: [],
      };
    });
}

function escapeRegExp(value: string) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function countPhysicalCacheManifests() {
  return listPhysicalCacheManifestPaths().length;
}

function listCacheArchiveManifestPaths() {
  const archiveRoot = path.join(root, "examples", "template", ".dx", "forge", "cache-archive");
  const manifestPaths: string[] = [];

  if (!fs.existsSync(archiveRoot)) {
    return manifestPaths;
  }

  function visit(directory: string) {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const fullPath = path.join(directory, entry.name);
      if (entry.isDirectory()) {
        visit(fullPath);
      } else if (entry.name === ".dx/build-cache/manifest.json") {
        manifestPaths.push(
          path.relative(path.join(root, "examples", "template"), fullPath).replace(/\\/g, "/"),
        );
      }
    }
  }

  visit(archiveRoot);
  return manifestPaths.sort();
}

function listPhysicalCacheManifestPaths() {
  const cacheRoot = path.join(root, "examples", "template", ".dx", "forge", "cache");
  const manifestPaths: string[] = [];

  function visit(directory: string) {
    for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
      const fullPath = path.join(directory, entry.name);
      if (entry.isDirectory()) {
        visit(fullPath);
      } else if (entry.name === ".dx/build-cache/manifest.json") {
        manifestPaths.push(
          path.relative(path.join(root, "examples", "template"), fullPath).replace(/\\/g, "/"),
        );
      }
    }
  }

  visit(cacheRoot);
  return manifestPaths.sort();
}

test("Forge reality model separates lock-backed packages from real controls", async () => {
  const { defaultTemplateLockReality } = await import(
    "../examples/template/components/template-app/package-lock-reality.ts"
  );
  const reality = await import("../examples/template/components/template-app/package-reality.ts");
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const lock = readJson("examples/template/.dx/forge/package-lock.json");
  const sourceManifest = readJson("examples/template/.dx/forge/source-.dx/build-cache/manifest.json");
  const realitySource = read("examples/template/components/template-app/package-reality.ts");
  const dashboardSource = read("examples/template/components/template-app/dashboard-page.tsx");
  const panelSource = read("examples/template/components/template-app/package-reality-panel.tsx");
  const physicalCacheManifestPaths = listPhysicalCacheManifestPaths();
  const archivedCacheManifestPaths = listCacheArchiveManifestPaths();
  const physicalCacheManifestCount = physicalCacheManifestPaths.length;
  const currentPhysicalCacheManifestPaths = physicalCacheManifestPaths.filter((manifestPath) =>
    status.cache.manifests.includes(manifestPath),
  );
  const stalePhysicalCacheManifestPaths = physicalCacheManifestPaths.filter(
    (manifestPath) => !status.cache.manifests.includes(manifestPath),
  );

  assert.deepEqual(defaultTemplateLockReality.packageIds, lockBackedPackageIds);
  assert.deepEqual(status.locked_package_names, lockBackedPackageIds);
  assert.equal(lock.packages.length, lockBackedPackageIds.length);
  assert.equal(status.cache.manifests.length, lockBackedPackageIds.length);
  assert.deepEqual(
    currentPhysicalCacheManifestPaths,
    [...status.cache.manifests].sort(),
    "current package-status manifests must exist in active .dx/forge/cache",
  );
  assert.equal(status.cache.physical_manifest_count, physicalCacheManifestCount);
  assert.equal(status.cache.physical_cache_manifest_count, physicalCacheManifestCount);
  assert.equal(status.cache.stale_physical_manifest_count, stalePhysicalCacheManifestPaths.length);
  assert.equal(status.cache.stale_physical_cache_manifest_count, stalePhysicalCacheManifestPaths.length);
  assert.deepEqual(status.cache.stale_physical_manifest_paths, stalePhysicalCacheManifestPaths);
  assert.deepEqual(status.cache.stale_physical_cache_manifest_paths, stalePhysicalCacheManifestPaths);
  assert.equal(status.cache.archived_manifest_count, archivedCacheManifestPaths.length);
  assert.equal(status.cache.archived_cache_manifest_count, archivedCacheManifestPaths.length);
  assert.deepEqual(status.cache.archived_manifest_paths, archivedCacheManifestPaths);
  assert.deepEqual(status.cache.archived_cache_manifest_paths, archivedCacheManifestPaths);
  assert.equal(status.cache.cache_archive_root, ".dx/forge/cache-archive");
  assert.equal(status.cache.cache_archive_caveat_id, "cache-archive-excluded-from-current-lock");
  for (const manifestPath of archivedCacheManifestPaths) {
    assert.equal(
      status.cache.manifests.includes(manifestPath),
      false,
      `${manifestPath} should be archive-only and excluded from current package-status manifests`,
    );
  }
  assert.equal(status.package_lane_visibility.length, 20);
  for (const packageId of lockBackedPackageIds) {
    assert.ok(
      lock.packages.some((entry: { name: string }) => entry.name === packageId),
      `${packageId} should be promoted into the Forge package lock`,
    );
    assert.ok(
      sourceManifest.packages.some((entry: { package_id: string }) => entry.package_id === packageId),
      `${packageId} should be backed by the Forge source manifest`,
    );
  }
  assert.equal(reality.forgeRealitySummary.lockBackedPackageCount, lockBackedPackageIds.length);
  assert.equal(
    reality.forgeRealitySummary.currentCacheManifestCount,
    status.cache.manifests.length,
  );
  assert.equal(
    reality.forgeRealitySummary.currentLockBackedManifests,
    status.cache.manifests.length,
  );
  assert.equal(
    reality.forgeRealitySummary.physicalCacheManifests,
    physicalCacheManifestCount,
  );
  assert.equal(
    reality.forgeRealitySummary.stalePhysicalCacheManifests,
    stalePhysicalCacheManifestPaths.length,
  );
  assert.deepEqual(
    status.cache.stale_physical_cache_manifest_paths,
    stalePhysicalCacheManifestPaths,
  );
  assert.deepEqual(
    reality.forgeRealitySummary.stalePhysicalCacheManifestPaths,
    stalePhysicalCacheManifestPaths,
  );
  assert.equal(
    reality.forgeRealitySummary.archivedCacheManifestCount,
    archivedCacheManifestPaths.length,
  );
  assert.deepEqual(
    reality.forgeRealitySummary.archivedCacheManifestPaths,
    archivedCacheManifestPaths,
  );
  assert.equal(
    reality.forgeRealitySummary.cacheArchiveCaveatId,
    "cache-archive-excluded-from-current-lock",
  );
  assert.equal(
    reality.forgeRealitySummary.stalePhysicalCacheManifestPaths.length,
    reality.forgeRealitySummary.stalePhysicalCacheManifestCount,
  );
  assert.equal(
    reality.forgeRealitySummary.cacheManifestSource,
    "package-status-current-manifests",
  );
  assert.equal(
    reality.forgeRealitySummary.cacheManifestCaveatId,
    "physical-cache-matches-current-manifests",
  );
  assert.equal(
    reality.forgeRealitySummary.realControlCount,
    reality.interactiveForgePackageRows.length,
  );
  assert.equal(
    reality.forgeRealitySummary.realLockBackedControlCount,
    reality.realLockBackedForgePackageRows.length,
  );
  assert.equal(
    reality.forgeRealitySummary.statusOnlyPackageCount,
    reality.statusOnlyForgePackageRows.length,
  );
  assert.equal(reality.forgeRealitySummary.dummyOrMisleadingCount, 0);
  assert.equal(
    reality.forgeRealitySummary.providerGatedCount,
    reality.forgeRealityRows.filter((row) => row.maturityKind === "provider-gated").length,
  );
  assert.deepEqual(
    reality.providerGatedReadinessRows.map((row: { packageId: string }) => row.packageId).sort(),
    reality.providerGatedForgePackageRows.map((row) => row.packageId).sort(),
    "provider readiness cards should cover every provider-gated package",
  );
  assert.equal(
    reality.forgeRealitySummary.adapterBoundaryReadinessCount,
    reality.forgeRealityRows.filter((row) => row.maturityKind === "adapter-boundary-readiness").length,
  );
  assert.equal(
    reality.forgeRealityRows.find((row) => row.packageId === "payments/stripe-js")?.maturityKind,
    "provider-gated",
  );
  assert.equal(
    reality.forgeRealityRows.find((row) => row.packageId === "supabase/client")?.maturityKind,
    "provider-gated",
  );
  assert.equal(
    reality.forgeRealityRows.find((row) => row.packageId === "tanstack/query")?.maturityKind,
    "adapter-boundary-readiness",
  );
  assert.equal(
    reality.forgeRealityRows.find((row) => row.packageId === "3d/launch-scene")?.maturityKind,
    "source-owned-limited-proof",
  );
  assert.ok(
    reality.interactiveForgePackageRows.some(
      (row) => row.packageId === "tanstack/query" && row.realityLevelId === "lock-backed-adapter-boundary",
    ),
    "TanStack Query should expose a real source-owned control while staying adapter-boundary until live QueryClient proof exists",
  );
  for (const packageId of ["wasm/bindgen", "animation/motion"]) {
    assert.ok(
      reality.interactiveForgePackageRows.some(
        (row) =>
          row.packageId === packageId &&
          row.realityLevelId === "source-owned-not-runtime-proven" &&
          row.maturityKind === "source-owned-limited-proof",
      ),
      `${packageId} should expose an interactive source-owned dashboard control without runtime-proof overclaim`,
    );
  }

  assert.match(realitySource, /export function classifyForgePackageReality/);
  assert.match(realitySource, /"real lock-backed Forge package"/);
  assert.match(realitySource, /"source-owned but not runtime-proven"/);
  assert.match(realitySource, /"adapter-boundary"/);
  assert.match(realitySource, /"docs\/receipt\/source-guard only"/);
  assert.match(realitySource, /lockBackedPackageCount: defaultTemplateLockReality\.packageIds\.length/);
  assert.match(realitySource, /currentCacheManifestCount/);
  assert.match(realitySource, /currentLockBackedManifests/);
  assert.match(realitySource, /stalePhysicalCacheManifestPaths/);
  assert.match(realitySource, /archivedCacheManifestPaths/);
  assert.match(realitySource, /package-status-current-manifests/);
  assert.match(realitySource, /physical-cache-matches-current-manifests/);
  assert.match(realitySource, /cache-archive-excluded-from-current-lock/);
  assert.match(realitySource, /provider-gated/);
  assert.match(realitySource, /adapter-boundary-readiness/);
  assert.match(realitySource, /realControlCount/);
  assert.match(realitySource, /nonInteractivePackageLaneCount/);
  assert.match(realitySource, /scoreComponents/);
  assert.match(realitySource, /scoreGate/);
  assert.match(realitySource, /scoreCeilingWithoutLiveProof/);
  assert.match(realitySource, /providerBoundaryCoverage/);
  assert.equal(
    new Set(reality.forgeRealitySummary.scoreComponents.map((component) => component.id)).size,
    reality.forgeRealitySummary.scoreComponents.length,
    "score component ids should be unique so launch scoring is auditable",
  );

  assert.match(dashboardSource, /ForgeRealityPanel/);
  assert.match(panelSource, /className="forge-reality-audit-details"/);
  assert.match(panelSource, /Package readiness details/);
  assert.match(panelSource, /data-dx-forge-stale-cache-manifest-paths/);
  assert.match(panelSource, /data-dx-forge-cache-archive-manifest-count/);
  assert.match(panelSource, /data-dx-cache-archive-manifest-list/);
  assert.match(panelSource, /Workspace controls/);
  assert.match(panelSource, /Integration readiness details/);
  assert.match(panelSource, /Package readiness summary/);
  assert.match(panelSource, /Package set/);
  assert.match(panelSource, /Runtime model/);
  assert.match(panelSource, /Current packages/);
  assert.match(panelSource, /Setup required/);
  assert.match(panelSource, /Route checks/);
  assert.match(panelSource, /Next proof needed/);
  assert.match(panelSource, /data-dx-forge-provider-boundary-coverage/);
  assert.match(panelSource, /data-dx-forge-score-ceiling/);
  assert.match(panelSource, /data-dx-forge-score-gate/);
  assert.match(panelSource, /Launch score gate/);
  assert.match(panelSource, /data-dx-forge-score-components/);
  assert.match(
    panelSource,
    /All\s+\{forgeRealitySummary\.totalVisiblePackageLanes\}\s+packages have visible template surfaces/,
  );
  assert.doesNotMatch(panelSource, /lanes are readiness surfaces/);
  assert.match(panelSource, /providerGatedReadinessRows/);
  assert.doesNotMatch(
    panelSource,
    /Available Forge controls|Status and readiness lanes|Provider credentials and browser\/runtime proof stay explicitly gated|Forge package reality summary|Package maturity details|Current receipts|Setup gates|Missing to reach 90\+|Route proof|Rows without provider, adapter, or limited-proof caveats|Files and receipts are present; runtime proof is still limited/,
  );
  assert.match(dashboardSource, /forgeRealitySummary\.score/);
  assert.match(dashboardSource, /forgeRealitySummary\.lockBackedPackageCount/);
  assert.match(dashboardSource, /forgeRealitySummary\.realControlCount/);
  assert.doesNotMatch(dashboardSource, /templatePackageModules\.map/);
  assert.doesNotMatch(dashboardSource, /manager@dx\.local|Signed in/);
});

test("materialized dashboard proves real controls without fake runtime claims", async () => {
  const reality = await import("../examples/template/components/template-app/package-reality.ts");
  const realControlPackageIds = reality.interactiveForgePackageRows.map((row) => row.packageId);
  const realLockBackedControlPackageIds = reality.realLockBackedForgePackageRows.map((row) => row.packageId);
  const statusOnlyPackageIds = reality.statusOnlyForgePackageRows.map((row) => row.packageId);
  const lockBackedPackageIdsByMaturity = reality.forgeRealityRows
    .filter((row) => row.maturityKind === "lock-backed-package")
    .map((row) => row.packageId);
  const providerGatedPackageIds = reality.providerGatedForgePackageRows.map((row) => row.packageId);
  const adapterBoundaryPackageIds = reality.adapterBoundaryReadinessForgePackageRows.map(
    (row) => row.packageId,
  );
  const sourceOwnedLimitedProofPackageIds = reality.sourceOwnedLimitedProofForgePackageRows.map(
    (row) => row.packageId,
  );
  const sourceGuardOnlyPackageIds = reality.sourceGuardOnlyForgePackageRows.map((row) => row.packageId);
  const maturityCoverage = {
    schema: "dx.forge.package_maturity_coverage",
    visiblePackageLaneCount: reality.forgeRealityRows.length,
    coveredPackageLaneCount: reality.forgeRealityRows.length,
    unknownPackageLaneCount: 0,
    allVisibleLanesClassified: true,
    allowedMaturityKinds: [
      "lock-backed-package",
      "provider-gated",
      "adapter-boundary-readiness",
      "source-owned-limited-proof",
      "source-guard-only",
    ],
    counts: {
      lockBackedPackage: lockBackedPackageIdsByMaturity.length,
      providerGated: providerGatedPackageIds.length,
      adapterBoundaryReadiness: adapterBoundaryPackageIds.length,
      sourceOwnedLimitedProof: sourceOwnedLimitedProofPackageIds.length,
      sourceGuardOnly: sourceGuardOnlyPackageIds.length,
    },
    packageIdsByMaturity: {
      lockBackedPackage: lockBackedPackageIdsByMaturity,
      providerGated: providerGatedPackageIds,
      adapterBoundaryReadiness: adapterBoundaryPackageIds,
      sourceOwnedLimitedProof: sourceOwnedLimitedProofPackageIds,
      sourceGuardOnly: sourceGuardOnlyPackageIds,
      unknown: [],
    },
    publicSummaryFirst: true,
    auditDetailsDefault: "collapsed",
  };
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const physicalCacheManifestPaths = listPhysicalCacheManifestPaths();
  const archivedCacheManifestPaths = listCacheArchiveManifestPaths();
  const physicalCacheManifestCount = physicalCacheManifestPaths.length;
  const stalePhysicalCacheManifestPaths = physicalCacheManifestPaths.filter(
    (manifestPath) => !status.cache.manifests.includes(manifestPath),
  );
  const stalePhysicalCacheManifestCount = stalePhysicalCacheManifestPaths.length;
  const archivedCacheManifestCount = archivedCacheManifestPaths.length;
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-forge-reality-"));
  try {
    execFileSync(process.execPath, [materializer, dir], { cwd: root, stdio: "pipe" });

    const dashboard = fs.readFileSync(path.join(dir, "pages", "dashboard.html"), "utf8");
    const manifest = JSON.parse(
      fs.readFileSync(path.join(dir, "public", "preview-.dx/build-cache/manifest.json"), "utf8"),
    );
  const dashboardRoute = manifest.routes.find((entry: { route: string }) => entry.route === "/dashboard");
  const realControls = [
    ...dashboard.matchAll(/<article[^>]*data-dx-package-control="source-owned-template-control"[\s\S]*?<\/article>/g),
  ].map(([row]) => row);

  assert.match(dashboard, /data-dx-component="forge-package-reality-dashboard"/);
  assert.match(dashboard, /data-dx-forge-reality-score="\d+"/);
  assert.match(
    dashboard,
    new RegExp(`data-dx-forge-lock-backed-package-count="${lockBackedPackageIds.length}"`),
  );
  assert.match(
    dashboard,
    new RegExp(
      `data-dx-forge-current-cache-manifest-count="${status.cache.manifests.length}"`,
    ),
  );
  assert.match(
    dashboard,
    new RegExp(
      `data-dx-forge-current-lock-backed-manifests="${status.cache.manifests.length}"`,
    ),
  );
  assert.match(
    dashboard,
    new RegExp(
      `data-dx-forge-physical-cache-manifest-count="${physicalCacheManifestCount}"`,
    ),
  );
  assert.match(
    dashboard,
    new RegExp(`data-dx-forge-physical-cache-manifests="${physicalCacheManifestCount}"`),
  );
  assert.match(
    dashboard,
    new RegExp(
      `data-dx-forge-stale-physical-cache-manifest-count="${stalePhysicalCacheManifestCount}"`,
    ),
  );
  assert.match(
    dashboard,
    new RegExp(`data-dx-forge-stale-physical-cache-manifests="${stalePhysicalCacheManifestCount}"`),
  );
  assert.match(
    dashboard,
    new RegExp(
      `data-dx-forge-stale-cache-manifest-paths="${escapeRegExp(
        stalePhysicalCacheManifestPaths.join(" "),
      )}"`,
    ),
  );
  assert.doesNotMatch(dashboard, /data-dx-stale-cache-manifest-list/);
  for (const manifestPath of stalePhysicalCacheManifestPaths) {
    assert.match(dashboard, new RegExp(escapeRegExp(manifestPath)));
  }
  assert.match(
    dashboard,
    /data-dx-forge-cache-manifest-source="package-status-current-manifests"/,
  );
  assert.match(
    dashboard,
    /data-dx-forge-cache-manifest-caveat="physical-cache-matches-current-manifests"/,
  );
  assert.match(
    dashboard,
    /data-dx-forge-cache-archive-root="\.dx\/forge\/cache-archive"/,
  );
  assert.match(
    dashboard,
    new RegExp(`data-dx-forge-cache-archive-manifest-count="${archivedCacheManifestCount}"`),
  );
  assert.match(
    dashboard,
    new RegExp(
      `data-dx-forge-cache-archive-manifest-paths="${escapeRegExp(
        archivedCacheManifestPaths.join(" "),
      )}"`,
    ),
  );
  assert.match(
    dashboard,
    /data-dx-forge-cache-archive-caveat="cache-archive-excluded-from-current-lock"/,
  );
  assert.match(dashboard, /data-dx-cache-archive-manifest-list/);
  for (const manifestPath of archivedCacheManifestPaths) {
    assert.match(dashboard, new RegExp(escapeRegExp(manifestPath)));
  }
  assert.match(
    dashboard,
    new RegExp(`data-dx-forge-real-control-count="${realControlPackageIds.length}"`),
  );
  assert.match(
    dashboard,
    new RegExp(`data-dx-forge-real-lock-backed-count="${realLockBackedControlPackageIds.length}"`),
  );
  assert.match(
    dashboard,
    new RegExp(`data-dx-forge-interactive-surface-count="${realControlPackageIds.length}"`),
  );
  assert.match(
    dashboard,
    new RegExp(`data-dx-forge-status-lane-count="${statusOnlyPackageIds.length}"`),
  );
  assert.match(
    dashboard,
    new RegExp(`data-dx-forge-provider-gated-count="${providerGatedPackageIds.length}"`),
  );
  assert.match(
    dashboard,
    new RegExp(
      `data-dx-forge-adapter-boundary-readiness-count="${adapterBoundaryPackageIds.length}"`,
    ),
  );
  assert.match(
    dashboard,
    new RegExp(
      `data-dx-forge-source-owned-limited-proof-count="${sourceOwnedLimitedProofPackageIds.length}"`,
    ),
  );
  assert.match(
    dashboard,
    new RegExp(`data-dx-forge-source-guard-only-count="${sourceGuardOnlyPackageIds.length}"`),
  );
  assert.match(
    dashboard,
    /data-dx-forge-reality-level="lock-backed-adapter-boundary"[\s\S]*tanstack\/query/,
  );
  assert.match(
    dashboard,
    /data-dx-package-maturity="provider-gated"[\s\S]*payments\/stripe-js/,
  );
  assert.match(
    dashboard,
    /data-dx-package-maturity="source-owned-limited-proof"[\s\S]*3d\/launch-scene/,
  );
  assert.match(
    dashboard,
    /data-dx-package-id="wasm\/bindgen"[\s\S]*data-dx-package-maturity="source-owned-limited-proof"[\s\S]*data-template-module="wasm-bindgen-readiness"/,
  );
  assert.match(
    dashboard,
    /data-dx-package-id="animation\/motion"[\s\S]*data-dx-package-maturity="source-owned-limited-proof"[\s\S]*data-template-module="lane7-motion-stage"/,
  );
  assert.match(dashboard, /data-dx-component="forge-package-maturity-details"/);
  assert.match(dashboard, /data-dx-forge-public-summary-first="true"/);
  assert.match(dashboard, /data-dx-forge-audit-details-default="collapsed"/);
  assert.match(
    dashboard,
    /<details class="forge-reality-audit-details"[\s\S]*?data-dx-component="forge-package-maturity-details"[\s\S]*?>/,
  );
  assert.match(
    dashboard,
    /<details class="forge-reality-audit-details"[\s\S]*data-dx-component="forge-package-maturity-details"[\s\S]*<div class="forge-reality-table"/,
  );
  assert.match(dashboard, /<summary>Package readiness details<\/summary>/);
  assert.match(dashboard, /Workspace controls/);
  assert.match(dashboard, /Integration readiness details/);
  assert.match(dashboard, /Package readiness summary/);
  assert.match(dashboard, /data-dx-component="forge-package-maturity-summary"/);
  assert.match(dashboard, /data-dx-package-maturity-summary="visible"/);
  assert.match(
    dashboard,
    new RegExp(
      `data-dx-package-maturity-kind="lock-backed-package"[\\s\\S]*data-dx-package-maturity-count="${lockBackedPackageIdsByMaturity.length}"`,
    ),
  );
  assert.match(
    dashboard,
    new RegExp(
      `data-dx-package-maturity-kind="provider-gated"[\\s\\S]*data-dx-package-maturity-count="${providerGatedPackageIds.length}"`,
    ),
  );
  assert.match(
    dashboard,
    new RegExp(
      `data-dx-package-maturity-kind="adapter-boundary-readiness"[\\s\\S]*data-dx-package-maturity-count="${adapterBoundaryPackageIds.length}"`,
    ),
  );
  assert.match(
    dashboard,
    new RegExp(
      `data-dx-package-maturity-kind="source-owned-limited-proof"[\\s\\S]*data-dx-package-maturity-count="${sourceOwnedLimitedProofPackageIds.length}"`,
    ),
  );
  assert.match(
    dashboard,
    new RegExp(
      `data-dx-package-maturity-kind="source-guard-only"[\\s\\S]*data-dx-package-maturity-count="${sourceGuardOnlyPackageIds.length}"`,
    ),
  );
  assert.match(dashboard, /Ready modules/);
  assert.doesNotMatch(dashboard, /Ready surfaces/);
  assert.match(dashboard, /Provider setup/);
  assert.match(dashboard, /Adapter boundary/);
  assert.match(dashboard, /Source-ready modules/);
  assert.match(dashboard, /Guarded source/);
  assert.match(dashboard, /Package set/);
  assert.match(dashboard, /Credentialed providers and browser checks remain gated until the app supplies evidence\./);
  assert.match(dashboard, /Runtime model/);
  assert.match(dashboard, /Current packages/);
  assert.match(dashboard, /Setup required/);
  assert.match(dashboard, /Route checks/);
  assert.match(dashboard, /Next proof needed/);
  assert.doesNotMatch(
    dashboard,
    />Route proof<|>Browser proof<|>Package evidence<|>Source proof<|Rows without provider, adapter, or limited-proof caveats|Files and receipts are present; runtime proof is still limited\./,
  );
  assert.match(dashboard, /data-dx-forge-provider-boundary-coverage="true"/);
  assert.match(dashboard, /data-dx-forge-score-components/);
  assert.match(dashboard, /Provider readiness coverage/);
  assert.match(
    dashboard,
    new RegExp(
      `All ${reality.forgeRealitySummary.totalVisiblePackageLanes} packages have visible template surfaces\\.`,
    ),
  );
  assert.match(
    dashboard,
    new RegExp(
      `${reality.forgeRealitySummary.providerGatedCount} provider integrations still need credentials\\.`,
    ),
  );
  assert.doesNotMatch(dashboard, /lanes are readiness surfaces/);
  for (const packageId of providerGatedPackageIds) {
    assert.match(
      dashboard,
      new RegExp(`data-dx-package-id="${escapeRegExp(packageId)}"[\\s\\S]*data-dx-runtime-proof="false"`),
      `${packageId} should have a provider readiness card without runtime proof`,
    );
  }
  for (const endpoint of [
    "/api/auth/readiness",
    "/api/instant/readiness",
    "/api/supabase/readiness",
    "/api/payments/stripe-js/readiness",
    "/api/ai/chat",
    "/api/automations/n8n/dry-run",
  ]) {
    assert.match(dashboard, new RegExp(escapeRegExp(endpoint)));
  }
  assert.doesNotMatch(
    dashboard,
    /Real Forge controls|Status and readiness lanes|No hosted auth, payments, database, AI, deployment, or renderer runtime is claimed here|Only packages with visible DX control proof get interactions|Forge package reality summary|Credentialed providers and browser\/runtime checks remain gated until the app supplies proof|Package maturity details|Current receipts|Setup gates|Missing to reach 90\+/,
  );

  assert.equal(realControls.length, realControlPackageIds.length);
  for (const packageId of realControlPackageIds) {
    assert.ok(
      realControls.some((row) => row.includes(packageId)),
      `${packageId} should have a real lock-backed control`,
    );
  }
  for (const packageId of statusOnlyPackageIds) {
    assert.equal(
      realControls.some((row) => row.includes(packageId)),
      false,
      `${packageId} must remain lock-backed without a fake interactive control`,
    );
  }

  assert.doesNotMatch(dashboard, /data-app-auth-label>Signed in|data-app-session-email>manager@dx\.local/);
  assert.ok(dashboardRoute, "dashboard route should be present");
  const expectedTemplateReadinessReceipts = readTemplateReadinessReceiptRows();
  assert.deepEqual(manifest.forgePackageReality, {
    schema: "dx.forge.template_package_reality",
    score: reality.forgeRealitySummary.score,
    packageAverageScore: reality.forgeRealitySummary.packageAverageScore,
    scoreCeilingWithoutLiveProof: reality.forgeRealitySummary.scoreCeilingWithoutLiveProof,
    unboundedSourceScore: reality.forgeRealitySummary.unboundedSourceScore,
    scoreComponents: reality.forgeRealitySummary.scoreComponents,
    scoreGate: reality.forgeRealitySummary.scoreGate,
    launchEvidenceSummaryRows: reality.launchEvidenceSummaryRows.map(
      (row: {
        id: string;
        label: string;
        value: string;
        status: string;
        statusLabel: string;
        description: string;
        scoreImpact: string;
        iconName: string;
        routes?: readonly string[];
        packageIds?: readonly string[];
      }) => ({
        id: row.id,
        label: row.label,
        value: row.value,
        status: row.status,
        statusLabel: row.statusLabel,
        description: row.description,
        scoreImpact: row.scoreImpact,
        iconName: row.iconName,
        routes: row.routes ?? [],
        packageIds: row.packageIds ?? [],
      }),
    ),
    lockBackedPackageCount: lockBackedPackageIds.length,
    visiblePackageLaneCount: reality.forgeRealitySummary.totalVisiblePackageLanes,
    realControlCount: realControlPackageIds.length,
    readinessOnlyLaneCount: statusOnlyPackageIds.length,
    currentLockBackedManifests: status.cache.manifests.length,
    physicalCacheManifestCount,
    stalePhysicalCacheManifestCount,
    stalePhysicalCacheManifestPaths,
    cacheArchiveRoot: ".dx/forge/cache-archive",
    archivedCacheManifestCount,
    archivedCacheManifestPaths,
    providerGatedCount: reality.forgeRealitySummary.providerGatedCount,
    providerBoundaryCoverage: reality.forgeRealitySummary.providerBoundaryCoverage,
    readinessExecutionProofCount: reality.forgeRealitySummary.readinessExecutionProofCount,
    readinessExecutionProofPackageCount:
      reality.forgeRealitySummary.readinessExecutionProofPackageCount,
    readinessExecutionProofPackageIds:
      reality.forgeRealitySummary.readinessExecutionProofPackageIds,
    readinessExecutionProofCoverage:
      reality.forgeRealitySummary.readinessExecutionProofCoverage,
    adapterBoundaryReadinessCount:
      reality.forgeRealitySummary.adapterBoundaryReadinessCount,
    sourceOwnedLimitedProofCount:
      reality.forgeRealitySummary.sourceOwnedLimitedProofCount,
    sourceGuardOnlyCount: reality.forgeRealitySummary.sourceGuardOnlyCount,
    dummyOrMisleadingCount: 0,
    cacheManifestSource: "package-status-current-manifests",
    cacheManifestCaveatId: "physical-cache-matches-current-manifests",
    cacheArchiveCaveatId: "cache-archive-excluded-from-current-lock",
    maturityCoverage,
    templateReadinessReceipts: expectedTemplateReadinessReceipts,
  });
  assert.deepEqual(manifest.forgePackageReality.maturityCoverage, maturityCoverage);
  assert.equal(
    manifest.forgePackageReality.templateReadinessReceipts.length,
    expectedTemplateReadinessReceipts.length,
    "preview manifest should list every copied template-readiness receipt without inflating package counts",
  );
  assert.deepEqual(
    manifest.forgePackageReality.templateReadinessReceipts.map(
      (receipt: { runtimeProof: boolean; secretValues: readonly string[] }) => ({
        runtimeProof: receipt.runtimeProof,
        secretValues: receipt.secretValues,
      }),
    ),
    manifest.forgePackageReality.templateReadinessReceipts.map(() => ({
      runtimeProof: false,
      secretValues: [],
    })),
  );
  assert.equal(manifest.forgePackageRealityRows.length, reality.forgeRealityRows.length);
  assert.deepEqual(
    manifest.forgePackageRealityRows.map(
      (row: {
        packageId: string;
        packageName: string;
        maturityKind: string;
        realityLevelId: string;
        score: number;
        receiptStatus: string;
        templateSurface: string;
        hasInteractiveControl: boolean;
        controlId: string | null;
      }) => row,
    ),
    reality.forgeRealityRows.map((row) => ({
      packageId: row.packageId,
      packageName: row.packageName,
      maturityKind: row.maturityKind,
      realityLevelId: row.realityLevelId,
      score: row.score,
      receiptStatus: row.receiptStatus,
      templateSurface: row.controlId ? "interactive-control" : "readiness-only",
      hasInteractiveControl: Boolean(row.controlId),
      controlId: row.controlId ?? null,
    })),
  );
  assert.ok(
    manifest.forgePackageRealityRows.some(
      (row: { packageId: string; maturityKind: string; templateSurface: string }) =>
        row.packageId === "payments/stripe-js" &&
        row.maturityKind === "provider-gated" &&
        row.templateSurface === "interactive-control",
    ),
  );
  assert.ok(
    manifest.forgePackageRealityRows.some(
      (row: { packageId: string; maturityKind: string; templateSurface: string }) =>
        row.packageId === "tanstack/query" &&
        row.maturityKind === "adapter-boundary-readiness" &&
        row.templateSurface === "interactive-control",
    ),
  );
  assert.ok(
    manifest.forgePackageRealityRows.some(
      (row: { packageId: string; maturityKind: string; templateSurface: string }) =>
        row.packageId === "3d/launch-scene" &&
        row.maturityKind === "source-owned-limited-proof" &&
        row.templateSurface === "interactive-control",
    ),
  );
  assert.ok(dashboardRoute.forgePackages.includes("content/react-markdown"));
  for (const packageId of lane4PackageIds) {
    assert.ok(dashboardRoute.forgePackages.includes(packageId));
  }
  assert.equal(dashboardRoute.forgePackages.includes("lucide-react"), false);
  for (const packageId of [...lockBackedPackageIds, "auth/better-auth"]) {
    assert.match(dashboard, new RegExp(escapeRegExp(packageId)));
  }
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("database and api lane packages expose source-owned default-template files", () => {
  const sourceManifest = readJson("examples/template/.dx/forge/source-.dx/build-cache/manifest.json");
  const status = readJson("examples/template/.dx/forge/package-status.json");
  const routeHandler = read("examples/template/app/api/trpc/[trpc]/route.ts");
  const routeHandlerFactory = read("examples/template/lib/trpc/route-handler.ts");
  const trpcRouter = read("examples/template/lib/trpc/router.ts");

  for (const file of [
    "examples/template/db/drizzle/schema.ts",
    "examples/template/db/drizzle/dashboard-workflow.ts",
    "examples/template/lib/instant/schema.ts",
    "examples/template/lib/instant/status.ts",
    "examples/template/lib/supabase/env.ts",
    "examples/template/lib/supabase/profiles.ts",
    "examples/template/lib/trpc/router.ts",
    "examples/template/app/api/trpc/[trpc]/route.ts",
  ]) {
    assert.ok(fs.existsSync(path.join(root, file)), `${file} should be materialized`);
  }

  for (const packageId of lane4PackageIds) {
    const sourcePackage = sourceManifest.packages.find(
      (entry: { package_id: string }) => entry.package_id === packageId,
    );
    const statusPackage = status.packages.find((entry: { name: string }) => entry.name === packageId);
    assert.equal(statusPackage?.integrity_state, "valid", `${packageId} should have valid lock integrity`);
    assert.ok(sourcePackage?.files?.length > 0, `${packageId} should expose source-owned files`);
    assert.ok(
      status.cache.manifests.some((manifest: string) => manifest.includes(packageId.replace(/[^a-z0-9]+/gi, "-"))),
      `${packageId} should have a Forge cache manifest`,
    );
  }

  assert.match(routeHandler, /dxTrpcRouteHandler/);
  assert.match(routeHandlerFactory, /createDxTrpcRouteHandler/);
  assert.match(trpcRouter, /launchReadiness|health/);
  assert.doesNotMatch(routeHandler, /node_modules|lucide-react/);
});

test("launch auth surface reviews boundaries without faking a session", () => {
  const authStatus = read("examples/template/auth-session-status.tsx");
  const launchPage = read("tools/launch/runtime-template/pages/index.html");
  const runtime = read("tools/launch/runtime-template/assets/launch-runtime.ts");
  const routeContract = read("examples/template/template-route-contract.ts");
  const forbiddenFakeSessionPatterns = [
    /Preview account session/,
    /preview-local-session/,
    /local-demo-session-preview/,
    /data-dx-auth-local-demo-session/,
    /data-dx-auth-local-demo-state/,
    /dx-better-auth-demo-session/,
    /No local demo session/,
    /No local demo account/,
  ];

  for (const source of [authStatus, launchPage, runtime, routeContract]) {
    for (const pattern of forbiddenFakeSessionPatterns) {
      assert.doesNotMatch(source, pattern);
    }
  }

  assert.match(authStatus, /type LaunchAuthBoundaryReview = \{/);
  assert.match(authStatus, /function markBoundaryReviewed/);
  assert.match(authStatus, /data-dx-auth-boundary-review=\{/);
  assert.match(authStatus, /data-dx-auth-interaction="mark-boundary-reviewed"/);
  assert.match(launchPage, /data-dx-component="better-auth-boundary-review"/);
  assert.match(launchPage, /data-dx-auth-boundary-review="idle"/);
  assert.match(launchPage, /data-dx-auth-interaction="mark-boundary-reviewed"/);
  assert.match(runtime, /better-auth-boundary-review/);
  assert.match(runtime, /data-dx-auth-boundary-review/);
  assert.match(runtime, /function markBoundaryReviewed\(\)/);

  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-auth-boundary-"));
  try {
    execFileSync(process.execPath, [materializer, dir], { cwd: root, stdio: "pipe" });
    const materializedLaunch = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");

    assert.match(materializedLaunch, /data-dx-component="better-auth-boundary-review"/);
    assert.match(materializedLaunch, /data-dx-auth-boundary-review="idle"/);
    for (const pattern of forbiddenFakeSessionPatterns) {
      assert.doesNotMatch(materializedLaunch, pattern);
    }
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("checked-in preview manifests stay synced with Forge reality source", async () => {
  const reality = await import("../examples/template/components/template-app/package-reality.ts");
  const previewManifests = [
    ".dx/template-app-browser-preview/public/preview-.dx/build-cache/manifest.json",
    "examples/template/public/preview-.dx/build-cache/manifest.json",
  ];

  for (const manifestPath of previewManifests) {
    const manifest = readJson(manifestPath);
    const forgeReality = manifest.forgePackageReality;

    assert.equal(
      forgeReality.score,
      reality.forgeRealitySummary.score,
      `${manifestPath} should not report a stale Forge readiness score`,
    );
    assert.deepEqual(
      forgeReality.scoreComponents,
      reality.forgeRealitySummary.scoreComponents,
      `${manifestPath} should expose the current score component receipt`,
    );
    assert.deepEqual(
      forgeReality.scoreGate,
      reality.forgeRealitySummary.scoreGate,
      `${manifestPath} should expose the current launch score gate`,
    );
    assert.equal(
      forgeReality.unboundedSourceScore,
      reality.forgeRealitySummary.unboundedSourceScore,
      `${manifestPath} should expose the uncapped source score`,
    );
    assert.equal(
      forgeReality.readinessExecutionProofPackageCount,
      reality.forgeRealitySummary.readinessExecutionProofPackageCount,
      `${manifestPath} should expose current route-helper execution proof`,
    );
    assert.deepEqual(
      forgeReality.readinessExecutionProofPackageIds,
      reality.forgeRealitySummary.readinessExecutionProofPackageIds,
      `${manifestPath} should list current route-helper execution packages`,
    );
  }
});
