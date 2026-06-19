const fs = require("node:fs");
const path = require("node:path");

function readJson(filePath) {
  if (!fs.existsSync(filePath)) {
    return { ok: false, value: null };
  }
  try {
    return { ok: true, value: JSON.parse(fs.readFileSync(filePath, "utf8")) };
  } catch (error) {
    return { ok: false, value: null, error: error.message };
  }
}

function writeJson(filePath, value) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`);
}

function pick(source, fields) {
  const output = {};
  for (const field of fields) {
    output[field] = source[field];
  }
  return output;
}

function relativePath(root, target) {
  return path.relative(root, target).replace(/\\/g, "/");
}

function normalizeArtifactPath(value) {
  return String(value || "")
    .replace(/\\/g, "/")
    .replace(/^\.\//, "");
}

module.exports = {
  normalizeArtifactPath,
  pick,
  readJson,
  relativePath,
  writeJson,
};
