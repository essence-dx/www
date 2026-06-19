function summarizeSourceModuleResolver(manifest) {
  const modules = sourceModules(manifest);
  const diagnosticModules = new Map();
  const nodeModuleModules = new Map();
  const nodeModulePathModules = new Map();
  const nodeModuleDependencies = new Map();
  const nodeModuleBoundaryDependencies = new Map();
  let diagnosticCount = 0;

  for (const module of modules) {
    const sourcePath = sourceModulePath(module);
    const moduleDiagnostics = countDiagnostics(module?.diagnostics);
    if (moduleDiagnostics > 0) {
      diagnosticCount += moduleDiagnostics;
      diagnosticModules.set(sourcePath, { sourcePath, diagnosticCount: moduleDiagnostics });
    }
    if (module?.node_modules_required === true) {
      nodeModuleModules.set(sourcePath, { sourcePath });
    }
    for (const pathEvidence of moduleNodeModulesPathEvidence(module)) {
      nodeModulePathModules.set(`${sourcePath}\0${pathEvidence.pathKind}\0${pathEvidence.path}`, {
        sourcePath,
        ...pathEvidence,
      });
    }
    for (const dependency of dependencies(module)) {
      if (!dependencyRequiresNodeModules(dependency)) {
        if (dependencyHasNodeModulesBoundary(dependency)) {
          const specifier = dependencySpecifier(dependency);
          const resolverSource = stringOrNull(dependency?.resolver_source);
          const resolverDetail = stringOrNull(dependency?.resolver_detail);
          const key = `${sourcePath}\0${specifier}\0${resolverSource}\0${resolverDetail}`;
          nodeModuleBoundaryDependencies.set(key, { sourcePath, specifier, resolverSource, resolverDetail });
        }
        continue;
      }
      const specifier = dependencySpecifier(dependency);
      const resolvedPath = dependencyNodeModulesPath(dependency) || dependencyResolvedPath(dependency);
      nodeModuleDependencies.set(`${sourcePath}\0${specifier}\0${resolvedPath}`, {
        sourcePath,
        specifier,
        resolvedPath,
      });
    }
  }

  return {
    present: modules.length > 0,
    moduleCount: modules.length,
    diagnosticCount,
    nodeModuleModuleCount: nodeModuleModules.size,
    nodeModulePathModuleCount: nodeModulePathModules.size,
    nodeModuleDependencyCount: nodeModuleDependencies.size,
    nodeModuleBoundaryDependencyCount: nodeModuleBoundaryDependencies.size,
    diagnosticModules: Array.from(diagnosticModules.values()),
    nodeModuleModules: Array.from(nodeModuleModules.values()),
    nodeModulePathModules: Array.from(nodeModulePathModules.values()),
    nodeModuleDependencies: Array.from(nodeModuleDependencies.values()),
    nodeModuleBoundaryDependencies: Array.from(nodeModuleBoundaryDependencies.values()),
  };
}

function sourceModuleResolverFailures(summary) {
  if (!summary?.present) return ["source-build manifest is missing source module resolver evidence"];
  const failures = [];
  for (const module of summary.diagnosticModules || []) {
    const count = module.diagnosticCount || 1;
    const suffix = count === 1 ? "resolver diagnostic" : "resolver diagnostics";
    failures.push(`source-build source module ${module.sourcePath} has ${count} ${suffix}`);
  }
  for (const module of summary.nodeModuleModules || []) {
    failures.push(`source-build source module ${module.sourcePath} requires node_modules`);
  }
  for (const module of summary.nodeModulePathModules || []) {
    failures.push(`source-build source module ${module.sourcePath} has node_modules ${module.pathKind} ${module.path}`);
  }
  for (const dependency of summary.nodeModuleDependencies || []) {
    failures.push(`source-build source module ${dependency.sourcePath} dependency ${dependency.specifier} requires node_modules`);
  }
  for (const dependency of summary.nodeModuleBoundaryDependencies || []) {
    const detail = dependency.resolverDetail || dependency.resolverSource || "node_modules-boundary";
    failures.push(`source-build source module ${dependency.sourcePath} dependency ${dependency.specifier} crosses node_modules adapter boundary (${detail})`);
  }
  return failures;
}

function sourceModules(manifest) {
  const routeModules = Array.isArray(manifest?.route_outputs)
    ? manifest.route_outputs.flatMap((output) => Array.isArray(output?.source_module_chunks) ? output.source_module_chunks : [])
    : [];
  const manifestModules = Array.isArray(manifest?.source_modules) ? manifest.source_modules : [];
  return routeModules.concat(manifestModules).filter(Boolean);
}

function dependencies(module) {
  return Array.isArray(module?.dependencies) ? module.dependencies.filter(Boolean) : [];
}

function moduleNodeModulesPathEvidence(module) {
  const evidence = [];
  const seen = new Set();
  for (const [pathKind, path] of [
    ["source_path", module?.source_path],
    ["path", module?.path],
    ["chunk_output", module?.chunk_output],
    ["output", module?.output],
  ]) {
    if (!pathContainsNodeModulesSegment(path)) {
      continue;
    }
    const key = `${pathKind}\0${path}`;
    if (!seen.has(key)) {
      seen.add(key);
      evidence.push({ pathKind, path });
    }
  }
  return evidence;
}

function dependencyRequiresNodeModules(dependency) {
  return dependency?.node_modules_required === true ||
    Boolean(dependencyNodeModulesPath(dependency));
}

function dependencyHasNodeModulesBoundary(dependency) {
  if (!dependency || dependencyNodeModulesPath(dependency)) return false;
  return pathContainsNodeModulesSegment(dependency.specifier) ||
    [dependency.kind, dependency.resolver_source, dependency.resolver_detail]
      .some(namesNodeModulesBoundary);
}

function countDiagnostics(diagnostics) {
  if (Array.isArray(diagnostics)) return diagnostics.length;
  if (typeof diagnostics === "number") return diagnostics > 0 ? diagnostics : 0;
  return diagnostics ? 1 : 0;
}

function sourceModulePath(module) {
  return module?.source_path || module?.path || "<unknown-source-module>";
}

function dependencySpecifier(dependency) {
  return dependency?.specifier || dependencyResolvedPath(dependency) || "<unknown-import>";
}

function dependencyResolvedPath(dependency) {
  return dependency?.resolved_path || dependency?.path || dependency?.output || null;
}

function dependencyNodeModulesPath(dependency) {
  return [
    dependency?.resolved_path,
    dependency?.path,
    dependency?.output,
    dependency?.chunk_output,
  ].find(pathContainsNodeModulesSegment) || null;
}

function pathContainsNodeModulesSegment(value) {
  return String(value || "").replace(/\\/g, "/").split("/")
    .some((segment) => segment.toLowerCase() === "node_modules");
}

function namesNodeModulesBoundary(value) {
  return /(^|[-_])node[-_]?modules($|[-_])/i.test(String(value || ""));
}

function stringOrNull(value) {
  return typeof value === "string" && value.length > 0 ? value : null;
}

module.exports = {
  sourceModuleResolverFailures,
  summarizeSourceModuleResolver,
};
