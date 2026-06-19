import assert from "node:assert/strict";
import {
  basename,
  dirname,
  extname,
  join,
  relative as pathRelative,
  resolve,
  sep,
} from "node:path";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const cliPath = join(repoRoot, "dx-www", "src", "cli", "mod.rs");
const cliCoreImplPath = join(repoRoot, "dx-www", "src", "cli", "mod_parts", "cli_core_impl.rs");

const starterRoots = [
  join(repoRoot, "examples", "template"),
  join(repoRoot, ".dx", "template-app-browser-preview"),
  join(repoRoot, "examples", "conversion-proof"),
];

const forbiddenRuntimePatterns = [
  { label: "Tailwind CDN", pattern: /cdn\.tailwindcss\.com/i },
  { label: "Tailwind runtime script", pattern: /<script\b[^>]*tailwindcss/i },
  { label: "Tailwind CSS import", pattern: /@import\s+["']tailwindcss["']/i },
  { label: "Tailwind directive", pattern: /@tailwind\b/i },
];

const starterRuntimeExtensions = new Set([".html", ".css", ".json"]);
const configFilePatterns = [
  /^tailwind\.config\.(js|cjs|mjs|ts)$/,
  /^postcss\.config\.(js|cjs|mjs|ts)$/,
];
const dependencyFields = [
  "dependencies",
  "devDependencies",
  "peerDependencies",
  "optionalDependencies",
];
const forbiddenPackageDependencies = new Set([
  "tailwindcss",
  "@tailwindcss/postcss",
  "postcss",
  "autoprefixer",
]);

function read(filePath: string): string {
  assert.ok(existsSync(filePath), `missing ${pathRelative(repoRoot, filePath)}`);
  return readFileSync(filePath, "utf8");
}

function relative(filePath: string): string {
  return pathRelative(repoRoot, filePath).replaceAll(sep, "/");
}

function walkFiles(root: string): string[] {
  if (!existsSync(root)) {
    return [];
  }

  const files: string[] = [];
  const stack = [root];
  while (stack.length > 0) {
    const current = stack.pop();
    assert.ok(current, "walk stack should contain a path");
    for (const entry of readdirSync(current, { withFileTypes: true })) {
      const child = join(current, entry.name);
      if (entry.isDirectory()) {
        if (entry.name !== "node_modules" && entry.name !== ".git") {
          stack.push(child);
        }
      } else {
        files.push(child);
      }
    }
  }
  return files;
}

function assertNoForbiddenRuntime(text: string, sourceLabel: string): void {
  for (const { label, pattern } of forbiddenRuntimePatterns) {
    const match = pattern.exec(text);
    if (match) {
      const line = text.slice(0, match.index).split(/\r?\n/).length;
      assert.fail(`${sourceLabel}:${line} contains ${label}: ${match[0]}`);
    }
  }
}

test("legacy HTML shell source does not inject Tailwind at runtime", () => {
  const cliRuntimeSource = [
    read(cliPath).split("\n#[cfg(test)]\nmod tests;")[0],
    read(cliCoreImplPath),
  ].join("\n");

  assertNoForbiddenRuntime(cliRuntimeSource, "dx-www/src/cli/mod.rs runtime source");
  assert.match(
    cliRuntimeSource,
    /<link rel="stylesheet" href="\/styles\/globals\.css">/,
    "legacy shell should load the DX-owned generated style entry",
  );
});

test("materialized DX starter output has no Tailwind runtime dependency", () => {
  const runtimeFiles = starterRoots.flatMap((root) =>
    walkFiles(root).filter((filePath) => {
      const ext = extname(filePath);
      const baseName = basename(filePath);
      return starterRuntimeExtensions.has(ext) || baseName === "package.json";
    }),
  );

  assert.ok(runtimeFiles.length > 0, "expected starter runtime files to scan");

  for (const filePath of runtimeFiles) {
    const label = relative(filePath);
    assertNoForbiddenRuntime(read(filePath), label);
  }
});

test("DX starter output does not ship Tailwind or PostCSS config files", () => {
  for (const root of starterRoots) {
    for (const filePath of walkFiles(root)) {
      const baseName = basename(filePath);
      assert.equal(
        configFilePatterns.some((pattern) => pattern.test(baseName)),
        false,
        `${relative(filePath)} must not be a template-local CSS tool config`,
      );
    }
  }
});

test("DX starter package manifests do not depend on Tailwind or PostCSS", () => {
  for (const root of starterRoots) {
    for (const filePath of walkFiles(root).filter((file) => basename(file) === "package.json")) {
      const manifest = JSON.parse(read(filePath));
      for (const field of dependencyFields) {
        const dependencies = manifest[field] ?? {};
        for (const packageName of forbiddenPackageDependencies) {
          assert.equal(
            Object.prototype.hasOwnProperty.call(dependencies, packageName),
            false,
            `${relative(filePath)} ${field} must not include ${packageName}`,
          );
        }
      }
    }
  }
});
