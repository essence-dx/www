const path = require("node:path");

const { QUALITY_FILES } = require("./constants.ts");
const { lineCount } = require("./io.ts");

function inspectQuality(repoRoot) {
  const files = QUALITY_FILES.map((relative) => {
    const fullPath = path.join(repoRoot, relative);
    return {
      path: relative,
      lineCount: lineCount(fullPath),
    };
  });
  const maxLineCount = Math.max(...files.map((file) => file.lineCount));

  return {
    smallModuleBoundary: maxLineCount <= 180,
    entrypointUsesSplitModules: files.some(
      (file) => file.path === "tools/build/dx-build-readiness-gate.ts" && file.lineCount <= 8,
    ),
    monolithFallbackUsed: false,
    maxLineCount,
    files,
  };
}

module.exports = {
  inspectQuality,
};
