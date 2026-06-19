const { REMOVED_PARITY_DEPLOY_ARTIFACT_KEY } = require("./next-familiar-compatibility.ts");

function summarizeServerContracts(input) {
  const contracts = Array.isArray(input.serverContracts.value) ? input.serverContracts.value : [];
  return {
    present: input.serverContracts.ok,
    count: contracts.length,
    hasHealthRouteHandler: contracts.some(
      (contract) =>
        contract.source_path === "app/api/health/route.ts" &&
        contract.endpoint === "/api/health",
    ),
    hasCheckoutRouteHandler: contracts.some(
      (contract) =>
        contract.source_path === "app/api/checkout/route.ts" &&
        contract.endpoint === "/api/checkout",
    ),
    hasServerLoader: contracts.some(
      (contract) => contract.source_path === "server/loaders.ts",
    ),
    lifecycleScriptsExecuted: contracts.some(
      (contract) => contract.lifecycle_scripts_executed === true,
    ),
  };
}

function summarizeDeployAdapter(input) {
  const deploy = input.deployAdapter.value || {};
  const routes = Array.isArray(deploy.routes) ? deploy.routes : [];
  const healthChecks = Array.isArray(deploy.health_checks) ? deploy.health_checks : [];
  const routeHandlers = Array.isArray(deploy.route_handlers) ? deploy.route_handlers : [];
  const nextFamiliarCompatibilityEvidence = deploy.next_familiar_compatibility_evidence || {};
  const checkoutRouteHandler = summarizeCheckoutRouteHandler(routeHandlers);
  return {
    present: input.deployAdapter.ok,
    noNodeModulesRequired: deploy.no_node_modules_required === true,
    hasNextFamiliarCompatibilityEvidence:
      nextFamiliarCompatibilityEvidence.path === "next-familiar-compatibility-evidence.json" &&
      nextFamiliarCompatibilityEvidence.evidence_kind === "next-familiar-compatibility" &&
      nextFamiliarCompatibilityEvidence.evidence_mode === "next-familiar-source-output-readiness" &&
      nextFamiliarCompatibilityEvidence.score === 100,
    hasRemovedParityDeployArtifact:
      Object.prototype.hasOwnProperty.call(deploy, REMOVED_PARITY_DEPLOY_ARTIFACT_KEY),
    hasRootServerDataRoute: routes.some(
      (route) => route.path === "/" && route.server_data === "app/server-data.json",
    ),
    hasHealthCheck: healthChecks.some(
      (check) =>
        check.path === "/api/health" &&
        check.source_path === "app/api/health/route.ts" &&
        check.method === "GET",
    ),
    hasCheckoutRouteHandler: checkoutRouteHandler.complete,
    checkoutRouteHandlerPresent: checkoutRouteHandler.present,
    checkoutRouteHandlerSourceOwnedRuntimeBoundary:
      checkoutRouteHandler.sourceOwnedRuntimeBoundary,
    checkoutRouteHandlerExternalRuntimeRequired:
      checkoutRouteHandler.externalRuntimeRequired,
    checkoutRouteHandlerExternalRuntimeExecuted:
      checkoutRouteHandler.externalRuntimeExecuted,
  };
}

function serverArtifactFailures(report) {
  const failures = [];
  const contracts = report.build.serverContracts;
  const deploy = report.build.deployAdapter;

  pushIf(failures, !contracts.present, "dx build did not write server-contracts.json");
  pushIf(
    failures,
    !contracts.hasHealthRouteHandler,
    "server contracts are missing app/api/health/route.ts",
  );
  pushIf(
    failures,
    !contracts.hasCheckoutRouteHandler,
    "server contracts are missing app/api/checkout/route.ts",
  );
  pushIf(failures, !contracts.hasServerLoader, "server contracts are missing server/loaders.ts");
  pushIf(
    failures,
    contracts.lifecycleScriptsExecuted,
    "server contracts executed lifecycle scripts",
  );
  pushIf(failures, !deploy.present, "dx build did not write deploy-adapter.json");
  pushIf(failures, !deploy.noNodeModulesRequired, "deploy adapter does not prove no-node_modules");
  pushIf(failures, !deploy.hasRootServerDataRoute, "deploy adapter is missing root server-data route");
  pushIf(failures, !deploy.hasHealthCheck, "deploy adapter is missing /api/health check");
  pushIf(
    failures,
    !deploy.checkoutRouteHandlerPresent,
    "deploy adapter is missing /api/checkout route handler",
  );
  if (deploy.checkoutRouteHandlerPresent) {
    pushIf(
      failures,
      !deploy.checkoutRouteHandlerSourceOwnedRuntimeBoundary,
      "deploy adapter /api/checkout route handler does not declare source-owned runtime boundary",
    );
    pushIf(
      failures,
      deploy.checkoutRouteHandlerExternalRuntimeRequired,
      "deploy adapter /api/checkout route handler requires an external runtime",
    );
    pushIf(
      failures,
      deploy.checkoutRouteHandlerExternalRuntimeExecuted,
      "deploy adapter /api/checkout route handler executed an external runtime",
    );
  }

  return failures;
}

function summarizeCheckoutRouteHandler(routeHandlers) {
  const handler = routeHandlers.find(
    (candidate) =>
      candidate.path === "/api/checkout" &&
      candidate.source_path === "app/api/checkout/route.ts" &&
      Array.isArray(candidate.methods) &&
      candidate.methods.includes("POST") &&
      Array.isArray(candidate.skipped_build_methods) &&
      candidate.skipped_build_methods.includes("POST") &&
      candidate.build_execution === "skipped-build-execution" &&
      candidate.node_modules_required === false,
  );
  const runtimeBoundary = handler?.runtime_boundary || {};
  const sourceOwnedRuntimeBoundary = runtimeBoundary.source_owned === true;
  const externalRuntimeRequired = runtimeBoundary.external_runtime_required === true;
  const externalRuntimeExecuted = runtimeBoundary.external_runtime_executed === true;

  return {
    present: Boolean(handler),
    sourceOwnedRuntimeBoundary,
    externalRuntimeRequired,
    externalRuntimeExecuted,
    complete:
      Boolean(handler) &&
      sourceOwnedRuntimeBoundary &&
      !externalRuntimeRequired &&
      !externalRuntimeExecuted,
  };
}

function pushIf(failures, condition, message) {
  if (condition) {
    failures.push(message);
  }
}

module.exports = {
  serverArtifactFailures,
  summarizeDeployAdapter,
  summarizeServerContracts,
};
