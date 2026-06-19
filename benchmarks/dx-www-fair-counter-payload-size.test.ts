import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

const root = join(import.meta.dirname, "..");

function read(relativePath: string): string {
  return readFileSync(join(root, relativePath), "utf8");
}

function byteLength(source: string): number {
  return Buffer.byteLength(source, "utf8");
}

test("WWW fair-counter source payload stays smaller than the Svelte first-route shell", () => {
  const www = read("demo/index.html");
  const svelte = read("benchmarks/fair-counter/svelte/dist/index.html");

  assert.ok(
    byteLength(www) < byteLength(svelte),
    `${byteLength(www)} should be less than ${byteLength(svelte)}`,
  );
  assert.match(www, /<output id=c aria-live=polite>0<\/output>/);
  assert.match(www, /<button data-x=1>\+<\/button>/);
  assert.match(www, /<button data-x=-1>-<\/button>/);
  assert.match(www, /<button data-x=r>0<\/button>/);
  assert.match(www, /addEventListener\("click"/);
  assert.doesNotMatch(www, /stylesheet|\/assets\/|type=module|node_modules|React|Svelte|Astro/);
});
