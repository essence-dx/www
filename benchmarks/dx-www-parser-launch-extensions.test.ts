import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("component parser uses launch extensions for pages and components", () => {
  const parser = read("dx-www/src/parser/mod.rs");

  assert.match(parser, /ComponentType::from_path\(Path::new\("about\.html"\)\),\s*ComponentType::Page/s);
  assert.match(parser, /ComponentType::from_path\(Path::new\("Button\.tsx"\)\),\s*ComponentType::Component/s);
  assert.match(parser, /ComponentType::from_path\(Path::new\("_layout\.html"\)\),\s*ComponentType::Layout/s);
  assert.match(parser, /extension == "html"/);
  assert.doesNotMatch(parser, /extension == "pg"/);
  assert.doesNotMatch(parser, /extension == "cp"/);
});

test("production asset classifier does not preserve legacy template extensions", () => {
  const production = read("dx-www/src/production/mod.rs");

  assert.match(production, /"html"\s*\|\s*"htm"\s*=>\s*AssetType::Html/);
  assert.doesNotMatch(production, /"pg"\s*\|\s*"cp"/);
  assert.doesNotMatch(production, /AssetType::BinaryTemplate/);
  assert.doesNotMatch(production, /\bBinaryTemplate\b/);
});

test("public extension constants match the launch source contract", () => {
  const lib = read("dx-www/src/lib.rs");
  const project = read("dx-www/src/project.rs");

  assert.match(lib, /pub const PAGE_EXTENSION:\s*&str\s*=\s*"html";/);
  assert.match(lib, /pub const COMPONENT_EXTENSION:\s*&str\s*=\s*"tsx";/);
  assert.doesNotMatch(lib, /pub const PAGE_EXTENSION:\s*&str\s*=\s*"pg";/);
  assert.doesNotMatch(lib, /pub const COMPONENT_EXTENSION:\s*&str\s*=\s*"cp";/);
  assert.doesNotMatch(project, /Some\(PAGE_EXTENSION\)\s*\|\s*Some\("html"\)/);
});

test("data loader discovery and public contract use launch source extensions", () => {
  const data = read("dx-www/src/data/mod.rs");
  const contract = read("docs/dx-www-developer-contract.md");

  assert.match(data, /ext == "html"/);
  assert.match(data, /project_root\.join\("app"\)/);
  assert.match(data, /scan_app_directory/);
  assert.match(data, /is_app_page_source/);
  assert.match(data, /file_to_app_route_path/);
  assert.match(data, /has_loader\("\/dashboard\/\[team\]"\)/);
  assert.doesNotMatch(data, /ext == "pg"/);

  assert.match(contract, /app\/\*\*\/\*\.\{tsx,jsx,ts,js,html,lyt\}/);
  assert.match(contract, /components\/\*\*\/\*\.\{tsx,jsx,ts,js\}/);
  assert.doesNotMatch(contract, /app\/\*\*\/\*\.\{[^}]*pg/);
  assert.doesNotMatch(contract, /components\/\*\*\/\*\.\{[^}]*cp/);
});
