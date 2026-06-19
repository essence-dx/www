const MAX_ROUTE_HANDLER_STALE_EVIDENCE_ENTRIES = 20;

const ARTIFACT_ORDER = new Map([
  ["source-build-manifest", 0],
  ["dx-build-graph", 1],
  ["route-handler-receipts", 2],
  ["route-handler-skipped", 3],
]);

function summarizeRouteHandlerStaleEvidence(sourceBuild) {
  const entries = [
    ...normalizeEntries(
      "source-build-manifest",
      sourceBuild?.manifest?.unexpectedRouteHandlers,
    ),
    ...normalizeEntries(
      "dx-build-graph",
      sourceBuild?.graphReceipt?.unexpectedRouteHandlerNodes,
    ),
    ...normalizeEntries(
      "route-handler-receipts",
      sourceBuild?.routeHandlerReceipt?.unexpectedReceipts,
    ),
    ...normalizeEntries(
      "route-handler-skipped",
      sourceBuild?.routeHandlerReceipt?.unexpectedSkips,
    ),
  ].sort(compareEvidenceEntries);
  const shownEntries = entries.slice(0, MAX_ROUTE_HANDLER_STALE_EVIDENCE_ENTRIES);

  return {
    present: entries.length > 0,
    count: entries.length,
    omittedCount: Math.max(0, entries.length - shownEntries.length),
    entries: shownEntries,
  };
}

function formatRouteHandlerStaleEvidenceEntry(entry) {
  const method = Array.isArray(entry?.methods) && entry.methods.length > 0
    ? entry.methods.join(",")
    : "METHOD";
  const route = entry?.route || "(unknown route)";
  const sourcePath = entry?.sourcePath || "unknown source";
  const label = `${entry?.artifact || "route-handler-artifact"}: ${method} ${route} (${sourcePath})`;
  const details = [];

  if (entry?.reason) {
    details.push(`skipped: ${entry.reason}`);
  }
  if (entry?.sourceOwnedContract !== null && entry?.sourceOwnedContract !== undefined) {
    details.push(`source-owned: ${formatYesNo(entry.sourceOwnedContract)}`);
  }
  if (entry?.declaresNoNodeModules !== null && entry?.declaresNoNodeModules !== undefined) {
    details.push(`no node_modules: ${formatYesNo(routeHandlerEvidenceDeclaresNoNodeModules(entry))}`);
  }
  if (entry?.duplicateCount > 0) {
    details.push(`duplicates: ${entry.duplicateCount}`);
  }
  if (entry?.lifecycleScriptsExecuted === true) {
    details.push("lifecycle scripts executed: yes");
  }
  if (entry?.externalRuntimeRequired === true) {
    details.push("external runtime required: yes");
  }
  if (entry?.externalRuntimeExecuted === true) {
    details.push("external runtime executed: yes");
  }

  return details.length > 0 ? `${label}; ${details.join("; ")}` : label;
}

function normalizeEntries(artifact, entries) {
  if (!Array.isArray(entries)) {
    return [];
  }
  return entries.map((entry) => normalizeEntry(artifact, entry));
}

function normalizeEntry(artifact, entry) {
  return {
    artifact,
    sourcePath: stringOrNull(entry?.sourcePath),
    route: stringOrNull(entry?.route),
    methods: normalizeMethods(entry),
    duplicateCount: Number.isInteger(entry?.duplicateCount) ? entry.duplicateCount : 0,
    sourceOwnedContract: booleanOrNull(entry?.sourceOwnedContract),
    declaresNoNodeModules: booleanOrNull(entry?.declaresNoNodeModules),
    nodeModulesRequired: booleanOrNull(entry?.nodeModulesRequired),
    nodeModulesPresent: booleanOrNull(entry?.nodeModulesPresent),
    lifecycleScriptsExecuted: booleanOrNull(entry?.lifecycleScriptsExecuted),
    externalRuntimeRequired: booleanOrNull(entry?.externalRuntimeRequired),
    externalRuntimeExecuted: booleanOrNull(entry?.externalRuntimeExecuted),
    reason: stringOrNull(entry?.reason),
  };
}

function normalizeMethods(entry) {
  const methods = Array.isArray(entry?.methods) ? entry.methods : [entry?.method];
  return methods
    .filter((method) => typeof method === "string" && method.length > 0)
    .sort((left, right) => left.localeCompare(right));
}

function compareEvidenceEntries(left, right) {
  return (
    artifactOrder(left.artifact) - artifactOrder(right.artifact) ||
    compareText(left.sourcePath, right.sourcePath) ||
    compareText(left.route, right.route) ||
    compareText(left.methods.join(","), right.methods.join(","))
  );
}

function artifactOrder(artifact) {
  return ARTIFACT_ORDER.get(artifact) ?? 99;
}

function routeHandlerEvidenceDeclaresNoNodeModules(entry) {
  return entry.declaresNoNodeModules === true &&
    entry.nodeModulesRequired !== true &&
    entry.nodeModulesPresent !== true;
}

function formatYesNo(value) {
  return value === true ? "yes" : "no";
}

function compareText(left, right) {
  return String(left || "").localeCompare(String(right || ""));
}

function booleanOrNull(value) {
  return typeof value === "boolean" ? value : null;
}

function stringOrNull(value) {
  return typeof value === "string" ? value : null;
}

module.exports = {
  formatRouteHandlerStaleEvidenceEntry,
  summarizeRouteHandlerStaleEvidence,
};
