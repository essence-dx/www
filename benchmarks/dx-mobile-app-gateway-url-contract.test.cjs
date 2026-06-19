const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");
const vm = require("node:vm");

const runtimePath = path.resolve(__dirname, "../dx-mobile-app/public/mobile-companion-runtime.js");

function loadRuntime(overrides = {}) {
  const sandbox = {
    URL,
    URLSearchParams,
    FormData,
    WebSocket: class WebSocket {},
    crypto: { randomUUID: () => "session-test" },
    fetch: () => Promise.reject(new Error("network disabled in contract test")),
    sessionStorage: {
      getItem: () => null,
      removeItem: () => undefined,
      setItem: () => undefined,
    },
    window: {},
    document: {
      readyState: "loading",
      addEventListener: () => undefined,
      getElementById: () => null,
      querySelector: () => null,
    },
  };
  Object.assign(sandbox, overrides);
  sandbox.window = sandbox;
  vm.runInNewContext(fs.readFileSync(runtimePath, "utf8"), sandbox, {
    filename: runtimePath,
  });
  return sandbox.window.DXMobileCompanionRuntime;
}

test("DX Mobile runtime preserves gateway path prefixes while normalizing URLs", () => {
  const runtime = loadRuntime();

  assert.equal(
    runtime.normalizeGatewayBaseUrl("https://desktop.example/gw/?ignored=true#fragment"),
    "https://desktop.example/gw",
  );
  assert.equal(
    runtime.normalizeGatewayBaseUrl("http://127.0.0.1:42617/zeroclaw/"),
    "http://127.0.0.1:42617/zeroclaw",
  );
  assert.throws(
    () => runtime.normalizeGatewayBaseUrl("http://192.168.1.24:42617/gw"),
    /Gateway base URL must be HTTPS or loopback HTTP/,
  );
});

test("DX Mobile runtime joins gateway HTTP and WebSocket endpoints after a path prefix", () => {
  const runtime = loadRuntime();

  assert.equal(
    runtime.gatewayEndpoint("https://desktop.example/gw", "/api/pair"),
    "https://desktop.example/gw/api/pair",
  );
  assert.equal(
    runtime.gatewayEndpoint("https://desktop.example/gw/", "api/sessions"),
    "https://desktop.example/gw/api/sessions",
  );
  assert.equal(
    runtime.gatewayWebSocketEndpoint("https://desktop.example/gw", "/ws/chat", "agent=friday&session_id=abc"),
    "wss://desktop.example/gw/ws/chat?agent=friday&session_id=abc",
  );
});

test("DX Mobile runtime rejects token-bearing WebSocket query strings", () => {
  const runtime = loadRuntime();

  assert.equal(
    runtime.gatewayWebSocketEndpoint("https://desktop.example/gw", "/ws/chat", "?agent=friday&session_id=abc"),
    "wss://desktop.example/gw/ws/chat?agent=friday&session_id=abc",
  );

  for (const query of [
    "agent=friday&token=secret",
    "access_token=secret",
    "access-token=secret",
    "refresh_token=secret",
    "id_token=secret",
    "client_secret=secret",
    "api_key=secret",
    "api-key=secret",
    "x-api-key=secret",
    "authorization=Bearer%20secret",
    "bearer=secret",
    "auth=Bearer%20secret",
    "auth=prefix%20Bearer%20secret",
    "protocol=bearer.secret",
  ]) {
    assert.throws(
      () => runtime.gatewayWebSocketEndpoint("https://desktop.example/gw", "/ws/chat", query),
      /WebSocket query must not include bearer credentials/,
    );
  }
});

test("DX Mobile runtime only allows public chat WebSocket query fields", () => {
  const runtime = loadRuntime();

  assert.equal(
    runtime.gatewayWebSocketEndpoint("https://desktop.example/gw", "/ws/chat", "agent=friday&session_id=abc"),
    "wss://desktop.example/gw/ws/chat?agent=friday&session_id=abc",
  );
  assert.equal(
    runtime.gatewayWebSocketEndpoint("https://desktop.example/gw", "/ws/chat", "?session_id=abc&agent=friday"),
    "wss://desktop.example/gw/ws/chat?session_id=abc&agent=friday",
  );

  for (const query of [
    "workspace=zed",
    "thread_id=chat-1",
    "session-id=abc",
    "agent=friday&session_id=abc&auth_token=secret",
    "agent=friday&session_id=abc&jwt=secret",
    "agent=friday&session_id=abc&foo=bar",
  ]) {
    assert.throws(
      () => runtime.gatewayWebSocketEndpoint("https://desktop.example/gw", "/ws/chat", query),
      /WebSocket query must only include agent and session_id/,
    );
  }
});

test("DX Mobile runtime rejects unsafe WebSocket identity query values", () => {
  const runtime = loadRuntime();

  for (const query of [
    "agent=fri%20day&session_id=abc",
    "agent=friday&session_id=session%0A1",
    `agent=${"f".repeat(65)}&session_id=abc`,
    `agent=friday&session_id=${"s".repeat(129)}`,
  ]) {
    assert.throws(
      () => runtime.gatewayWebSocketEndpoint("https://desktop.example/gw", "/ws/chat", query),
      /must not contain whitespace or control characters|is too long/,
    );
  }
});

test("DX Mobile runtime parses direct gateway URL QR payloads", async () => {
  const runtime = loadRuntime();

  const httpsPreview = await runtime.parsePairingPayload(
    "https://desktop.example/agent/gateway/?code=123456&session_id=session-1&agent=friday",
  );
  assert.equal(httpsPreview.gateway, "https://desktop.example/agent/gateway");
  assert.equal(httpsPreview.code, "123456");
  assert.equal(httpsPreview.sessionId, "session-1");
  assert.equal(httpsPreview.agentAlias, "friday");

  const loopbackPreview = await runtime.parsePairingPayload(
    "http://127.0.0.1:42617/zeroclaw/?pairing_code=654321&session_key=gw_session&agent_alias=friday",
  );
  assert.equal(loopbackPreview.gateway, "http://127.0.0.1:42617/zeroclaw");
  assert.equal(loopbackPreview.code, "654321");
  assert.equal(loopbackPreview.sessionKey, "gw_session");
  assert.equal(loopbackPreview.agentAlias, "friday");
});

test("DX Mobile runtime rejects ambiguous pairing QR field aliases", async () => {
  const runtime = loadRuntime();

  await assert.rejects(
    () =>
      runtime.parsePairingPayload(
        '{"gateway":"https://desktop.example/gw","origin":"https://desktop.example/other","code":"123456"}',
      ),
    /Pairing payload gateway fields must not conflict/,
  );
  await assert.rejects(
    () =>
      runtime.parsePairingPayload(
        '{"gateway":"https://desktop.example/gw","code":"123456","pairing_code":"654321"}',
      ),
    /Pairing payload code fields must not conflict/,
  );
  await assert.rejects(
    () =>
      runtime.parsePairingPayload(
        '{"gateway":"https://desktop.example/gw","code":"123456","agent":"friday","agent_alias":"saturday"}',
      ),
    /Pairing payload agent fields must not conflict/,
  );
  await assert.rejects(
    () => runtime.parsePairingPayload("https://desktop.example/gw/?code=123456&code=654321"),
    /Pairing payload code fields must not repeat/,
  );
  await assert.rejects(
    () =>
      runtime.parsePairingPayload(
        "dxmobile://pair?gateway=https://desktop.example/gw&origin=https://desktop.example/other&code=123456",
      ),
    /Pairing payload gateway fields must not conflict/,
  );
});

test("DX Mobile runtime rejects malformed pairing and session identity fields", async () => {
  const runtime = loadRuntime();

  await assert.rejects(
    () => runtime.parsePairingPayload("https://desktop.example/gw/?code=123%20456"),
    /Pairing code must not contain whitespace or control characters/,
  );
  await assert.rejects(
    () => runtime.parsePairingPayload("https://desktop.example/gw/?code=123456&session_id=session%0A1"),
    /Session id must not contain whitespace or control characters/,
  );
  await assert.rejects(
    () => runtime.parsePairingPayload("https://desktop.example/gw/?code=123456&session_key=gw_%00hidden"),
    /Session key must not contain whitespace or control characters/,
  );
  await assert.rejects(
    () => runtime.parsePairingPayload("https://desktop.example/gw/?code=123456&agent_alias=fri%20day"),
    /Agent alias must not contain whitespace or control characters/,
  );
  await assert.rejects(
    () => runtime.parsePairingPayload(`https://desktop.example/gw/?code=${"1".repeat(129)}`),
    /Pairing code is too long/,
  );
});

test("DX Mobile runtime rejects pairing QR expiries beyond the challenge TTL", async () => {
  const runtime = loadRuntime();
  const acceptedExpiry = new Date(Date.now() + 60_000).toISOString();
  const replayExpiry = new Date(Date.now() + 600_000).toISOString();

  const preview = await runtime.parsePairingPayload(
    `https://desktop.example/agent/gateway/?code=123456&expires_at=${encodeURIComponent(acceptedExpiry)}`,
  );

  assert.equal(preview.expiresAt, acceptedExpiry);
  await assert.rejects(
    () =>
      runtime.parsePairingPayload(
        `https://desktop.example/agent/gateway/?code=123456&expires_at=${encodeURIComponent(replayExpiry)}`,
      ),
    /Pairing payload expiry exceeds 300 seconds/,
  );
});

test("DX Mobile runtime builds a token-free gateway connect frame", () => {
  const runtime = loadRuntime();

  assert.deepEqual(Array.from(runtime.mobileCapabilities), [
    "mobile.pairing.scan_qr",
    "mobile.chat.read",
    "mobile.chat.send",
    "mobile.approvals.respond",
    "mobile.context.preview",
  ]);

  const frame = JSON.parse(JSON.stringify(runtime.buildGatewayConnectFrame("session-1")));

  assert.deepEqual(frame, {
    type: "connect",
    session_id: "session-1",
    device_name: "DX Mobile App",
    capabilities: Array.from(runtime.mobileCapabilities),
  });
  assert.doesNotMatch(JSON.stringify(frame), /token|bearer|authorization|access_token/i);
});

test("DX Mobile runtime carries QR session context into the pairing request", () => {
  const runtime = loadRuntime();

  const body = JSON.parse(
    JSON.stringify(
      runtime.buildPairingRequestBody(" 123456 ", {
        sessionId: "session-1",
        sessionKey: "gw_session-1",
      }),
    ),
  );

  assert.deepEqual(body, {
    code: "123456",
    device_name: "DX Mobile App",
    device_type: "mobile",
    session_id: "session-1",
    session_key: "gw_session-1",
  });
  assert.doesNotMatch(JSON.stringify(body), /token|bearer|authorization|access_token/i);

  assert.throws(
    () => runtime.buildPairingRequestBody("12 3456", { sessionId: "session-1" }),
    /Pairing code must not contain whitespace or control characters/,
  );
  assert.throws(
    () => runtime.buildPairingRequestBody("123456", { sessionId: "session\t1" }),
    /Session id must not contain whitespace or control characters/,
  );
});

test("DX Mobile runtime does not use composite session keys as WebSocket session ids", () => {
  const runtime = loadRuntime();

  assert.equal(
    runtime.sessionIdFromPairingPreview({
      sessionId: " session-1 ",
      sessionKey: "gw_ignored",
    }),
    "session-1",
  );
  assert.equal(runtime.sessionIdFromPairingPreview({ sessionKey: "gw_session-2" }), "session-2");
  assert.equal(
    runtime.sessionIdFromPairingPreview({
      sessionKey: "workspace:thread:session-3",
    }),
    "",
  );
});

test("DX Mobile runtime validates scanned QR payloads before touching visible fields", async () => {
  const fields = {
    payload: { value: "" },
    gateway: { value: "" },
    code: { value: "" },
    agent: { value: "" },
  };
  const document = {
    readyState: "loading",
    addEventListener: () => undefined,
    getElementById: () => null,
    querySelector: (selector) => {
      if (selector === "textarea[name='pairing_payload']") return fields.payload;
      if (selector === "input[name='gateway_origin']") return fields.gateway;
      if (selector === "input[name='pairing_code']") return fields.code;
      if (selector === "input[name='agent_alias']") return fields.agent;
      return null;
    },
  };
  const runtime = loadRuntime({ document });

  await assert.rejects(
    () =>
      runtime.applyScannedPairingPayload(
        "dxmobile://pair?gateway=https://desktop.example/gw&code=123456&token=secret",
      ),
    /Rejecting QR payload that contains a bearer token/,
  );
  assert.equal(fields.payload.value, "");
  assert.equal(fields.gateway.value, "");
  assert.equal(fields.code.value, "");

  await runtime.applyScannedPairingPayload(
    "https://desktop.example/gw/?code=123456&agent=friday",
  );
  assert.equal(fields.payload.value, "");
  assert.equal(fields.gateway.value, "https://desktop.example/gw");
  assert.equal(fields.code.value, "123456");
  assert.equal(fields.agent.value, "friday");
});
