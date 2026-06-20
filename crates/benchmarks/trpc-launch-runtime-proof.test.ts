const assert = require("node:assert");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const root = path.resolve(__dirname, "..");
const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("api/trpc is visibly demoable in the live launch runtime source", () => {
  const launchPage = read("tools/launch/runtime-template/pages/index.html");
  const backendPage = read("tools/launch/runtime-template/pages/backend.html");
  const runtimeJs = read("tools/launch/runtime-template/assets/launch-runtime.ts");
  const appRoute = read("examples/template/app/page.tsx");
  const cli = read("dx-www/src/cli/mod.rs");

  assert.match(launchPage, /data-dx-package="api\/trpc"/);
  assert.match(launchPage, /data-dx-component="launch-trpc-api-dashboard-workflow"/);
  assert.match(launchPage, /data-dx-dashboard-card="typed-api"/);
  assert.match(launchPage, /data-dx-dashboard-workflow="typed-api-readiness"/);
  assert.match(launchPage, /data-dx-trpc-workflow="launch-api-readiness"/);
  assert.match(launchPage, /data-dx-style-surface="theme-token"/);
  assert.match(launchPage, /<dx-icon name="api:trpc" aria-hidden="true"><\/dx-icon>/);
  assert.match(launchPage, /data-trpc-endpoint="\/api\/trpc\/health"/);
  assert.match(launchPage, /data-trpc-procedure="health\.query,launchEvent\.mutation"/);
  assert.match(launchPage, /data-trpc-interaction="health-query"/);
  assert.match(launchPage, /data-trpc-interaction="local-launch-event-mutation"/);
  assert.match(launchPage, /data-dx-trpc-action="check-health"/);
  assert.match(launchPage, /data-dx-trpc-action="prepare-launch-event"/);
  assert.match(launchPage, /id="mission-trpc-status"/);
  assert.doesNotMatch(launchPage, /id="trpc-runtime-proof"/);
  assert.doesNotMatch(launchPage, /data-dx-component="trpc-launch-health-proof"/);
  assert.doesNotMatch(launchPage, /id="trpc-action"/);
  assert.doesNotMatch(launchPage, /id="trpc-status"/);
  assert.doesNotMatch(launchPage, /data-dx-action="trpc-health"/);
  assert.match(launchPage, /data-dx-backend-action="open-typed-api-dashboard"/);
  assert.match(launchPage, /href="#mission-trpc"/);
  assert.match(launchPage, /data-trpc-mutation-state="idle"/);
  assert.match(launchPage, /data-dx-node-modules="forbidden"/);

  assert.match(runtimeJs, /#mission-trpc-health-check/);
  assert.match(runtimeJs, /#mission-trpc-launch-event/);
  assert.match(runtimeJs, /#mission-trpc-status/);
  assert.doesNotMatch(runtimeJs, /#trpc-runtime-proof/);
  assert.doesNotMatch(runtimeJs, /#trpc-proof-status/);
  assert.doesNotMatch(runtimeJs, /#trpc-health-check/);
  assert.doesNotMatch(runtimeJs, /#trpc-launch-event/);
  assert.doesNotMatch(runtimeJs, /#trpc-action/);
  assert.doesNotMatch(runtimeJs, /#trpc-status/);
  assert.match(runtimeJs, /setTrpcWorkflow/);
  assert.match(runtimeJs, /fetch\("\/api\/trpc\/health"/);
  assert.match(runtimeJs, /Local launchEvent accepted; route execution still app-owned/);

  assert.doesNotMatch(appRoute, /import \{ TrpcLaunchHealth \}/);
  assert.doesNotMatch(appRoute, /data-dx-component="launch-trpc-route-proof"/);
  assert.doesNotMatch(appRoute, /data-trpc-route-visible="true"/);
  assert.match(appRoute, /<TemplateShell \/>/);

  assert.doesNotMatch(cli, /import \{ TrpcLaunchHealth \} from "\.\.\/\.\.\/components\/template-app\/trpc-launch-health";/);
  assert.doesNotMatch(cli, /data-dx-component="launch-trpc-route-proof"/);
  assert.doesNotMatch(cli, /data-trpc-route-visible="true"/);

  assert.match(backendPage, /data-dx-component="trpc-backend-workflow"/);
  assert.match(backendPage, /data-dx-dashboard-workflow="typed-api-readiness"/);
  assert.match(backendPage, /href="\/#mission-trpc"/);
  assert.match(backendPage, /data-dx-trpc-action="open-launch-workflow"/);
  assert.doesNotMatch(backendPage, /data-dx-component="trpc-backend-proof"/);
  assert.doesNotMatch(backendPage, /id="trpc-action"/);
  assert.doesNotMatch(backendPage, /id="trpc-status"/);
});

test("api/trpc materializes into generated launch output without node_modules", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-trpc-launch-"));
  fs.mkdirSync(path.join(dir, "app"), { recursive: true });
  fs.writeFileSync(
    path.join(dir, "app", "page.tsx"),
    "export default function Page(){ return <>{children}</>; }\n",
  );

  const output = execFileSync(process.execPath, [materializer, dir], {
    cwd: root,
    encoding: "utf8",
  });
  const result = JSON.parse(output);
  const launchPage = readGenerated(dir, "pages/index.html");
  const backendPage = readGenerated(dir, "pages/backend.html");
  const runtimeJs = readGenerated(dir, "public/launch-runtime.js");
  const trpcRoute = readGenerated(dir, "app/api/trpc/health/route.ts");
  const previewManifest = JSON.parse(readGenerated(dir, "public/preview-.dx/build-cache/manifest.json"));
  const receipt = readGenerated(
    dir,
    ".dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json",
  );

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);
  assert.ok(!fs.existsSync(path.join(dir, "node_modules")));
  assert.ok(
    result.files.includes(".dx/forge/receipts/2026-05-22-api-trpc-dashboard-workflow.json"),
  );
  assert.match(launchPage, /data-dx-route="\/"/);
  assert.match(launchPage, /data-dx-package="api\/trpc"/);
  assert.match(launchPage, /data-dx-component="launch-trpc-api-dashboard-workflow"/);
  assert.match(launchPage, /data-dx-dashboard-card="typed-api"/);
  assert.match(launchPage, /data-dx-dashboard-workflow="typed-api-readiness"/);
  assert.match(launchPage, /data-dx-trpc-workflow="launch-api-readiness"/);
  assert.match(launchPage, /data-dx-style-surface="theme-token"/);
  assert.match(
    launchPage,
    /<dx-icon name="api:trpc" aria-hidden="true"[^>]*data-dx-icon="api:trpc"[^>]*><\/dx-icon>/,
  );
  assert.match(launchPage, /data-trpc-interaction="health-query"/);
  assert.match(launchPage, /data-trpc-interaction="local-launch-event-mutation"/);
  assert.match(launchPage, /data-dx-trpc-action="check-health"/);
  assert.match(launchPage, /data-dx-trpc-action="prepare-launch-event"/);
  assert.match(launchPage, /id="mission-trpc-status"/);
  assert.doesNotMatch(launchPage, /id="trpc-runtime-proof"/);
  assert.doesNotMatch(launchPage, /data-dx-component="trpc-launch-health-proof"/);
  assert.doesNotMatch(launchPage, /id="trpc-action"/);
  assert.doesNotMatch(launchPage, /id="trpc-status"/);
  assert.match(launchPage, /data-dx-backend-action="open-typed-api-dashboard"/);
  assert.match(backendPage, /data-dx-component="trpc-backend-workflow"/);
  assert.match(backendPage, /href="\/#mission-trpc"/);
  assert.match(backendPage, /data-dx-trpc-action="open-launch-workflow"/);
  assert.doesNotMatch(backendPage, /trpc-backend-proof/);
  assert.doesNotMatch(backendPage, /id="trpc-action"/);
  assert.match(runtimeJs, /#mission-trpc-health-check/);
  assert.match(runtimeJs, /#mission-trpc-launch-event/);
  assert.doesNotMatch(runtimeJs, /#trpc-health-check/);
  assert.doesNotMatch(runtimeJs, /#trpc-launch-event/);
  assert.doesNotMatch(runtimeJs, /#trpc-action/);
  assert.doesNotMatch(runtimeJs, /#trpc-status/);
  assert.match(runtimeJs, /setTrpcWorkflow/);
  assert.match(trpcRoute, /export function GET\(\)/);
  assert.match(trpcRoute, /export async function POST\(request: Request\)/);
  assert.match(trpcRoute, /procedure: "launchEvent"/);
  assert.match(receipt, /"package_id": "api\/trpc"/);
  assert.match(receipt, /"component": "launch-trpc-api-dashboard-workflow"/);

  const launchRoute = previewManifest.routes.find(
    (route) => route.route === "/" && route.forgePackages.includes("api/trpc"),
  );
  assert.ok(launchRoute, "expected generated preview manifest to include /");
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-trpc-workflow"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-dx-trpc-action"));
  assert.ok(launchRoute.dataDxMarkers.includes("data-trpc-interaction"));
  const trpcSurface = previewManifest.editContract.editableSurfaces.find(
    (surface) => surface.id === "launch-runtime-trpc-api-dashboard",
  );
  assert.ok(trpcSurface, "expected generated preview manifest to expose the tRPC dashboard surface");
  assert.equal(trpcSurface.selector, '[data-dx-component="launch-trpc-api-dashboard-workflow"]');
  assert.deepEqual(trpcSurface.packageIds, ["api/trpc"]);
  assert.equal(trpcSurface.sourceFile, "pages/index.html");
  assert.equal(trpcSurface.noNodeModulesRequired, true);
});

test("api/trpc Studio manifest exposes the real dashboard workflow to Zed", () => {
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");

  assert.match(studioManifest, /"data-dx-trpc-workflow"/);
  assert.match(studioManifest, /"data-dx-trpc-action"/);
  assert.match(studioManifest, /"data-trpc-interaction"/);
  assert.match(studioManifest, /"data-trpc-mutation-state"/);
  assert.match(studioManifest, /"launch-trpc-api-dashboard-workflow"/);
  assert.match(studioManifest, /"\[data-dx-component=\\\"launch-trpc-api-dashboard-workflow\\\"\]"/);
  assert.match(studioManifest, /"examples\/template\/template-shell\.tsx"/);
  assert.match(studioManifest, /"components\/template-app\/template-shell\.tsx"/);
  assert.match(studioManifest, /"typed-api-readiness"/);
  assert.match(
    studioManifest,
    /"examples\/template\/\.dx\/forge\/receipts\/2026-05-22-api-trpc-dashboard-workflow\.json"/,
  );

  assert.match(editContract, /id: "launch-trpc-api-dashboard-workflow"/);
  assert.match(editContract, /\[data-dx-trpc-action="check-health"\]/);
  assert.match(editContract, /\[data-trpc-interaction="local-launch-event-mutation"\]/);
  assert.match(editContract, /"data-dx-trpc-workflow"/);
  assert.match(
    editContract,
    /receiptPath: "examples\/template\/\.dx\/forge\/receipts\/2026-05-22-api-trpc-dashboard-workflow\.json"/,
  );
});

test("api/trpc launch runtime buttons update visible workflow state", async () => {
  const runtimeJs = read("tools/launch/runtime-template/assets/launch-runtime.ts");
  const missionTrpc = new FakeElement("mission-trpc");
  const missionTrpcStatus = new FakeElement("mission-trpc-status");
  const missionHealthButton = new FakeElement("mission-trpc-health-check");
  const missionLaunchButton = new FakeElement("mission-trpc-launch-event");
  const documentElement = new FakeElement("documentElement");
  const calls = [];
  const selectors = new Map([
    ["#mission-trpc", missionTrpc],
    ["#mission-trpc-status", missionTrpcStatus],
    ["#mission-trpc-health-check", missionHealthButton],
    ["#mission-trpc-launch-event", missionLaunchButton],
  ]);

  vm.runInNewContext(runtimeJs, {
    HTMLCanvasElement: class HTMLCanvasElement {},
    WebAssembly,
    console,
    devicePixelRatio: 1,
    document: {
      documentElement,
      addEventListener(event, handler) {
        if (event === "DOMContentLoaded") handler();
      },
      createElement(tagName) {
        return new FakeElement(tagName);
      },
      querySelector(selector) {
        return selectors.get(selector) || null;
      },
      querySelectorAll() {
        return [];
      },
    },
    fetch: async (url, options = {}) => {
      calls.push({ options, url });
      const isPost = options.method === "POST";
      return {
        ok: true,
        status: isPost ? 202 : 200,
        json: async () =>
          isPost
            ? { procedure: "launchEvent", status: "accepted" }
            : { procedure: "health", status: "ready" },
      };
    },
    localStorage: storage(),
    requestAnimationFrame() {},
    sessionStorage: storage(),
  });

  await missionHealthButton.click();
  assert.equal(calls[0].url, "/api/trpc/health");
  assert.equal(calls[0].options.method, undefined);
  assert.equal(missionTrpc.dataset.trpcMutationState, "accepted");
  assert.equal(missionTrpcStatus.dataset.trpcRequestId, "dx-trpc-health-1");
  assert.match(missionTrpcStatus.textContent, /Health health: ready/);

  await missionLaunchButton.click();
  assert.equal(calls[1].url, "/api/trpc/health");
  assert.equal(calls[1].options.method, "POST");
  assert.match(calls[1].options.body, /"event":"validated"/);
  assert.equal(missionTrpc.dataset.trpcMutationState, "accepted");
  assert.equal(missionTrpcStatus.dataset.trpcMutationState, "accepted");
  assert.equal(missionTrpcStatus.dataset.trpcRequestId, "dx-trpc-local-2");
  assert.match(missionTrpcStatus.textContent, /launchEvent accepted for \//);
});

function readGenerated(projectDir, relativePath) {
  return fs.readFileSync(path.join(projectDir, relativePath), "utf8");
}

function storage() {
  const values = new Map();
  return {
    getItem(key) {
      return values.has(key) ? values.get(key) : null;
    },
    removeItem(key) {
      values.delete(key);
    },
    setItem(key, value) {
      values.set(key, String(value));
    },
  };
}

class FakeElement {
  constructor(id) {
    this.id = id;
    this.attributes = new Map();
    this.children = [];
    this.classList = { toggle() {} };
    this.dataset = {};
    this.listeners = new Map();
    this.style = {};
    this.textContent = "";
    this.value = "";
  }

  addEventListener(event, handler) {
    this.listeners.set(event, handler);
  }

  append(...nodes) {
    this.children.push(...nodes);
  }

  appendChild(node) {
    this.children.push(node);
    return node;
  }

  click() {
    const handler = this.listeners.get("click");
    return handler ? handler({ preventDefault() {} }) : undefined;
  }

  getAttribute(name) {
    return this.attributes.get(name) || null;
  }

  querySelector() {
    return null;
  }

  replaceChildren(...nodes) {
    this.children = nodes;
  }

  setAttribute(name, value) {
    this.attributes.set(name, String(value));
  }
}
