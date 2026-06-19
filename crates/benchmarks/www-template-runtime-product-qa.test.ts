import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function materializedPage(dir, pageName) {
  return fs.readFileSync(path.join(dir, "pages", `${pageName}.html`), "utf8");
}

test("launch template auth routes use product-facing copy while preserving auth boundaries", () => {
  const authSource = read("examples/template/components/template-app/auth-pages.tsx");
  const dashboardSource = read("examples/template/components/template-app/dashboard-page.tsx");
  const runtime = read("tools/launch/runtime-template/assets/launch-runtime.ts");
  const loginRuntime = read("tools/launch/runtime-template/pages/login.html");
  const logoutRuntime = read("tools/launch/runtime-template/pages/logout.html");
  const launchRuntime = read("tools/launch/runtime-template/pages/index.html");

  assert.match(authSource, /Sign in to www/);
  assert.match(authSource, /Review sign-in readiness without creating a hosted session\./);
  assert.doesNotMatch(authSource, /<h1[^>]*>Auth boundary\.<\/h1>/);
  assert.match(dashboardSource, /<span>Sign in<\/span>/);
  assert.match(dashboardSource, /<span>Sign-in readiness<\/span>/);
  assert.match(runtime, /Sign-in readiness reviewed\. Opening dashboard\./);
  assert.match(runtime, /reviewed \? "Sign-in reviewed" : "Sign-in readiness"/);

  assert.match(loginRuntime, /<title>Sign in \| DX Template<\/title>/);
  assert.match(loginRuntime, /<h1>Sign in to www<\/h1>/);
  assert.match(loginRuntime, />Review sign-in<\/button>/);
  assert.doesNotMatch(loginRuntime, /<title>Auth boundary \| DX Template<\/title>|<h1>Auth boundary\.<\/h1>/);
  assert.match(logoutRuntime, /<title>Clear sign-in review \| DX Template<\/title>/);
  assert.match(logoutRuntime, /<a href="\/login">Sign in<\/a>/);
  assert.match(launchRuntime, />Sign-in readiness<\/span>/);
  assert.match(launchRuntime, /Sign-in readiness mounted; credentials stay app-owned\./);
  assert.doesNotMatch(
    launchRuntime,
    />Auth boundary<\/span>|Authentication boundary mounted\.|Boundary review idle|local boundary review|No local boundary review marker/,
  );

  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-template-product-qa-"));
  try {
    execFileSync(process.execPath, [materializer, dir], { cwd: root, stdio: "pipe" });
    const login = materializedPage(dir, "login");
    const logout = materializedPage(dir, "logout");
    const dashboard = materializedPage(dir, "dashboard");
    const launch = materializedPage(dir, "index");

    assert.match(login, /<title>Sign in \| DX Template<\/title>/);
    assert.match(login, /<h1>Sign in to www<\/h1>/);
    assert.match(logout, /<a href="\/login">Sign in<\/a>/);
    assert.match(dashboard, /<span>Sign in<\/span>/);
    assert.match(dashboard, /<span data-app-auth-label>Sign-in readiness<\/span>/);
    assert.doesNotMatch(dashboard, /<span>Auth boundary<\/span>|<span data-app-auth-label>Auth boundary<\/span>/);
    assert.match(launch, />Sign-in readiness<\/span>/);
    assert.doesNotMatch(
      launch,
      />Auth boundary<\/span>|Authentication boundary mounted\.|Boundary review idle|local boundary review|No local boundary review marker/,
    );
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
