const fs = require("node:fs");
const crypto = require("node:crypto");
const path = require("node:path");

const { readCssAssetReferences } = require("./asset-references.ts");
const { CONTRACT_NAMES, nodeId, toPosixPath } = require("./types.ts");

const SCAN_ROOTS = [
  "app",
  "pages",
  "components",
  "lib",
  "server",
  "src",
  "styles",
  "public",
  ".dx/forge",
  ".dx/receipts/check",
  ".dx/deploy",
  ".dx/build",
  "out",
];

const SKIP_DIRS = new Set([".git", ".next", "node_modules", "target"]);

function scanProject(projectRoot) {
  const nodes = [];
  const edges = [];
  const rawImports = [];
  const nodesByPath = new Map();

  for (const scanRoot of SCAN_ROOTS) {
    const absoluteRoot = path.join(projectRoot, scanRoot);
    if (!fs.existsSync(absoluteRoot)) continue;
    for (const filePath of walkFiles(absoluteRoot)) {
      const relativePath = toPosixPath(path.relative(projectRoot, filePath));
      const kind = classifyPath(relativePath);
      if (!kind) continue;

      const node = createFileNode(kind, relativePath, filePath);
      addNode(nodes, nodesByPath, node);

      if (isSourceModule(kind)) {
        rawImports.push(...readImportSpecifiers(filePath, node));
      }
      if (kind === "dx-style-css") {
        rawImports.push(...readCssAssetReferences(filePath, node));
      }
    }
  }

  addForgeSurfaceNodes(projectRoot, nodes, edges, nodesByPath);
  addReceiptSourceEdges(projectRoot, edges, nodesByPath);
  addDeploySourceEdges(projectRoot, edges, nodesByPath);

  return { nodes, edges, rawImports, nodesByPath };
}

function* walkFiles(root) {
  for (const entry of fs.readdirSync(root, { withFileTypes: true })) {
    const absolute = path.join(root, entry.name);
    if (entry.isDirectory()) {
      if (!SKIP_DIRS.has(entry.name)) {
        yield* walkFiles(absolute);
      }
      continue;
    }
    if (entry.isFile()) {
      yield absolute;
    }
  }
}

function classifyPath(relativePath) {
  if (relativePath.startsWith("public/")) return "public-asset";
  if (relativePath.startsWith(".dx/receipts/check/") && relativePath.endsWith(".json")) {
    return "dx-check-receipt";
  }
  if (
    (relativePath.startsWith(".dx/deploy/") || relativePath.startsWith(".dx/build/")) &&
    !relativePath.endsWith("/")
  ) {
    return "deploy-output";
  }
  if (relativePath.startsWith("out/")) return "deploy-output";
  if (relativePath.endsWith(".generated.css") || relativePath === ".dx/generated/dx-style.css") {
    return "dx-style-css";
  }
  if (relativePath.startsWith("styles/") && relativePath.endsWith(".css")) {
    return "dx-style-css";
  }
  if (isRoutePath(relativePath)) return "tsx-route";
  if (isComponentPath(relativePath)) return "tsx-component";
  if (isSupportSourceModulePath(relativePath)) return "source-module";
  return null;
}

function isRoutePath(relativePath) {
  if (!/\.(tsx|ts|jsx|js|html)$/.test(relativePath)) return false;
  const routePath = stripSourcePrefix(relativePath);
  return (
    routePath.startsWith("pages/") ||
    /^app\/.*\/?(page|layout|route)\.(tsx|ts|jsx|js)$/.test(routePath)
  );
}

function isComponentPath(relativePath) {
  const componentPath = stripSourcePrefix(relativePath);
  return componentPath.startsWith("components/") && /\.(tsx|ts|jsx|js)$/.test(componentPath);
}

function isSupportSourceModulePath(relativePath) {
  return /^(lib|server|src)\//.test(relativePath) && /\.(tsx|ts|jsx|js)$/.test(relativePath);
}

function stripSourcePrefix(relativePath) {
  return relativePath.startsWith("src/") ? relativePath.slice(4) : relativePath;
}

function isSourceModule(kind) {
  return kind === "tsx-route" || kind === "tsx-component" || kind === "source-module";
}

function createFileNode(kind, relativePath, absolutePath) {
  const stat = fs.statSync(absolutePath);
  const contentHash = crypto
    .createHash("sha256")
    .update(fs.readFileSync(absolutePath))
    .digest("hex");
  return {
    id: nodeId(kind, relativePath),
    kind,
    path: relativePath,
    contract: contractForKind(kind),
    bytes: stat.size,
    contentHash,
  };
}

function contractForKind(kind) {
  if (kind === "forge-surface") return CONTRACT_NAMES.forgeSourceGraph;
  if (kind === "tsx-route" || kind === "tsx-component" || kind === "source-module") {
    return CONTRACT_NAMES.wwwModuleGraph;
  }
  return CONTRACT_NAMES.buildGraph;
}

function addNode(nodes, nodesByPath, node) {
  if (!nodesByPath.has(node.path)) {
    nodes.push(node);
    nodesByPath.set(node.path, node);
  }
}

function readImportSpecifiers(filePath, fromNode) {
  const source = fs.readFileSync(filePath, "utf8");
  const specifiers = [];
  const importPattern =
    /\bimport\s+(?:(?:[\s\S]*?)\s+from\s+)?["']([^"']+)["']|import\(\s*["']([^"']+)["']\s*\)|require\(\s*["']([^"']+)["']\s*\)/g;
  let match;
  while ((match = importPattern.exec(source)) !== null) {
    const specifier = match[1] || match[2] || match[3];
    if (specifier && isSourceOwnedImportSpecifier(specifier)) {
      specifiers.push({ from: fromNode, specifier });
    }
  }
  return specifiers;
}

function isSourceOwnedImportSpecifier(specifier) {
  return specifier.startsWith(".") || specifier.startsWith("@/");
}

function addForgeSurfaceNodes(projectRoot, nodes, edges, nodesByPath) {
  const manifestPath = path.join(projectRoot, ".dx", "forge", "source-.dx/build-cache/manifest.json");
  if (!fs.existsSync(manifestPath)) return;

  const manifest = readJson(manifestPath);
  for (const sourcePackage of manifest.packages || []) {
    const packageId = sourcePackage.package_id || sourcePackage.packageId || "unknown";
    const surface = sourcePackage.surface || sourcePackage.variant || "default";
    const node = {
      id: `forge:${packageId}#${surface}`,
      kind: "forge-surface",
      path: ".dx/forge/source-.dx/build-cache/manifest.json",
      contract: CONTRACT_NAMES.forgeSourceGraph,
      packageId,
      surface,
    };
    nodes.push(node);

    for (const file of sourcePackage.files || sourcePackage.exported_files || []) {
      const target = nodesByPath.get(toPosixPath(file));
      if (target) {
        edges.push({ from: node.id, to: target.id, kind: "owns-source" });
      }
    }
    for (const receiptPath of sourcePackage.receipt_paths || []) {
      const target = nodesByPath.get(toPosixPath(receiptPath));
      if (target) {
        edges.push({ from: node.id, to: target.id, kind: "expects-receipt" });
      }
    }
  }
}

function addReceiptSourceEdges(projectRoot, edges, nodesByPath) {
  for (const node of nodesByPath.values()) {
    if (node.kind !== "dx-check-receipt") continue;
    const receipt = readJson(path.join(projectRoot, node.path));
    for (const checkedFile of receipt.checked_files || receipt.sources || []) {
      const target = nodesByPath.get(toPosixPath(checkedFile));
      if (target) {
        edges.push({ from: node.id, to: target.id, kind: "checks" });
      }
    }
  }
}

function addDeploySourceEdges(projectRoot, edges, nodesByPath) {
  for (const node of nodesByPath.values()) {
    if (node.kind !== "deploy-output" || !node.path.endsWith(".json")) continue;
    const receipt = readJson(path.join(projectRoot, node.path));
    for (const source of receipt.sources || []) {
      const target = nodesByPath.get(toPosixPath(source));
      if (target) {
        edges.push({ from: node.id, to: target.id, kind: "emitted-from" });
      }
    }
  }
}

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

module.exports = {
  classifyPath,
  scanProject,
};
