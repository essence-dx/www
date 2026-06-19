import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const repoRoot = path.resolve(__dirname, "..");

function hotReloadClientJs() {
  const source = fs.readFileSync(
    path.join(repoRoot, "dx-www/src/cli/dev_hot_reload_client.rs"),
    "utf8",
  );
  const match = source.match(
    /r#"<script type="module" data-dx-hot-reload>\r?\n([\s\S]*?)\r?\n<\/script>"#/,
  );
  assert.ok(match, "expected generated hot-reload client script in Rust source");

  return match[1]
    .replaceAll("{token}", '"1-token"')
    .replaceAll("{version_endpoint}", "/_dx/hot-reload/version")
    .replaceAll("{event_stream_endpoint}", "/_dx/hot-reload/events")
    .replaceAll("{event_stream_event}", "dx-hot-reload")
    .replaceAll("{resource_marker}", "data-dx-hot-reload-target")
    .replaceAll("{resource_marker_selector}", "[data-dx-hot-reload-target]")
    .replaceAll("{{", "{")
    .replaceAll("}}", "}");
}

test("DX dev hot reload reports malformed SSE frames and falls back to polling", async () => {
  const eventSources = [];
  const intervalCallbacks = [];
  const overlayFields = new Map();
  let overlayNode = null;
  let fetchCount = 0;

  class EventSourceStub {
    constructor(url) {
      this.url = url;
      this.closed = false;
      this.listeners = new Map();
      eventSources.push(this);
    }

    addEventListener(name, listener) {
      this.listeners.set(name, listener);
    }

    close() {
      this.closed = true;
    }

    emitRaw(name, data) {
      const listener = this.listeners.get(name);
      assert.ok(listener, `expected listener for ${name}`);
      listener({ data });
    }
  }

  function overlayField(selector) {
    if (!overlayFields.has(selector)) {
      overlayFields.set(selector, {
        addEventListener() {},
        style: {},
        textContent: "",
      });
    }
    return overlayFields.get(selector);
  }

  const document = {
    hidden: false,
    body: {
      appendChild(node) {
        if (node.getAttribute?.("data-dx-www-error-overlay") === "true") {
          overlayNode = node;
        }
      },
    },
    addEventListener() {},
    createElement() {
      const attributes = {};
      return {
        style: {},
        innerHTML: "",
        setAttribute(name, value) {
          attributes[name] = value;
        },
        getAttribute(name) {
          return attributes[name] || null;
        },
        querySelector(selector) {
          return overlayField(selector);
        },
      };
    },
    querySelector(selector) {
      if (selector === "[data-dx-www-error-overlay]") return overlayNode;
      return null;
    },
    querySelectorAll() {
      return [];
    },
  };

  const context = {
    URL,
    console,
    document,
    history: {
      pushState() {},
      replaceState() {},
    },
    location: {
      href: "http://127.0.0.1:3000/dashboard",
      pathname: "/dashboard",
      reload() {
        throw new Error("malformed SSE recovery must not reload");
      },
    },
    navigator: { onLine: true },
    EventSource: EventSourceStub,
    fetch() {
      fetchCount += 1;
      return Promise.resolve({
        ok: true,
        json() {
          return Promise.resolve({
            token: "2-token",
            resource: { id: "route:/dashboard" },
            instruction: { type: "disabled" },
          });
        },
      });
    },
    queueMicrotask(listener) {
      listener();
    },
    setInterval(listener) {
      intervalCallbacks.push(listener);
      return intervalCallbacks.length;
    },
    addEventListener() {},
  };
  context.window = context;

  vm.runInNewContext(hotReloadClientJs(), context);
  assert.equal(eventSources.length, 1, "client should open one event stream");
  eventSources[0].onopen();

  eventSources[0].emitRaw("dx-hot-reload", "{broken-json");

  assert.equal(overlayNode?.style.display, "flex");
  assert.match(
    overlayFields.get("[data-dx-www-error-message]")?.textContent || "",
    /invalid event stream payload/i,
  );

  await intervalCallbacks[0]();

  assert.equal(fetchCount, 1, "polling fallback should run after malformed SSE");
  assert.equal(eventSources[0].closed, true);
  assert.equal(eventSources.length, 2, "client should reopen SSE after fallback");
  assert.equal(
    overlayNode?.style.display,
    "none",
    "valid polling recovery should clear the stale transport overlay",
  );
});

test("DX dev hot reload recovery does not hide a newer runtime overlay", async () => {
  const eventSources = [];
  const intervalCallbacks = [];
  const overlayFields = new Map();
  let overlayNode = null;

  class EventSourceStub {
    constructor() {
      this.closed = false;
      this.listeners = new Map();
      eventSources.push(this);
    }

    addEventListener(name, listener) {
      this.listeners.set(name, listener);
    }

    close() {
      this.closed = true;
    }

    emitRaw(name, data) {
      const listener = this.listeners.get(name);
      assert.ok(listener, `expected listener for ${name}`);
      listener({ data });
    }
  }

  function overlayField(selector) {
    if (!overlayFields.has(selector)) {
      overlayFields.set(selector, {
        addEventListener() {},
        style: {},
        textContent: "",
      });
    }
    return overlayFields.get(selector);
  }

  const document = {
    hidden: false,
    body: {
      appendChild(node) {
        if (node.getAttribute?.("data-dx-www-error-overlay") === "true") {
          overlayNode = node;
        }
      },
    },
    addEventListener() {},
    createElement() {
      const attributes = {};
      return {
        style: {},
        innerHTML: "",
        setAttribute(name, value) {
          attributes[name] = value;
        },
        getAttribute(name) {
          return attributes[name] || null;
        },
        querySelector(selector) {
          return overlayField(selector);
        },
      };
    },
    querySelector(selector) {
      if (selector === "[data-dx-www-error-overlay]") return overlayNode;
      return null;
    },
    querySelectorAll() {
      return [];
    },
  };

  const context = {
    URL,
    console,
    document,
    history: {
      pushState() {},
      replaceState() {},
    },
    location: {
      href: "http://127.0.0.1:3000/dashboard",
      pathname: "/dashboard",
      reload() {
        throw new Error("polling recovery must not reload");
      },
    },
    navigator: { onLine: true },
    EventSource: EventSourceStub,
    fetch() {
      return Promise.resolve({
        ok: true,
        json() {
          return Promise.resolve({
            token: "2-token",
            resource: { id: "route:/dashboard" },
            instruction: { type: "disabled" },
          });
        },
      });
    },
    queueMicrotask(listener) {
      listener();
    },
    setInterval(listener) {
      intervalCallbacks.push(listener);
      return intervalCallbacks.length;
    },
    addEventListener() {},
  };
  context.window = context;

  vm.runInNewContext(hotReloadClientJs(), context);
  eventSources[0].onopen();
  eventSources[0].emitRaw("dx-hot-reload", "{broken-json");
  context.window.__DX_SHOW_ERROR__({
    message: "Runtime exploded",
    codeFrame: "app/page.tsx:1:1",
  });

  await intervalCallbacks[0]();

  assert.equal(overlayNode?.style.display, "flex");
  assert.equal(
    overlayFields.get("[data-dx-www-error-message]")?.textContent,
    "Runtime exploded",
    "polling recovery must not hide a newer non-transport overlay",
  );
});

test("DX dev stylesheet refresh preserves unrelated runtime overlays", () => {
  const eventSources = [];
  const overlayFields = new Map();
  let overlayNode = null;
  let reloadCount = 0;
  const stylesheet = (() => {
    const state = { href: "/styles/app.css" };
    return {
      getAttribute(name) {
        return state[name] || null;
      },
      setAttribute(name, value) {
        state[name] = value;
      },
      value(name) {
        return state[name] || "";
      },
    };
  })();

  class EventSourceStub {
    constructor() {
      this.closed = false;
      this.listeners = new Map();
      eventSources.push(this);
    }

    addEventListener(name, listener) {
      this.listeners.set(name, listener);
    }

    close() {
      this.closed = true;
    }

    emit(name, payload) {
      const listener = this.listeners.get(name);
      assert.ok(listener, `expected listener for ${name}`);
      listener({ data: JSON.stringify(payload) });
    }
  }

  function overlayField(selector) {
    if (!overlayFields.has(selector)) {
      overlayFields.set(selector, {
        addEventListener() {},
        style: {},
        textContent: "",
      });
    }
    return overlayFields.get(selector);
  }

  const document = {
    hidden: false,
    body: {
      appendChild(node) {
        if (node.getAttribute?.("data-dx-www-error-overlay") === "true") {
          overlayNode = node;
        }
      },
    },
    addEventListener() {},
    createElement() {
      const attributes = {};
      return {
        style: {},
        innerHTML: "",
        setAttribute(name, value) {
          attributes[name] = value;
        },
        getAttribute(name) {
          return attributes[name] || null;
        },
        querySelector(selector) {
          return overlayField(selector);
        },
      };
    },
    querySelector(selector) {
      if (selector === "[data-dx-www-error-overlay]") return overlayNode;
      return null;
    },
    querySelectorAll(selector) {
      if (selector.includes('link[rel~="stylesheet"]')) return [stylesheet];
      return [];
    },
  };

  const context = {
    URL,
    console,
    document,
    history: {
      pushState() {},
      replaceState() {},
    },
    location: {
      href: "http://127.0.0.1:3000/dashboard",
      pathname: "/dashboard",
      reload() {
        reloadCount += 1;
      },
    },
    navigator: { onLine: true },
    EventSource: EventSourceStub,
    fetch() {
      throw new Error("stylesheet refresh should not poll");
    },
    queueMicrotask(listener) {
      listener();
    },
    setInterval() {
      return 1;
    },
    addEventListener() {},
  };
  context.window = context;

  vm.runInNewContext(hotReloadClientJs(), context);
  context.window.__DX_SHOW_ERROR__({
    message: "Runtime exploded",
    codeFrame: "app/page.tsx:1:1",
  });

  eventSources[0].emit("dx-hot-reload", {
    token: "2-token",
    resource: { id: "style:styles/app.css" },
    instruction: {
      type: "refresh-style",
      resource: { id: "style:styles/app.css" },
    },
  });

  assert.match(stylesheet.value("href"), /[?&]_dx_reload=2-token/);
  assert.equal(reloadCount, 0, "stylesheet hot-swap should not reload");
  assert.equal(overlayNode?.style.display, "flex");
  assert.equal(
    overlayFields.get("[data-dx-www-error-message]")?.textContent,
    "Runtime exploded",
    "stylesheet hot-swap must not hide an unrelated runtime overlay",
  );
});
