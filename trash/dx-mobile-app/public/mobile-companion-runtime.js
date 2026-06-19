(function () {
  "use strict";

  const DEFAULT_AGENT_ALIAS = "friday";
  const DEVICE_NAME = "DX Mobile App";
  const PAIRING_CHALLENGE_TTL_SECONDS = 300;
  const PAIRING_CODE_MAX_LENGTH = 128;
  const SESSION_ID_MAX_LENGTH = 128;
  const SESSION_KEY_MAX_LENGTH = 256;
  const AGENT_ALIAS_MAX_LENGTH = 64;
  const SESSION_STORAGE_GATEWAY = "dx_mobile_gateway_origin";
  const SESSION_STORAGE_AGENT = "dx_mobile_agent_alias";
  const deviceRegistryPairEndpoint = "/api/pair";
  const MOBILE_CAPABILITIES = Object.freeze([
    "mobile.pairing.scan_qr",
    "mobile.chat.read",
    "mobile.chat.send",
    "mobile.approvals.respond",
    "mobile.context.preview",
  ]);
  const FORBIDDEN_PAIRING_FIELD_NAMES = Object.freeze([
    "token",
    "bearer",
    "authorization",
    "access_token",
    "refresh_token",
    "id_token",
    "client_secret",
    "api_key",
    "x_api_key",
  ]);
  const ALLOWED_WEBSOCKET_QUERY_FIELD_NAMES = Object.freeze(["agent", "session_id"]);
  const FORBIDDEN_WEBSOCKET_QUERY_FIELD_NAMES = Object.freeze([
    "token",
    "bearer",
    "authorization",
    "access_token",
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
  ]);
  const STATUS_REDACTION_FIELDS = Object.freeze([
    "access_token",
    "access-token",
    "refresh_token",
    "refresh-token",
    "id_token",
    "id-token",
    "token",
    "authorization",
    "client_secret",
    "client-secret",
    "session_id",
    "session_key",
    "session-key",
    "pairing_code",
    "pairing-code",
    "api_key",
    "api-key",
    "x_api_key",
    "x-api-key",
  ]);
  const STATUS_REDACTED = "[redacted]";
  const STATUS_FIELD_VALUE_PATTERN = new RegExp(
    `(["']?)(\\b(?:${STATUS_REDACTION_FIELDS.join("|")})\\b)\\1(\\s*[:=]\\s*)(["']?)([^\\s&,'"{}\\[\\]]+)(\\4)`,
    "gi",
  );
  const STATUS_AUTHORIZATION_PATTERN =
    /(["']?)(\bauthorization\b)\1(\s*[:=]\s*)(["']?)(?:[A-Za-z]+\s+)?[^\s&,'"{}\[\]]+(\4)/gi;
  const STATUS_BEARER_FIELD_PATTERN =
    /(["']?)(\bbearer\b)\1(\s*[:=]\s*)(["']?)([^\s&,'"{}\[\]]+)(\4)/gi;
  const STATUS_BEARER_PATTERN = /\b(Bearer)(\s+)(?!tokens?\b)([A-Za-z0-9._~+/=-]{6,})/gi;
  const STATUS_BEARER_PROTOCOL_PATTERN = /\b(bearer\.)(?!tokens?\b)([A-Za-z0-9._~+/=-]{3,})/gi;

  const pairingState = Object.freeze({
    idle: "idle",
    parsing: "parsing",
    pairing: "pairing",
    pairedSessionOnly: "paired-session-only",
    pairedDurable: "paired-durable",
    failed: "failed",
  });

  const scannerState = Object.freeze({
    unavailable: "unavailable",
    ready: "ready",
    scanning: "scanning",
    scanned: "scanned",
    permissionDenied: "permission-denied",
    error: "error",
  });

  const socketState = Object.freeze({
    disconnected: "disconnected",
    connecting: "connecting",
    connectedLiveChat: "connected-live-chat",
    failed: "failed",
  });

  const runtimeState = {
    gatewayOrigin: "",
    activeCredential: null,
    activeSessionId: "",
    activeSessionKey: "",
    socket: null,
    pairingState: pairingState.idle,
    scannerState: scannerState.unavailable,
    socketState: socketState.disconnected,
  };

  function statusNode() {
    return document.getElementById("dx-mobile-runtime-status");
  }

  function sanitizeStatusMessage(message) {
    return String(message ?? "")
      .replace(STATUS_AUTHORIZATION_PATTERN, `$1$2$1$3$4${STATUS_REDACTED}$5`)
      .replace(STATUS_BEARER_FIELD_PATTERN, `$1$2$1 ${STATUS_REDACTED}`)
      .replace(STATUS_FIELD_VALUE_PATTERN, `$1$2$1$3$4${STATUS_REDACTED}$6`)
      .replace(STATUS_BEARER_PATTERN, `$1$2${STATUS_REDACTED}`)
      .replace(STATUS_BEARER_PROTOCOL_PATTERN, `$1${STATUS_REDACTED}`);
  }

  function writeStatus(message, tone) {
    const output = statusNode();
    if (!output) return;
    output.textContent = sanitizeStatusMessage(message);
    output.dataset.dxStatusTone = tone || "neutral";
    output.dataset.dxPairingState = runtimeState.pairingState;
    output.dataset.dxScannerState = runtimeState.scannerState;
    output.dataset.dxSocketState = runtimeState.socketState;
  }

  function setPairingState(state) {
    runtimeState.pairingState = state;
    const output = statusNode();
    if (output) output.dataset.dxPairingState = state;
  }

  function setSocketState(state) {
    runtimeState.socketState = state;
    const output = statusNode();
    if (output) output.dataset.dxSocketState = state;
  }

  function invokeNative(command, payload) {
    const invoke = window.__TAURI__?.core?.invoke;
    if (typeof invoke !== "function") {
      return Promise.resolve({ available: false, stored: false, reason: "native bridge unavailable" });
    }
    return invoke(command, payload);
  }

  function normalizePairingFieldName(field) {
    return String(field || "").trim().toLowerCase().replace(/[-\s]+/g, "_");
  }

  function forbiddenPairingFieldMarkers(field) {
    const normalized = normalizePairingFieldName(field);
    const hyphenated = normalized.replace(/_/g, "-");
    const markers = [`${normalized}=`, `"${normalized}"`];
    if (hyphenated !== normalized) markers.push(`${hyphenated}=`, `"${hyphenated}"`);
    return markers;
  }

  function rejectBearerQrPayload(raw) {
    const lower = String(raw || "").toLowerCase();
    const hasBearerCredential = lower.includes("bearer ") || lower.includes("bearer.");
    const hasForbiddenField = FORBIDDEN_PAIRING_FIELD_NAMES.some((field) =>
      forbiddenPairingFieldMarkers(field).some((marker) => lower.includes(marker)),
    );

    if (hasBearerCredential || hasForbiddenField) {
      throw new Error("Rejecting QR payload that contains a bearer token");
    }
  }

  function rejectForbiddenPairingFields(value) {
    if (!value || typeof value !== "object") return;
    if (Array.isArray(value)) {
      value.forEach(rejectForbiddenPairingFields);
      return;
    }

    Object.entries(value).forEach(([field, nested]) => {
      if (FORBIDDEN_PAIRING_FIELD_NAMES.includes(normalizePairingFieldName(field))) {
        throw new Error("Rejecting QR payload that contains a bearer token");
      }
      rejectForbiddenPairingFields(nested);
    });
  }

  function rejectDecodedPairingEntry(field, value) {
    if (FORBIDDEN_PAIRING_FIELD_NAMES.includes(normalizePairingFieldName(field))) {
      throw new Error("Rejecting QR payload that contains a bearer token");
    }
    rejectBearerQrPayload(value);
  }

  function rejectForbiddenUrlFields(url) {
    url.searchParams.forEach((value, field) => rejectDecodedPairingEntry(field, value));
  }

  function rejectDecodedPairingPayloadCredentials(raw) {
    const trimmed = String(raw || "").trim();
    rejectBearerQrPayload(trimmed);
    if (!trimmed) return;

    if (trimmed.startsWith("{")) {
      rejectForbiddenPairingFields(JSON.parse(trimmed));
      return;
    }

    let url;
    try {
      url = new URL(trimmed);
    } catch {
      return;
    }
    rejectForbiddenUrlFields(url);
  }

  function pairingAliasValue(value) {
    if (value === null || value === undefined) return "";
    return String(value).trim();
  }

  function selectObjectPairingAlias(source, fields, label) {
    const candidates = fields
      .filter((field) => Object.prototype.hasOwnProperty.call(source, field))
      .map((field) => ({ field, value: pairingAliasValue(source[field]) }))
      .filter((candidate) => candidate.value);

    if (candidates.length > 1) throw new Error(`Pairing payload ${label} fields must not conflict`);
    return candidates[0]?.value || "";
  }

  function selectUrlPairingAlias(url, fields, label) {
    const candidates = [];

    fields.forEach((field) => {
      const values = url.searchParams.getAll(field).map(pairingAliasValue).filter(Boolean);
      if (values.length > 1) throw new Error(`Pairing payload ${label} fields must not repeat`);
      if (values.length === 1) candidates.push({ field, value: values[0] });
    });

    if (candidates.length > 1) throw new Error(`Pairing payload ${label} fields must not conflict`);
    return candidates[0]?.value || "";
  }

  function rejectAmbiguousPairingPayloadAliases(raw) {
    const trimmed = String(raw || "").trim();
    if (!trimmed) return;

    if (trimmed.startsWith("{")) {
      const value = JSON.parse(trimmed);
      if (!value || typeof value !== "object" || Array.isArray(value)) return;
      selectObjectPairingAlias(value, ["gateway", "gateway_origin", "origin"], "gateway");
      selectObjectPairingAlias(value, ["code", "pairing_code"], "code");
      selectObjectPairingAlias(value, ["agent_alias", "agent"], "agent");
      return;
    }

    let url;
    try {
      url = new URL(trimmed);
    } catch {
      return;
    }
    selectUrlPairingAlias(url, ["gateway", "gateway_origin", "origin"], "gateway");
    selectUrlPairingAlias(url, ["code", "pairing_code"], "code");
    selectUrlPairingAlias(url, ["session_id"], "session id");
    selectUrlPairingAlias(url, ["session_key"], "session key");
    selectUrlPairingAlias(url, ["agent_alias", "agent"], "agent");
    selectUrlPairingAlias(url, ["expires_at"], "expiry");
  }

  function rejectExpiredPairingPayload(expiresAt) {
    if (!expiresAt) return;
    const expires = Date.parse(String(expiresAt));
    if (!Number.isFinite(expires)) throw new Error("Pairing payload expiry must be RFC3339");
    const now = Date.now();
    if (expires <= now) throw new Error("Pairing payload is expired");
    if (expires - now > PAIRING_CHALLENGE_TTL_SECONDS * 1000) {
      throw new Error(`Pairing payload expiry exceeds ${PAIRING_CHALLENGE_TTL_SECONDS} seconds`);
    }
  }

  function isLoopbackHost(hostname) {
    return hostname === "127.0.0.1" || hostname === "localhost" || hostname === "::1" || hostname === "[::1]";
  }

  function normalizeGatewayBaseUrl(baseUrl) {
    const parsed = new URL(String(baseUrl || ""));
    if (parsed.username || parsed.password) throw new Error("Gateway base URL must not include credentials");
    const secure = parsed.protocol === "https:";
    const local = parsed.protocol === "http:" && isLoopbackHost(parsed.hostname);
    if (!secure && !local) throw new Error("Gateway base URL must be HTTPS or loopback HTTP");
    parsed.pathname = parsed.pathname.replace(/\/+$/, "") || "/";
    parsed.search = "";
    parsed.hash = "";
    return parsed.toString().replace(/\/$/, "");
  }

  function gatewayEndpoint(baseUrl, endpoint) {
    const suffix = String(endpoint || "").replace(/^\/+/, "");
    const normalizedBase = normalizeGatewayBaseUrl(baseUrl);
    return suffix ? `${normalizedBase}/${suffix}` : normalizedBase;
  }

  function tokenFreeWebSocketQuery(query) {
    const normalizedQuery = String(query || "").replace(/^\?/, "");
    if (!normalizedQuery) return "";

    const params = new URLSearchParams(normalizedQuery);
    const safeParams = new URLSearchParams();
    for (const [name, value] of params) {
      const field = String(name || "").trim().toLowerCase();
      const fieldValue = String(value || "").trim().toLowerCase();
      if (
        FORBIDDEN_WEBSOCKET_QUERY_FIELD_NAMES.includes(field) ||
        fieldValue.includes("bearer ") ||
        fieldValue.includes("bearer.")
      ) {
        throw new Error("WebSocket query must not include bearer credentials");
      }
      if (!ALLOWED_WEBSOCKET_QUERY_FIELD_NAMES.includes(field)) {
        throw new Error("WebSocket query must only include agent and session_id");
      }
      safeParams.append(field, field === "agent" ? normalizeAgentAlias(value) : normalizeSessionId(value));
    }

    return safeParams.toString();
  }

  function gatewayWebSocketEndpoint(baseUrl, endpoint, query) {
    const url = new URL(gatewayEndpoint(baseUrl, endpoint));
    url.protocol = url.protocol === "https:" ? "wss:" : "ws:";
    const normalizedQuery = tokenFreeWebSocketQuery(query);
    if (normalizedQuery) url.search = normalizedQuery;
    return url.toString();
  }

  function parseFallbackPairingPayload(raw) {
    const trimmed = String(raw || "").trim();
    rejectDecodedPairingPayloadCredentials(trimmed);
    if (!trimmed) return {};

    if (trimmed.startsWith("{")) {
      const value = JSON.parse(trimmed);
      rejectForbiddenPairingFields(value);
      rejectExpiredPairingPayload(value.expires_at);
      const gateway = selectObjectPairingAlias(value, ["gateway", "gateway_origin", "origin"], "gateway");
      return {
        gateway: gateway ? normalizeGatewayBaseUrl(gateway) : "",
        code: normalizePairingCode(selectObjectPairingAlias(value, ["code", "pairing_code"], "code")),
        sessionId: normalizeSessionId(value.session_id),
        sessionKey: normalizeSessionKey(value.session_key),
        agentAlias: normalizeOptionalAgentAlias(selectObjectPairingAlias(value, ["agent_alias", "agent"], "agent")),
        expiresAt: value.expires_at || "",
      };
    }

    const url = new URL(trimmed);
    rejectForbiddenUrlFields(url);
    const expiresAt = selectUrlPairingAlias(url, ["expires_at"], "expiry");
    rejectExpiredPairingPayload(expiresAt);
    const gateway =
      selectUrlPairingAlias(url, ["gateway", "gateway_origin", "origin"], "gateway") || gatewayFromUrlPayload(url);
    return {
      gateway: gateway ? normalizeGatewayBaseUrl(gateway) : "",
      code: normalizePairingCode(selectUrlPairingAlias(url, ["code", "pairing_code"], "code")),
      sessionId: normalizeSessionId(selectUrlPairingAlias(url, ["session_id"], "session id")),
      sessionKey: normalizeSessionKey(selectUrlPairingAlias(url, ["session_key"], "session key")),
      agentAlias: normalizeOptionalAgentAlias(selectUrlPairingAlias(url, ["agent_alias", "agent"], "agent")),
      expiresAt,
    };
  }

  function gatewayFromUrlPayload(url) {
    return url.protocol === "https:" || url.protocol === "http:" ? url.toString() : "";
  }

  async function parsePairingPayload(raw) {
    rejectDecodedPairingPayloadCredentials(raw);
    rejectAmbiguousPairingPayloadAliases(raw);
    const nativePreview = await invokeNative("dx_mobile_parse_pairing_payload", { raw });
    if (nativePreview && nativePreview.available !== false && nativePreview.gateway_origin) {
      rejectForbiddenPairingFields(nativePreview);
      rejectExpiredPairingPayload(nativePreview.expires_at);
      return {
        gateway: normalizeGatewayBaseUrl(nativePreview.gateway_origin),
        code: normalizePairingCode(nativePreview.pairing_code),
        sessionId: normalizeSessionId(nativePreview.session_id),
        sessionKey: normalizeSessionKey(nativePreview.session_key),
        agentAlias: normalizeOptionalAgentAlias(nativePreview.agent_alias),
      };
    }
    return parseFallbackPairingPayload(raw);
  }

  function rememberGatewayOrigin(origin) {
    runtimeState.gatewayOrigin = origin;
    sessionStorage.setItem(SESSION_STORAGE_GATEWAY, origin);
  }

  function rememberAgentAlias(agentAlias) {
    sessionStorage.setItem(SESSION_STORAGE_AGENT, normalizeAgentAlias(agentAlias));
  }

  function normalizeAgentAlias(value) {
    return normalizeIdentityField(value, "Agent alias", AGENT_ALIAS_MAX_LENGTH) || DEFAULT_AGENT_ALIAS;
  }

  function normalizeOptionalAgentAlias(value) {
    return normalizeIdentityField(value, "Agent alias", AGENT_ALIAS_MAX_LENGTH);
  }

  function normalizePairingCode(value) {
    return normalizeIdentityField(value, "Pairing code", PAIRING_CODE_MAX_LENGTH, { required: true });
  }

  function normalizeSessionId(value) {
    return normalizeIdentityField(value, "Session id", SESSION_ID_MAX_LENGTH);
  }

  function normalizeSessionKey(value) {
    return normalizeIdentityField(value, "Session key", SESSION_KEY_MAX_LENGTH);
  }

  function normalizeIdentityField(value, label, maxLength, options = {}) {
    const normalized = String(value || "").trim();
    if (!normalized) {
      if (options.required) throw new Error(`${label} is required`);
      return "";
    }
    if (/\s|[\u0000-\u001f\u007f]/.test(normalized)) {
      throw new Error(`${label} must not contain whitespace or control characters`);
    }
    if (normalized.length > maxLength) {
      throw new Error(`${label} is too long`);
    }
    return normalized;
  }

  function gatewayInput() {
    return document.querySelector("input[name='gateway_origin']");
  }

  function pairingCodeInput() {
    return document.querySelector("input[name='pairing_code']");
  }

  function agentInput() {
    return document.querySelector("input[name='agent_alias']");
  }

  function messageButton() {
    return document.querySelector(".dx-mobile-message-form button[type='submit']");
  }

  function scannerButton() {
    return document.querySelector("[data-dx-native-command='dx_mobile_scan_pairing_qr']");
  }

  function setMessageFormEnabled(enabled) {
    const button = messageButton();
    if (!button) return;
    button.disabled = !enabled;
    button.setAttribute("aria-disabled", enabled ? "false" : "true");
  }

  function setScannerState(state, enabled) {
    runtimeState.scannerState = state;
    const output = statusNode();
    if (output) output.dataset.dxScannerState = state;
    const button = scannerButton();
    if (!button) return;
    button.dataset.dxScannerState = state;
    button.disabled = !enabled;
    button.setAttribute("aria-disabled", enabled ? "false" : "true");
  }

  function barcodeScannerApi() {
    return window.__TAURI__?.barcodeScanner || null;
  }

  function scannerQrFormat(scanner) {
    return scanner?.Format?.QRCode || "QR_CODE";
  }

  function scanContent(scanned) {
    if (typeof scanned === "string") return scanned;
    return scanned?.content || scanned?.rawValue || scanned?.value || "";
  }

  async function ensureScannerPermission(scanner) {
    if (typeof scanner.checkPermissions !== "function") return "granted";
    let permission = await scanner.checkPermissions();
    if (
      (permission === "prompt" || permission === "prompt-with-rationale") &&
      typeof scanner.requestPermissions === "function"
    ) {
      permission = await scanner.requestPermissions();
    }
    return permission;
  }

  function authorizationHeaders(credential) {
    const token = normalizeGatewayBearerToken(credential?.token);
    return {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    };
  }

  function normalizeGatewayBearerToken(token) {
    let normalized = String(token || "").trim();
    if (normalized.toLowerCase().startsWith("bearer ")) {
      normalized = normalized.slice("bearer ".length).trim();
    }
    if (!normalized) throw new Error("Gateway bearer token is required");
    if (/\s|[\u0000-\u001f\u007f]/.test(normalized) || normalized.toLowerCase().startsWith("bearer ")) {
      throw new Error("Gateway bearer token must not contain whitespace or control characters");
    }
    return normalized;
  }

  function buildPairingRequestBody(code, context) {
    const sessionId = normalizeSessionId(context?.sessionId || context?.session_id).replace(/^gw_/, "");
    const sessionKey = normalizeSessionKey(context?.sessionKey || context?.session_key);
    const body = {
      code: normalizePairingCode(code),
      device_name: DEVICE_NAME,
      device_type: "mobile",
    };
    if (sessionId) body.session_id = sessionId;
    if (sessionKey) body.session_key = sessionKey;
    return body;
  }

  function pairingFailureMessage(status) {
    return `Pairing failed (${status}). Check the desktop pairing code and try again.`;
  }

  async function pairWithGateway(origin, code, context) {
    const gatewayOrigin = normalizeGatewayBaseUrl(origin);
    const response = await fetch(gatewayEndpoint(gatewayOrigin, deviceRegistryPairEndpoint), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(buildPairingRequestBody(code, context)),
    });

    if (!response.ok) {
      throw new Error(pairingFailureMessage(response.status));
    }

    return response.json();
  }

  async function storeCredential(credential) {
    const normalizedCredential = normalizeDeviceCredential(credential);
    const result = await invokeNative("dx_mobile_store_device_credential", { credential: normalizedCredential });
    runtimeState.activeCredential = normalizedCredential;
    return Boolean(result && result.stored === true);
  }

  async function loadCredential() {
    const result = await invokeNative("dx_mobile_load_device_credential", {});
    if (result && result.credential && result.credential.token) {
      const credential = normalizeDeviceCredential({
        ...result.credential,
        gateway_origin: normalizeGatewayBaseUrl(result.credential.gateway_origin),
      });
      runtimeState.activeCredential = credential;
      return credential;
    }
    return runtimeState.activeCredential ? normalizeDeviceCredential(runtimeState.activeCredential) : null;
  }

  function normalizeDeviceCredential(credential) {
    return {
      ...credential,
      gateway_origin: normalizeGatewayBaseUrl(credential?.gateway_origin),
      token: normalizeGatewayBearerToken(credential?.token),
    };
  }

  async function readCredentialStoreStatus() {
    const status = await invokeNative("dx_mobile_credential_store_status", {});
    return status && typeof status === "object" ? status : { available: false, durable: false };
  }

  async function updateCapabilities(credential) {
    await fetch(gatewayEndpoint(credential.gateway_origin, "/api/devices/me/capabilities"), {
      method: "POST",
      headers: authorizationHeaders(credential),
      body: JSON.stringify({
        capabilities: MOBILE_CAPABILITIES,
      }),
    }).catch((error) => {
      writeStatus(`Capabilities update skipped: ${error.message}`, "warning");
    });
  }

  function sessionIdFromSession(session) {
    const explicit = normalizeSessionId(session.session_id || session.id || "");
    if (explicit) return String(explicit).replace(/^gw_/, "");
    const sessionKey = normalizeSessionKey(session.session_key || "");
    return sessionKey.startsWith("gw_") ? sessionKey.replace(/^gw_/, "") : "";
  }

  function sessionKeyFromSession(session) {
    if (session.session_key) return normalizeSessionKey(session.session_key);
    if (session.session_id) return `gw_${normalizeSessionId(session.session_id).replace(/^gw_/, "")}`;
    return normalizeSessionKey(session.id || "");
  }

  function sessionIdFromPairingPreview(preview) {
    const explicit = normalizeSessionId(preview?.sessionId || preview?.session_id);
    if (explicit) return explicit.replace(/^gw_/, "");
    const sessionKey = normalizeSessionKey(preview?.sessionKey || preview?.session_key);
    return sessionKey.startsWith("gw_") ? sessionKey.replace(/^gw_/, "") : "";
  }

  function renderSessions(sessions) {
    const list = document.querySelector(".dx-mobile-chat-list");
    if (!list) return;
    list.textContent = "";

    if (!sessions.length) {
      const empty = document.createElement("p");
      empty.className = "companion-empty-state";
      empty.textContent = "No Zed chats yet.";
      list.append(empty);
      return;
    }

    sessions.slice(0, 8).forEach((session, index) => {
      const sessionId = sessionIdFromSession(session);
      const sessionKey = sessionKeyFromSession(session);
      const button = document.createElement("button");
      button.type = "button";
      button.className = `companion-session-row${index === 0 ? " is-selected" : ""}`;
      button.innerHTML = `<span><strong></strong><small></small></span><span class="companion-row-arrow" aria-hidden="true">Open</span>`;
      button.querySelector("strong").textContent = session.title || session.name || session.agent || "Zed chat";
      button.querySelector("small").textContent = "Secure session";
      button.addEventListener("click", () => loadMessages(sessionKey, sessionId));
      list.append(button);
    });
  }

  async function loadSessions() {
    const credential = await loadCredential();
    if (!credential) return;

    const response = await fetch(gatewayEndpoint(credential.gateway_origin, "/api/sessions"), {
      headers: authorizationHeaders(credential),
    });
    if (!response.ok) throw new Error(`Sessions failed (${response.status})`);
    const data = await response.json();
    const sessions = Array.isArray(data) ? data : data.sessions || [];
    renderSessions(sessions);
    if (sessions[0]) {
      await loadMessages(sessionKeyFromSession(sessions[0]), sessionIdFromSession(sessions[0]));
    }
  }

  function renderMessages(messages) {
    const transcript = document.querySelector(".dx-mobile-transcript");
    if (!transcript) return;
    transcript.textContent = "";

    if (!messages.length) {
      const empty = document.createElement("p");
      empty.textContent = "No messages in this session yet.";
      transcript.append(empty);
      return;
    }

    messages.slice(-40).forEach((message) => {
      const item = document.createElement("article");
      item.className = "companion-message";
      const role = document.createElement("strong");
      role.textContent = message.role || message.kind || "event";
      const body = document.createElement("p");
      body.textContent = message.content || message.text || message.message || "";
      item.append(role, body);
      transcript.append(item);
    });
  }

  async function loadMessages(sessionKey, sessionId) {
    const historyKey = normalizeSessionKey(sessionKey);
    if (!historyKey) return;
    runtimeState.activeSessionKey = historyKey;
    const explicitSessionId = normalizeSessionId(sessionId);
    runtimeState.activeSessionId = explicitSessionId
      ? explicitSessionId.replace(/^gw_/, "")
      : historyKey.startsWith("gw_")
        ? historyKey.replace(/^gw_/, "")
        : "";
    const credential = await loadCredential();
    if (!credential) return;

    const response = await fetch(
      gatewayEndpoint(credential.gateway_origin, `/api/sessions/${encodeURIComponent(historyKey)}/messages`),
      { headers: authorizationHeaders(credential) },
    );
    if (!response.ok) throw new Error(`Messages failed (${response.status})`);
    const data = await response.json();
    renderMessages(data.messages || data.entries || []);
  }

  function closeSocket() {
    if (!runtimeState.socket) return;
    runtimeState.socket.close();
    runtimeState.socket = null;
    setSocketState(socketState.disconnected);
    setMessageFormEnabled(false);
  }

  function buildGatewayConnectFrame(sessionId) {
    return {
      type: "connect",
      session_id: normalizeSessionId(sessionId || runtimeState.activeSessionId),
      device_name: DEVICE_NAME,
      capabilities: MOBILE_CAPABILITIES,
    };
  }

  function connectSocket(credential, agentAlias) {
    closeSocket();
    const sessionId = normalizeSessionId(runtimeState.activeSessionId || crypto.randomUUID());
    const token = normalizeGatewayBearerToken(credential?.token);
    runtimeState.activeSessionId = sessionId;
    const params = new URLSearchParams();
    params.set("session_id", sessionId);
    params.set("agent", normalizeAgentAlias(agentAlias));
    const protocols = ["dx-agents.v1", "zeroclaw.v1", `bearer.${token}`];
    // The token is carried through Sec-WebSocket-Protocol, never through the URL.
    setSocketState(socketState.connecting);
    const socket = new WebSocket(
      gatewayWebSocketEndpoint(credential.gateway_origin, "/ws/chat", params.toString()),
      protocols,
    );
    runtimeState.socket = socket;
    socket.addEventListener("open", (event) => {
      if (event.currentTarget !== runtimeState.socket) return;
      socket.send(JSON.stringify(buildGatewayConnectFrame(sessionId)));
      setSocketState(socketState.connectedLiveChat);
      setMessageFormEnabled(true);
      writeStatus("Connected to DX Agents gateway", "success");
    });
    socket.addEventListener("close", (event) => {
      if (event.currentTarget !== runtimeState.socket) return;
      if (runtimeState.socketState !== socketState.failed) setSocketState(socketState.disconnected);
      setMessageFormEnabled(false);
    });
    socket.addEventListener("error", (event) => {
      if (event.currentTarget !== runtimeState.socket) return;
      setSocketState(socketState.failed);
      setMessageFormEnabled(false);
      writeStatus("Gateway socket failed; check agent alias and pairing", "danger");
    });
    socket.addEventListener("message", (event) => {
      if (event.currentTarget !== runtimeState.socket) return;
      const transcript = document.querySelector(".dx-mobile-transcript");
      if (!transcript) return;
      const row = document.createElement("article");
      row.className = "companion-message";
      const body = document.createElement("p");
      try {
        const payload = JSON.parse(event.data);
        body.textContent = payload.content || payload.text || payload.type || "gateway event";
      } catch {
        body.textContent = String(event.data);
      }
      row.append(body);
      transcript.append(row);
    });
  }

  function applyPairingPreview(preview) {
    if (preview.gateway && gatewayInput()) gatewayInput().value = normalizeGatewayBaseUrl(preview.gateway);
    if (preview.code && pairingCodeInput()) pairingCodeInput().value = normalizePairingCode(preview.code);
    if (preview.agentAlias && agentInput()) agentInput().value = normalizeAgentAlias(preview.agentAlias);
  }

  async function applyScannedPairingPayload(payload) {
    const preview = await parsePairingPayload(payload);
    const textarea = document.querySelector("textarea[name='pairing_payload']");
    if (textarea) textarea.value = "";
    applyPairingPreview(preview);
    return preview;
  }

  async function scanPairingQrWithPlugin() {
    const scanner = barcodeScannerApi();
    if (!scanner || typeof scanner.scan !== "function") {
      return { available: false, reason: "Native barcode scanner plugin is not available in this build" };
    }

    const permission = await ensureScannerPermission(scanner);
    if (permission !== "granted") {
      setScannerState(scannerState.permissionDenied, true);
      return {
        available: false,
        reason: "Camera permission is required. Use manual pairing or enable camera access in app settings.",
      };
    }

    setScannerState(scannerState.scanning, false);
    const scanned = await scanner.scan({
      cameraDirection: "back",
      formats: [scannerQrFormat(scanner)],
    });
    const payload = scanContent(scanned);
    return payload
      ? { available: true, payload }
      : { available: false, reason: "Scanner returned no QR payload" };
  }

  async function handleScanClick() {
    writeStatus("Opening native QR scanner", "neutral");
    let result;
    try {
      result = await scanPairingQrWithPlugin();
    } catch (error) {
      setScannerState(scannerState.error, Boolean(barcodeScannerApi()));
      throw error;
    }
    if (!result || result.available === false) {
      writeStatus(result?.reason || "Camera scanner adapter is not configured", "warning");
      if (!barcodeScannerApi()) setScannerState(scannerState.unavailable, false);
      return;
    }
    const payload = result.raw_payload || result.payload || "";
    if (!payload) {
      writeStatus("Scanner returned no QR payload", "warning");
      setScannerState(scannerState.ready, true);
      return;
    }
    await applyScannedPairingPayload(payload);
    setScannerState(scannerState.scanned, true);
    writeStatus("QR payload ready", "success");
  }

  async function handlePairingSubmit(event) {
    event.preventDefault();
    setPairingState(pairingState.parsing);
    const form = event.currentTarget;
    const formData = new FormData(form);
    const rawPayload = formData.get("pairing_payload") || "";
    const preview = rawPayload ? await parsePairingPayload(rawPayload) : {};
    applyPairingPreview(preview);
    const origin = normalizeGatewayBaseUrl(preview.gateway || formData.get("gateway_origin") || "");
    const code = normalizePairingCode(preview.code || formData.get("pairing_code"));
    const agentAlias = normalizeAgentAlias(preview.agentAlias || formData.get("agent_alias") || DEFAULT_AGENT_ALIAS);

    if (!origin || !code) {
      setPairingState(pairingState.failed);
      writeStatus("Gateway and pairing code are required", "warning");
      return;
    }

    runtimeState.activeSessionKey = normalizeSessionKey(preview.sessionKey);
    runtimeState.activeSessionId = sessionIdFromPairingPreview(preview);
    setPairingState(pairingState.pairing);
    writeStatus("Pairing with desktop", "neutral");
    const paired = await pairWithGateway(origin, code, {
      sessionId: preview.sessionId,
      sessionKey: preview.sessionKey,
    });
    const token = normalizeGatewayBearerToken(paired.token);

    const credential = {
      gateway_origin: origin,
      token,
      device_id: paired.device_id || "dx-mobile",
      paired_at: new Date().toISOString(),
    };

    const persisted = await storeCredential(credential);
    rememberGatewayOrigin(origin);
    rememberAgentAlias(agentAlias);
    await updateCapabilities(credential);
    await loadSessions();
    connectSocket(credential, agentAlias);
    setPairingState(persisted ? pairingState.pairedDurable : pairingState.pairedSessionOnly);
    writeStatus(
      persisted ? "Paired and stored in native secure storage" : "Paired for this session; secure storage not proven",
      persisted ? "success" : "warning",
    );
  }

  function handleMessageSubmit(event) {
    event.preventDefault();
    const form = event.currentTarget;
    const textarea = form.querySelector("textarea[name='message']");
    const content = textarea?.value?.trim();
    if (!content || !runtimeState.socket || runtimeState.socket.readyState !== WebSocket.OPEN) return;
    runtimeState.socket.send(JSON.stringify({ type: "message", content }));
    textarea.value = "";
  }

  async function forgetConnection() {
    closeSocket();
    runtimeState.gatewayOrigin = "";
    runtimeState.activeCredential = null;
    runtimeState.activeSessionId = "";
    runtimeState.activeSessionKey = "";
    setPairingState(pairingState.idle);
    setSocketState(socketState.disconnected);
    sessionStorage.removeItem(SESSION_STORAGE_GATEWAY);
    sessionStorage.removeItem(SESSION_STORAGE_AGENT);
    await invokeNative("dx_mobile_clear_device_credential", {});
    renderSessions([]);
    renderMessages([]);
    writeStatus("Connection forgotten on this device", "neutral");
  }

  async function boot() {
    document.querySelector(".dx-mobile-pairing-form")?.addEventListener("submit", (event) => {
      handlePairingSubmit(event).catch((error) => {
        setPairingState(pairingState.failed);
        writeStatus(error.message, "danger");
      });
    });
    document.querySelector(".dx-mobile-message-form")?.addEventListener("submit", handleMessageSubmit);
    document
      .querySelector("[data-dx-native-command='dx_mobile_scan_pairing_qr']")
      ?.addEventListener("click", () =>
        handleScanClick().catch((error) => {
          setScannerState(scannerState.error, Boolean(barcodeScannerApi()));
          writeStatus(error.message, "danger");
        }),
      );
    document
      .querySelector("[data-dx-forget-connection]")
      ?.addEventListener("click", () => forgetConnection().catch((error) => writeStatus(error.message, "danger")));

    const rememberedOrigin = sessionStorage.getItem(SESSION_STORAGE_GATEWAY);
    if (rememberedOrigin && gatewayInput()) gatewayInput().value = rememberedOrigin;
    const rememberedAgent = sessionStorage.getItem(SESSION_STORAGE_AGENT) || DEFAULT_AGENT_ALIAS;
    if (agentInput()) agentInput().value = rememberedAgent;
    setMessageFormEnabled(false);

    const storeStatus = await readCredentialStoreStatus();
    if (storeStatus.available === false) {
      writeStatus(storeStatus.reason || "Native secure storage adapter is not configured", "warning");
    }
    setScannerState(barcodeScannerApi() ? scannerState.ready : scannerState.unavailable, Boolean(barcodeScannerApi()));
  }

  window.DXMobileCompanionRuntime = {
    mobileCapabilities: MOBILE_CAPABILITIES,
    pairingChallengeTtlSeconds: PAIRING_CHALLENGE_TTL_SECONDS,
    buildGatewayConnectFrame,
    buildPairingRequestBody,
    applyScannedPairingPayload,
    sessionIdFromPairingPreview,
    normalizeGatewayBaseUrl,
    gatewayEndpoint,
    gatewayWebSocketEndpoint,
    parsePairingPayload,
    loadSessions,
    loadMessages,
    forget: forgetConnection,
  };

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", () => boot().catch((error) => writeStatus(error.message, "danger")));
  } else {
    boot().catch((error) => writeStatus(error.message, "danger"));
  }
})();
