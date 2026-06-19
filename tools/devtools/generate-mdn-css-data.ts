#!/usr/bin/env node
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import process from "node:process";

const repoRoot = process.cwd();
const mdnRoot = path.resolve(repoRoot, process.argv[2] || "target/mdn-data");
const outPath = path.resolve(
  repoRoot,
  "dx-www/src/cli/devtools/css_data.generated.json",
);

const readJson = (relativePath) =>
  JSON.parse(fs.readFileSync(path.join(mdnRoot, relativePath), "utf8"));

const git = (...args) =>
  execFileSync("git", ["-C", mdnRoot, ...args], {
    encoding: "utf8",
    stdio: ["ignore", "pipe", "ignore"],
  }).trim();

const packageJson = readJson("package.json");
const properties = readJson("css/properties.json");
const selectors = readJson("css/selectors.json");
const atRules = readJson("css/at-rules.json");
const syntaxes = readJson("css/syntaxes.json");
const types = readJson("css/types.json");
const units = readJson("css/units.json");
const functions = readJson("css/functions.json");

const keywordDenylist = new Set([
  "and",
  "as",
  "by",
  "from",
  "in",
  "of",
  "or",
  "to",
  "type",
]);

function compactEntry(name, value) {
  return {
    name,
    syntax: value.syntax || "",
    status: value.status || "unknown",
    groups: value.groups || [],
    mdnUrl: value.mdn_url || null,
  };
}

function propertyEntry(name, value, valueHints) {
  return {
    name,
    syntax: value.syntax || "",
    initial: value.initial ?? null,
    inherited: Boolean(value.inherited),
    status: value.status || "unknown",
    groups: value.groups || [],
    appliesTo: value.appliesto || null,
    animationType: value.animationType || null,
    percentages: value.percentages || null,
    mdnUrl: value.mdn_url || null,
    valueHints,
  };
}

function syntaxSourceForReference(reference) {
  const normalized = reference.replace(/^'/, "").replace(/'$/, "");
  if (syntaxes[normalized]?.syntax) return syntaxes[normalized].syntax;
  if (types[normalized]?.syntax) return types[normalized].syntax;
  return "";
}

function keywordHintsForSyntax(syntax, depth = 0, seen = new Set()) {
  if (!syntax || depth > 4) return [];
  const hints = new Set();
  const references = Array.from(syntax.matchAll(/<([^>]+)>/g)).map((match) => match[1]);
  for (const reference of references) {
    const normalized = reference.replace(/^'/, "").replace(/'$/, "");
    if (seen.has(normalized)) continue;
    seen.add(normalized);
    for (const hint of keywordHintsForSyntax(syntaxSourceForReference(reference), depth + 1, seen)) {
      hints.add(hint);
    }
  }
  const withoutReferences = syntax
    .replace(/<[^>]+>/g, " ")
    .replace(/'[^']+'/g, " ")
    .replace(/\[[^\]]*\]/g, " ")
    .replace(/[|#?*!+{}(),/]/g, " ");
  for (const token of withoutReferences.match(/[a-zA-Z_-][a-zA-Z0-9_-]*/g) || []) {
    const keyword = token.toLowerCase();
    if (keyword.length < 2 || keywordDenylist.has(keyword)) continue;
    hints.add(keyword);
  }
  return Array.from(hints).sort().slice(0, 48);
}

function sortedEntries(record, mapper = compactEntry) {
  return Object.entries(record)
    .sort(([a], [b]) => a.localeCompare(b))
    .map(([name, value]) => mapper(name, value));
}

const propertyEntries = sortedEntries(properties, (name, value) =>
  propertyEntry(name, value, keywordHintsForSyntax(value.syntax || "")),
);
const selectorEntries = sortedEntries(selectors);
const atRuleEntries = sortedEntries(atRules);
const syntaxEntries = sortedEntries(syntaxes);
const typeEntries = sortedEntries(types);
const unitEntries = sortedEntries(units, (name, value) => ({
  name,
  groups: value.groups || [],
  status: value.status || "unknown",
  mdnUrl: value.mdn_url || null,
}));
const functionEntries = sortedEntries(functions);

const catalog = {
  schema: "dx.devtools.css_data",
  format: 1,
  source: {
    repository: "https://github.com/mdn/data",
    package: packageJson.name,
    version: packageJson.version,
    commit: git("rev-parse", "HEAD"),
    generatedAt: new Date().toISOString(),
  },
  counts: {
    properties: propertyEntries.length,
    selectors: selectorEntries.length,
    atRules: atRuleEntries.length,
    syntaxes: syntaxEntries.length,
    types: typeEntries.length,
    units: unitEntries.length,
    functions: functionEntries.length,
    propertyValueSyntaxes: propertyEntries.filter((entry) => entry.syntax).length,
    derivedPropertyValueHints: propertyEntries.reduce(
      (total, entry) => total + entry.valueHints.length,
      0,
    ),
  },
  properties: propertyEntries,
  selectors: selectorEntries,
  atRules: atRuleEntries,
  syntaxes: syntaxEntries,
  types: typeEntries,
  units: unitEntries,
  functions: functionEntries,
};

fs.mkdirSync(path.dirname(outPath), { recursive: true });
fs.writeFileSync(outPath, `${JSON.stringify(catalog)}\n`);
console.log(
  JSON.stringify(
    {
      output: path.relative(repoRoot, outPath),
      source: catalog.source,
      counts: catalog.counts,
    },
    null,
    2,
  ),
);
