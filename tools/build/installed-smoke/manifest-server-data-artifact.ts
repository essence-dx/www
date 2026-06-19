const fs = require("node:fs");
const path = require("node:path");

function emitted(root, outputPath) {
  const fullPath = emittedOutputPath(root, outputPath);
  return {
    path: outputPath || null,
    fullPath,
    insideProject: Boolean(fullPath),
    present: Boolean(fullPath && fs.existsSync(fullPath)),
  };
}

function emittedOutputPath(root, outputPath) {
  if (typeof outputPath !== "string" || outputPath.length === 0) {
    return null;
  }

  const rootPath = path.resolve(root);
  const resolved = path.resolve(rootPath, outputPath);
  const relative = path.relative(rootPath, resolved);
  if (relative.startsWith("..") || path.isAbsolute(relative)) {
    return null;
  }
  return resolved;
}

function readServerDataArtifact(fullPath) {
  const empty = {
    present: false,
    parseOk: false,
    route: null,
    routeSourcePath: null,
    status: null,
    entryCount: null,
    executionModel: null,
    declaresNoNodeModules: false,
    lifecycleScriptsExecuted: false,
    sourceOwnedContract: false,
    externalRuntimeRequired: false,
    externalRuntimeExecuted: false,
    routeParams: {},
    searchParams: {},
    routeParamKeys: [],
    searchParamKeys: [],
  };

  if (!fullPath || !fs.existsSync(fullPath)) {
    return empty;
  }

  let value;
  try {
    value = JSON.parse(fs.readFileSync(fullPath, "utf8"));
  } catch (error) {
    return {
      ...empty,
      present: true,
      parseError: error.message,
    };
  }

  return {
    present: true,
    parseOk: true,
    route: typeof value.route === "string" ? value.route : null,
    routeSourcePath: typeof value.route_source_path === "string" ? value.route_source_path : null,
    status: typeof value.status === "string" ? value.status : null,
    entryCount: Number.isInteger(value.entry_count) ? value.entry_count : null,
    executionModel: typeof value.execution_model === "string" ? value.execution_model : null,
    declaresNoNodeModules: value.node_modules_required === false,
    lifecycleScriptsExecuted: value.lifecycle_scripts_executed === true,
    sourceOwnedContract: value.source_owned_contract === true,
    externalRuntimeRequired: value.external_runtime_required === true,
    externalRuntimeExecuted: value.external_runtime_executed === true,
    routeParams: requestPropObject(value, "route_params"),
    searchParams: requestPropObject(value, "search_params"),
    routeParamKeys: requestPropKeys(value, "route_params"),
    searchParamKeys: requestPropKeys(value, "search_params"),
  };
}

function serverDataArtifactMatchesManifest(route, artifact) {
  if (!route || !artifact.parseOk) {
    return false;
  }

  return (
    artifact.route === (route.route || null) &&
    artifact.routeSourcePath === (route.route_source_path || null) &&
    artifact.status === (route.status || null) &&
    artifact.entryCount === (Number.isInteger(route.entry_count) ? route.entry_count : null) &&
    artifact.executionModel === (route.execution_model || null) &&
    artifact.declaresNoNodeModules === (route.node_modules_required === false) &&
    artifact.lifecycleScriptsExecuted === (route.lifecycle_scripts_executed === true) &&
    artifact.sourceOwnedContract === (route.source_owned_contract === true) &&
    artifact.externalRuntimeRequired === (route.external_runtime_required === true) &&
    artifact.externalRuntimeExecuted === (route.external_runtime_executed === true) &&
    arraysEqual(artifact.routeParamKeys, requestPropKeys(route, "route_params")) &&
    arraysEqual(artifact.searchParamKeys, requestPropKeys(route, "search_params"))
  );
}

function requestPropKeys(value, field) {
  return Object.keys(requestPropObject(value, field)).sort();
}

function requestPropObject(value, field) {
  const props = value?.request?.[field];
  if (!props || typeof props !== "object" || Array.isArray(props)) {
    return {};
  }
  return props;
}

function requestPropsEqual(left, right) {
  return JSON.stringify(stableJson(left)) === JSON.stringify(stableJson(right));
}

function requestPropsMatchArtifact(route, artifact) {
  return (
    requestPropsEqual(requestPropObject(route, "route_params"), artifact.routeParams) &&
    requestPropsEqual(requestPropObject(route, "search_params"), artifact.searchParams)
  );
}

function stableJson(value) {
  if (Array.isArray(value)) {
    return value.map(stableJson);
  }
  if (!value || typeof value !== "object") {
    return value;
  }
  return Object.fromEntries(
    Object.keys(value)
      .sort()
      .map((key) => [key, stableJson(value[key])]),
  );
}

function arraysEqual(left, right) {
  return left.length === right.length && left.every((value, index) => value === right[index]);
}

module.exports = {
  emitted,
  readServerDataArtifact,
  requestPropKeys,
  requestPropObject,
  requestPropsEqual,
  requestPropsMatchArtifact,
  serverDataArtifactMatchesManifest,
};
