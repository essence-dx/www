const { SCHEMAS } = require("./constants.js");

const RECEIPT_PATHS = [
  ".dx/receipts/next-rust/vendor-boundary.json",
  ".dx/receipts/next-rust/vendor-boundary-consumer.json",
];

function status(ok) {
  return ok ? "ok" : "fail";
}

function publicClaimsBlocked(boundary) {
  return (
    boundary.publicDependencyClaimsBlocked === true &&
    boundary.vendoredCargoDependencyClaimsBlocked === true &&
    boundary.vendorSourceInclusionBlocked === true &&
    boundary.publicSourceExposureBlocked === true
  );
}

function activeScopeBadges(activeScope) {
  if (!activeScope) {
    return [];
  }

  return [
    {
      label: "DevTools targets removed",
      status: status(activeScope.removedTargetsBlocked === true),
    },
    {
      label: "Turbopack adoption blocked",
      status: status(
        activeScope.excludedRuntimeTargetsBlocked === true &&
          activeScope.runtimeBuildAdoption === false &&
          activeScope.turbopackPublicArchitecture === false,
      ),
    },
  ];
}

function badges(boundary, blockers, activeScope) {
  return [
    { label: "receipts fresh", status: status(blockers.length === 0) },
    { label: "DX runtime protected", status: status(boundary.runtimeTakeoverBlocked === true) },
    { label: "public claims blocked", status: status(publicClaimsBlocked(boundary)) },
    ...activeScopeBadges(activeScope),
  ];
}

function surfaceStatus(surfaceBadges) {
  return status(surfaceBadges.every((badge) => badge.status === "ok"));
}

function buildEditorSurface(kind, rendererName, boundary, blockers, activeScope) {
  const surfaceBadges = badges(boundary, blockers, activeScope);
  const surface = {
    id: `next-rust.vendor-boundary.${rendererName.toLowerCase()}`,
    kind,
    title: "Next/Turbopack Rust vendor boundary",
    status: surfaceStatus(surfaceBadges),
    severity: "blocking",
    messages: blockers,
    badges: surfaceBadges,
    receiptPaths: RECEIPT_PATHS,
    blockedUntilProven: [`native ${rendererName} surface rendering`],
  };
  if (activeScope) {
    surface.activeScope = activeScope;
  }
  return surface;
}

function buildEditorSurfaceProjection(snapshot, blockers, activeScope) {
  const surfaces = [
    buildEditorSurface("zed.statusSurface", "Zed", snapshot.boundary, blockers, activeScope),
    buildEditorSurface("studio.statusSurface", "Studio", snapshot.boundary, blockers, activeScope),
  ];

  return {
    schema: SCHEMAS.editorSurfaceProjection,
    status: status(surfaces.every((surface) => surface.status === "ok")),
    adapterBoundary: "source-owned JSON projection for Zed/Studio receipt consumers; native renderers unimplemented",
    surfaces,
  };
}

module.exports = {
  buildEditorSurfaceProjection,
};
