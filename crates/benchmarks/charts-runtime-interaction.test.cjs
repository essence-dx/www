const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const repoRoot = path.resolve(__dirname, "..");
const runtimePath = path.join(repoRoot, "examples", "charts", "public", "chart-runtime.js");
const framePath = path.join(repoRoot, "examples", "charts", "components", "charts", "chart-frame.tsx");

class FakeElement {
  constructor(attributes = {}) {
    this.attributes = new Map(Object.entries(attributes));
    this.style = {};
    this.textContent = "";
    this.selectedCount = 0;
  }

  closest(selector) {
    if (selector === "[data-dx-chart-hit]" && this.attributes.has("data-dx-chart-hit")) return this;
    if (selector === "[data-dx-chart-id]" && this.attributes.has("data-dx-chart-id")) return this;
    return this.parent?.closest(selector) ?? null;
  }

  getAttribute(name) {
    return this.attributes.get(name) ?? null;
  }

  setAttribute(name, value) {
    this.attributes.set(name, String(value));
  }

  removeAttribute(name) {
    this.attributes.delete(name);
  }

  hasAttribute(name) {
    return this.attributes.has(name);
  }

  toggleAttribute(name, force) {
    const next = force === undefined ? !this.hasAttribute(name) : Boolean(force);
    if (next) this.attributes.set(name, "");
    else this.attributes.delete(name);
  }

  querySelectorAll(selector) {
    if (selector !== "[data-selected]") return [];
    return (this.children ?? []).filter((child) => child.hasAttribute("data-selected"));
  }
}

function createRuntimeHarness() {
  const listeners = new Map();
  const frame = new FakeElement({ "data-dx-chart-id": "chart-a" });
  const mark = new FakeElement({ "data-dx-chart-hit": "Revenue: 42", "aria-pressed": "false" });
  const secondMark = new FakeElement({ "data-dx-chart-hit": "Margin: 12", "aria-pressed": "false" });
  const tooltip = new FakeElement({ "data-dx-chart-tooltip": "" });
  mark.parent = frame;
  secondMark.parent = frame;
  frame.children = [mark, secondMark];
  const document = {
    querySelector(selector) {
      return selector === "[data-dx-chart-tooltip]" ? tooltip : null;
    },
    addEventListener(type, handler) {
      listeners.set(type, handler);
    },
  };

  return { document, frame, mark, secondMark, tooltip, listeners };
}

test("chart runtime clamps tooltips and toggles keyboard selection state", () => {
  const runtime = fs.readFileSync(runtimePath, "utf8");
  const harness = createRuntimeHarness();
  const context = {
    document: harness.document,
    window: { innerWidth: 80, innerHeight: 60 },
    Element: FakeElement,
  };

  vm.runInNewContext(runtime, context);

  harness.listeners.get("pointermove")({ target: harness.mark, clientX: 900, clientY: 700 });
  assert.equal(harness.tooltip.textContent, "Revenue: 42");
  assert.equal(harness.tooltip.getAttribute("data-visible"), "true");
  assert.equal(harness.tooltip.style.transform, "translate(68px, 48px)");

  harness.listeners.get("keydown")({ target: harness.mark, key: "Enter", preventDefault() {} });
  assert.equal(harness.mark.hasAttribute("data-selected"), true);
  assert.equal(harness.mark.getAttribute("aria-pressed"), "true");
  assert.equal(harness.frame.getAttribute("data-selected-count"), "1");

  harness.listeners.get("keydown")({ target: harness.mark, key: "Escape", preventDefault() {} });
  assert.equal(harness.mark.hasAttribute("data-selected"), false);
  assert.equal(harness.mark.getAttribute("aria-pressed"), "false");
  assert.equal(harness.frame.getAttribute("data-selected-count"), "0");
  assert.equal(harness.tooltip.textContent, "");
  assert.equal(harness.tooltip.getAttribute("data-visible"), null);
});

test("chart runtime covers click space focus and no-target tooltip behavior", () => {
  const runtime = fs.readFileSync(runtimePath, "utf8");
  const harness = createRuntimeHarness();
  const context = {
    document: harness.document,
    window: { innerWidth: 320, innerHeight: 240 },
    Element: FakeElement,
  };

  vm.runInNewContext(runtime, context);

  harness.listeners.get("click")({ target: harness.mark, clientX: 44, clientY: 28 });
  assert.equal(harness.mark.hasAttribute("data-selected"), true);
  assert.equal(harness.mark.getAttribute("aria-pressed"), "true");
  assert.equal(harness.frame.getAttribute("data-selected-count"), "1");
  assert.equal(harness.tooltip.textContent, "Revenue: 42");
  assert.equal(harness.tooltip.getAttribute("data-visible"), "true");
  assert.equal(harness.tooltip.style.transform, "translate(58px, 42px)");

  let prevented = false;
  harness.listeners.get("keydown")({
    target: harness.secondMark,
    key: " ",
    preventDefault() {
      prevented = true;
    },
  });
  assert.equal(prevented, true);
  assert.equal(harness.secondMark.hasAttribute("data-selected"), true);
  assert.equal(harness.secondMark.getAttribute("aria-pressed"), "true");
  assert.equal(harness.frame.getAttribute("data-selected-count"), "2");
  assert.equal(harness.tooltip.textContent, "Margin: 12");
  assert.equal(harness.tooltip.getAttribute("data-visible"), "true");
  assert.equal(harness.tooltip.style.transform, "translate(38px, 38px)");

  harness.listeners.get("focusin")({ target: harness.secondMark });
  assert.equal(harness.tooltip.textContent, "Margin: 12");
  assert.equal(harness.tooltip.getAttribute("data-visible"), "true");
  assert.equal(harness.tooltip.style.transform, "translate(38px, 38px)");

  harness.listeners.get("focusout")({ target: harness.secondMark });
  assert.equal(harness.tooltip.textContent, "");
  assert.equal(harness.tooltip.getAttribute("data-visible"), null);

  harness.listeners.get("pointermove")({ target: new FakeElement(), clientX: 30, clientY: 30 });
  assert.equal(harness.tooltip.textContent, "");
  assert.equal(harness.tooltip.getAttribute("data-visible"), null);
});

test("chart frame keeps labelled marks accessible and hit-testable", () => {
  const source = fs.readFileSync(framePath, "utf8");

  assert.match(source, /role: "label" in element && element\.label \? "button" : undefined/);
  assert.match(source, /"aria-pressed": "label" in element && element\.label \? "false" : undefined/);
  assert.match(source, /"data-dx-chart-hit": "label" in element \? element\.label : undefined/);
  assert.match(source, /tabIndex: "label" in element && element\.label \? 0 : undefined/);
});
