const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const mirror = path.resolve(root, "..", "..", "WWW", "inspirations", "instantdb");

function read(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

test("InstantDB slice materializes real Next SSR and Suspense helpers", () => {
  const upstreamNextIndex = read(
    path.join(mirror, "client", "packages", "react", "src", "next-ssr", "index.tsx"),
  );
  const upstreamNextDatabase = read(
    path.join(
      mirror,
      "client",
      "packages",
      "react",
      "src",
      "next-ssr",
      "InstantNextDatabase.tsx",
    ),
  );
  const upstreamProvider = read(
    path.join(
      mirror,
      "client",
      "sandbox",
      "react-nextjs",
      "app",
      "play",
      "ssr",
      "InstantProvider.tsx",
    ),
  );
  const upstreamLayout = read(
    path.join(
      mirror,
      "client",
      "sandbox",
      "react-nextjs",
      "app",
      "play",
      "ssr",
      "layout.tsx",
    ),
  );
  const upstreamTodos = read(
    path.join(
      mirror,
      "client",
      "sandbox",
      "react-nextjs",
      "app",
      "play",
      "ssr",
      "TodosWithSuspense.tsx",
    ),
  );
  const slice = read(
    path.join(root, "core", "src", "ecosystem", "forge_instantdb.rs"),
  );
  const registry = read(
    path.join(root, "core", "src", "ecosystem", "forge_registry.rs"),
  );
  const launchProof = read(
    path.join(root, "examples", "template", "instantdb-status.tsx"),
  );

  assert.match(upstreamNextIndex, /export function init</);
  assert.match(upstreamNextIndex, /InstantSuspenseProvider/);
  assert.match(upstreamNextIndex, /getUnverifiedUserFromInstantCookie/);
  assert.match(upstreamNextDatabase, /public useSuspenseQuery/);
  assert.match(upstreamProvider, /<InstantSuspenseProvider user=\{user\} db=\{db\}>/);
  assert.match(upstreamLayout, /getUnverifiedUserFromInstantCookie/);
  assert.match(upstreamTodos, /db\.useSuspenseQuery/);

  assert.match(slice, /"js\/instant\/next-client\.tsx"/);
  assert.match(slice, /"js\/instant\/next-server\.ts"/);
  assert.match(slice, /import \{ init, InstantSuspenseProvider \} from "@instantdb\/react\/nextjs"/);
  assert.match(slice, /createDxInstantNextClient/);
  assert.match(slice, /InstantLaunchSuspenseProvider/);
  assert.match(slice, /nextDb\.useSuspenseQuery\(instantLaunchTodosQuery\)/);
  assert.match(slice, /getUnverifiedUserFromInstantCookie/);
  assert.match(slice, /getInstantLaunchSsrUser/);
  assert.match(slice, /suspenseQuery: "useInstantLaunchTodosSuspense\(\)"/);

  assert.match(registry, /lib\/instant\/next-client\.tsx/);
  assert.match(registry, /lib\/instant\/next-server\.ts/);
  assert.match(registry, /next_client\.contains\("InstantLaunchSuspenseProvider"\)/);
  assert.match(registry, /next_server\.contains\("getInstantLaunchSsrUser"\)/);

  assert.match(launchProof, /InstantLaunchSuspenseProvider/);
  assert.match(launchProof, /useInstantLaunchTodosSuspense/);
  assert.match(launchProof, /getInstantLaunchSsrUser/);
  assert.match(launchProof, /data-dx-instant-ssr/);
  assert.match(launchProof, /next SSR helpers wired/);
});
