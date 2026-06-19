const fs = require("node:fs");
const path = require("node:path");
const { readText } = require("./paths.js");

function collectCargoGroups(vendorRoot) {
  const roots = ["crates", "turbopack/crates"];
  const groups = [];

  for (const root of roots) {
    const absoluteRoot = path.join(vendorRoot, ...root.split("/"));
    if (!fs.existsSync(absoluteRoot)) {
      continue;
    }

    const names = fs
      .readdirSync(absoluteRoot, { withFileTypes: true })
      .filter((entry) => entry.isDirectory())
      .map((entry) => entry.name)
      .sort((left, right) => left.localeCompare(right));

    for (const name of names) {
      const group = `${root}/${name}`;
      if (fs.existsSync(path.join(vendorRoot, ...group.split("/"), "Cargo.toml"))) {
        groups.push(group);
      }
    }
  }

  return groups;
}

function parseReadmeImportedGroups(readme) {
  const groups = [];
  let inImportedGroupSection = false;
  let sawImportedGroup = false;

  for (const line of readme.split(/\r?\n/)) {
    if (line.trim() === "Imported Rust groups:") {
      inImportedGroupSection = true;
      continue;
    }

    if (inImportedGroupSection && line.trim() === "" && sawImportedGroup) {
      break;
    }

    if (inImportedGroupSection) {
      const match = line.match(/^-\s+`([^`]+)`\s*$/);
      if (match) {
        groups.push(match[1]);
        sawImportedGroup = true;
      }
    }
  }

  return groups;
}

function parseRustUpstreamGroups(rustSource) {
  return [...rustSource.matchAll(/upstream:\s*"([^"]+)"/g)].map((match) => match[1]);
}

function findMissingNeedles(text, needles) {
  return needles.filter((needle) => !text.includes(needle));
}

function cargoGroupHasMetadata(repoRoot, group) {
  return fs.existsSync(path.join(repoRoot, "vendor", "next-rust", ...group.split("/"), "Cargo.toml"));
}

function readWorkspaceCargoText(repoRoot) {
  const files = ["Cargo.toml", "dx-www/Cargo.toml"];
  return files
    .map((file) => path.join(repoRoot, ...file.split("/")))
    .filter((file) => fs.existsSync(file))
    .map(readText)
    .join("\n");
}

module.exports = {
  cargoGroupHasMetadata,
  collectCargoGroups,
  findMissingNeedles,
  parseReadmeImportedGroups,
  parseRustUpstreamGroups,
  readWorkspaceCargoText,
};
