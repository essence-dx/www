
(() => {
  if (window.__DX_DEVTOOLS_ACTIVE__) return;
  window.__DX_DEVTOOLS_ACTIVE__ = true;

  const ROOT_ID = "dx-devtools-root";
  const ENDPOINTS = {
    session: "/_dx/devtools/session",
    route: "/_dx/devtools/route",
    diagnostics: "/_dx/devtools/diagnostics",
    sourceMap: "/_dx/devtools/source-map",
    stylePreview: "/_dx/devtools/style-preview",
    styleApply: "/_dx/devtools/style-apply",
    styleUndo: "/_dx/devtools/style-undo",
    cssData: "/_dx/devtools/css-data"
  };
  const EDGES = ["left", "top", "right", "bottom"];
  const TABS = ["style", "layout", "spacing", "responsive", "source"];
  const CSS_GROUPS = {
    text: ["color", "font-family", "font-size", "font-weight", "font-style", "line-height", "letter-spacing", "text-align", "text-decoration-line", "text-transform", "white-space"],
    box: ["display", "position", "box-sizing", "width", "height", "min-width", "max-width", "min-height", "max-height", "overflow", "overflow-x", "overflow-y"],
    spacing: ["margin-top", "margin-right", "margin-bottom", "margin-left", "padding-top", "padding-right", "padding-bottom", "padding-left", "gap", "row-gap", "column-gap"],
    border: ["border-top-width", "border-right-width", "border-bottom-width", "border-left-width", "border-style", "border-color", "border-radius", "outline-width", "outline-color", "outline-style"],
    paint: ["background", "background-color", "background-image", "background-size", "background-position", "background-repeat", "opacity", "box-shadow", "filter", "backdrop-filter"],
    layout: ["inset", "top", "right", "bottom", "left", "z-index", "flex-direction", "justify-content", "align-items", "align-content", "grid-template-columns", "grid-template-rows"]
  };
  const SOLID_PRESETS = [
    ["geist", "#ededed"],
    ["gray", "#a1a1aa"],
    ["blue", "#3291ff"],
    ["cyan", "#50e3c2"],
    ["green", "#27c93f"],
    ["amber", "#f5a623"],
    ["red", "#ff5f56"],
    ["pink", "#ff0080"]
  ];
  const GRADIENT_PRESETS = [
    ["vercel glow", "linear-gradient(135deg, #ffffff 0%, #888888 45%, #111111 100%)"],
    ["blue steel", "linear-gradient(135deg, #0070f3 0%, #00dfd8 100%)"],
    ["pink flash", "linear-gradient(135deg, #ff0080 0%, #7928ca 100%)"],
    ["solar", "linear-gradient(135deg, #f5a623 0%, #ff4d4f 100%)"],
    ["radial focus", "radial-gradient(circle at 35% 30%, #ffffff 0%, #3291ff 36%, #000000 100%)"],
    ["mesh night", "radial-gradient(circle at 20% 20%, #3291ff 0 18%, transparent 38%), radial-gradient(circle at 80% 30%, #ff0080 0 16%, transparent 36%), radial-gradient(circle at 45% 80%, #50e3c2 0 18%, transparent 40%), linear-gradient(135deg, #000000, #111111)"]
  ];
  const STYLE_PROPERTY_OPTIONS = [
    "color",
    "background-color",
    "background-image",
    "border-color",
    "border-radius",
    "box-shadow",
    "display",
    "gap",
    "font-size",
    "font-weight",
    "line-height",
    "margin",
    "padding",
    "width",
    "height",
    "opacity",
    "transform"
  ];
  const STYLE_VALUE_OPTIONS = {
    display: ["block", "inline", "inline-block", "flex", "inline-flex", "grid", "none"],
    position: ["static", "relative", "absolute", "fixed", "sticky"],
    "font-weight": ["400", "500", "600", "700", "800", "900"],
    "text-align": ["left", "center", "right", "justify", "start", "end"],
    "border-style": ["none", "solid", "dashed", "dotted", "double"],
    "background-repeat": ["repeat", "no-repeat", "repeat-x", "repeat-y", "round", "space"],
    "background-size": ["auto", "cover", "contain"],
    "white-space": ["normal", "nowrap", "pre", "pre-wrap", "break-spaces"],
    "text-transform": ["none", "uppercase", "lowercase", "capitalize"]
  };
  const COLOR_PROPERTIES = new Set(["color", "background-color", "border-color", "outline-color", "fill", "stroke"]);
  const READINESS_VISUAL_EDIT_REPLAY_CURRENT = "current";
  const READINESS_VISUAL_EDIT_REPLAY_MISSING = "missing";
  const READINESS_VISUAL_EDIT_PHASES = ["inspect", "cascade", "preview", "apply", "undo", "receipt"];
  const STATE = {
    openPanels: new Set(),
    lastClosedEdge: null,
    edgeCooldownUntil: 0,
    activeTab: "style",
    puck: { x: 348, y: 104 },
    dragging: false,
    dragMoved: false,
    menuOpen: false,
    inspectMode: false,
    colorPopoverOpen: false,
    propertyPopoverOpen: false,
    valuePopoverOpen: false,
    activePopoverIndex: {},
    popoverQuery: {},
    colorMode: "solid",
    activeColorStop: "stopA",
    colorPicker: {
      red: 237,
      green: 237,
      blue: 237,
      alpha: 100,
      hue: 0,
      saturation: 0,
      lightness: 93,
      linearAngle: 135,
      radialShape: "circle",
      radialX: 35,
      radialY: 35,
      stopA: "#ffffff",
      stopB: "#3291ff",
      stopC: "#000000",
      stopAOffset: 0,
      stopBOffset: 52,
      stopCOffset: 100,
      meshX1: 18,
      meshY1: 24,
      meshX2: 82,
      meshY2: 28,
      meshX3: 50,
      meshY3: 82,
      meshSize1: 36,
      meshSize2: 36,
      meshSize3: 40
    },
    styleDraft: { property: "color", value: "#ededed" },
    selectedElement: null,
    selectedSnapshot: null,
    session: null,
    route: null,
    diagnostics: null,
    cssData: null,
    cssDataByProperty: new Map(),
    styleResult: null,
    sourceResult: null,
    readinessVisualEdit: {
      phases: new Set(),
      inspectedElementPresent: false,
      cascadeInspected: false,
      previewProtocolOk: false,
      applyProtocolOk: false,
      undoProtocolOk: false,
      previewSourceMutated: false,
      applySourceMutated: false,
      undoSourceRestored: false,
      safeLocalSourceTargetKnown: false,
      applyReceiptWritten: false,
      undoReceiptWritten: false,
      receiptDurability: null,
      inspectedSelector: null,
      inspectedElementFingerprint: null,
      sourceTarget: null,
      styleProperty: null,
      styleValue: null,
      computedStyleBefore: null,
      computedStyleAfterPreview: null,
      computedStyleAfterUndo: null,
      inlineStyleBefore: null
    },
    runtimeIssues: []
  };
  let root;
  let puck;
  let menu;
  let overlay;
  let renderFrame = 0;
  let inspectedTarget = null;
  let lastSourceLookupKey = "";
  const controller = new AbortController();
  const listenerSignal = controller.signal;

  const cssEscape = (value) => {
    if (window.CSS && typeof window.CSS.escape === "function") return window.CSS.escape(value);
    return String(value).replace(/[^a-zA-Z0-9_-]/g, "\\$&");
  };

  const el = (tag, attrs = {}, children = []) => {
    const node = document.createElement(tag);
    for (const [name, value] of Object.entries(attrs)) {
      if (value === false || value == null) continue;
      if (name === "className") node.className = value;
      else if (name === "text") node.textContent = value;
      else if (name === "html") node.innerHTML = value;
      else if (name.startsWith("on") && typeof value === "function") {
        node.addEventListener(name.slice(2).toLowerCase(), value);
      } else {
        node.setAttribute(name, value === true ? "" : String(value));
      }
    }
    for (const child of children) {
      node.append(child && child.nodeType ? child : document.createTextNode(String(child ?? "")));
    }
    return node;
  };

  const text = (value, fallback = "preview-only / not writable") => {
    if (value == null || value === "") return fallback;
    return String(value);
  };

  const pathOnly = () => window.location.pathname || "/";
  const routeQuery = () => encodeURIComponent(pathOnly());

  async function requestJson(url, options = {}) {
    const response = await fetch(url, {
      cache: "no-store",
      credentials: "same-origin",
      ...options,
      headers: {
        accept: "application/json",
        ...(options.body ? { "content-type": "application/json" } : {}),
        ...(options.headers || {})
      }
    });
    const body = await response.json().catch(() => ({
      schema: "dx.devtools.invalid_json",
      status: response.status
    }));
    return { ok: response.ok, status: response.status, body };
  }

  async function refreshProtocol() {
    const [session, route, diagnostics, cssData] = await Promise.all([
      requestJson(ENDPOINTS.session),
      requestJson(`${ENDPOINTS.route}?route=${routeQuery()}`),
      requestJson(ENDPOINTS.diagnostics),
      requestJson(ENDPOINTS.cssData).catch((error) => ({
        ok: false,
        body: { schema: "dx.devtools.css_data_unavailable", message: error.message }
      }))
    ]);
    STATE.session = session.body;
    STATE.route = route.body;
    STATE.diagnostics = diagnostics.body;
    if (cssData.ok && cssData.body?.schema === "dx.devtools.css_data") {
      STATE.cssData = cssData.body;
      STATE.cssDataByProperty = new Map((cssData.body.properties || []).map((entry) => [entry.name, entry]));
    }
    renderAll();
  }

  function sourceTargetFromElement(target) {
    if (!target) return null;
    const sourcePath =
      target.getAttribute("data-dx-source-path") ||
      target.getAttribute("data-dx-source") ||
      target.dataset.dxSourcePath ||
      target.dataset.dxSource;
    if (!sourcePath) return null;

    const startText =
      target.getAttribute("data-dx-source-start-byte") ||
      target.getAttribute("data-dx-source-start") ||
      target.dataset.dxSourceStartByte ||
      target.dataset.dxSourceStart;
    const endText =
      target.getAttribute("data-dx-source-end-byte") ||
      target.getAttribute("data-dx-source-end") ||
      target.dataset.dxSourceEndByte ||
      target.dataset.dxSourceEnd;
    const expectedText =
      target.getAttribute("data-dx-source-expected-text") ||
      target.dataset.dxSourceExpectedText ||
      null;
    const startByte = Number(startText);
    const endByte = Number(endText);
    const kind =
      target.getAttribute("data-dx-source-kind") ||
      target.dataset.dxSourceKind ||
      (sourcePath.endsWith(".css") ? "authored-css" : "unknown");

    if (Number.isInteger(startByte) && Number.isInteger(endByte) && expectedText != null) {
      return {
        relativePath: sourcePath,
        kind,
        range: { startByte, endByte, expectedText }
      };
    }

    return {
      relativePath: sourcePath,
      kind,
      range: null
    };
  }

  function selectorFor(target) {
    if (!target) return "preview-only / not writable";
    if (target.id) return `#${cssEscape(target.id)}`;
    const sourceName = target.getAttribute("data-dx-source") || target.getAttribute("data-dx-component");
    if (sourceName) return `[data-dx-source="${sourceName.replaceAll('"', '\\"')}"]`;
    const classes = Array.from(target.classList || []).filter(Boolean).slice(0, 3);
    if (classes.length) return `${target.tagName.toLowerCase()}.${classes.map(cssEscape).join(".")}`;
    const parent = target.parentElement;
    if (!parent) return target.tagName.toLowerCase();
    const index = Array.from(parent.children).indexOf(target) + 1;
    return `${target.tagName.toLowerCase()}:nth-child(${index})`;
  }

  function selectedAttributes(target) {
    const output = {};
    for (const attribute of Array.from(target.attributes || [])) {
      if (
        attribute.name === "id" ||
        attribute.name === "class" ||
        attribute.name.startsWith("data-dx-") ||
        attribute.name.startsWith("aria-")
      ) {
        output[attribute.name] = attribute.value;
      }
    }
    return output;
  }

  function elementStatePayload(target) {
    if (!target) return null;
    const state = {};
    if ("value" in target) state.value = target.value;
    if ("checked" in target) state.checked = target.checked;
    if ("selectedIndex" in target) state.selectedIndex = target.selectedIndex;
    if (target.isContentEditable) state.textContent = target.textContent || "";
    return Object.keys(state).length ? state : null;
  }

  function computedPayload(target) {
    const styles = target ? window.getComputedStyle(target) : null;
    const properties = {};
    const styleNames = styles ? Array.from(styles).sort() : null;
    const names = styleNames ? styleNames.filter(Boolean) : [...new Set(Object.values(CSS_GROUPS).flat())].sort();
    for (const property of names) {
      properties[property] = styles ? styles.getPropertyValue(property) : "";
    }
    return {
      properties,
      all: names.map((name) => ({ name, value: properties[name] || "" })),
      groups: Object.fromEntries(Object.entries(CSS_GROUPS).map(([name, keys]) => [
        name,
        Object.fromEntries(keys.map((key) => [key, properties[key] || ""]))
      ]))
    };
  }

  const numberCss = (styles, property) => {
    const value = parseFloat(styles.getPropertyValue(property));
    return Number.isFinite(value) ? value : 0;
  };

  function boxModelPayload(target) {
    const rect = target ? target.getBoundingClientRect() : { x: 0, y: 0, width: 0, height: 0 };
    const styles = target ? window.getComputedStyle(target) : null;
    const edge = (prefix) => {
      const property = (side) => `${prefix}-${side}${prefix === "border" ? "-width" : ""}`;
      const read = (side) => {
        const name = property(side);
        const value = styles ? styles.getPropertyValue(name) : "0px";
        return { value: value || "0px", px: styles ? numberCss(styles, name) : 0 };
      };
      return {
        top: read("top"),
        right: read("right"),
        bottom: read("bottom"),
        left: read("left")
      };
    };
    const edgePx = (values, side) => values?.[side]?.px || 0;
    const padding = edge("padding");
    const border = edge("border");
    const margin = edge("margin");
    const contentWidth = styles
      ? Math.max(0, rect.width - edgePx(padding, "left") - edgePx(padding, "right") - edgePx(border, "left") - edgePx(border, "right"))
      : rect.width;
    const contentHeight = styles
      ? Math.max(0, rect.height - edgePx(padding, "top") - edgePx(padding, "bottom") - edgePx(border, "top") - edgePx(border, "bottom"))
      : rect.height;
    const borderBox = { x: rect.x, y: rect.y, width: rect.width, height: rect.height };
    const paddingBox = {
      x: rect.x + edgePx(border, "left"),
      y: rect.y + edgePx(border, "top"),
      width: Math.max(0, rect.width - edgePx(border, "left") - edgePx(border, "right")),
      height: Math.max(0, rect.height - edgePx(border, "top") - edgePx(border, "bottom"))
    };
    const contentBox = {
      x: rect.x + edgePx(border, "left") + edgePx(padding, "left"),
      y: rect.y + edgePx(border, "top") + edgePx(padding, "top"),
      width: contentWidth,
      height: contentHeight
    };
    const marginBox = {
      x: rect.x - edgePx(margin, "left"),
      y: rect.y - edgePx(margin, "top"),
      width: rect.width + edgePx(margin, "left") + edgePx(margin, "right"),
      height: rect.height + edgePx(margin, "top") + edgePx(margin, "bottom")
    };
    return {
      marginBox,
      borderBox,
      paddingBox,
      contentBox,
      content: contentBox,
      padding,
      border,
      margin
    };
  }

  function boxEdgePx(edges, side) {
    const value = edges?.[side];
    if (typeof value === "number") return value;
    if (Number.isFinite(value?.px)) return value.px;
    const parsed = parseFloat(value?.value);
    return Number.isFinite(parsed) ? parsed : 0;
  }

  function boxRectPayload(rect) {
    return {
      x: Number.isFinite(rect?.x) ? rect.x : 0,
      y: Number.isFinite(rect?.y) ? rect.y : 0,
      width: Number.isFinite(rect?.width) ? rect.width : 0,
      height: Number.isFinite(rect?.height) ? rect.height : 0
    };
  }

  function styleOperationBoxModelPayload(model) {
    const box = model || boxModelPayload(null);
    const edge = (name) => ({
      top: boxEdgePx(box[name], "top"),
      right: boxEdgePx(box[name], "right"),
      bottom: boxEdgePx(box[name], "bottom"),
      left: boxEdgePx(box[name], "left")
    });
    return {
      content: boxRectPayload(box.content || box.contentBox),
      padding: edge("padding"),
      border: edge("border"),
      margin: edge("margin")
    };
  }

  function breakpointLabel() {
    const width = window.innerWidth;
    if (width >= 1280) return "xl";
    if (width >= 1024) return "lg";
    if (width >= 768) return "md";
    if (width >= 640) return "sm";
    return "base";
  }

  function captureElement(target) {
    if (!target || root.contains(target) || target === document.documentElement || target === document.body) {
      return;
    }
    const chain = [];
    let cursor = target;
    while (cursor && cursor !== document.body && chain.length < 7) {
      chain.push({
        depth: chain.length,
        selector: selectorFor(cursor),
        tagName: cursor.tagName.toLowerCase(),
        id: cursor.id || null,
        classes: Array.from(cursor.classList || []).slice(0, 8),
        dataDxComponent: cursor.getAttribute("data-dx-component"),
        dataDxEditableSection: cursor.getAttribute("data-dx-editable-section"),
        sourceTarget: sourceTargetFromElement(cursor)
      });
      cursor = cursor.parentElement;
    }
    const sourceTarget = sourceTargetFromElement(target);
    STATE.selectedElement = target;
    STATE.selectedSnapshot = {
      schema: "dx.visual_edit.element_selection",
      format: 1,
      selectedElement: {
        stableId: target.id || target.getAttribute("data-dx-component") || selectorFor(target),
        selector: selectorFor(target),
        tagName: target.tagName.toLowerCase(),
        id: target.id || null,
        classes: Array.from(target.classList || []).slice(0, 8),
        attributes: selectedAttributes(target),
        state: elementStatePayload(target),
        sourceTarget
