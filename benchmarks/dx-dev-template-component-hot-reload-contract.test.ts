import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

function read(path: string): string {
  return readFileSync(new URL(`../${path}`, import.meta.url), "utf8");
}

test("www-template component edits have route-scoped hot reload coverage", () => {
  const dashboardRoute = read("examples/template/app/dashboard/page.tsx");
  const hotReloadStream = read("dx-www/src/dev/hot_reload_stream.rs");

  assert.match(
    dashboardRoute,
    /@\/components\/template-app\/dashboard-page/,
    "the real launch template dashboard route should import the editable dashboard component",
  );
  assert.match(
    hotReloadStream,
    /fn changed_imported_component_files_become_route_scoped_hot_reload_resources\(/,
    "component edit route scoping should have a focused Rust regression test",
  );
  assert.match(
    hotReloadStream,
    /route_resource_for_imported_component/,
    "component edit resource mapping should resolve the route owner",
  );
  assert.match(
    hotReloadStream,
    /Some\("route:\/dashboard"\.to_string\(\)\)/,
    "the dashboard component fixture should map to the dashboard route",
  );
  assert.doesNotMatch(
    hotReloadStream,
    /_dx\/devtools|Next DevTools clone/i,
    "WWW hot reload should remain separate from devtools work",
  );
});
