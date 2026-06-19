const fs = require("node:fs");
const path = require("node:path");

function readJson(projectRoot, receiptSource) {
  if (!receiptSource) {
    return missingReceipt(null);
  }

  if (!fs.existsSync(receiptSource.absolutePath)) {
    return missingReceipt(receiptSource);
  }

  try {
    return {
      malformed: false,
      path: relativePath(projectRoot, receiptSource.absolutePath),
      present: true,
      source: receiptSource.source,
      value: JSON.parse(fs.readFileSync(receiptSource.absolutePath, "utf8")),
    };
  } catch (error) {
    return {
      error: error.message,
      malformed: true,
      path: relativePath(projectRoot, receiptSource.absolutePath),
      present: true,
      source: receiptSource.source,
      value: null,
    };
  }
}

function missingReceipt(receiptSource) {
  return {
    malformed: false,
    path: receiptSource ? receiptSource.relativePath : null,
    present: false,
    source: receiptSource ? receiptSource.source : null,
    value: null,
  };
}

function writeJson(filePath, value) {
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, `${JSON.stringify(value, null, 2)}\n`);
}

function relativePath(root, target) {
  return normalizePath(path.relative(root, target));
}

function normalizePath(value) {
  return value.replace(/\\/g, "/");
}

function lineCount(filePath) {
  return fs.readFileSync(filePath, "utf8").split(/\r?\n/).length;
}

module.exports = {
  lineCount,
  normalizePath,
  readJson,
  relativePath,
  writeJson,
};
