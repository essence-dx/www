import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");

function readWorkspaceFile(path) {
  return readFileSync(resolve(repoRoot, path), "utf8");
}

test("route-handler runtime prefers explicit HEAD exports before GET fallback", () => {
  const serverContract = readWorkspaceFile("core/src/delivery/server_contract.rs");
  const routeTests = readWorkspaceFile("core/src/delivery/tests.rs");

  assert.match(
    serverContract,
    /let export_method = route_handler_export_method\(&source\.source,\s*&request\.method\);/,
    "execute_react_route_handler must choose method with source visibility",
  );
  assert.match(
    serverContract,
    /fn route_handler_export_method(?:<[^>]+>)?\(source: &str,\s*request_method: &(?:'a\s+)?str\) -> &(?:'a\s+)?str/,
    "route_handler_export_method must inspect route source before mapping HEAD",
  );
  assert.match(
    serverContract,
    /request_method == "HEAD" && route_handler_method_is_exported\(source,\s*"HEAD"\)/,
    "explicit HEAD export must win over the GET fallback",
  );
  assert.match(
    serverContract,
    /request_method == "HEAD"[\s\S]*"GET"/,
    "GET fallback for HEAD must remain when no HEAD export exists",
  );
  assert.match(
    routeTests,
    /react_route_handler_runtime_prefers_explicit_head_export_before_get_fallback/,
    "Rust behavior test must lock the explicit HEAD route-handler path",
  );
  assert.match(routeTests, /method: "HEAD"\.to_string\(\)/);
  assert.match(routeTests, /headOnly/);
  assert.match(routeTests, /assert_eq!\(response\.body\["method"\], "HEAD"\)/);
});
