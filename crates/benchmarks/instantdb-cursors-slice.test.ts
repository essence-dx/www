const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes the real Cursors component surface", () => {
  const upstreamCursors = read(
    path.join(mirror, "client", "packages", "react", "src", "Cursors.tsx"),
  );
  const upstreamExample = read(
    path.join(mirror, "client", "sandbox", "react-nextjs", "pages", "play", "cursors.tsx"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));
  const launchProof = read(path.join(root, "examples", "template", "instantdb-status.tsx"));

  assert.match(upstreamCursors, /export function Cursors/);
  assert.match(upstreamCursors, /room\.usePresence/);
  assert.match(upstreamCursors, /publishPresence/);
  assert.match(upstreamExample, /import \{ init, Cursors \} from '@instantdb\/react'/);
  assert.match(upstreamExample, /<Cursors room=\{room\}>/);

  assert.match(slice, /"js\/components\/instant\/instant-cursors\.tsx"/);
  assert.match(slice, /"dx-launch-cursors": i\.json\(\)\.optional\(\)/);
  assert.match(slice, /import \{ Cursors \} from "@instantdb\/react"/);
  assert.match(slice, /export function InstantLaunchCursors/);
  assert.match(slice, /DX_INSTANT_CURSOR_COLOR_TOKEN/);
  assert.match(slice, /var\(--dx-ui-primary-bg\)/);
  assert.doesNotMatch(slice, /#2563eb/);
  assert.match(slice, /spaceId = "dx-launch-cursors"/);
  assert.match(slice, /cursorsComponent: "InstantLaunchCursors"/);
  assert.match(slice, /real `Cursors` component/);

  assert.match(registry, /components\/instant\/instant-cursors\.tsx/);
  assert.match(registry, /InstantLaunchCursors/);
  assert.match(registry, /spaceId = "dx-launch-cursors"/);

  const cursorComponent = read(
    path.join(root, "examples", "template", "components", "instant", "instant-cursors.tsx"),
  );

  assert.match(cursorComponent, /DX_INSTANT_CURSOR_COLOR_TOKEN/);
  assert.match(cursorComponent, /var\(--dx-ui-primary-bg\)/);
  assert.match(cursorComponent, /color = DX_INSTANT_CURSOR_COLOR_TOKEN/);
  assert.doesNotMatch(cursorComponent, /#2563eb/);

  assert.match(launchProof, /InstantLaunchCursors/);
  assert.match(launchProof, /data-dx-instant-cursors/);
  assert.match(launchProof, /cursor wrapper wired/);
});
