const {
  normalizeRouteHandlerMethod,
  normalizeRouteHandlerRequirement,
  normalizeRouteHandlerRoute,
  normalizeRouteHandlerSourcePath,
} = require("./route-handler-requirements.ts");

function summarizeUnexpectedRouteHandlerEvidence(entries, expectedHandlers, options = {}) {
  const expected = expectedHandlers.map((handler) => normalizeRouteHandlerRequirement(handler));
  const grouped = new Map();

  for (const entry of entries) {
    if (typeof options.entryFilter === "function" && !options.entryFilter(entry)) {
      continue;
    }
    const evidence = routeHandlerEvidence(entry, options);
    if (expected.some((handler) => routeHandlerEvidenceMatches(evidence, handler, options))) {
      continue;
    }

    const key = routeHandlerEvidenceKey(evidence);
    const existing = grouped.get(key);
    if (existing) {
      existing.duplicateCount += 1;
      continue;
    }
    grouped.set(key, { ...evidence, duplicateCount: 0 });
  }

  return [...grouped.values()].sort((left, right) =>
    routeHandlerEvidenceKey(left).localeCompare(routeHandlerEvidenceKey(right)),
  );
}

function routeHandlerEvidence(entry, options) {
  const runtimeBoundary = entry?.runtime_boundary || {};
  const methods = routeHandlerEvidenceMethods(entry, options);
  const evidence = {
    sourcePath: normalizeRouteHandlerSourcePath(routeHandlerEvidenceField(entry, options.sourcePathField || "source_path")),
    route: normalizeRouteHandlerRoute(routeHandlerEvidenceField(entry, options.routeField || "request_path")),
  };
  if (options.includeMethods === true) {
    evidence.methods = methods;
  } else {
    evidence.method = normalizeRouteHandlerMethod(routeHandlerEvidenceField(entry, options.methodField || "method"));
  }
  if (options.includeExecutionContract === true) {
    evidence.sourceOwnedContract = entry?.execution_model === "source-owned-route-handler-contract";
  }
  if (options.includeNoNodeModules === true) {
    evidence.declaresNoNodeModules = entry?.node_modules_required === false;
    evidence.nodeModulesRequired = entry?.node_modules_required === true;
  }
  if (options.includeLifecycleScripts === true) {
    evidence.lifecycleScriptsExecuted = entry?.lifecycle_scripts_executed === true;
  }
  if (options.includeRuntimeBoundary === true) {
    evidence.sourceOwnedContract = runtimeBoundary.source_owned === true;
    evidence.declaresNoNodeModules = entry?.node_modules_required === false;
    evidence.nodeModulesRequired = entry?.node_modules_required === true;
    evidence.nodeModulesPresent = entry?.node_modules_present === true;
    evidence.lifecycleScriptsExecuted = entry?.lifecycle_scripts_executed === true;
    evidence.externalRuntimeRequired = runtimeBoundary.external_runtime_required === true;
    evidence.externalRuntimeExecuted = runtimeBoundary.external_runtime_executed === true;
  }
  if (options.includeSkipReason === true) {
    evidence.reason = typeof entry?.reason === "string" ? entry.reason : null;
  }
  return evidence;
}

function routeHandlerEvidenceMatches(evidence, expected, options) {
  if (evidence.sourcePath !== expected.sourcePath || evidence.route !== expected.route) {
    return false;
  }
  if (options.matchMethod === false) {
    return true;
  }
  if (evidence.methods) {
    return optionsMatchMethodArray(evidence.methods, expected);
  }
  return (
    evidence.method === expected.method
  );
}

function routeHandlerEvidenceKey(evidence) {
  if (Array.isArray(evidence.methods)) {
    return [evidence.sourcePath || "", evidence.route || "", evidence.methods.join(",")].join("|");
  }
  return [evidence.sourcePath || "", evidence.method || "", evidence.route || ""].join("|");
}

function optionsMatchMethodArray(methods, expected) {
  return expected.method ? methods.includes(expected.method) : true;
}

function routeHandlerEvidenceMethods(entry, options) {
  const methods = routeHandlerEvidenceField(entry, options.methodsField || "methods");
  return Array.isArray(methods)
    ? methods
        .map((method) => normalizeRouteHandlerMethod(method))
        .filter(Boolean)
    : [];
}

function routeHandlerEvidenceField(entry, field) {
  if (typeof field === "function") {
    return field(entry);
  }
  return entry?.[field];
}

module.exports = { summarizeUnexpectedRouteHandlerEvidence };
