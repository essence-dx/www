const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const root = path.resolve(__dirname, "..");
const runtimeScriptPath = path.join(
  root,
  "examples",
  "template",
  "public",
  "launch-runtime.js",
);

class FakeElement {
  constructor() {
    this.attributes = new Map();
    this.children = [];
    this.dataset = {};
    this.disabled = false;
    this.listeners = new Map();
    this.style = {};
    this.textContent = "";
    this.classNames = new Set();
    this.classList = {
      toggle: (name, force) => {
        const shouldHave = force ?? !this.classNames.has(name);
        if (shouldHave) this.classNames.add(name);
        else this.classNames.delete(name);
        return shouldHave;
      },
    };
  }

  addEventListener(type, handler) {
    this.listeners.set(type, handler);
  }

  setAttribute(name, value) {
    const normalizedValue = String(value);
    this.attributes.set(name, normalizedValue);
    if (name.startsWith("data-")) {
      const key = name
        .slice(5)
        .replace(/-([a-z])/g, (_, letter) => letter.toUpperCase());
      this.dataset[key] = normalizedValue;
    }
  }

  appendChild(child) {
    this.children = this.children.filter((item) => item !== child);
    this.children.push(child);
  }

  click() {
    const handler = this.listeners.get("click");
    assert.equal(typeof handler, "function", "expected a click listener");
    handler();
  }

  keyDown(key) {
    const handler = this.listeners.get("keydown");
    assert.equal(typeof handler, "function", "expected a keydown listener");
    let defaultPrevented = false;
    handler({
      key,
      preventDefault: () => {
        defaultPrevented = true;
      },
    });
    return defaultPrevented;
  }
}

class FakeStageList extends FakeElement {
  constructor(stages) {
    super();
    this.stages = new Map(stages.map((stage) => [stage.dataset.dxMotionStage, stage]));
    this.children = stages;
  }

  querySelector(selector) {
    const match = selector.match(/\[data-dx-motion-stage="([^"]+)"\]/);
    return match ? this.stages.get(match[1]) ?? null : null;
  }
}

function createMotionDom() {
  const stageSource = new FakeElement();
  stageSource.dataset.dxMotionStage = "source-owned";
  const stageInteractive = new FakeElement();
  stageInteractive.dataset.dxMotionStage = "interactive";
  const stagePreview = new FakeElement();
  stagePreview.dataset.dxMotionStage = "preview-ready";

  const nodes = {
    dashboard: new FakeElement(),
    dashboardStatus: new FakeElement(),
    motionDashboardStatus: new FakeElement(),
    motionDashboardDetail: new FakeElement(),
    motionDashboardPolicy: new FakeElement(),
    card: new FakeElement(),
    policyStatus: new FakeElement(),
    status: new FakeElement(),
    progress: new FakeElement(),
    advance: new FakeElement(),
    reorder: new FakeElement(),
    movePrevious: new FakeElement(),
    moveNext: new FakeElement(),
    reset: new FakeElement(),
    reducedMotion: new FakeElement(),
    stageList: new FakeStageList([stageSource, stageInteractive, stagePreview]),
  };
  nodes.card.dataset.dxPackage = "animation/motion";
  nodes.card.dataset.dxComponent = "motion-animation-card";
  nodes.dashboard.dataset.dxComponent = "launch-operating-dashboard";

  const queryMap = new Map([
    ['[data-dx-component="launch-operating-dashboard"]', nodes.dashboard],
    ["#mission-dashboard-status", nodes.dashboardStatus],
    ["#mission-motion-status", nodes.motionDashboardStatus],
    ["#mission-motion-detail", nodes.motionDashboardDetail],
    ["#mission-motion-policy", nodes.motionDashboardPolicy],
    ["#motion-proof", nodes.card],
    ["#motion-policy-status", nodes.policyStatus],
    ["#motion-status", nodes.status],
    ["[data-dx-motion-progress-bar]", nodes.progress],
    ["#motion-stage-list", nodes.stageList],
    ["[data-dx-motion-interaction='advance-stage']", nodes.advance],
    ["[data-dx-motion-interaction='reverse-order']", nodes.reorder],
    ["[data-dx-motion-interaction='move-stage-previous']", nodes.movePrevious],
    ["[data-dx-motion-interaction='move-stage-next']", nodes.moveNext],
    ["[data-dx-motion-interaction='reset-proof']", nodes.reset],
    ["[data-dx-motion-interaction='toggle-reduced-motion']", nodes.reducedMotion],
  ]);

  const documentListeners = new Map();
  const document = {
    documentElement: {},
    addEventListener: (type, handler) => documentListeners.set(type, handler),
    createElement: () => new FakeElement(),
    querySelector: (selector) => queryMap.get(selector) ?? null,
    querySelectorAll: () => [],
    fire: (type) => {
      const handler = documentListeners.get(type);
      assert.equal(typeof handler, "function", `expected ${type} listener`);
      handler();
    },
  };

  return { document, nodes };
}

test("Motion runtime proof advances, reorders, and resets browser-visible state", () => {
  const script = fs.readFileSync(runtimeScriptPath, "utf8");
  const { document, nodes } = createMotionDom();
  const storage = new Map();
  const localStorage = {
    getItem: (key) => storage.get(key) ?? null,
    setItem: (key, value) => storage.set(key, String(value)),
  };
  const preferenceStorageKey = "dx.launch.motion.dashboard";

  vm.runInNewContext(script, {
    HTMLCanvasElement: class {},
    $$: () => [],
    console,
    document,
    fetch: async () => ({ json: async () => ({}), ok: true, status: 200 }),
    localStorage,
    requestAnimationFrame: () => 0,
    WebAssembly,
  });

  document.fire("DOMContentLoaded");

  assert.equal(nodes.card.dataset.dxPackage, "animation/motion");
  assert.equal(nodes.card.dataset.dxComponent, "motion-animation-card");
  assert.equal(nodes.card.dataset.dxMotionState, "source-owned");
  assert.equal(nodes.card.dataset.dxMotionProgress, "34");
  assert.equal(nodes.card.dataset.dxMotionOrder, "source-owned,interactive,preview-ready");
  assert.equal(nodes.card.dataset.dxMotionReduced, "system");
  assert.equal(nodes.card.dataset.dxMotionPreferenceStorage, "local-storage");
  assert.equal(nodes.card.dataset.dxMotionStorageKey, preferenceStorageKey);
  assert.equal(nodes.progress.style.width, "34%");
  assert.match(nodes.status.textContent, /Source-owned adapters/);
  assert.match(nodes.policyStatus.textContent, /system/);
  assert.equal(nodes.dashboard.dataset.dxDashboardMotion, "source-owned");
  assert.equal(nodes.dashboard.dataset.dxDashboardMotionReduced, "system");
  assert.equal(nodes.motionDashboardStatus.textContent, "Source-owned adapters");
  assert.match(nodes.motionDashboardDetail.textContent, /34%/);
  assert.match(nodes.motionDashboardPolicy.textContent, /system/);
  assert.equal(nodes.movePrevious.dataset.dxMotionOrderAvailable, "false");
  assert.equal(nodes.movePrevious.disabled, true);
  assert.equal(nodes.moveNext.dataset.dxMotionOrderAvailable, "true");
  assert.equal(nodes.moveNext.disabled, false);
  assert.equal(nodes.stageList.dataset.dxMotionKeyboardReorder, "arrow-home-end");
  assert.equal(nodes.stageList.dataset.dxMotionKeyboardState, "source-owned");
  assert.equal(nodes.stageList.attributes.get("aria-activedescendant"), "motion-stage-source-owned");

  nodes.advance.click();
  assert.equal(nodes.card.dataset.dxMotionState, "interactive");
  assert.equal(nodes.card.dataset.dxMotionProgress, "67");
  assert.equal(nodes.progress.style.width, "67%");
  assert.equal(nodes.card.classNames.has("is-active"), true);
  assert.equal(nodes.dashboard.dataset.dxDashboardMotion, "interactive");
  assert.equal(nodes.motionDashboardStatus.textContent, "Local interaction");
  assert.equal(nodes.movePrevious.dataset.dxMotionOrderAvailable, "true");
  assert.equal(nodes.movePrevious.disabled, false);
  assert.equal(nodes.moveNext.dataset.dxMotionOrderAvailable, "true");
  assert.equal(nodes.moveNext.disabled, false);
  assert.equal(nodes.stageList.dataset.dxMotionKeyboardState, "interactive");
  assert.equal(nodes.stageList.attributes.get("aria-activedescendant"), "motion-stage-interactive");

  assert.equal(nodes.stageList.keyDown("End"), true);
  assert.equal(nodes.card.dataset.dxMotionOrder, "source-owned,preview-ready,interactive");
  assert.equal(nodes.stageList.dataset.dxMotionKeyboardState, "interactive");
  assert.equal(nodes.moveNext.dataset.dxMotionOrderAvailable, "false");

  assert.equal(nodes.stageList.keyDown("Home"), true);
  assert.equal(nodes.card.dataset.dxMotionOrder, "interactive,source-owned,preview-ready");
  assert.equal(nodes.movePrevious.dataset.dxMotionOrderAvailable, "false");
  assert.equal(nodes.moveNext.dataset.dxMotionOrderAvailable, "true");

  assert.equal(nodes.stageList.keyDown("ArrowRight"), true);
  assert.equal(nodes.card.dataset.dxMotionOrder, "source-owned,interactive,preview-ready");
  assert.equal(nodes.movePrevious.dataset.dxMotionOrderAvailable, "true");
  assert.equal(nodes.moveNext.dataset.dxMotionOrderAvailable, "true");

  nodes.moveNext.click();
  assert.equal(nodes.card.dataset.dxMotionOrder, "source-owned,preview-ready,interactive");
  assert.equal(nodes.movePrevious.dataset.dxMotionOrderAvailable, "true");
  assert.equal(nodes.moveNext.dataset.dxMotionOrderAvailable, "false");
  assert.equal(nodes.moveNext.disabled, true);
  assert.deepEqual(
    nodes.stageList.children.map((stage) => stage.dataset.dxMotionStage),
    ["source-owned", "preview-ready", "interactive"],
  );

  nodes.movePrevious.click();
  assert.equal(nodes.card.dataset.dxMotionOrder, "source-owned,interactive,preview-ready");
  assert.equal(nodes.movePrevious.dataset.dxMotionOrderAvailable, "true");
  assert.equal(nodes.moveNext.dataset.dxMotionOrderAvailable, "true");

  nodes.reducedMotion.click();
  assert.equal(nodes.card.dataset.dxMotionReduced, "preview");
  assert.equal(nodes.dashboard.dataset.dxDashboardMotionReduced, "preview");
  assert.match(nodes.policyStatus.textContent, /Reduced motion preview/);
  assert.match(nodes.motionDashboardPolicy.textContent, /Reduced motion preview/);
  assert.equal(nodes.reducedMotion.attributes.get("aria-pressed"), "true");
  assert.match(storage.get(preferenceStorageKey), /"motionReduced":true/);

  nodes.reorder.click();
  assert.equal(nodes.card.dataset.dxMotionOrder, "preview-ready,interactive,source-owned");
  assert.match(storage.get(preferenceStorageKey), /"motionOrder":\["preview-ready","interactive","source-owned"\]/);
  assert.deepEqual(
    nodes.stageList.children.map((stage) => stage.dataset.dxMotionStage),
    ["preview-ready", "interactive", "source-owned"],
  );

  nodes.reset.click();
  assert.equal(nodes.card.dataset.dxMotionState, "source-owned");
  assert.equal(nodes.card.dataset.dxMotionProgress, "34");
  assert.equal(nodes.card.dataset.dxMotionOrder, "source-owned,interactive,preview-ready");
  assert.equal(nodes.progress.style.width, "34%");
  assert.equal(nodes.dashboard.dataset.dxDashboardMotion, "source-owned");
  assert.equal(nodes.dashboard.dataset.dxDashboardMotionReduced, "system");
  assert.equal(nodes.movePrevious.dataset.dxMotionOrderAvailable, "false");
  assert.equal(nodes.movePrevious.disabled, true);
  assert.equal(nodes.moveNext.dataset.dxMotionOrderAvailable, "true");
  assert.equal(nodes.stageList.dataset.dxMotionKeyboardState, "source-owned");
  assert.match(nodes.motionDashboardDetail.textContent, /34%/);
  assert.match(nodes.motionDashboardPolicy.textContent, /system/);
  assert.match(storage.get(preferenceStorageKey), /"motionReduced":false/);
});

test("Motion runtime proof restores persisted local preference state", () => {
  const script = fs.readFileSync(runtimeScriptPath, "utf8");
  const { document, nodes } = createMotionDom();
  const preferenceStorageKey = "dx.launch.motion.dashboard";
  const storage = new Map([
    [
      preferenceStorageKey,
      JSON.stringify({
        motionOrder: ["preview-ready", "interactive", "source-owned"],
        motionReduced: true,
      }),
    ],
  ]);
  const localStorage = {
    getItem: (key) => storage.get(key) ?? null,
    setItem: (key, value) => storage.set(key, String(value)),
  };

  vm.runInNewContext(script, {
    HTMLCanvasElement: class {},
    $$: () => [],
    console,
    document,
    fetch: async () => ({ json: async () => ({}), ok: true, status: 200 }),
    localStorage,
    requestAnimationFrame: () => 0,
    WebAssembly,
  });

  document.fire("DOMContentLoaded");

  assert.equal(nodes.card.dataset.dxMotionOrder, "preview-ready,interactive,source-owned");
  assert.equal(nodes.card.dataset.dxMotionReduced, "preview");
  assert.equal(nodes.dashboard.dataset.dxDashboardMotionReduced, "preview");
  assert.deepEqual(
    nodes.stageList.children.map((stage) => stage.dataset.dxMotionStage),
    ["preview-ready", "interactive", "source-owned"],
  );
});
