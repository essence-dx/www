export type ForgeSafetyArchiveRunbookFixture = {
  readonly packageId: "forge/safety-archive";
  readonly officialPackageName: "Forge Safety Archive";
  readonly upstreamPackage: "dx-forge";
  readonly upstreamVersion: "launch-local";
  readonly sourceMirror: "G:/Dx/www";
  readonly route: "/";
  readonly fixture: "docs/packages/forge-safety-archive.source-guard-runbook.json";
  readonly guardId: "forge-safety-archive-rollback-coverage";
  readonly schema: "dx.forge.safety_archive.source_guard_runbook_fixture";
  readonly honestyLabel: "SOURCE-ONLY";
  readonly runtimeProof: false;
  readonly zedVisibility: "forge-safety-archive:rollback-coverage";
  readonly command: "dx run --test .\\benchmarks\\www-forge-package-lock.test.ts";
  readonly commandPurpose: string;
  readonly scope: "source-only";
  readonly writesFiles: false;
  readonly startsServer: false;
  readonly runsPackageInstall: false;
  readonly runsFullBuild: false;
  readonly nodeModulesRequired: false;
};

export type ForgeSafetyArchivePreviewManifestRoute = {
  readonly route: string;
  readonly sourceGuardRunbookFixtures?: readonly string[];
};

export type ForgeSafetyArchivePreviewManifest = {
  readonly schema: "dx.studio.preview_manifest";
  readonly sourceGuardRunbookFixtures?: readonly ForgeSafetyArchiveRunbookFixture[];
  readonly routes?: readonly ForgeSafetyArchivePreviewManifestRoute[];
};

export type ForgeSafetyArchiveRunbookReadModel = {
  readonly schema: "dx.forge.safety_archive_runbook_read_model";
  readonly previewManifestSource: "public/preview-.dx/build-cache/manifest.json";
  readonly previewManifestRootField: "sourceGuardRunbookFixtures";
  readonly previewManifestRouteField: "routes[].sourceGuardRunbookFixtures";
  readonly route: "/";
  readonly fixturePath: "docs/packages/forge-safety-archive.source-guard-runbook.json";
  readonly guardId: "forge-safety-archive-rollback-coverage";
  readonly zedVisibility: "forge-safety-archive:rollback-coverage";
  readonly command: "dx run --test .\\benchmarks\\www-forge-package-lock.test.ts";
  readonly commandPurpose: string;
  readonly sourceOnly: true;
  readonly runtimeProof: false;
  readonly writesFiles: false;
  readonly startsServer: false;
  readonly runsPackageInstall: false;
  readonly runsFullBuild: false;
  readonly nodeModulesRequired: false;
  readonly availableInPreviewManifest: boolean;
  readonly availableOnLaunchRoute: boolean;
  readonly nextAction: string;
};

const forgeSafetyArchiveFixturePath =
  "docs/packages/forge-safety-archive.source-guard-runbook.json" as const;
const forgeSafetyArchiveGuardId =
  "forge-safety-archive-rollback-coverage" as const;
const forgeSafetyArchiveCommand =
  "dx run --test .\\benchmarks\\www-forge-package-lock.test.ts" as const;
const forgeSafetyArchiveCommandPurpose =
  "Validate source-owned Forge package lock, archive receipt, rollback receipt, cache-file, and safety_archive status coverage for the launch template.";

export const forgeSafetyArchivePreviewManifestFixture = {
  packageId: "forge/safety-archive",
  officialPackageName: "Forge Safety Archive",
  upstreamPackage: "dx-forge",
  upstreamVersion: "launch-local",
  sourceMirror: "G:/Dx/www",
  route: "/",
  fixture: forgeSafetyArchiveFixturePath,
  guardId: forgeSafetyArchiveGuardId,
  schema: "dx.forge.safety_archive.source_guard_runbook_fixture",
  honestyLabel: "SOURCE-ONLY",
  runtimeProof: false,
  zedVisibility: "forge-safety-archive:rollback-coverage",
  command: forgeSafetyArchiveCommand,
  commandPurpose: forgeSafetyArchiveCommandPurpose,
  scope: "source-only",
  writesFiles: false,
  startsServer: false,
  runsPackageInstall: false,
  runsFullBuild: false,
  nodeModulesRequired: false,
} as const satisfies ForgeSafetyArchiveRunbookFixture;

export function readForgeSafetyArchiveRunbookFromPreviewManifest(
  manifest: ForgeSafetyArchivePreviewManifest,
): ForgeSafetyArchiveRunbookReadModel {
  const fixture = manifest.sourceGuardRunbookFixtures?.find(
    (entry) =>
      entry.fixture === forgeSafetyArchiveFixturePath ||
      entry.guardId === forgeSafetyArchiveGuardId,
  );
  const launchRoute = manifest.routes?.find((route) => route.route === "/");
  const availableOnLaunchRoute = Boolean(
    launchRoute?.sourceGuardRunbookFixtures?.includes(
      forgeSafetyArchiveFixturePath,
    ),
  );

  return {
    schema: "dx.forge.safety_archive_runbook_read_model",
    previewManifestSource: "public/preview-.dx/build-cache/manifest.json",
    previewManifestRootField: "sourceGuardRunbookFixtures",
    previewManifestRouteField: "routes[].sourceGuardRunbookFixtures",
    route: "/",
    fixturePath: forgeSafetyArchiveFixturePath,
    guardId: forgeSafetyArchiveGuardId,
    zedVisibility:
      fixture?.zedVisibility ?? "forge-safety-archive:rollback-coverage",
    command: fixture?.command ?? forgeSafetyArchiveCommand,
    commandPurpose: fixture?.commandPurpose ?? forgeSafetyArchiveCommandPurpose,
    sourceOnly: true,
    runtimeProof: false,
    writesFiles: false,
    startsServer: false,
    runsPackageInstall: false,
    runsFullBuild: false,
    nodeModulesRequired: false,
    availableInPreviewManifest: Boolean(fixture),
    availableOnLaunchRoute,
    nextAction: availableOnLaunchRoute
      ? "Show this source-only rollback proof command beside the safety/archive row before React hydration or fresh package-status receipts."
      : "Regenerate public/preview-.dx/build-cache/manifest.json so the /launch route links the safety/archive rollback proof fixture.",
  };
}

export const forgeSafetyArchiveRunbookReadModel =
  readForgeSafetyArchiveRunbookFromPreviewManifest({
    schema: "dx.studio.preview_manifest",
    sourceGuardRunbookFixtures: [forgeSafetyArchivePreviewManifestFixture],
    routes: [
      {
        route: "/",
        sourceGuardRunbookFixtures: [forgeSafetyArchiveFixturePath],
      },
    ],
  });
