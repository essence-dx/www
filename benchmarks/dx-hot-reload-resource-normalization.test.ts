import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

function hotReloadClientJs() {
  const source = read("dx-www/src/cli/dev_hot_reload_client.rs");
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

function runHotReloadClient({ assetElements = [], stylesheetLinks = [] } = {}) {
  const eventSources = [];
  let reloadCount = 0;

  class EventSourceStub {
    constructor(url) {
      this.url = url;
      this.listeners = new Map();
      eventSources.push(this);
    }

    addEventListener(name, listener) {
      this.listeners.set(name, listener);
    }

    close() {}

    emit(name, payload) {
      const listener = this.listeners.get(name);
      assert.ok(listener, `expected listener for ${name}`);
      listener({ data: JSON.stringify(payload) });
    }
  }

  const document = {
    hidden: false,
    body: { appendChild() {} },
    addEventListener() {},
    createElement() {
      return {
        style: {},
        setAttribute() {},
        querySelector() {
          return { addEventListener() {}, style: {}, textContent: "" };
        },
      };
    },
    querySelector() {
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
    EventSource: EventSourceStub,
    fetch() {
      return Promise.resolve({ ok: false });
    },
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
    queueMicrotask(listener) {
      listener();
    },
    setInterval() {},
    addEventListener() {},
  };
  context.window = context;

  vm.runInNewContext(hotReloadClientJs(), context);
  assert.equal(eventSources.length, 1, "client should open one event stream");

  return {
    eventSource: eventSources[0],
    reloadCount: () => reloadCount,
  };
}

function reloadToken(value) {
  return new URL(value, "http://127.0.0.1:3000").searchParams.get("_dx_reload");
}

test("DX hot reload normalizes static resource ids without node_modules fallback", () => {
  const protocol = read("dx-www/src/hot_reload_protocol.rs");

  assert.match(protocol, /fn normalize_static_resource_id/);
  assert.match(protocol, /fn strip_static_resource_suffix/);
  assert.match(protocol, /strip_static_resource_suffix\(raw_path\)/);
  assert.match(protocol, /trim_start_matches\('\/'\)/);
  assert.match(protocol, /strip_prefix\("public\/"\)/);
  assert.match(protocol, /segment == "node_modules"/);
  assert.match(protocol, /segment == "\.\."/);
  assert.match(protocol, /static_resource_ids_strip_query_hash_and_public_prefixes/);
  assert.match(protocol, /"asset:logo\.svg\?v=1#icon"/);
  assert.match(protocol, /"asset:public\/logo\.svg"/);
  assert.match(protocol, /"style:styles\/app\.css\?v=1#sheet"/);
  assert.doesNotMatch(protocol, /turbopack-hmr|turbopack-subscribe|_next\/hmr/);
});

test("DX hot reload client tolerates raw static resource suffixes in SSE payloads", () => {
  const stylesheet = element({ href: "/styles/app.css?before=1" });
  const favicon = element({ href: "/logo.svg?before=1" });
  const { eventSource, reloadCount } = runHotReloadClient({
    assetElements: [favicon],
    stylesheetLinks: [stylesheet],
  });

  eventSource.emit("dx-hot-reload", {
    token: "2-token",
    resource: { id: "style:styles/app.css?mtime=1#sheet" },
    instruction: {
      type: "refresh-style",
      resource: { id: "style:styles/app.css?mtime=1#sheet" },
    },
  });

  assert.equal(reloadToken(stylesheet.value("href")), "2-token");
  assert.equal(reloadCount(), 0, "raw style id should not force a page reload");

  eventSource.emit("dx-hot-reload", {
    token: "3-token",
    resource: { id: "asset:public/logo.svg?mtime=1#icon" },
    instruction: {
      type: "refresh-asset",
      resource: { id: "asset:public/logo.svg?mtime=1#icon" },
    },
  });

  assert.equal(reloadToken(favicon.value("href")), "3-token");
  assert.equal(reloadCount(), 0, "raw asset id should not force a page reload");
});
