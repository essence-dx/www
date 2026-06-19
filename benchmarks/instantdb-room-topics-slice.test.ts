const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice exposes real room topics and typing APIs from upstream", () => {
  const upstreamRoom = read(
    path.join(mirror, "client", "packages", "react-common", "src", "InstantReactRoom.ts"),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const launchProof = read(
    path.join(root, "examples", "template", "instantdb-status.tsx"),
  );

  assert.match(upstreamRoom, /useTopicEffect/);
  assert.match(upstreamRoom, /usePublishTopic/);
  assert.match(upstreamRoom, /useTypingIndicator/);
  assert.match(upstreamRoom, /useSyncPresence/);

  assert.match(slice, /topics: \{/);
  assert.match(slice, /launchPing: i\.entity/);
  assert.match(slice, /db\.rooms\.useSyncPresence/);
  assert.match(slice, /db\.rooms\.useTopicEffect/);
  assert.match(slice, /db\.rooms\.usePublishTopic/);
  assert.match(slice, /db\.rooms\.useTypingIndicator/);
  assert.match(slice, /presenceSync: "db\.rooms\.useSyncPresence\(launchRoom, presence\)"/);
  assert.match(slice, /presenceName/);
  assert.match(slice, /publishLaunchPing/);
  assert.match(slice, /typingIndicator: "db\.rooms\.useTypingIndicator\(launchRoom, \\"launch-input\\"\)"/);
  assert.match(slice, /topic payload policy for production events/);

  assert.match(launchProof, /db\.rooms\.useSyncPresence/);
  assert.match(launchProof, /db\.rooms\.useTypingIndicator/);
  assert.match(launchProof, /data-dx-instant-typing/);
});
