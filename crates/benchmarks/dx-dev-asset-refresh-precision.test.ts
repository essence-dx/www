import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const repoRoot = path.resolve(__dirname, "..");

function readRepo(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function hotReloadClientJs() {
  const source = readRepo("dx-www/src/cli/dev_hot_reload_client.rs");
  const match = source.match(
    /r#"<script type="module" data-dx-hot-reload>\r?\n([\s\S]*?)\r?\n<\/script>"#/,
  );
  assert.ok(match, "expected generated hot-reload client script in Rust source");

  let script = match[1]
    .replaceAll("{token}", '"1-token"')
    .replaceAll("{version_endpoint}", "/_dx/hot-reload/version")
    .replaceAll("{event_stream_endpoint}", "/_dx/hot-reload/events")
    .replaceAll("{event_stream_event}", "dx-hot-reload")
    .replaceAll("{resource_marker}", "data-dx-hot-reload-target")
    .replaceAll(
      "{resource_marker_selector}",
      "[data-dx-hot-reload-target]",
    );

  script = script.replaceAll("{{", "{").replaceAll("}}", "}");
  return script;
}

function element(attributes) {
  const state = { ...attributes };
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
}

function runHotReloadClient({
  assetElements = [],
  fetchPayloads = [],
  stylesheetLinks = [],
  pathname = "/dashboard",
} = {}) {
  const eventSources = [];
  const documentListeners = new Map();
  const intervalCallbacks = [];
  const windowListeners = new Map();
  const overlayFields = new Map();
  let overlayNode = null;
  let reloadCount = 0;

  class EventSourceStub {
    constructor(url) {
      this.url = url;
      this.listeners = new Map();
      this.closed = false;
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

  function createNode() {
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
    addEventListener(name, listener) {
      documentListeners.set(name, listener);
    },
    createElement() {
      return createNode();
    },
    querySelector(selector) {
      if (selector === "[data-dx-www-error-overlay]") return overlayNode;
      return null;
    },
    querySelectorAll(selector) {
      if (selector.includes('link[rel~="stylesheet"]')) return stylesheetLinks;
      if (selector.includes("img[src]")) return assetElements;
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
      href: `http://127.0.0.1:3000${pathname}`,
      pathname,
      reload() {
        reloadCount += 1;
      },
    },
    navigator: { onLine: true },
    EventSource: EventSourceStub,
    fetch() {
      assert.ok(fetchPayloads.length, "unexpected hot reload polling request");
      const payload = fetchPayloads.shift();
      return Promise.resolve({
        ok: true,
        json() {
          return Promise.resolve(payload);
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
    addEventListener(name, listener) {
      windowListeners.set(name, listener);
    },
  };
  context.window = context;

  vm.runInNewContext(hotReloadClientJs(), context);
  assert.equal(eventSources.length, 1, "client should open one event stream");
  return {
    eventSource: eventSources[0],
    eventSources,
    intervalCallbacks,
    documentListeners,
    windowListeners,
    reloadCount: () => reloadCount,
    overlayDisplay: () => overlayNode?.style.display || "",
    overlayMessage: () =>
      overlayFields.get("[data-dx-www-error-message]")?.textContent || "",
  };
}

function reloadToken(value) {
  return new URL(value, "http://127.0.0.1:3000").searchParams.get("_dx_reload");
}

test("DX dev asset refresh only cache-busts exact public asset matches", () => {
  const rootIcon = element({ href: "/logo.svg" });
  const publicIcon = element({ href: "/public/logo.svg" });
  const nestedImage = element({ src: "/images/logo.svg" });
  const unrelatedIcon = element({ href: "/icons/logo.svg" });
  const srcsetImage = element({
    srcset: "/images/logo.svg 1x, /images/logo@2x.svg 2x",
  });

  const { eventSource, reloadCount } = runHotReloadClient({
    assetElements: [
      rootIcon,
      publicIcon,
      nestedImage,
      unrelatedIcon,
      srcsetImage,
    ],
  });

  eventSource.emit("dx-hot-reload", {
    token: "2-token",
    resource: { id: "asset:logo.svg" },
    instruction: {
      type: "refresh-asset",
      resource: { id: "asset:logo.svg" },
    },
  });

  assert.equal(reloadToken(rootIcon.value("href")), "2-token");
  assert.equal(reloadToken(publicIcon.value("href")), "2-token");
  assert.equal(
    reloadToken(unrelatedIcon.value("href")),
    null,
    "same basename under another directory must not be refreshed",
  );
  assert.equal(
    reloadToken(nestedImage.value("src")),
    null,
    "nested asset should wait for its own asset id",
  );
  assert.equal(
    reloadToken(srcsetImage.value("srcset").split(/\s+/)[0]),
    null,
    "nested srcset asset should wait for its own asset id",
  );

  eventSource.emit("dx-hot-reload", {
    token: "3-token",
    resource: { id: "asset:images/logo.svg" },
    instruction: {
      type: "refresh-asset",
      resource: { id: "asset:images/logo.svg" },
    },
  });

  assert.equal(reloadToken(nestedImage.value("src")), "3-token");
  assert.equal(reloadToken(srcsetImage.value("srcset").split(/\s+/)[0]), "3-token");
  assert.equal(
    reloadToken(unrelatedIcon.value("href")),
    null,
    "unrelated icon remains untouched after nested asset refresh",
  );
  assert.equal(reloadCount(), 0, "matched asset refresh should not reload");

  eventSource.emit("dx-hot-reload", {
    token: "4-token",
    resource: { id: "asset:images/missing.svg" },
    instruction: {
      type: "refresh-asset",
      resource: { id: "asset:images/missing.svg" },
    },
  });

  assert.equal(
    reloadCount(),
    0,
    "missing asset refresh should not reload an unrelated page",
  );
});

test("DX dev stylesheet refresh only cache-busts exact stylesheet matches", () => {
  const appStylesheet = element({ href: "/styles/app.css" });
  const nestedStylesheet = element({ href: "/themes/styles/app.css" });
  const otherStylesheet = element({ href: "/styles/other.css" });

  const { eventSource, reloadCount } = runHotReloadClient({
    stylesheetLinks: [appStylesheet, nestedStylesheet, otherStylesheet],
  });

  eventSource.emit("dx-hot-reload", {
    token: "2-token",
    resource: { id: "style:styles/app.css" },
    instruction: {
      type: "refresh-style",
      resource: { id: "style:styles/app.css" },
    },
  });

  assert.equal(reloadToken(appStylesheet.value("href")), "2-token");
  assert.equal(
    reloadToken(nestedStylesheet.value("href")),
    null,
    "same stylesheet suffix under another directory must not be refreshed",
  );
  assert.equal(reloadToken(otherStylesheet.value("href")), null);
  assert.equal(reloadCount(), 0, "matched stylesheet refresh should not reload");

  eventSource.emit("dx-hot-reload", {
    token: "3-token",
    resource: { id: "style:styles/missing.css" },
    instruction: {
      type: "refresh-style",
      resource: { id: "style:styles/missing.css" },
    },
  });

  assert.equal(
    reloadToken(otherStylesheet.value("href")),
    null,
    "missing stylesheet should not cache-bust unrelated links",
  );
  assert.equal(
    reloadCount(),
    0,
    "missing stylesheet should not reload an unrelated page",
  );
});

test("DX dev route restarts only reload the matching route shape", () => {
  const { eventSource, reloadCount } = runHotReloadClient({
    pathname: "/dashboard/settings",
  });

  eventSource.emit("dx-hot-reload", {
    token: "2-token",
    resource: { id: "route:/dashboard" },
    instruction: {
      type: "restart",
      resource: { id: "route:/dashboard" },
    },
  });

  assert.equal(
    reloadCount(),
    0,
    "parent page route changes should not reload deeper sibling pages",
  );

  eventSource.emit("dx-hot-reload", {
    token: "3-token",
    resource: { id: "route:/dashboard/settings" },
    instruction: {
      type: "restart",
      resource: { id: "route:/dashboard/settings" },
    },
  });

  assert.equal(reloadCount(), 1, "exact route changes should reload the page");

  eventSource.emit("dx-hot-reload", {
    token: "4-token",
    resource: { id: "route:/dashboard/[tab]" },
    instruction: {
      type: "restart",
      resource: { id: "route:/dashboard/[tab]" },
    },
  });

  assert.equal(reloadCount(), 2, "dynamic route shape should reload matching depth");

  eventSource.emit("dx-hot-reload", {
    token: "5-token",
    resource: { id: "route:/dashboard/[tab]/details" },
    instruction: {
      type: "restart",
      resource: { id: "route:/dashboard/[tab]/details" },
    },
  });

  assert.equal(
    reloadCount(),
    2,
    "deeper dynamic route shape should not reload a shorter current route",
  );
});

test("DX dev diagnostics recovery clears the overlay without reloading the page", () => {
  const { eventSource, overlayDisplay, overlayMessage, reloadCount } =
    runHotReloadClient({
      pathname: "/dashboard",
    });

  eventSource.emit("dx-hot-reload", {
    token: "2-token",
    resource: { id: "route:/dashboard" },
    instruction: {
      type: "report-issue",
      resource: { id: "route:/dashboard" },
    },
    issues: [
      {
        message: "Unexpected token",
        file: "app/dashboard/page.tsx",
        line: 4,
        column: 12,
      },
    ],
  });

  assert.equal(overlayDisplay(), "flex");
  assert.equal(overlayMessage(), "Unexpected token");
  assert.equal(reloadCount(), 0, "issue reports should not reload the page");

  eventSource.emit("dx-hot-reload", {
    token: "3-token",
    resource: { id: "route:/dashboard" },
    instruction: {
      type: "clear-issue",
      resource: { id: "route:/dashboard" },
    },
    issues: [],
  });

  assert.equal(overlayDisplay(), "none");
  assert.equal(
    reloadCount(),
    0,
    "diagnostics recovery should clear the overlay without a full reload",
  );
});

test("DX dev initial SSE diagnostics replay the active route issue", () => {
  const { eventSource, overlayDisplay, overlayMessage, reloadCount } =
    runHotReloadClient({
      pathname: "/dashboard",
    });

  eventSource.emit("dx-hot-reload", {
    token: "2-token",
    event_stream: {
      initial: true,
    },
    resource: { id: "route:/dashboard" },
    instruction: {
      type: "report-issue",
      resource: { id: "route:/dashboard" },
    },
    issues: [
      {
        message: "Existing compile error",
        file: "app/dashboard/page.tsx",
        line: 4,
        column: 12,
      },
    ],
  });

  assert.equal(overlayDisplay(), "flex");
  assert.equal(overlayMessage(), "Existing compile error");
  assert.equal(reloadCount(), 0, "initial diagnostic replay must not reload");
});

test("DX dev issue stream only affects pages using the changed static resource", () => {
  const appStylesheet = element({ href: "/styles/app.css" });
  const appIcon = element({ href: "/logo.svg" });
  const { eventSource, overlayDisplay, overlayMessage, reloadCount } =
    runHotReloadClient({
      assetElements: [appIcon],
      pathname: "/dashboard",
      stylesheetLinks: [appStylesheet],
    });

  eventSource.emit("dx-hot-reload", {
    token: "2-token",
    resource: { id: "style:styles/other.css" },
    instruction: {
      type: "report-issue",
      resource: { id: "style:styles/other.css" },
    },
    issues: [{ message: "Other stylesheet failed" }],
  });

  assert.equal(overlayDisplay(), "");
  assert.equal(reloadCount(), 0, "unmatched stylesheet issues should not reload");

  eventSource.emit("dx-hot-reload", {
    token: "3-token",
    resource: { id: "style:styles/app.css" },
    instruction: {
      type: "report-issue",
      resource: { id: "style:styles/app.css" },
    },
    issues: [{ message: "App stylesheet failed" }],
  });

  assert.equal(overlayDisplay(), "flex");
  assert.equal(overlayMessage(), "App stylesheet failed");

  eventSource.emit("dx-hot-reload", {
    token: "4-token",
    resource: { id: "asset:icons/missing.svg" },
    instruction: {
      type: "clear-issue",
      resource: { id: "asset:icons/missing.svg" },
    },
    issues: [],
  });

  assert.equal(
    overlayDisplay(),
    "flex",
    "unmatched asset recovery should not clear the active overlay",
  );

  eventSource.emit("dx-hot-reload", {
    token: "5-token",
    resource: { id: "style:styles/app.css" },
    instruction: {
      type: "clear-issue",
      resource: { id: "style:styles/app.css" },
    },
    issues: [],
  });

  assert.equal(overlayDisplay(), "none");
});

test("DX dev polling recovery resubscribes an unhealthy event stream", async () => {
  const { eventSource, eventSources, intervalCallbacks, reloadCount } =
    runHotReloadClient({
      fetchPayloads: [
        {
          token: "2-token",
          resource: { id: "route:/dashboard" },
          instruction: {
            type: "clear-issue",
            resource: { id: "route:/dashboard" },
          },
          issues: [],
        },
      ],
      pathname: "/dashboard",
    });

  eventSource.onerror();
  assert.equal(eventSources.length, 1, "SSE error alone should not duplicate streams");
  assert.equal(eventSource.closed, false, "errored stream is closed when recovery starts");

  await intervalCallbacks[0]();

  assert.equal(reloadCount(), 0, "poll recovery should not reload");
  assert.equal(eventSource.closed, true, "poll recovery should close the unhealthy stream");
  assert.equal(eventSources.length, 2, "poll recovery should reopen SSE after fallback works");
  assert.match(
    eventSources[1].url,
    /\/_dx\/hot-reload\/events\?resource=route%3A%2Fdashboard$/,
  );
});
