const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "fumadocs");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

const forge = read(path.join(root, "core", "src", "ecosystem", "forge_fumadocs.rs"));
const docsStatus = read(path.join(root, "examples", "template", "docs-status.tsx"));
const packageCatalog = read(path.join(root, "examples", "template", "package-catalog.ts"));
const cli = read(path.join(root, "dx-www", "src", "cli", "mod.rs"));
const scorecard = read(path.join(root, "core", "src", "ecosystem", "forge_scorecard.rs"));
const security = read(path.join(root, "core", "src", "ecosystem", "forge_security.rs"));
const trustPolicy = read(path.join(root, "core", "src", "ecosystem", "forge_trust_policy.rs"));

test("upstream fumadocs exposes real toc APIs", () => {
  const packageJson = read(path.join(mirror, "packages", "core", "package.json"));
  const contentToc = read(path.join(mirror, "packages", "core", "src", "content", "toc.ts"));
  const toc = read(path.join(mirror, "packages", "core", "src", "toc.tsx"));
  const docs = read(
    path.join(mirror, "apps", "docs", "content", "docs", "headless", "utils", "get-toc.mdx"),
  );
  const example = read(
    path.join(
      mirror,
      "apps",
      "docs",
      "content",
      "docs",
      "headless",
      "components",
      "toc-example.tsx",
    ),
  );

  assert.match(packageJson, /"\.\/content\/toc"/);
  assert.match(packageJson, /"\.\/toc"/);
  assert.match(contentToc, /export function getTableOfContents/);
  assert.match(contentToc, /remarkHeading/);
  assert.match(toc, /export interface TOCItemType/);
  assert.match(toc, /export type TableOfContents/);
  assert.match(toc, /export function AnchorProvider/);
  assert.match(toc, /export function TOCItem/);
  assert.match(docs, /import \{ getTableOfContents \} from 'fumadocs-core\/content\/toc'/);
  assert.match(example, /type TOCItemType.*fumadocs-core\/toc/);
});

test("fumadocs slice materializes a toc contract and uses it in the docs page", () => {
  assert.match(forge, /js\/lib\/fumadocs\/toc\.ts/);
  assert.match(forge, /import \{ getTableOfContents \} from "fumadocs-core\/content\/toc"/);
  assert.match(forge, /import type \{ TOCItemType \} from "fumadocs-core\/toc"/);
  assert.match(forge, /dxFumadocsTocContract/);
  assert.match(forge, /getDxFumadocsMarkdownToc/);
  assert.match(forge, /getDxFumadocsPageToc/);
  assert.match(forge, /getDxFumadocsPageTocSummary/);
  assert.match(forge, /getTableOfContents\(content\)/);
  assert.match(forge, /toc=\{getDxFumadocsPageToc\(page\)\}/);
});

test("fumadocs toc metadata is discoverable and honest", () => {
  assert.match(forge, /tocFile: "lib\/fumadocs\/toc\.ts"/);
  assert.match(forge, /getTableOfContents from fumadocs-core\/content\/toc/);
  assert.match(forge, /TOCItemType from fumadocs-core\/toc/);
  assert.match(packageCatalog, /toc policy/);
  assert.match(scorecard, /toc summary/);
  assert.match(security, /getTableOfContents/);
  assert.match(trustPolicy, /toc policy/);
  assert.match(cli, /dxFumadocsTocContract/);
});

test("launch template consumes the fumadocs toc contract", () => {
  assert.match(docsStatus, /dxFumadocsTocContract/);
  assert.match(docsStatus, /data-dx-docs-toc/);
  assert.match(docsStatus, /toc summary/);
});
