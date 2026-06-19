import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");

function read(relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function listFiles(dir: string): string[] {
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  return entries.flatMap((entry) => {
    const fullPath = path.join(dir, entry.name);
    return entry.isDirectory() ? listFiles(fullPath) : [fullPath];
  });
}

test("default WWW template materializes one App Router page", () => {
  const sources = read("dx-www/src/cli/default_template_sources.rs");
  const newCommand = read("dx-www/src/cli/new_command.rs");
  const page = read("examples/template/app/page.tsx");
  const readme = read("examples/template/README.md");
  const templateAppDir = path.join(root, "examples/template/app");
  const appPages = listFiles(templateAppDir)
    .filter((file) => /[/\\]page\.(t|j)sx?$/.test(file))
    .map((file) => path.relative(root, file).replace(/\\/g, "/"));

  assert.match(sources, /route: "\/"/);
  assert.match(sources, /materialized_file: "app\/page\.tsx"/);
  assert.doesNotMatch(sources, /route: "\/state-runtime"/);
  assert.doesNotMatch(sources, /route: "\/islands"/);
  assert.doesNotMatch(sources, /materialized_file: "app\/state-runtime\/page\.tsx"/);
  assert.doesNotMatch(sources, /materialized_file: "app\/islands\/page\.tsx"/);
  assert.doesNotMatch(sources, /materialized_file: "components\/state-runtime-probe\.tsx"/);
  assert.doesNotMatch(sources, /materialized_file: "components\/island-runtime-probe\.tsx"/);

  assert.match(newCommand, /"app_router_files": \["app\/layout\.tsx", "app\/page\.tsx"\]/);
  assert.match(newCommand, /"tooling": \["dx-style", "dx-icons", "dx-imports", "dx-check", "serializer"\]/);
  assert.match(newCommand, /Created one App Router page at app\/page\.tsx/);

  assert.match(page, /data-dx-template="minimal"/);
  assert.match(page, /<img className="starter-logo" src="\/logo\.svg" alt="Dx WWW" \/>/);
  assert.doesNotMatch(page, /href="\/state-runtime"|href="\/islands"|action="\/state-runtime"/);
  assert.doesNotMatch(page, /proof route|proof target/i);
  assert.deepEqual(appPages, ["examples/template/app/page.tsx"]);
  assert.equal(fs.existsSync(path.join(templateAppDir, "islands/page.tsx")), false);
  assert.equal(fs.existsSync(path.join(templateAppDir, "state-runtime/page.tsx")), false);

  assert.match(readme, /one page: `app\/page\.tsx`/);
  assert.match(readme, /Add routes only when the product needs them/);
  assert.match(readme, /first-party `Icon` component/);
  assert.match(readme, /Forge\/no-node_modules policy/);
});
