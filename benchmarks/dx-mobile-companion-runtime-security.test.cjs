const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const root = path.resolve(__dirname, "..");
const runtimePath = path.join(root, "dx-mobile-app", "public", "mobile-companion-runtime.js");

function loadRuntime(overrides = {}) {
  const context = {
    URL,
    URLSearchParams,
    fetch: overrides.fetch,
    FormData: overrides.FormData,
    WebSocket: overrides.WebSocket,
    crypto: {
      randomUUID: () => "session-test",
    },
    document: {
      readyState: overrides.readyState || "loading",
      addEventListener: overrides.addDocumentEventListener || (() => {}),
      createElement: overrides.createElement || (() => null),
      getElementById: overrides.getElementById || (() => null),
      querySelector(selector) {
        if (overrides.querySelector) return overrides.querySelector(selector);
        return null;
      },
    },
    sessionStorage: {
      getItem() {
        return null;
      },
      setItem() {},
      removeItem() {},
    },
    window: overrides.window || {},
  };

  vm.createContext(context);
  vm.runInContext(fs.readFileSync(runtimePath, "utf8"), context, { filename: runtimePath });
  return context.window.DXMobileCompanionRuntime;
}

class RuntimeFormData {
  constructor(source) {
    this.source = source;
  }

  get(name) {
    return this.source.fields[name] || "";
  }
}

test("DX Mobile runtime rejects decoded token-bearing JSON pairing payloads", async () => {
  const runtime = loadRuntime();

  await assert.rejects(
    runtime.parsePairingPayload(
      String.raw`{"gateway":"https://desktop.example","code":"123456","tok\u0065n":"secret"}`,
    ),
    /Rejecting QR payload that contains a bearer token/,
  );
});

test("DX Mobile runtime rejects decoded token-bearing URL pairing payloads", async () => {
  const runtime = loadRuntime();

  await assert.rejects(
    runtime.parsePairingPayload("dxmobile://pair?gateway=https://desktop.example&code=123456&%74oken=secret"),
    /Rejecting QR payload that contains a bearer token/,
  );
  await assert.rejects(
    runtime.parsePairingPayload("dxmobile://pair?gateway=https://desktop.example&code=123456&auth=Bearer%20secret"),
    /Rejecting QR payload that contains a bearer token/,
  );
});

test("DX Mobile runtime rejects credential-shaped QR pairing fields", async () => {
  const runtime = loadRuntime();
  const credentialFields = [
    "access-token",
    "refresh_token",
    "refresh-token",
    "id_token",
    "id-token",
    "client_secret",
    "client-secret",
    "api_key",
    "api-key",
    "x_api_key",
    "x-api-key",
  ];

  for (const field of credentialFields) {
    await assert.rejects(
      runtime.parsePairingPayload(
        `dxmobile://pair?gateway=https://desktop.example&code=123456&${field}=secret`,
      ),
      /Rejecting QR payload that contains a bearer token/,
      `expected URL QR field ${field} to be rejected`,
    );

    await assert.rejects(
      runtime.parsePairingPayload(
        JSON.stringify({
          gateway: "https://desktop.example",
          code: "123456",
          [field]: "secret",
        }),
      ),
      /Rejecting QR payload that contains a bearer token/,
      `expected JSON QR field ${field} to be rejected`,
    );
  }
});

test("DX Mobile runtime validates QR credentials before accepting native previews", async () => {
  const runtime = loadRuntime({
    window: {
      __TAURI__: {
        core: {
          async invoke(command) {
            assert.equal(command, "dx_mobile_parse_pairing_payload");
            return {
              available: true,
              gateway_origin: "https://desktop.example/gw",
              pairing_code: "123456",
            };
          },
        },
      },
    },
  });

  await assert.rejects(
    runtime.parsePairingPayload(
      "dxmobile://pair?gateway=https://desktop.example&code=123456&%63lient_secret=secret",
    ),
    /Rejecting QR payload that contains a bearer token/,
  );
});

test("DX Mobile runtime validates QR alias ambiguity before accepting native previews", async () => {
  const runtime = loadRuntime({
    window: {
      __TAURI__: {
        core: {
          async invoke(command) {
            assert.equal(command, "dx_mobile_parse_pairing_payload");
            return {
              available: true,
              gateway_origin: "https://desktop.example/gw",
              pairing_code: "123456",
            };
          },
        },
      },
    },
  });

  await assert.rejects(
    runtime.parsePairingPayload("dxmobile://pair?gateway=https://desktop.example/gw&code=123456&code=654321"),
    /Pairing payload code fields must not repeat/,
  );
  await assert.rejects(
    runtime.parsePairingPayload(
      "dxmobile://pair?gateway=https://desktop.example/gw&origin=https://desktop.example/other&code=123456",
    ),
    /Pairing payload gateway fields must not conflict/,
  );
});

test("DX Mobile runtime normalizes native pairing preview gateways", async () => {
  const runtime = loadRuntime({
    window: {
      __TAURI__: {
        core: {
          async invoke(command) {
            assert.equal(command, "dx_mobile_parse_pairing_payload");
            return {
              available: true,
              gateway_origin: "https://desktop.example/gw/?ignored=true#fragment",
              pairing_code: "123456",
              session_id: "session-1",
            };
          },
        },
      },
    },
  });

  const preview = await runtime.parsePairingPayload("dxmobile-native://opaque-pairing-preview");

  assert.equal(preview.gateway, "https://desktop.example/gw");
  assert.equal(preview.code, "123456");
  assert.equal(preview.sessionId, "session-1");
});

test("DX Mobile runtime rejects gateway origins with embedded credentials", () => {
  const runtime = loadRuntime();

  assert.throws(
    () => runtime.normalizeGatewayBaseUrl("https://user:pass@desktop.example/gw"),
    /Gateway base URL must not include credentials/,
  );
});

test("DX Mobile runtime sanitizes pairing failure details before status display", async () => {
  const listeners = new Map();
  const status = {
    dataset: {},
    textContent: "",
  };
  const gatewayInput = { value: "" };
  const agentInput = { value: "" };
  const form = {
    fields: {
      pairing_payload: "",
      gateway_origin: "https://desktop.example",
      pairing_code: "123456",
      agent_alias: "friday",
    },
    addEventListener(type, listener) {
      listeners.set(type, listener);
    },
  };

  loadRuntime({
    readyState: "complete",
    FormData: RuntimeFormData,
    fetch: async () => ({
      ok: false,
      status: 401,
      statusText: "Unauthorized",
      text: async () => "Bearer desktop-secret access_token=abc123 session_key=gw_private pairing_code=123456",
    }),
    getElementById(id) {
      return id === "dx-mobile-runtime-status" ? status : null;
    },
    querySelector(selector) {
      if (selector === ".dx-mobile-pairing-form") return form;
      if (selector === "input[name='gateway_origin']") return gatewayInput;
      if (selector === "input[name='agent_alias']") return agentInput;
      return null;
    },
  });

  listeners.get("submit")({
    preventDefault() {},
    currentTarget: form,
  });
  await new Promise((resolve) => setImmediate(resolve));

  assert.match(status.textContent, /Pairing failed \(401\)/);
  assert.doesNotMatch(status.textContent, /desktop-secret|access_token|abc123|session_key|gw_private|123456/i);
});

test("DX Mobile runtime sanitizes native scanner errors before status display", async () => {
  const listeners = new Map();
  const status = {
    dataset: {},
    textContent: "",
  };
  const scanButton = {
    dataset: {},
    disabled: false,
    setAttribute() {},
    addEventListener(type, listener) {
      listeners.set(type, listener);
    },
  };

  loadRuntime({
    readyState: "complete",
    window: {
      __TAURI__: {
        barcodeScanner: {
          async scan() {
            throw new Error(
              'Bearer scanner-secret bearer=field-secret bearer:colon-secret token=abc123 refresh_token=refresh-secret id_token=id-secret client_secret=client-secret api-key=api-secret x-api-key=x-secret session_key=gw_private session-key=gw-hyphen pairing_code=123456 pairing-code=654321 {"access_token":"json-secret","authorization":"Bearer native-secret"}',
            );
          },
        },
      },
    },
    getElementById(id) {
      return id === "dx-mobile-runtime-status" ? status : null;
    },
    querySelector(selector) {
      if (selector === "[data-dx-native-command='dx_mobile_scan_pairing_qr']") return scanButton;
      return null;
    },
  });

  await listeners.get("click")();

  assert.match(status.textContent, /\[redacted\]/);
  assert.doesNotMatch(
    status.textContent,
    /scanner-secret|bearer=|bearer:|field-secret|colon-secret|token=abc123|refresh-secret|id-secret|client-secret|api-secret|x-secret|session_key=gw_private|session-key=gw-hyphen|pairing_code=123456|pairing-code=654321|json-secret|native-secret/i,
  );
});

test("DX Mobile runtime preserves ordinary bearer-token guidance in status display", async () => {
  const listeners = new Map();
  const status = {
    dataset: {},
    textContent: "",
  };
  const gatewayInput = { value: "" };
  const agentInput = { value: "" };
  const form = {
    fields: {
      pairing_payload: "",
      gateway_origin: "https://desktop.example",
      pairing_code: "123456",
      agent_alias: "friday",
    },
    addEventListener(type, listener) {
      listeners.set(type, listener);
    },
  };

  loadRuntime({
    readyState: "complete",
    FormData: RuntimeFormData,
    fetch: async () => ({
      ok: true,
      json: async () => ({}),
    }),
    getElementById(id) {
      return id === "dx-mobile-runtime-status" ? status : null;
    },
    querySelector(selector) {
      if (selector === ".dx-mobile-pairing-form") return form;
      if (selector === "input[name='gateway_origin']") return gatewayInput;
      if (selector === "input[name='agent_alias']") return agentInput;
      return null;
    },
  });

  listeners.get("submit")({
    preventDefault() {},
    currentTarget: form,
  });
  await new Promise((resolve) => setImmediate(resolve));

  assert.match(status.textContent, /bearer token/);
  assert.doesNotMatch(status.textContent, /\[redacted\]/);
});

test("DX Mobile runtime keeps concrete session identifiers out of rendered chat rows", async () => {
  const list = createTestElement("section");
  const fetches = [];
  const runtime = loadRuntime({
    fetch: async (url) => {
      fetches.push(String(url));
      if (String(url).includes("/api/sessions/")) {
        return {
          ok: true,
          json: async () => ({ messages: [] }),
        };
      }

      return {
        ok: true,
        json: async () => ({
          sessions: [
            {
              session_id: "desktop-session",
              session_key: "gw_private_session_key",
              title: "Friday planning",
            },
            {
              session_key: "gw_key_only_secret",
              title: "Key-only planning",
            },
          ],
        }),
      };
    },
    window: {
      __TAURI__: {
        core: {
          invoke: async () => ({
            credential: {
              gateway_origin: "https://desktop.example",
              token: "credential-token",
            },
          }),
        },
      },
    },
    querySelector(selector) {
      if (selector === ".dx-mobile-chat-list") return list;
      return null;
    },
    createElement: createTestElement,
  });

  await runtime.loadSessions();

  const row = list.children[0];
  assert.ok(row, "expected a rendered session row");
  assert.equal(row.dataset.dxSessionKey, undefined);
  assert.equal(row.dataset.dxSessionId, undefined);
  assert.equal(row.querySelector("small").textContent, "Secure session");
  assert.doesNotMatch(publicElementSnapshot(row), /desktop-session|gw_private_session_key/);
  assert.doesNotMatch(JSON.stringify(row.dataset), /session_key|desktop-session|gw_private_session_key/);

  const keyOnlyRow = list.children[1];
  assert.ok(keyOnlyRow, "expected a rendered key-only session row");
  assert.equal(keyOnlyRow.dataset.dxSessionId, undefined);
  assert.equal(keyOnlyRow.dataset.dxSessionKey, undefined);
  assert.equal(keyOnlyRow.querySelector("small").textContent, "Secure session");
  assert.doesNotMatch(publicElementSnapshot(keyOnlyRow), /gw_key_only_secret/);
  assert.doesNotMatch(JSON.stringify(keyOnlyRow.dataset), /session_key|gw_key_only_secret/);

  await row.listeners.get("click")();
  await keyOnlyRow.listeners.get("click")();

  const messageFetches = fetches.filter((url) => url.includes("/api/sessions/"));
  assert.equal(messageFetches.length, 3);
  assert.match(messageFetches[1], /gw_private_session_key/);
  assert.match(messageFetches[2], /gw_key_only_secret/);
});

test("DX Mobile runtime renders source-aware Code and CLI chat sessions", async () => {
  const list = createTestElement("section");
  const transcript = createTestElement("section");
  const runtime = loadRuntime({
    fetch: async (url) => {
      const requestUrl = String(url);
      if (requestUrl.includes("/api/sessions/")) {
        return {
          ok: true,
          json: async () => ({
            messages: [
              {
                role: "assistant",
                content: "Ready from the VPS agent.",
                source: "vps_cli",
                status: "delivered",
              },
            ],
          }),
        };
      }

      return {
        ok: true,
        json: async () => ({
          sessions: [
            {
              session_id: "zed-session-secret",
              session_key: "gw_zed_secret",
              title: "Desktop planning",
              source: "zed_editor",
              agent: "friday",
              status: "active",
            },
            {
              session_id: "cli-session-secret",
              session_key: "gw_cli_secret",
              title: "Server deploy",
              source: "vps_cli",
              agent: "dx",
              status: "idle",
            },
          ],
        }),
      };
    },
    window: {
      __TAURI__: {
        core: {
          invoke: async () => ({
            credential: {
              gateway_origin: "https://desktop.example",
              token: "credential-token",
            },
          }),
        },
      },
    },
    querySelector(selector) {
      if (selector === ".dx-mobile-chat-list") return list;
      if (selector === ".dx-mobile-transcript") return transcript;
      return null;
    },
    createElement: createTestElement,
  });

  await runtime.loadSessions();

  assert.equal(list.children[0].querySelector("strong").textContent, "Desktop planning");
  assert.equal(list.children[0].querySelector("small").textContent, "Zed editor - active - friday");
  assert.equal(list.children[1].querySelector("strong").textContent, "Server deploy");
  assert.equal(list.children[1].querySelector("small").textContent, "VPS CLI - idle - dx");
  assert.doesNotMatch(publicElementSnapshot(list), /zed-session-secret|gw_zed_secret|cli-session-secret|gw_cli_secret/);
  assert.match(publicElementSnapshot(transcript), /assistant/);
  assert.match(publicElementSnapshot(transcript), /VPS CLI/);
  assert.match(publicElementSnapshot(transcript), /Ready from the VPS agent/);
});

test("DX Mobile runtime rejects unsafe session message paths before fetching", async () => {
  const fetches = [];
  const runtime = loadRuntime({
    fetch: async (url) => {
      fetches.push(String(url));
      return {
        ok: true,
        json: async () => ({ messages: [] }),
      };
    },
    window: {
      __TAURI__: {
        core: {
          invoke: async () => ({
            credential: {
              gateway_origin: "https://desktop.example/gw",
              token: "credential-token",
            },
          }),
        },
      },
    },
  });

  await assert.rejects(
    () => runtime.loadMessages("gw_private\nkey", "session-1"),
    /Session key must not contain whitespace or control characters/,
  );
  await assert.rejects(
    () => runtime.loadMessages("gw_private_key", "session 1"),
    /Session id must not contain whitespace or control characters/,
  );
  assert.deepEqual(fetches, []);
});

test("DX Mobile runtime clears paired credentials when native secure storage rejects", async () => {
  const listeners = new Map();
  const status = {
    dataset: {},
    textContent: "",
  };
  const gatewayInput = { value: "" };
  const agentInput = { value: "" };
  const form = {
    fields: {
      pairing_payload: "",
      gateway_origin: "https://desktop.example",
      pairing_code: "123456",
      agent_alias: "friday",
    },
    addEventListener(type, listener) {
      listeners.set(type, listener);
    },
  };
  const sessionRequests = [];

  const runtime = loadRuntime({
    readyState: "complete",
    FormData: RuntimeFormData,
    fetch: async (url, init = {}) => {
      const requestUrl = String(url);
      if (requestUrl.includes("/api/pair")) {
        return {
          ok: true,
          json: async () => ({
            token: "paired-secret-token",
            device_id: "dx-mobile-test",
          }),
        };
      }
      if (requestUrl.includes("/api/sessions")) {
        sessionRequests.push({ url: requestUrl, authorization: init.headers?.Authorization || "" });
        return {
          ok: true,
          json: async () => ({ sessions: [] }),
        };
      }

      throw new Error(`Unexpected fetch: ${requestUrl}`);
    },
    window: {
      __TAURI__: {
        core: {
          invoke: async (command) => {
            if (command === "dx_mobile_store_device_credential") throw new Error("native store rejected");
            if (command === "dx_mobile_load_device_credential") return {};
            return {};
          },
        },
      },
    },
    getElementById(id) {
      return id === "dx-mobile-runtime-status" ? status : null;
    },
    querySelector(selector) {
      if (selector === ".dx-mobile-pairing-form") return form;
      if (selector === ".dx-mobile-chat-list") return createTestElement("section");
      if (selector === "input[name='gateway_origin']") return gatewayInput;
      if (selector === "input[name='agent_alias']") return agentInput;
      return null;
    },
    createElement: createTestElement,
  });

  listeners.get("submit")({
    preventDefault() {},
    currentTarget: form,
  });
  await new Promise((resolve) => setImmediate(resolve));

  assert.match(status.textContent, /native store rejected/);

  await runtime.loadSessions();

  assert.deepEqual(sessionRequests, []);
});

test("DX Mobile runtime normalizes bearer tokens before storage and transports", async () => {
  const listeners = new Map();
  const storeRequests = [];
  const capabilityRequests = [];
  const sessionRequests = [];
  const socketCalls = [];
  const status = {
    dataset: {},
    textContent: "",
  };
  const gatewayInput = { value: "" };
  const pairingCodeInput = { value: "" };
  const agentInput = { value: "" };
  const form = {
    fields: {
      pairing_payload: "https://desktop.example/gw/?code=123456&session_id=session-1&agent=friday",
      gateway_origin: "",
      pairing_code: "",
      agent_alias: "",
    },
    addEventListener(type, listener) {
      listeners.set(type, listener);
    },
  };

  class CapturedWebSocket {
    constructor(url, protocols) {
      socketCalls.push({ url: String(url), protocols: Array.from(protocols || []) });
    }

    addEventListener() {}
    close() {}
  }

  loadRuntime({
    readyState: "complete",
    FormData: RuntimeFormData,
    WebSocket: CapturedWebSocket,
    fetch: async (url, init = {}) => {
      const requestUrl = String(url);
      if (requestUrl.includes("/api/pair")) {
        return {
          ok: true,
          json: async () => ({
            token: " \tBearer paired-secret-token\n ",
            device_id: "dx-mobile-test",
          }),
        };
      }
      if (requestUrl.includes("/api/devices/me/capabilities")) {
        capabilityRequests.push({ url: requestUrl, authorization: init.headers?.Authorization || "" });
        return {
          ok: true,
          json: async () => ({}),
        };
      }
      if (requestUrl.includes("/api/sessions")) {
        sessionRequests.push({ url: requestUrl, authorization: init.headers?.Authorization || "" });
        return {
          ok: true,
          json: async () => ({ sessions: [] }),
        };
      }

      throw new Error(`Unexpected fetch: ${requestUrl}`);
    },
    window: {
      __TAURI__: {
        core: {
          invoke: async (command, payload) => {
            if (command === "dx_mobile_store_device_credential") {
              storeRequests.push(payload);
              return { stored: false };
            }
            if (command === "dx_mobile_load_device_credential") return {};
            return {};
          },
        },
      },
    },
    getElementById(id) {
      return id === "dx-mobile-runtime-status" ? status : null;
    },
    querySelector(selector) {
      if (selector === ".dx-mobile-pairing-form") return form;
      if (selector === ".dx-mobile-chat-list") return createTestElement("section");
      if (selector === "input[name='gateway_origin']") return gatewayInput;
      if (selector === "input[name='pairing_code']") return pairingCodeInput;
      if (selector === "input[name='agent_alias']") return agentInput;
      return null;
    },
    createElement: createTestElement,
  });

  listeners.get("submit")({
    preventDefault() {},
    currentTarget: form,
  });
  await new Promise((resolve) => setImmediate(resolve));

  assert.equal(storeRequests[0].credential.token, "paired-secret-token");
  assert.equal(capabilityRequests[0].authorization, "Bearer paired-secret-token");
  assert.equal(sessionRequests[0].authorization, "Bearer paired-secret-token");
  assert.deepEqual(socketCalls[0].protocols, ["dx-agents.v1", "zeroclaw.v1", "bearer.paired-secret-token"]);
  assert.doesNotMatch(JSON.stringify(storeRequests), /\s+paired-secret-token|paired-secret-token\s+/);
});

test("DX Mobile runtime normalizes loaded native bearer tokens before authenticated fetches", async () => {
  const sessionRequests = [];
  const runtime = loadRuntime({
    fetch: async (url, init = {}) => {
      sessionRequests.push({ url: String(url), authorization: init.headers?.Authorization || "" });
      return {
        ok: true,
        json: async () => ({ sessions: [] }),
      };
    },
    window: {
      __TAURI__: {
        core: {
          invoke: async (command) => {
            if (command === "dx_mobile_load_device_credential") {
              return {
                credential: {
                  gateway_origin: "https://desktop.example/gw",
                  token: " \tBearer stored-secret-token\n ",
                  device_id: "dx-mobile-test",
                  paired_at: "2026-06-11T00:00:00Z",
                },
              };
            }
            return {};
          },
        },
      },
    },
  });

  await runtime.loadSessions();

  assert.deepEqual(sessionRequests, [
    {
      url: "https://desktop.example/gw/api/sessions",
      authorization: "Bearer stored-secret-token",
    },
  ]);
});

test("DX Mobile runtime rejects malformed loaded native bearer tokens before authenticated fetches", async () => {
  for (const malformedToken of [
    "stored secret-token",
    "stored\tsecret-token",
    "stored\nsecret-token",
    "stored\rsecret-token",
    "stored\u0000secret-token",
    "stored\u007fsecret-token",
    "Bearer Bearer stored-secret-token",
  ]) {
    const sessionRequests = [];
    const runtime = loadRuntime({
      fetch: async (url, init = {}) => {
        sessionRequests.push({ url: String(url), authorization: init.headers?.Authorization || "" });
        return {
          ok: true,
          json: async () => ({ sessions: [] }),
        };
      },
      window: {
        __TAURI__: {
          core: {
            invoke: async (command) => {
              if (command === "dx_mobile_load_device_credential") {
                return {
                  credential: {
                    gateway_origin: "https://desktop.example/gw",
                    token: malformedToken,
                    device_id: "dx-mobile-test",
                    paired_at: "2026-06-11T00:00:00Z",
                  },
                };
              }
              return {};
            },
          },
        },
      },
    });

    await assert.rejects(
      () => runtime.loadSessions(),
      /Gateway bearer token must not contain whitespace or control characters/,
    );
    assert.deepEqual(sessionRequests, []);
  }
});

test("DX Mobile runtime rejects malformed bearer tokens before storage and transports", async () => {
  for (const malformedToken of [
    "paired secret-token",
    "paired\tsecret-token",
    "paired\nsecret-token",
    "paired\rsecret-token",
    "paired\u0000secret-token",
    "paired\u007fsecret-token",
    "Bearer Bearer paired-secret-token",
  ]) {
    const listeners = new Map();
    const storeRequests = [];
    const networkRequests = [];
    const socketCalls = [];
    const status = {
      dataset: {},
      textContent: "",
    };
    const gatewayInput = { value: "" };
    const pairingCodeInput = { value: "" };
    const agentInput = { value: "" };
    const form = {
      fields: {
        pairing_payload: "https://desktop.example/gw/?code=123456&session_id=session-1&agent=friday",
        gateway_origin: "",
        pairing_code: "",
        agent_alias: "",
      },
      addEventListener(type, listener) {
        listeners.set(type, listener);
      },
    };

    class CapturedWebSocket {
      constructor(url, protocols) {
        socketCalls.push({ url: String(url), protocols: Array.from(protocols || []) });
      }
    }

    loadRuntime({
      readyState: "complete",
      FormData: RuntimeFormData,
      WebSocket: CapturedWebSocket,
      fetch: async (url) => {
        const requestUrl = String(url);
        networkRequests.push(requestUrl);
        if (requestUrl.includes("/api/pair")) {
          return {
            ok: true,
            json: async () => ({
              token: malformedToken,
              device_id: "dx-mobile-test",
            }),
          };
        }

        throw new Error(`Unexpected fetch: ${requestUrl}`);
      },
      window: {
        __TAURI__: {
          core: {
            invoke: async (command, payload) => {
              if (command === "dx_mobile_store_device_credential") storeRequests.push(payload);
              if (command === "dx_mobile_load_device_credential") return {};
              return {};
            },
          },
        },
      },
      getElementById(id) {
        return id === "dx-mobile-runtime-status" ? status : null;
      },
      querySelector(selector) {
        if (selector === ".dx-mobile-pairing-form") return form;
        if (selector === "input[name='gateway_origin']") return gatewayInput;
        if (selector === "input[name='pairing_code']") return pairingCodeInput;
        if (selector === "input[name='agent_alias']") return agentInput;
        return null;
      },
    });

    listeners.get("submit")({
      preventDefault() {},
      currentTarget: form,
    });
    await new Promise((resolve) => setImmediate(resolve));

    assert.match(status.textContent, /Gateway bearer token must not contain whitespace or control characters/);
    assert.deepEqual(storeRequests, []);
    assert.deepEqual(networkRequests, ["https://desktop.example/gw/api/pair"]);
    assert.deepEqual(socketCalls, []);
    assert.doesNotMatch(status.textContent, /paired secret-token|Bearer Bearer paired-secret-token/);
  }
});

test("DX Mobile runtime opens live gateway sockets without token-bearing URLs", async () => {
  const listeners = new Map();
  const socketCalls = [];
  const status = {
    dataset: {},
    textContent: "",
  };
  const gatewayInput = { value: "" };
  const pairingCodeInput = { value: "" };
  const agentInput = { value: "" };
  const form = {
    fields: {
      pairing_payload: "https://desktop.example/gw/?code=123456&session_id=session-1&agent=friday",
      gateway_origin: "",
      pairing_code: "",
      agent_alias: "",
    },
    addEventListener(type, listener) {
      listeners.set(type, listener);
    },
  };
  const messageButton = {
    disabled: false,
    attributes: {},
    setAttribute(name, value) {
      this.attributes[name] = String(value);
    },
  };

  class CapturedWebSocket {
    constructor(url, protocols) {
      this.url = String(url);
      this.protocols = Array.from(protocols || []);
      socketCalls.push({ url: this.url, protocols: this.protocols });
    }

    addEventListener() {}
    close() {}
  }

  loadRuntime({
    readyState: "complete",
    FormData: RuntimeFormData,
    WebSocket: CapturedWebSocket,
    fetch: async (url) => {
      const requestUrl = String(url);
      if (requestUrl.includes("/api/pair")) {
        return {
          ok: true,
          json: async () => ({
            token: "paired-secret-token",
            device_id: "dx-mobile-test",
          }),
        };
      }
      if (requestUrl.includes("/api/devices/me/capabilities")) {
        return {
          ok: true,
          json: async () => ({}),
        };
      }
      if (requestUrl.includes("/api/sessions")) {
        return {
          ok: true,
          json: async () => ({ sessions: [] }),
        };
      }

      throw new Error(`Unexpected fetch: ${requestUrl}`);
    },
    window: {
      __TAURI__: {
        core: {
          invoke: async (command) => {
            if (command === "dx_mobile_store_device_credential") return { stored: false };
            if (command === "dx_mobile_load_device_credential") return {};
            return {};
          },
        },
      },
    },
    getElementById(id) {
      return id === "dx-mobile-runtime-status" ? status : null;
    },
    querySelector(selector) {
      if (selector === ".dx-mobile-pairing-form") return form;
      if (selector === "input[name='gateway_origin']") return gatewayInput;
      if (selector === "input[name='pairing_code']") return pairingCodeInput;
      if (selector === "input[name='agent_alias']") return agentInput;
      if (selector === ".dx-mobile-message-form button[type='submit']") return messageButton;
      return null;
    },
  });

  listeners.get("submit")({
    preventDefault() {},
    currentTarget: form,
  });
  await new Promise((resolve) => setImmediate(resolve));

  assert.equal(socketCalls.length, 1);
  const socketUrl = new URL(socketCalls[0].url);
  assert.equal(socketUrl.protocol, "wss:");
  assert.equal(socketUrl.pathname, "/gw/ws/chat");
  assert.equal(socketUrl.searchParams.get("session_id"), "session-1");
  assert.equal(socketUrl.searchParams.get("agent"), "friday");
  assert.deepEqual(Array.from(socketUrl.searchParams.keys()).sort(), ["agent", "session_id"]);
  assert.doesNotMatch(
    socketUrl.search,
    /token|bearer|authorization|access[_-]token|refresh[_-]token|id[_-]token|client[_-]secret|api[_-]key|x[_-]api[_-]key/i,
  );
  assert.deepEqual(socketCalls[0].protocols, ["dx-agents.v1", "zeroclaw.v1", "bearer.paired-secret-token"]);
});

test("DX Mobile runtime sends typed mobile chat frames with runtime-only session identity", () => {
  const sentFrames = [];
  const form = createTestElement("form");
  const textarea = createTestElement("textarea");
  textarea.value = "  Continue the CLI task  ";
  form.querySelector = (selector) => (selector === "textarea[name='message']" ? textarea : null);

  class CapturedWebSocket {
    static OPEN = 1;

    readyState = CapturedWebSocket.OPEN;

    addEventListener() {}

    close() {}

    send(frame) {
      sentFrames.push(JSON.parse(frame));
    }
  }

  const runtime = loadRuntime({
    WebSocket: CapturedWebSocket,
  });

  runtime.setActiveChatSession({
    session_key: "gw_private_cli_key",
    session_id: "private-cli-session",
    source: "dx_cli",
  });
  runtime.attachChatSocketForTesting(new CapturedWebSocket());
  runtime.handleMessageSubmit({
    preventDefault() {},
    currentTarget: form,
  });

  assert.deepEqual(sentFrames, [
    {
      type: "mobile.chat.message",
      content: "Continue the CLI task",
      session_key: "gw_private_cli_key",
      session_id: "private-cli-session",
      source: "dx-mobile",
      device_name: "DX Mobile App",
    },
  ]);
  assert.equal(textarea.value, "");
});

function createTestElement(tagName) {
  const selectorChildren = new Map();
  const element = {
    tagName: tagName.toUpperCase(),
    attributes: {},
    children: [],
    dataset: {},
    listeners: new Map(),
    textContent: "",
    append(...children) {
      this.children.push(...children);
    },
    addEventListener(type, listener) {
      this.listeners.set(type, listener);
    },
    querySelector(selector) {
      return selectorChildren.get(selector) || null;
    },
    setAttribute(name, value) {
      this.attributes[name] = String(value);
    },
  };

  Object.defineProperty(element, "innerHTML", {
    set(value) {
      if (!String(value).includes("<strong>")) return;
      const title = createTestElement("strong");
      const detail = createTestElement("small");
      selectorChildren.set("strong", title);
      selectorChildren.set("small", detail);
      this.children.push(title, detail);
    },
  });

  return element;
}

function publicElementSnapshot(element) {
  if (!element) return "";
  return [element.textContent, ...element.children.map((child) => publicElementSnapshot(child))].join(" ");
}
