const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes ruleParams and lookup helpers from upstream API", () => {
  const upstreamTx = read(path.join(mirror, "client", "packages", "core", "src", "instatx.ts"));
  const upstreamPlayground = read(
    path.join(mirror, "client", "sandbox", "react-nextjs", "pages", "play", "ruleParams.tsx"),
  );
  const upstreamValidation = read(
    path.join(
      mirror,
      "client",
      "packages",
      "core",
      "__tests__",
      "src",
      "transactionValidation.test.ts",
    ),
  );
  const slice = read(path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"));
  const registry = read(path.join(root, "core", "src", "ecosystem", "forge_registry.rs"));
  const launchProof = read(path.join(root, "examples", "template", "instantdb-status.tsx"));

  assert.match(upstreamTx, /ruleParams: \(args: RuleParams\)/);
  assert.match(upstreamTx, /export function lookup\(attribute: string, value: any\)/);
  assert.match(upstreamPlayground, /\.ruleParams\(\{ test: 'foo' \}\)\.update/);
  assert.match(upstreamPlayground, /db\.queryOnce\(\{ playDocs: \{\} \}, \{ ruleParams: \{ secret \} \}\)/);
  assert.match(upstreamValidation, /tx\.users\[lookup\('email', 'john@example.net'\)\]\.update/);

  assert.match(slice, /"js\/instant\/rules\.ts"/);
  assert.match(slice, /import \{ lookup \} from "@instantdb\/react"/);
  assert.match(slice, /queryInstantLaunchTodosWithRuleParams/);
  assert.match(slice, /db\.queryOnce\(instantLaunchTodosQuery, \{ ruleParams \}\)/);
  assert.match(slice, /updateInstantLaunchTodoWithRuleParams/);
  assert.match(slice, /db\.tx\.todos\[todoId\]\.ruleParams\(ruleParams\)\.update/);
  assert.match(slice, /updateInstantLaunchTodoByText/);
  assert.match(slice, /db\.tx\.todos\[lookup\("text", value\)\]\.update/);
  assert.match(slice, /ruleParams: "queryInstantLaunchTodosWithRuleParams\(ruleParams\)"/);
  assert.match(slice, /lookupMutation: "updateInstantLaunchTodoByText\(text, done\)"/);
  assert.match(slice, /unique lookup attribute design/);

  assert.match(registry, /lib\/instant\/rules\.ts/);
  assert.match(registry, /queryInstantLaunchTodosWithRuleParams/);
  assert.match(registry, /lookup\("text", value\)/);

  assert.match(launchProof, /queryInstantLaunchTodosWithRuleParams/);
  assert.match(launchProof, /updateInstantLaunchTodoByText/);
  assert.match(launchProof, /data-dx-instant-rules/);
  assert.match(launchProof, /rule helpers wired/);
});
