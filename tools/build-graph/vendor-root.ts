const fs = require("node:fs");
const path = require("node:path");

function findNextRustVendorWorkspaceRoot(startDir = __dirname) {
  let current = path.resolve(startDir);
  while (true) {
    if (fs.existsSync(path.join(current, "vendor", "next-rust"))) {
      return current;
    }

    const parent = path.dirname(current);
    if (parent === current) {
      return path.resolve(__dirname, "..", "..");
    }
    current = parent;
  }
}

module.exports = {
  findNextRustVendorWorkspaceRoot,
};
