import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function read(relativePath: string) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function listFiles(relativeRoot: string): string[] {
  const absoluteRoot = path.join(repoRoot, relativeRoot);
  const files: string[] = [];
  const visit = (dir: string) => {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const absolutePath = path.join(dir, entry.name);
      if (entry.isDirectory()) {
        visit(absolutePath);
      } else {
        files.push(path.relative(absoluteRoot, absolutePath).replaceAll("\\", "/"));
      }
    }
  };
  visit(absoluteRoot);
  return files.sort();
}

function devConfigLine(templateName: string) {
  const config = read(`examples/${templateName}/dx`);
  const match = config.match(/dev\(([^)]*)\)/s);
  assert.ok(match, `missing dev(...) in ${templateName}`);
  return match[1];
}

test("template-axum resolves to Axum under automatic server mode", () => {
  const dev = devConfigLine("template-axum");
  const files = listFiles("examples/template-axum");

  assert.match(dev, /server_mode=auto/);
  assert.match(dev, /hot_reload=true/);
  assert.match(dev, /devtools=true/);
  assert.ok(files.includes("app/api/health/route.ts"));
  assert.match(read("examples/template-axum/template-route-contract.ts"), /expectedServerMode: "axum"/);
});

test("template-may-minihttp resolves to tiny mode under automatic server mode", () => {
  const dev = devConfigLine("template-may-minihttp");
  const files = listFiles("examples/template-may-minihttp");

  assert.match(dev, /server_mode=auto/);
  assert.match(dev, /hot_reload=false/);
  assert.match(dev, /devtools=false/);
  assert.ok(!files.some((file) => file.startsWith("app/api/")));
  assert.ok(!files.some((file) => file.startsWith("server/")));
  assert.match(
    read("examples/template-may-minihttp/template-route-contract.ts"),
    /expectedServerMode: "may-minihttp"/,
  );
});

test("server mode templates avoid JavaScript-side script files", () => {
  const forbidden = [".js", ".cjs", ".mjs"];
  const files = [
    ...listFiles("examples/template-axum").map((file) => `template-axum/${file}`),
    ...listFiles("examples/template-may-minihttp").map((file) => `template-may-minihttp/${file}`),
  ];

  assert.deepEqual(
    files.filter((file) => forbidden.some((extension) => file.endsWith(extension))),
    [],
  );
});
