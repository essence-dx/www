const fs = require("node:fs");
const path = require("node:path");

const SCHEMA = "dx.www.launch.readinessGate";
const FORMAT = 1;

function readinessReport({
  baseUrl,
  binary,
  browserProof,
  blockers,
  compile,
  preview,
  routeSmoke,
  template,
}) {
  return {
    schema: SCHEMA,
    format: FORMAT,
    status: blockers.length === 0 ? "passed" : "failed",
    baseUrl,
    binary,
    blockers,
    compile,
    evidence: readinessEvidence({ browserProof, preview, routeSmoke }),
    preview,
    routeSmoke,
    template,
  };
}

function readinessEvidence({ browserProof, preview, routeSmoke }) {
  const routeSmokePassed = Boolean(routeSmoke && routeSmoke.passed);
  const browserRenderProof = browserProof ? browserProof.summary : null;
  const browserRuntimeProof = Boolean(
    browserProof && browserProof.valid && browserRenderProof,
  );
  const routes =
    routeSmoke && routeSmoke.report && Array.isArray(routeSmoke.report.routes)
      ? routeSmoke.report.routes.map((route) => route.route)
      : [];

  return {
    schema: "dx.www.launch.readinessEvidence",
    format: 1,
    routeSmokeProof: routeSmokePassed,
    routeContentProof: Boolean(
      routeSmokePassed &&
        routeSmoke.report &&
        routeSmoke.report.routeContentProof,
    ),
    routeContentContractRoutes:
      routeSmoke && routeSmoke.report
        ? routeSmoke.report.contentContractRouteCount || 0
        : 0,
    routeSmokeRoutes: routes,
    previewMode: preview && preview.mode ? preview.mode : "managed-preview",
    browserRuntimeProof,
    browserRenderProof,
    liveProviderProof: false,
    scoreGateEligible: false,
    boundary: browserRuntimeProof
      ? "This gate proves compile readiness, preview reachability, HTTP route status, bounded route content markers, and attached browser render screenshots; it does not prove live provider execution or WebGL interaction unless separate receipts say so."
      : "This gate proves compile readiness, preview reachability, HTTP route status, and bounded route content markers only; it does not prove browser canvas rendering, native browser scrolling, WebGL interaction, or live provider execution.",
    requiredFor90Plus: [
      ...(browserRuntimeProof
        ? []
        : ["Attach browser route proof for /, /dashboard, and /login."]),
      "Attach live-provider evidence only after real app-owned credentials execute without exposing secrets.",
    ],
  };
}

function writeReport(options, report) {
  const artifactPath = options.out ? writeReportArtifact(options.out, report) : null;

  if (options.json) {
    process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
    return;
  }
  process.stdout.write(
    `DX WWW launch readiness gate: ${report.status}\n` +
      `- compile: ${report.compile && report.compile.status}\n` +
      `- preview: ${report.preview && report.preview.ready ? "ready" : "not-ready"}\n` +
      `- routes: ${
        report.routeSmoke && report.routeSmoke.report
          ? report.routeSmoke.report.status
          : "not-run"
      }\n` +
      (artifactPath ? `- report: ${artifactPath}\n` : ""),
  );
}

function writeReportArtifact(outputPath, report) {
  const artifactPath = path.resolve(outputPath);
  fs.mkdirSync(path.dirname(artifactPath), { recursive: true });
  fs.writeFileSync(artifactPath, `${JSON.stringify(report, null, 2)}\n`);
  return artifactPath;
}

module.exports = {
  FORMAT,
  SCHEMA,
  readinessReport,
  readinessEvidence,
  writeReportArtifact,
  writeReport,
};
