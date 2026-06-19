const fs = require("node:fs");

const CSS_URL_PATTERN =
  /\burl\s*\(\s*(?:"([^"]+)"|'([^']+)'|([^'")]+))\s*\)/gi;

function readCssAssetReferences(filePath, fromNode) {
  const source = fs.readFileSync(filePath, "utf8");
  const references = [];
  let match;
  while ((match = CSS_URL_PATTERN.exec(source)) !== null) {
    const specifier = normalizeCssUrlSpecifier(
      match[1] || match[2] || match[3] || "",
    );
    if (isLocalAssetSpecifier(specifier)) {
      references.push({ from: fromNode, specifier });
    }
  }
  return references;
}

function normalizeCssUrlSpecifier(specifier) {
  return specifier.trim();
}

function isLocalAssetSpecifier(specifier) {
  if (!specifier || specifier.startsWith("#")) return false;
  if (/^(?:data|http|https|blob|mailto):/i.test(specifier)) return false;
  return specifier.startsWith(".") || specifier.startsWith("/");
}

module.exports = {
  isLocalAssetSpecifier,
  readCssAssetReferences,
};
