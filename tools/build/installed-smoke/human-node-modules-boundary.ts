function printNodeModulesBoundary(build) {
  if (!build || typeof build !== "object") {
    return;
  }
  const beforePaths = normalizePathList(build.nodeModulesBeforePaths);
  const createdPaths = normalizePathList(build.nodeModulesCreatedPaths);
  const afterPaths = normalizePathList(build.nodeModulesPaths);
  if (beforePaths.length === 0 && createdPaths.length === 0 && afterPaths.length === 0) {
    return;
  }
  process.stdout.write(`Node modules: ${afterPaths.length > 0 ? "present" : "absent"}\n`);
  printPathList("Node modules before build", beforePaths);
  printPathList("Node modules created by build", createdPaths);
  printPathList("Node modules after build", afterPaths);
}

function normalizePathList(paths) {
  return Array.isArray(paths)
    ? paths.filter((item) => typeof item === "string" && item.length > 0)
    : [];
}

function printPathList(label, paths) {
  if (paths.length > 0) {
    process.stdout.write(`${label}: ${paths.join(", ")}\n`);
  }
}

module.exports = { printNodeModulesBoundary };
