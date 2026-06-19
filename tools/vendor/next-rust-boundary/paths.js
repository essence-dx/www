const crypto = require("node:crypto");
const fs = require("node:fs");
const path = require("node:path");
const {
  CANONICAL_RECEIPT_PATH,
  CONSUMER_RECEIPT_PATH,
} = require("./constants.js");

function slashPath(value) {
  return value.split(path.sep).join("/");
}

function readText(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

function unique(values) {
  return [...new Set(values)];
}

function relativeFromRepo(repoRoot, filePath) {
  return slashPath(path.relative(repoRoot, filePath));
}

function sha256(text) {
  return crypto.createHash("sha256").update(text).digest("hex");
}

function defaultReceiptPath(repoRoot) {
  return path.join(repoRoot, ...CANONICAL_RECEIPT_PATH.split("/"));
}

function defaultConsumerReceiptPath(repoRoot) {
  return path.join(repoRoot, ...CONSUMER_RECEIPT_PATH.split("/"));
}

function pathInside(parentPath, childPath) {
  const relative = path.relative(parentPath, childPath);
  return relative === "" || (!relative.startsWith("..") && !path.isAbsolute(relative));
}

function receiptDisplayPath(repoRoot, receiptPath) {
  const absoluteReceiptPath = path.resolve(receiptPath);
  if (pathInside(repoRoot, absoluteReceiptPath)) {
    return relativeFromRepo(repoRoot, absoluteReceiptPath);
  }
  return slashPath(absoluteReceiptPath);
}

module.exports = {
  defaultConsumerReceiptPath,
  defaultReceiptPath,
  readText,
  receiptDisplayPath,
  relativeFromRepo,
  sha256,
  slashPath,
  unique,
};
