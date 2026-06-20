import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const materializer = path.join(root, "tools", "launch", "materialize-www-template.ts");

const routePages = {
  "/": "index",
  "/dashboard": "dashboard",
  "/login": "login",
} as const;

const runtimeRouteSources = {
  "runtime /": "tools/launch/runtime-template/pages/index.html",
  "runtime /login": "tools/launch/runtime-template/pages/login.html",
  "runtime /logout": "tools/launch/runtime-template/pages/logout.html",
} as const;

function read(relativePath: string) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

function cssBlocks(source: string, selector: string) {
  const escaped = selector.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  return Array.from(
    source.matchAll(new RegExp(`(?:^|\\n)${escaped}\\s*\\{([\\s\\S]*?)\\}`, "g")),
    (match) => match[1],
  );
}

function assertNativeDocumentScrollCss(source: string, label: string) {
  const blocks = [...cssBlocks(source, "html"), ...cssBlocks(source, "body")];
  assert.ok(blocks.length >= 2, `${label} should define html/body scroll roots`);

  for (const block of blocks) {
    assert.doesNotMatch(
      block,
      /(?:^|\n)\s*overflow(?:-y)?\s*:\s*(?:hidden|clip)\b/,
      `${label} should not disable document y-scrolling`,
    );
    assert.doesNotMatch(
      block,
      /(?:^|\n)\s*position\s*:\s*fixed\b/,
      `${label} should not pin the document scroll root`,
    );
  }

  assert.match(
    source,
    /body\[data-dx-scroll-system="native"\][\s\S]*overflow-y:\s*auto/,
    `${label} should expose an explicit native document scroll contract`,
  );
  assert.match(
    source,
    /\[data-dx-scroll-content="viewport-plus"\][\s\S]*min-height:\s*calc\(100(?:dvh|vh) \+ 1px\)/,
    `${label} should create real viewport-plus document overflow for visible native scrollbars`,
  );
  assert.match(
    source,
    /\[data-dx-scroll-trap="false"\][\s\S]*touch-action:\s*manipulation/,
    `${label} should keep mobile menu controls from installing a scroll lock`,
  );
  assert.doesNotMatch(
    source,
    /::-webkit-scrollbar|scrollbar-color|scrollbar-width|data-custom-scrollbar|dashboard-scrollbar/,
    `${label} should not reintroduce custom scrollbar styling`,
  );
  assert.doesNotMatch(
    source,
    /\.dx-scroll-(?:area|viewport)|\.dx-window-scroll/,
    `${label} should not retain legacy nested scroll viewport classes`,
  );
}

function joinedBlocks(source: string, selector: string) {
  return cssBlocks(source, selector).join("\n");
}

function firstTag(source: string, tagName: string) {
  return source.match(new RegExp(`<${tagName}\\b[^>]*>`, "i"))?.[0] ?? "";
}

function attrValue(tag: string, name: string) {
  return tag.match(new RegExp(`\\b${name}="([^"]*)"`))?.[1] ?? "";
}

function assertNoRouteScrollLock(html: string, label: string) {
  const htmlTag = firstTag(html, "html");
  const bodyTag = firstTag(html, "body");
  const routeRootTags = `${htmlTag}\n${bodyTag}`;
  assert.doesNotMatch(
    routeRootTags,
    /\bstyle="[^"]*(?:overflow(?:-y)?\s*:\s*(?:hidden|clip)\b|position\s*:\s*fixed\b)/,
    `${label} should not inline-lock the html/body scroll root`,
  );
  for (const tag of [htmlTag, bodyTag]) {
    const classes = attrValue(tag, "class").split(/\s+/).filter(Boolean);
    assert.equal(
      classes.some((className) =>
        ["no-scroll", "scroll-lock", "is-scroll-locked"].includes(className),
      ),
      false,
      `${label} should not use scroll-lock root classes`,
    );
  }
  assert.doesNotMatch(
    routeRootTags,
    /\bdata-(?:scroll-lock|body-scroll-lock)="(?:true|locked)"/,
    `${label} should not use root scroll-lock data attributes`,
  );
}

function assertMobileMenuDoesNotTrapDocumentScroll(source: string, label: string) {
  for (const selector of [".template-mobile-menu", ".dashboard-mobile-menu"]) {
    const block = joinedBlocks(source, selector);
    assert.match(block, /position:\s*relative/, `${label} ${selector} should stay in document flow`);
    assert.doesNotMatch(
      block,
      /position:\s*fixed|overflow(?:-y)?:\s*(?:hidden|clip)\b/,
      `${label} ${selector} should not install a modal scroll trap`,
    );
  }

  for (const selector of [".template-mobile-menu nav", ".dashboard-mobile-sheet"]) {
    const block = joinedBlocks(source, selector);
    assert.match(block, /position:\s*absolute/, `${label} ${selector} should be an anchored sheet`);
    assert.match(block, /max-height:\s*min\(/, `${label} ${selector} should stay viewport-bounded`);
    assert.match(block, /overflow-y:\s*auto/, `${label} ${selector} should scroll internally if crowded`);
    assert.doesNotMatch(
      block,
      /position:\s*fixed/,
      `${label} ${selector} should not pin the viewport or body`,
    );
  }
}

test("launch template keeps native document scrolling across primary routes", () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "dx-template-native-scroll-"));
  try {
    execFileSync(process.execPath, [materializer, dir], { cwd: root, stdio: "pipe" });

    const globals = read("examples/template/styles/globals.css");
    const runtimeCss = read("tools/launch/runtime-template/assets/launch-runtime.css");
    const materializedCss = fs.readFileSync(path.join(dir, "styles", "globals.css"), "utf8");
    const runtime = read("tools/launch/runtime-template/assets/launch-runtime.ts");
    const layout = read("examples/template/app/layout.tsx");
    const landingSource = read("examples/template/components/template-app/landing-page.tsx");
    const dashboardSource = read("examples/template/components/template-app/dashboard-page.tsx");
    const authSource = read("examples/template/components/template-app/auth-pages.tsx");
    const manifest = JSON.parse(
      fs.readFileSync(path.join(dir, "public", "preview-.dx/build-cache/manifest.json"), "utf8"),
    );

  assertNativeDocumentScrollCss(globals, "globals.css");
  assertNativeDocumentScrollCss(runtimeCss, "launch-runtime.css");
  assertMobileMenuDoesNotTrapDocumentScroll(globals, "globals.css");
  assertMobileMenuDoesNotTrapDocumentScroll(materializedCss, "materialized globals.css");

  assert.match(layout, /data-dx-scroll-root="document"/);
  assert.match(layout, /data-dx-scroll-system="native"/);
  assert.match(landingSource, /data-dx-scroll-surface="document"/);
  assert.match(landingSource, /data-dx-scroll-lock="none"/);
  assert.match(landingSource, /data-dx-scroll-proof="document-flow-no-lock"/);
  assert.match(landingSource, /data-dx-scroll-content="viewport-plus"/);
  assert.match(landingSource, /data-dx-wheel-scroll="native"/);
  assert.match(landingSource, /data-dx-scroll-trap="false"/);
  assert.match(dashboardSource, /data-dx-scroll-surface="document"/);
  assert.match(dashboardSource, /data-dx-scroll-lock="none"/);
  assert.match(dashboardSource, /data-dx-scroll-proof="document-flow-no-lock"/);
  assert.match(dashboardSource, /data-dx-scroll-content="viewport-plus"/);
  assert.match(dashboardSource, /data-dx-wheel-scroll="native"/);
  assert.match(dashboardSource, /data-dx-scroll-trap="false"/);
  assert.match(
    authSource,
    /export function TemplateLoginPage\(\)[\s\S]*data-dx-scroll-trap="false"/,
  );
  assert.match(
    authSource,
    /export function TemplateLogoutPage\(\)[\s\S]*data-dx-scroll-trap="false"/,
  );

  for (const [label, sourcePath] of Object.entries(runtimeRouteSources)) {
    const source = read(sourcePath);
    assertNoRouteScrollLock(source, label);
    assert.match(source, /data-dx-scroll-proof="document-flow-no-lock"/);
    assert.match(source, /data-dx-scroll-content="viewport-plus"/);
    assert.match(source, /data-dx-wheel-scroll="native"/);
  }

  assert.doesNotMatch(
    runtime,
    /(?:document\.body|document\.documentElement)\.style\.overflow|classList\.(?:add|toggle)\(["'](?:no-scroll|scroll-lock|is-scroll-locked)/,
    "runtime should not install a body/html scroll lock",
  );
  assert.doesNotMatch(runtime, /bindTemplateCustomScrollbar|scrollReveal|data-dashboard-scroll-action/);

  for (const [route, pageName] of Object.entries(routePages)) {
    const fileName = `${pageName}.html`;
    const html = fs.readFileSync(path.join(dir, "pages", fileName), "utf8");
    assertNoRouteScrollLock(html, route);
    assert.match(
      html,
      /<body[^>]+data-dx-scroll-root="document"[^>]+data-dx-scroll-system="native"/,
      `${route} should use the document as the native scroll root`,
    );
    assert.match(
      html,
      /<main[\s\S]*?data-dx-scroll-surface="document"[\s\S]*?data-dx-scroll-lock="none"/,
      `${route} should declare that its main surface does not lock document scrolling`,
    );
    assert.match(
      html,
      /<main[\s\S]*?data-dx-scroll-proof="document-flow-no-lock"[\s\S]*?data-dx-wheel-scroll="native"/,
      `${route} should expose an explicit wheel-scroll proof marker`,
    );
    assert.match(
      html,
      /<main[\s\S]*?data-dx-scroll-content="viewport-plus"/,
      `${route} should declare viewport-plus document-flow content instead of relying on body locks`,
    );
    assert.doesNotMatch(
      html,
      /data-dx-component="template-custom-scrollbar"|data-dashboard-scrollbar|data-scroll-reveal/,
      `${route} should not materialize the removed custom scrollbar surface`,
    );
  }

  const landing = fs.readFileSync(path.join(dir, "pages", "index.html"), "utf8");
  const dashboard = fs.readFileSync(path.join(dir, "pages", "dashboard.html"), "utf8");
  if (landing.includes("template-mobile-menu")) {
    assert.match(
      landing,
      /<[^>]+(?=[^>]*class="template-mobile-menu")(?=[^>]*data-dx-scroll-trap="false")/,
    );
  }
  assert.doesNotMatch(landing, /class="template-mobile-menu"[\s\S]*data-dx-scroll-trap="true"/);
  assert.match(
    dashboard,
    /<[^>]+(?=[^>]*class="dashboard-mobile-menu")(?=[^>]*data-dx-scroll-trap="false")/,
  );

  assert.equal(
    manifest.nativeScrollProof.schema,
    "dx.template.native_scroll_source_guard",
  );
  assert.equal(manifest.nativeScrollProof.sourceGuard, true);
  assert.equal(manifest.nativeScrollProof.browserRuntimeProof, false);
  assert.equal(manifest.nativeScrollProof.routesChecked, Object.keys(routePages).length);
  assert.deepEqual(manifest.nativeScrollProof.coverage, {
    expectedRoutes: Object.keys(routePages),
    routesCovered: Object.keys(routePages).length,
    routesWithNativeDocumentFlow: Object.keys(routePages).length,
    routesWithDocumentFlowContent: Object.keys(routePages).length,
    routesWithoutDocumentRootScrollLock: Object.keys(routePages).length,
    routesWithoutCustomScrollbarRuntime: Object.keys(routePages).length,
    routesWithoutScrollTrap: Object.keys(routePages).length,
    allRoutesNativeDocumentFlow: true,
    allRoutesHaveDocumentFlowContent: true,
    allRoutesDocumentRootUnlocked: true,
    allRoutesCustomScrollbarFree: true,
    allMobileMenusNonTrapping: true,
    cssDocumentRootUnlocked: true,
    sourceOnlyProof: true,
    browserRuntimeProof: false,
  });
  assert.equal(
    manifest.nativeScrollProof.cssContract.bodyNativeOverflowYAuto,
    true,
  );
  assert.equal(
    manifest.nativeScrollProof.cssContract.htmlBodyOverflowDisabled,
    false,
  );
  assert.equal(manifest.nativeScrollProof.cssContract.customScrollbarCss, false);
  const nativeScrollScoreComponent =
    manifest.forgePackageReality.scoreComponents.find(
      (component: { id: string }) => component.id === "native-scroll-source-proof",
    );
  assert.ok(
    nativeScrollScoreComponent,
    "native scroll source proof should contribute to the launch readiness score",
  );
  assert.deepEqual(nativeScrollScoreComponent, {
    id: "native-scroll-source-proof",
    label: "Native scroll source proof",
    points: 2,
    evidence:
      "Primary template routes use document-flow scrolling without custom scrollbar runtime or scroll traps.",
  });
  assert.equal(
    manifest.forgePackageReality.score,
    manifest.forgePackageReality.scoreCeilingWithoutLiveProof,
    "source-only native scroll proof should lift the score to the honest no-live-proof ceiling",
  );
  assert.equal(manifest.forgePackageReality.score, 89);
  assert.equal(
    manifest.nativeScrollProof.cssContract.documentHeightPolicy,
    "min-height-100vh-no-body-lock",
  );
  assert.equal(
    manifest.nativeScrollProof.cssContract.mobileMenuTriggerPositionFixed,
    false,
  );
  assert.equal(
    manifest.nativeScrollProof.cssContract.mobileMenuPanelsPositionFixed,
    false,
  );
  assert.equal(
    manifest.nativeScrollProof.cssContract.mobileMenuPanelsScrollable,
    true,
  );

  const routeProofByRoute = new Map(
    manifest.nativeScrollProof.routes.map((routeProof: { route: string }) => [
      routeProof.route,
      routeProof,
    ]),
  );
  for (const [route, pageName] of Object.entries(routePages)) {
    const proof = routeProofByRoute.get(route);
    assert.ok(proof, `${route} should have a native scroll manifest proof row`);
    assert.deepEqual(
      {
        sourceFile: proof.sourceFile,
        bodyScrollRoot: proof.bodyScrollRoot,
        bodyScrollSystem: proof.bodyScrollSystem,
        mainScrollSurface: proof.mainScrollSurface,
        mainScrollLock: proof.mainScrollLock,
        wheelScroll: proof.wheelScroll,
        documentFlowProof: proof.documentFlowProof,
        viewportPlusContent: proof.viewportPlusContent,
        documentFlowContent: proof.documentFlowContent,
        mainContentBytes: proof.mainContentBytes > 400,
        inlineScrollLock: proof.inlineScrollLock,
        classScrollLock: proof.classScrollLock,
        dataScrollLock: proof.dataScrollLock,
        customScrollbarRuntime: proof.customScrollbarRuntime,
      },
      {
        sourceFile: `pages/${pageName}.html`,
        bodyScrollRoot: true,
        bodyScrollSystem: true,
        mainScrollSurface: true,
        mainScrollLock: true,
        wheelScroll: true,
        documentFlowProof: true,
        viewportPlusContent: true,
        documentFlowContent: true,
        mainContentBytes: true,
        inlineScrollLock: false,
        classScrollLock: false,
        dataScrollLock: false,
        customScrollbarRuntime: false,
      },
    );
    assert.notEqual(
      proof.mobileMenuScrollTrap,
      true,
      `${route} should not report a mobile menu scroll trap`,
    );
  }
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
