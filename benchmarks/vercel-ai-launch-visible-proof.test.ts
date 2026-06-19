const assert = require("node:assert/strict");
const { execFileSync } = require("node:child_process");
const fs = require("node:fs");
const os = require("node:os");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");
const defaultAiRoute = "chat";

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function sliceFrom(source, startMarker, endMarker) {
  const start = source.indexOf(startMarker);
  assert.notEqual(start, -1, `missing start marker ${startMarker}`);
  const end = source.indexOf(endMarker, start + startMarker.length);
  assert.notEqual(end, -1, `missing end marker ${endMarker}`);
  return source.slice(start, end);
}

test("ai/vercel-ai is visible and interactive in the launch runtime source", () => {
  const launch = read("tools/launch/runtime-template/pages/index.html");
  const runtime = read("tools/launch/runtime-template/assets/launch-runtime.ts");
  const shell = read("examples/template/template-shell.tsx");
  const aiChat = read("examples/template/ai-chat-status.tsx");
  const catalog = read("examples/template/package-catalog.ts");
  const editContract = read("examples/template/dx-studio-edit-contract.ts");
  const routeContract = read("examples/template/template-route-contract.ts");
  const packageDoc = read("docs/packages/ai-vercel-ai.md");
  const workflowReceipt = read(
    "examples/template/.dx/forge/receipts/2026-05-22-ai-vercel-ai-launch-assistant.json",
  );
  const cli = read("dx-www/src/cli/mod.rs");
  const studioManifest = read("dx-www/src/cli/studio_manifest.rs");

  assert.match(shell, /<LaunchAiChatStatus \/>/);
  assert.match(shell, /data-dx-component="launch-ai-assistant-dashboard-workflow"/);
  assert.match(shell, /data-dx-dashboard-workflow="prompt-action-provider-readiness"/);
  assert.match(shell, /data-dx-edit-kind="dashboard-workflow"/);
  assert.match(shell, /data-dx-product-surface="launch-assistant"/);
  assert.match(shell, /data-dx-ai-route-contract="\/api\/ai\/chat"/);
  assert.match(shell, /data-dx-ai-config-state="missing-config"/);
  assert.match(shell, /data-dx-icon-search="pack:ai"/);
  assert.match(shell, /data-dx-package="ai\/vercel-ai"/);
  assert.doesNotMatch(shell, /data-dx-component="ai-route-proof"/);
  assert.doesNotMatch(shell, /<CardTitle>AI route<\/CardTitle>/);
  assert.match(aiChat, /data-dx-component="launch-ai-dashboard-workflow"/);
  assert.match(aiChat, /data-dx-dashboard-workflow="launch-ai-assistant"/);
  assert.match(aiChat, /data-dx-ai-route-contract="\/api\/ai\/chat"/);
  assert.match(aiChat, /data-dx-ai-config-state="missing-config"/);
  assert.match(aiChat, /data-dx-ai-interaction="provider-picker"/);
  assert.match(aiChat, /data-dx-ai-interaction="prompt-field"/);
  assert.match(aiChat, /data-dx-ai-provider-choice=\{option\.id\}/);
  assert.match(aiChat, /data-dx-ai-provider-env=\{option\.env\}/);
  assert.match(aiChat, /data-dx-ai-prompt-state=\{promptState\}/);
  assert.match(aiChat, /data-dx-ai-action="safe-stream-contract-preview"/);
  assert.match(aiChat, /data-dx-ai-action-state=\{/);
  assert.match(aiChat, /data-dx-ai-public-api=\{activeProvider\.publicApis\.join\(","\)\}/);
  assert.match(aiChat, /data-dx-ai-local-response=\{preview\.status\}/);
  assert.match(aiChat, /status: "invalid-prompt"/);
  assert.match(aiChat, /Enter a launch prompt before previewing the AI route contract/);
  assert.match(aiChat, /<dx-icon name="pack:ai"/);
  assert.match(aiChat, /<dx-icon name="pack:play"/);
  assert.match(aiChat, /fetch\("\/api\/ai\/chat"/);
  assert.match(aiChat, /AI_PROVIDER_API_KEY/);
  assert.match(aiChat, /AI_GATEWAY_API_KEY/);
  assert.match(aiChat, /streamText/);
  assert.match(aiChat, /createProviderRegistry/);
  assert.match(catalog, /env: \["AI_PROVIDER_API_KEY", "AI_GATEWAY_API_KEY"\]/);
  assert.match(catalog, /name: "AI SDK Launch Assistant"/);
  assert.match(catalog, /<dx-icon name=\\"pack:ai\\" \/>/);
  assert.match(
    routeContract,
    /"\.dx\/forge\/receipts\/2026-05-22-ai-vercel-ai-launch-assistant\.json"/,
  );
  assert.match(workflowReceipt, /"package_id": "ai\/vercel-ai"/);
  assert.match(workflowReceipt, /"component": "launch-ai-assistant-dashboard-workflow"/);
  assert.match(workflowReceipt, /"workflow": "prompt-action-provider-readiness"/);
  assert.match(workflowReceipt, /"source_mirror": "G:\/WWW\/inspirations\/vercel-ai"/);
  assert.match(workflowReceipt, /"AI_PROVIDER_API_KEY"/);
  assert.match(workflowReceipt, /"AI_GATEWAY_API_KEY"/);
  assert.match(workflowReceipt, /"streamText"/);
  assert.match(workflowReceipt, /"createProviderRegistry"/);
  assert.match(workflowReceipt, /"data-dx-ai-route-contract=\\"\/api\/ai\/chat\\""/);
  assert.match(workflowReceipt, /"data-dx-ai-prompt-state"/);
  assert.match(workflowReceipt, /"data-dx-ai-action-state"/);
  assert.match(workflowReceipt, /"data-dx-ai-request-id"/);
  assert.match(workflowReceipt, /"local_readiness_interactions": \[/);
  assert.doesNotMatch(workflowReceipt, /local_demo_interactions|data-dx-ai-demo/);
  assert.match(workflowReceipt, /"request_id_echo": true/);
  assert.match(workflowReceipt, /"no_runtime_execution": true/);
  assert.match(routeContract, /vercelAiLaunchAssistant: \{/);
  assert.match(routeContract, /packageId: "ai\/vercel-ai"/);
  assert.match(routeContract, /component: "launch-ai-assistant-dashboard-workflow"/);
  assert.match(routeContract, /dashboardWorkflow: "prompt-action-provider-readiness"/);
  assert.match(routeContract, /sourceFile: "examples\/template\/ai-chat-status\.tsx"/);
  assert.match(routeContract, /materializedFile: "components\/template-app\/ai-chat-status\.tsx"/);
  assert.match(routeContract, /routeContract: "\/api\/ai\/chat"/);
  assert.match(routeContract, /requiredEnv: \["AI_PROVIDER_API_KEY"\]/);
  assert.match(routeContract, /optionalEnv: \["AI_GATEWAY_API_KEY"\]/);
  assert.match(routeContract, /"data-dx-ai-local-response"/);
  assert.match(routeContract, /"data-dx-ai-prompt-state"/);
  assert.match(routeContract, /"data-dx-ai-action-state"/);
  assert.match(routeContract, /"data-dx-ai-request-id"/);
  assert.match(
    routeContract,
    /sourceGuard: "dx run --test \.\\\\benchmarks\\\\vercel-ai-launch-visible-proof\.test\.ts"/,
  );
  assert.match(cli, /"package_id": "ai\/vercel-ai"/);
  assert.match(cli, /"env": \["AI_PROVIDER_API_KEY", "AI_GATEWAY_API_KEY"\]/);
  assert.match(cli, /launch-ai-assistant-dashboard-workflow/);
  assert.match(cli, /"dashboard_usage": "\/launch uses the AI SDK Launch Assistant/);
  assert.match(cli, /"components\/template-app\/ai-chat-status\.tsx"/);
  assert.match(cli, /"LaunchAiChatStatus"/);
  assert.match(cli, /"createProviderRegistry"/);
  assert.match(studioManifest, /"front_facing_name": "AI SDK Launch Assistant"/);
  assert.match(studioManifest, /"official_dx_package_name": "AI SDK"/);
  assert.match(studioManifest, /"dx_icon": "pack:ai"/);
  assert.match(studioManifest, /"dashboard_workflow": "prompt-action-provider-readiness"/);
  assert.match(studioManifest, /"readiness": "visible-prompt-action-provider-missing-route-contract"/);
  assert.match(studioManifest, /"data-dx-ai-route-contract"/);
  assert.match(studioManifest, /"data-dx-ai-local-response"/);
  assert.match(studioManifest, /"data-dx-ai-prompt-state"/);
  assert.match(studioManifest, /"data-dx-ai-action-state"/);
  assert.match(studioManifest, /"data-dx-ai-request-id"/);
  assert.match(studioManifest, /"AI_GATEWAY_API_KEY"/);
  const aiStudioSurface = sliceFrom(
    studioManifest,
    '"package": "ai/vercel-ai"',
    '"package": "api/trpc"',
  );
  assert.match(aiStudioSurface, /"data-dx-check-package-lane-row"/);
  assert.match(aiStudioSurface, /"data-dx-check-package-lane-dx-style-status"/);
  const dxCheckSurface = sliceFrom(
    editContract,
    'id: "dx-check-health-panel"',
    'id: "next-intl-dashboard-locale-workflow"',
  );
  assert.match(dxCheckSurface, /"ai\/vercel-ai"/);
  assert.match(packageDoc, /data-dx-ai-request-id/);
  assert.match(packageDoc, /data-dx-ai-prompt-state/);
  assert.match(packageDoc, /invalid-prompt/);

  assert.match(launch, /data-dx-package="ai\/vercel-ai"/);
  assert.match(launch, /data-dx-component="launch-ai-assistant-dashboard-workflow"/);
  assert.match(launch, /data-dx-dashboard-workflow="prompt-action-provider-readiness"/);
  assert.match(launch, /data-dx-ai-readiness="prompt-action-provider-readiness"/);
  assert.doesNotMatch(launch, /data-dx-ai-demo=/);
  assert.match(launch, /data-dx-ai-config-state="missing-config"/);
  assert.match(launch, /data-dx-ai-provider="openai-compatible"/);
  assert.match(launch, /data-dx-ai-interaction="prompt-field"/);
  assert.match(launch, /data-dx-ai-interaction="provider-picker"/);
  assert.match(launch, /data-dx-ai-provider-choice="openai-compatible"/);
  assert.match(launch, /data-dx-ai-provider-choice="gateway"/);
  assert.match(launch, /data-dx-ai-provider-env="AI_PROVIDER_API_KEY"/);
  assert.match(launch, /data-dx-ai-action="safe-stream-contract-preview"/);
  assert.match(launch, /data-dx-ai-action-state="ready"/);
  assert.match(launch, /data-dx-ai-local-response="idle"/);
  assert.match(launch, /data-dx-ai-prompt-state="ready"/);
  assert.match(launch, /data-dx-ai-request-id="idle"/);
  assert.match(launch, /data-dx-node-modules="forbidden"/);

  assert.match(runtime, /data-dx-component="launch-ai-assistant-dashboard-workflow"/);
  assert.match(runtime, /data-dx-package="ai\/vercel-ai"/);
  assert.match(runtime, /data-dx-ai-provider-choice/);
  assert.match(runtime, /setAiProof/);
  assert.match(runtime, /AI_PROVIDER_API_KEY|provider missing credentials|appOwnedBoundary/);
  assert.match(runtime, /payload\.status === "missing-config"/);
  assert.match(runtime, /missingConfig \? "missing-config" : "error"/);
  assert.match(runtime, /syncAiPromptState/);
  assert.match(runtime, /"invalid-prompt"/);
  assert.match(runtime, /dataset\.dxAiRequestId = requestId/);
  assert.match(runtime, /payload\.requestId \?\? requestId/);
  assert.match(runtime, /local-readiness/);
  assert.match(runtime, /Local AI readiness/);
  assert.doesNotMatch(runtime, /local-placeholder|Local AI preview/);
  assert.match(runtime, /\/api\/ai\/chat/);
});

test("ai/vercel-ai materializes into generated /launch without node_modules", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-vercel-ai-launch-"));
  fs.mkdirSync(path.join(dir, "app", "launch"), { recursive: true });
  fs.writeFileSync(
    path.join(dir, "app", "launch", "page.tsx"),
    "export default function Page(){ return <>{children}</>; }\n",
  );

  const output = execFileSync(process.execPath, [materializer, dir], {
    cwd: root,
    encoding: "utf8",
  });
  const result = JSON.parse(output);
  const launch = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");
  const runtime = fs.readFileSync(path.join(dir, "public", "launch-runtime.js"), "utf8");
  const manifest = JSON.parse(
    fs.readFileSync(path.join(dir, "public", "preview-manifest.json"), "utf8"),
  );
  const aiRoute = fs.readFileSync(path.join(dir, "app", "api", "ai", "chat", "route.ts"), "utf8");
  const aiBoundary = fs.readFileSync(path.join(dir, "lib", "ai", "provider-boundary.ts"), "utf8");

  assert.equal(result.ok, true);
  assert.equal(result.noNodeModules, true);
  assert.match(launch, /data-dx-route="\/"/);
  assert.match(launch, /data-dx-package="ai\/vercel-ai"/);
  assert.match(launch, /data-dx-check-package-lane-template="ai\/vercel-ai"/);
  assert.match(launch, /data-dx-check-package-lane-row="ai\/vercel-ai"/);
  assert.match(launch, /data-dx-check-package-lane-name="AI SDK"/);
  assert.match(launch, /data-dx-check-package-lane-dx-style-status="present"/);
  assert.match(launch, /data-dx-style-surface="ai-sdk"/);
  assert.match(launch, /data-dx-token-scope="ai\/vercel-ai"/);
  assert.match(launch, /data-dx-component="launch-ai-assistant-dashboard-workflow"/);
  assert.match(launch, /data-dx-ai-readiness="prompt-action-provider-readiness"/);
  assert.doesNotMatch(launch, /data-dx-ai-demo=/);
  assert.match(launch, /data-dx-ai-provider-choice="gateway"/);
  assert.match(launch, /data-dx-ai-action="safe-stream-contract-preview"/);
  assert.match(launch, /data-dx-ai-local-response="idle"/);
  assert.match(launch, /data-dx-ai-prompt-state="ready"/);
  assert.match(runtime, /setAiProof/);
  assert.match(runtime, /local-readiness/);
  assert.doesNotMatch(runtime, /local-placeholder|Local AI preview/);
  assert.match(aiRoute, /createDxAiMissingProviderResponse/);
  assert.match(aiRoute, /openai-compatible/);
  assert.match(aiRoute, /AI_PROVIDER_API_KEY/);
  assert.match(aiRoute, /provider-configured-readiness-only/);
  assert.match(aiRoute, /Response\.json/);
  assert.match(aiBoundary, /status: "missing-config"/);
  assert.match(aiBoundary, /httpStatus: 501/);
  assert.match(aiBoundary, /credentialsConfigured: false/);
  assert.match(aiBoundary, /adapterBoundary: "provider-credential-boundary"/);
  assert.match(aiBoundary, /providerRuntime: false/);
  assert.match(aiBoundary, /\{ status: 501 \}/);
  assert.match(aiRoute, /runtimeExecution: false/);
  assert.match(aiRoute, /modelStreaming: false/);
  assert.match(aiRoute, /body\.requestId/);
  assert.match(aiRoute, /\{ status: 202 \}/);

  const launchRoute = manifest.routes.find((route) => route.route === "/");
  assert.ok(launchRoute, "expected materialized /launch route metadata");
  assert.ok(launchRoute.forgePackages.includes("ai/vercel-ai"));

  const checkPanel = manifest.editContract.editableSurfaces.find(
    (surface) => surface.id === "launch-runtime-dx-check-panel",
  );
  assert.ok(checkPanel, "expected dx-check panel edit surface");
  assert.equal(checkPanel.sourceFile, "pages/index.html");
  assert.ok(
    checkPanel.packageIds.includes("ai/vercel-ai"),
    "generated dx-check panel package scope must include AI SDK",
  );
  assert.ok(checkPanel.stateMarkers.includes("data-dx-check-package-lane-row"));
  assert.ok(checkPanel.stateMarkers.includes("data-dx-check-package-lane-dx-style-status"));
});

test("default AI chat route keeps provider credentials as readiness-only evidence", () => {
  const routePaths = [
    "examples/template/app/api/ai/chat/route.ts",
    ".dx/template-app-browser-preview/app/api/ai/chat/route.ts",
  ];

  for (const routePath of routePaths) {
    const route = read(routePath);

    assert.match(route, /createDxAiMissingProviderResponse/);
    assert.match(route, /AI_PROVIDER_API_KEY/);
    assert.match(route, /provider-configured-readiness-only/);
    assert.match(route, /credentialsConfigured:\s*Boolean\(process\.env\.AI_PROVIDER_API_KEY\)/);
    assert.match(route, /runtimeExecution:\s*false/);
    assert.match(route, /modelStreaming:\s*false/);
    assert.match(route, /providerRuntime:\s*false/);
    assert.match(route, /runtimeProof:\s*false/);
    assert.match(route, /liveProviderProof:\s*false/);
    assert.doesNotMatch(route, /credentialsConfigured:\s*true/);
    assert.doesNotMatch(route, /providerRuntime:\s*true/);
  }
});

test("extended AI provider routes are opt-in boundaries outside the default launch proof", () => {
  const boundary = read("examples/template/lib/ai/provider-boundary.ts");
  const launch = read("tools/launch/runtime-template/pages/index.html");
  const generatedDashboard = read("tools/launch/runtime-template/pages/index.html");
  const packageDoc = read("docs/packages/ai-vercel-ai.md");
  const packageReality = read("examples/template/components/template-app/package-reality.ts");
  const extendedRoutes = [
    ["agent", "agent-loop", "AI_PROVIDER_API_KEY"],
    ["image", "image-generation", "AI_PROVIDER_API_KEY"],
    ["object", "object-generation", "AI_PROVIDER_API_KEY"],
    ["speech", "speech-generation", "AI_PROVIDER_API_KEY"],
    ["text-stream", "text-stream-bridge", "AI_PROVIDER_API_KEY"],
    ["transcribe", "audio-transcription", "AI_PROVIDER_API_KEY"],
    ["ui-stream", "ui-message-stream-bridge", "AI_PROVIDER_API_KEY"],
    ["upload-file", "provider-file-upload", "AI_PROVIDER_API_KEY"],
    ["video", "video-generation", "AI_GATEWAY_API_KEY"],
  ];
  const routeDirectories = fs
    .readdirSync(path.join(root, "examples", "template", "app", "api", "ai"), {
      withFileTypes: true,
    })
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .filter((route) => route !== defaultAiRoute)
    .sort();

  assert.match(boundary, /DX_ENABLE_EXTENDED_AI_ROUTES/);
  assert.match(boundary, /createDxAiExtendedRouteDisabledResponse/);
  assert.match(boundary, /adapterBoundary: "extended-provider-route-boundary"/);
  assert.match(packageDoc, /DX_ENABLE_EXTENDED_AI_ROUTES/);
  assert.match(packageDoc, /outside\s+the default launch AI proof surface/);
  assert.deepEqual(
    extendedRoutes.map(([route]) => route).sort(),
    routeDirectories,
    "every non-chat AI route must be inventoried as outside the default launch proof",
  );

  for (const [route, capability, requiredEnv] of extendedRoutes) {
    const routePath = `/api/ai/${route}`;
    const source = read(`examples/template/app/api/ai/${route}/route.ts`);
    assert.match(source, /isDxAiExtendedRouteEnabled/);
    assert.match(source, /createDxAiExtendedRouteDisabledResponse/);
    assert.match(source, /createDxAiMissingProviderResponse/);
    assert.match(
      source,
      new RegExp(`!process\\.env\\.${requiredEnv}`),
      `${routePath} must stop at missing-config before any opt-in route work when ${requiredEnv} is absent`,
    );
    assert.match(source, new RegExp(`route: "${routePath}"`));
    assert.match(source, new RegExp(`capability: "${capability}"`));
    assert.match(source, new RegExp(`requiredEnv: "${requiredEnv}"`));
    assert.match(packageReality, new RegExp(`"${routePath}"`));
    assert.match(generatedDashboard, new RegExp(routePath));
    assert.doesNotMatch(launch, new RegExp(routePath));
  }
});
