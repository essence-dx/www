(() => {
  const root = document.querySelector("[data-whiteboard-route]");
  if (!root || root.dataset.whiteboardRuntimeActive === "true") return;
  root.dataset.whiteboardRuntimeActive = "true";

  const stage = root.querySelector("[data-whiteboard-stage]");
  const svg = root.querySelector(".whiteboard-svg-preview");
  const scrollHost = root.querySelector(".wb-canvas-scroll");
  const textEditor = root.querySelector("[data-whiteboard-text-editor]");
  if (!stage || !svg) return;

  const SVG_NS = "http://www.w3.org/2000/svg";
  const STORAGE_KEY = "dx.whiteboard.example.document.workspace";
  const CREATE_THRESHOLD = 8;
  const MAX_INLINE_MEDIA_BYTES = 8 * 1024 * 1024;
  const DEFAULT_TEXT_FONT = "JetBrains Mono";
  const TEXT_FONT_FALLBACK = "'JetBrains Mono', ui-monospace, SFMono-Regular, Consolas, monospace";

  const toolLabels = {
    select: "Move",
    hand: "Hand",
    freehand: "Pen",
    line: "Line",
    arrow: "Arrow",
    rectangle: "Rectangle",
    diamond: "Diamond",
    ellipse: "Ellipse",
    table: "Table",
    math: "Math",
    text: "Text",
    icon: "Icon",
    image: "Image",
    audio: "Audio",
    video: "Video"
  };

  const shapeTools = new Set(["rectangle", "diamond", "ellipse", "line", "arrow", "text", "table", "math"]);
  const shapeToolIcons = {
    rectangle: "whiteboard:rectangle",
    diamond: "whiteboard:diamond",
    ellipse: "whiteboard:ellipse",
    line: "whiteboard:line",
    arrow: "whiteboard:arrow",
    text: "whiteboard:text",
    table: "whiteboard:table",
    math: "whiteboard:math"
  };

  const dxIconBodies = {
    align: '<path d="M12 3v18"/><rect width="6" height="4" x="3" y="5" rx="1"/><rect width="6" height="4" x="15" y="15" rx="1"/>',
    arrow: '<path d="M5 12h14"/><path d="m12 5 7 7-7 7"/>',
    audio: '<path d="M11 5 6 9H2v6h4l5 4Z"/><path d="M15.54 8.46a5 5 0 0 1 0 7.07"/><path d="M19.07 4.93a10 10 0 0 1 0 14.14"/>',
    copy: '<rect width="14" height="14" x="8" y="8" rx="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>',
    diamond: '<path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.4l7.6 7.6a2.41 2.41 0 0 0 3.4 0l7.6-7.6a2.41 2.41 0 0 0 0-3.4l-7.6-7.6a2.41 2.41 0 0 0-3.4 0Z"/>',
    download: '<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="M7 10l5 5 5-5"/><path d="M12 15V3"/>',
    ellipse: '<ellipse cx="12" cy="12" rx="9" ry="6"/>',
    eraser: '<path d="m7 21-4-4 9.5-9.5 4 4L7 21Z"/><path d="m14.5 5.5 2-2a2.12 2.12 0 0 1 3 3l-2 2"/><path d="M12 21h9"/>',
    eye: '<path d="M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7S2 12 2 12Z"/><circle cx="12" cy="12" r="3"/>',
    "eye-off": '<path d="m3 3 18 18"/><path d="M10.6 10.6A3 3 0 0 0 14 14"/><path d="M9.9 5.2A9.8 9.8 0 0 1 12 5c6.5 0 10 7 10 7a18.6 18.6 0 0 1-2.2 3.2"/><path d="M6.1 6.1C3.4 7.9 2 12 2 12s3.5 7 10 7a9.7 9.7 0 0 0 4.1-.9"/>',
    fit: '<path d="M8 3H5a2 2 0 0 0-2 2v3"/><path d="M21 8V5a2 2 0 0 0-2-2h-3"/><path d="M3 16v3a2 2 0 0 0 2 2h3"/><path d="M16 21h3a2 2 0 0 0 2-2v-3"/>',
    grid: '<path d="M3 3h18v18H3z"/><path d="M3 9h18"/><path d="M3 15h18"/><path d="M9 3v18"/><path d="M15 3v18"/>',
    hand: '<path d="M18 11V6a2 2 0 0 0-4 0"/><path d="M14 10V4a2 2 0 0 0-4 0v2"/><path d="M10 10.5V6a2 2 0 0 0-4 0v8"/><path d="M18 8a2 2 0 1 1 4 0v6a8 8 0 0 1-8 8h-2c-2.8 0-4.8-1-6-3l-2.7-4.5a2 2 0 0 1 3.4-2L8 14"/>',
    icons: '<path d="M8.3 10a.7.7 0 0 1-.63-1.08L11.4 3.5a.7.7 0 0 1 1.25.03l3.58 5.42a.7.7 0 0 1-.59 1.05Z"/><rect width="7" height="7" x="3" y="14" rx="1"/><circle cx="17.5" cy="17.5" r="3.5"/>',
    image: '<rect width="18" height="18" x="3" y="3" rx="2"/><circle cx="9" cy="9" r="2"/><path d="m21 15-3.1-3.1a2 2 0 0 0-2.8 0L6 21"/>',
    keyboard: '<rect width="20" height="16" x="2" y="4" rx="2"/><path d="M6 8h.01"/><path d="M10 8h.01"/><path d="M14 8h.01"/><path d="M18 8h.01"/><path d="M8 12h.01"/><path d="M12 12h.01"/><path d="M16 12h.01"/><path d="M7 16h10"/>',
    library: '<path d="M4 19.5V5a2 2 0 0 1 2-2h14v18H6a2 2 0 0 1-2-1.5Z"/><path d="M8 7h8"/><path d="M8 11h8"/><path d="M8 15h5"/>',
    line: '<path d="M7 17 17 7"/>',
    lock: '<rect width="18" height="11" x="3" y="11" rx="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/>',
    math: '<path d="M18 7V4H6l6 8-6 8h12v-3"/>',
    move: '<path d="M12 2v20"/><path d="m15 19-3 3-3-3"/><path d="m15 5-3-3-3 3"/><path d="M2 12h20"/><path d="m19 15 3-3-3-3"/><path d="m5 15-3-3 3-3"/>',
    panel: '<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M15 3v18"/>',
    pen: '<path d="M21.17 6.81a1 1 0 0 0-3.98-3.98L3.84 16.17a2 2 0 0 0-.5.83L2 22l5-1.34a2 2 0 0 0 .83-.5Z"/><path d="m15 5 4 4"/>',
    rectangle: '<rect width="18" height="12" x="3" y="6" rx="2"/>',
    redo: '<path d="m15 14 5-5-5-5"/><path d="M20 9H9.5A5.5 5.5 0 0 0 4 14.5 5.5 5.5 0 0 0 9.5 20H13"/>',
    select: '<path d="m4 4 7.07 17 2.51-7.39L21 11.07Z"/><path d="m11.07 11.07 5.66 5.66"/>',
    snap: '<path d="M4 6V4h2"/><path d="M18 4h2v2"/><path d="M20 18v2h-2"/><path d="M6 20H4v-2"/><path d="M9 12h6"/><path d="M12 9v6"/>',
    table: '<rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 9h18"/><path d="M3 15h18"/><path d="M12 3v18"/>',
    text: '<path d="M4 7V5a1 1 0 0 1 1-1h14a1 1 0 0 1 1 1v2"/><path d="M12 4v16"/><path d="M9 20h6"/>',
    trash: '<path d="M3 6h18"/><path d="M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/><path d="m19 6-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"/><path d="M10 11v6"/><path d="M14 11v6"/>',
    undo: '<path d="m9 14-5-5 5-5"/><path d="M4 9h10.5A5.5 5.5 0 0 1 20 14.5 5.5 5.5 0 0 1 14.5 20H11"/>',
    unlock: '<rect width="18" height="11" x="3" y="11" rx="2"/><path d="M7 11V7a5 5 0 0 1 9.9-1"/>',
    video: '<rect width="14" height="12" x="2" y="6" rx="2"/><path d="m16 13 5.2 3.5a.5.5 0 0 0 .8-.4V7.9a.5.5 0 0 0-.8-.4L16 11"/>',
    "zoom-in": '<circle cx="11" cy="11" r="8"/><path d="M21 21l-4.3-4.3"/><path d="M11 8v6"/><path d="M8 11h6"/>',
    "zoom-out": '<circle cx="11" cy="11" r="8"/><path d="M21 21l-4.3-4.3"/><path d="M8 11h6"/>'
  };

  const swatches = {
    ink: { type: "solid", value: "hsl(0 0% 96%)" },
    graphite: { type: "solid", value: "hsl(0 0% 58%)" },
    sky: { type: "solid", value: "hsl(204 88% 55%)" },
    violet: { type: "solid", value: "hsl(258 90% 66%)" },
    amber: { type: "solid", value: "hsl(38 92% 50%)" },
    mint: { type: "solid", value: "hsl(156 70% 45%)" },
    rose: { type: "solid", value: "hsl(348 84% 62%)" },
    transparent: { type: "solid", value: "transparent" },
    aurora: { type: "linear", value: "linear-gradient(135deg, #00c6ff 0%, #0072ff 100%)" },
    sunset: { type: "linear", value: "linear-gradient(135deg, #ff7e5f 0%, #feb47b 100%)" },
    ocean: { type: "linear", value: "linear-gradient(135deg, #2193b0 0%, #6dd5ed 100%)" },
    candy: { type: "linear", value: "linear-gradient(135deg, #f857a6 0%, #ff5858 100%)" },
    forest: { type: "linear", value: "linear-gradient(135deg, #11998e 0%, #38ef7d 100%)" },
    grape: { type: "linear", value: "linear-gradient(135deg, #7f00ff 0%, #e100ff 100%)" },
    fire: { type: "linear", value: "linear-gradient(135deg, #f12711 0%, #f5af19 100%)" },
    steel: { type: "linear", value: "linear-gradient(135deg, #485563 0%, #29323c 100%)" },
    custom: { type: "solid", value: "#ffffff" }
  };

  const swatchLabels = {
    ink: "Ink",
    graphite: "Graphite",
    sky: "Sky",
    violet: "Violet",
    amber: "Amber",
    mint: "Mint",
    rose: "Rose",
    transparent: "Transparent",
    aurora: "Aurora",
    sunset: "Sunset",
    ocean: "Ocean",
    candy: "Candy",
    forest: "Forest",
    grape: "Grape",
    fire: "Fire",
    steel: "Steel",
    custom: "Custom"
  };

  const toolHelp = {
    select: "Move (V). Click once to select and drag objects. Double-click an object for controls.",
    hand: "Hand (H). Drag the canvas without changing objects.",
    freehand: "Pen (P). Drag to draw freehand strokes.",
    line: "Line (L). Drag to draw a straight connector.",
    arrow: "Arrow (A). Drag to draw an arrow connector.",
    rectangle: "Rectangle (R). Drag to create a rectangle.",
    diamond: "Diamond (D). Drag to create a diamond.",
    ellipse: "Ellipse (O). Drag to create an ellipse.",
    table: "Table (B). Drag to create a table grid.",
    math: "Math (M). Click to insert math text, then double-click to edit.",
    text: "Text (T). Click to insert text, then double-click to edit."
  };

  const commandHelp = {
    clear: "Clear board. Removes every object from the canvas.",
    undo: "Undo (Ctrl+Z). Revert the last whiteboard change.",
    redo: "Redo (Ctrl+Y or Ctrl+Shift+Z). Restore the next change."
  };

  const toggleHelp = {
    "side-panel": "Panel. Show position, size, colors, layers, and library.",
    shortcuts: "Shortcuts (?). Show the keyboard command sheet.",
    minimap: "Minimap. Toggle a small overview of the board."
  };

  const zoomHelp = {
    in: "Zoom in. Increase the canvas scale.",
    out: "Zoom out. Decrease the canvas scale.",
    fit: "Fit canvas. Reset the canvas zoom to 100%."
  };

  const state = {
    tool: "select",
    selectedId: "",
    controllerId: "",
    activeSwatch: "aurora",
    paintMode: "linear",
    customSolid: "#ffffff",
    customGradientStart: "#00c6ff",
    customGradientEnd: "#0072ff",
    customGradientAngle: 135,
    sidePanelOpen: false,
    shortcutPanelOpen: false,
    shapeMenuOpen: false,
    iconMenuOpen: false,
    currentShapeTool: "rectangle",
    editingTextId: "",
    editingOriginalText: "",
    minimapVisible: false,
    zoom: 1,
    nextId: 1,
    drag: null,
    stepHold: null,
    history: [],
    future: []
  };

  function nodes(selector) {
    return Array.from(root.querySelectorAll(selector));
  }

  function svgNodes(selector) {
    return Array.from(svg.querySelectorAll(selector));
  }

  function markDxIcons() {
    for (const icon of nodes("[data-dx-icon], dx-icon[name]")) {
      const rendered = renderDxIcon(icon);
      if (!rendered) continue;
      rendered.dataset.iconSource = "dx-icons";
      rendered.setAttribute("aria-hidden", "true");
    }
  }

  function setAttribute(node, name, value) {
    if (value == null) node.removeAttribute(name);
    else node.setAttribute(name, String(value));
  }

  function createSvg(tag, attrs) {
    const node = document.createElementNS(SVG_NS, tag);
    for (const [name, value] of Object.entries(attrs || {})) setAttribute(node, name, value);
    return node;
  }

  function createHtml(tag, attrs) {
    const node = document.createElement(tag);
    for (const [name, value] of Object.entries(attrs || {})) {
      if (name === "class") node.className = value;
      else if (name === "text") node.textContent = value;
      else setAttribute(node, name, value);
    }
    return node;
  }

  function splitDxIconToken(token) {
    const [set, name] = String(token || "").split(":");
    if (set !== "whiteboard" || !dxIconBodies[name]) return { token: "whiteboard:select", set: "whiteboard", name: "select" };
    return { token: `whiteboard:${name}`, set, name };
  }

  function createDxIcon(token, className) {
    const icon = splitDxIconToken(token);
    const node = createSvg("svg", {
      class: className || "wb-icon",
      fill: "none",
      stroke: "currentColor",
      viewBox: "0 0 24 24",
      "aria-hidden": "true",
      "data-icon-source": "dx-icons",
      "data-dx-icon": icon.token,
      "data-dx-icon-set": icon.set,
      "data-dx-icon-name": icon.name
    });
    node.innerHTML = dxIconBodies[icon.name];
    return node;
  }

  function renderDxIcon(node, token) {
    if (!node) return null;
    const icon = splitDxIconToken(token || node.getAttribute("data-dx-icon") || node.getAttribute("name"));
    if (node.namespaceURI === SVG_NS || node.tagName?.toLowerCase() === "svg") {
      node.setAttribute("class", node.getAttribute("class") || "wb-icon");
      node.setAttribute("fill", "none");
      node.setAttribute("stroke", "currentColor");
      node.setAttribute("viewBox", "0 0 24 24");
      node.setAttribute("aria-hidden", "true");
      node.setAttribute("data-icon-source", "dx-icons");
      node.setAttribute("data-dx-icon", icon.token);
      node.setAttribute("data-dx-icon-set", icon.set);
      node.setAttribute("data-dx-icon-name", icon.name);
      node.innerHTML = dxIconBodies[icon.name];
      return node;
    }
    const replacement = createDxIcon(icon.token, node.getAttribute("class") || "wb-icon");
    for (const attribute of Array.from(node.attributes || [])) {
      if (attribute.name.startsWith("data-whiteboard-")) replacement.setAttribute(attribute.name, attribute.value);
      if (attribute.name === "aria-label") replacement.setAttribute(attribute.name, attribute.value);
    }
    node.replaceWith(replacement);
    return replacement;
  }

  function round(value) {
    return Math.round(value * 10) / 10;
  }

  function clamp(value, min, max) {
    return Math.max(min, Math.min(max, value));
  }

  function distanceBetween(a, b) {
    return Math.hypot((b.x || 0) - (a.x || 0), (b.y || 0) - (a.y || 0));
  }

  function pointFromEvent(event) {
    const point = svg.createSVGPoint();
    point.x = event.clientX;
    point.y = event.clientY;
    const matrix = svg.getScreenCTM();
    if (!matrix) return { x: event.clientX, y: event.clientY };
    const local = point.matrixTransform(matrix.inverse());
    return { x: local.x, y: local.y };
  }

  function safeSelector(id) {
    return window.CSS?.escape ? CSS.escape(id) : id.replace(/[^a-zA-Z0-9_-]/g, "\\$&");
  }

  function objectNodes(id) {
    if (!id) return [];
    const escaped = safeSelector(id);
    return svgNodes(`[data-whiteboard-object-id="${escaped}"], [data-whiteboard-owned-by="${escaped}"]`);
  }

  function primaryObject(id) {
    return objectNodes(id).find((node) => node.hasAttribute("data-whiteboard-object-id")) || null;
  }

  function selectedPrimary() {
    return primaryObject(state.selectedId);
  }

  function objectOffset(id) {
    const node = primaryObject(id);
    return {
      x: Number(node?.dataset.whiteboardOffsetX || 0),
      y: Number(node?.dataset.whiteboardOffsetY || 0)
    };
  }

  function setObjectOffset(id, x, y) {
    for (const node of objectNodes(id)) {
      node.dataset.whiteboardOffsetX = String(x);
      node.dataset.whiteboardOffsetY = String(y);
      node.setAttribute("transform", `translate(${round(x)} ${round(y)})`);
    }
  }

  function objectBounds(id) {
    const items = objectNodes(id).filter((node) => typeof node.getBBox === "function");
    if (!items.length) return null;
    let minX = Number.POSITIVE_INFINITY;
    let minY = Number.POSITIVE_INFINITY;
    let maxX = Number.NEGATIVE_INFINITY;
    let maxY = Number.NEGATIVE_INFINITY;
    for (const node of items) {
      const box = node.getBBox();
      const offset = {
        x: Number(node.dataset.whiteboardOffsetX || 0),
        y: Number(node.dataset.whiteboardOffsetY || 0)
      };
      minX = Math.min(minX, box.x + offset.x);
      minY = Math.min(minY, box.y + offset.y);
      maxX = Math.max(maxX, box.x + box.width + offset.x);
      maxY = Math.max(maxY, box.y + box.height + offset.y);
    }
    return Number.isFinite(minX)
      ? { x: minX, y: minY, width: maxX - minX, height: maxY - minY }
      : null;
  }

  function inspectBounds(id) {
    const node = primaryObject(id);
    if (!node) return objectBounds(id);
    const offset = objectOffset(id);
    if (node.tagName === "rect") {
      return {
        x: Number(node.getAttribute("x") || 0) + offset.x,
        y: Number(node.getAttribute("y") || 0) + offset.y,
        width: Number(node.getAttribute("width") || 0),
        height: Number(node.getAttribute("height") || 0)
      };
    }
    if (node.tagName === "ellipse") {
      const rx = Number(node.getAttribute("rx") || 0);
      const ry = Number(node.getAttribute("ry") || 0);
      return {
        x: Number(node.getAttribute("cx") || 0) - rx + offset.x,
        y: Number(node.getAttribute("cy") || 0) - ry + offset.y,
        width: rx * 2,
        height: ry * 2
      };
    }
    return objectBounds(id);
  }

  function ensureDefs() {
    let defs = svg.querySelector("defs");
    if (!defs) {
      defs = createSvg("defs");
      svg.insertBefore(defs, svg.firstChild || null);
    }
    return defs;
  }

  function parseGradient(value) {
    const body = value.slice(value.indexOf("(") + 1, value.lastIndexOf(")"));
    const parts = body.split(",").map((part) => part.trim()).filter(Boolean);
    let angle = 135;
    let stopParts = parts;
    if (parts[0]?.endsWith("deg")) {
      angle = Number(parts[0].replace("deg", "")) || angle;
      stopParts = parts.slice(1);
    }
    const stops = stopParts.map((part, index) => {
      const pieces = part.split(/\s+/).filter(Boolean);
      return {
        color: pieces[0] || "#ffffff",
        offset: pieces[1] || `${Math.round((index / Math.max(1, stopParts.length - 1)) * 100)}%`
      };
    });
    return {
      angle,
      stops: stops.length ? stops : [{ color: "#ffffff", offset: "0%" }, { color: "#000000", offset: "100%" }]
    };
  }

  function gradientVector(angle) {
    const radians = ((angle - 90) * Math.PI) / 180;
    const x = Math.cos(radians) * 50;
    const y = Math.sin(radians) * 50;
    return {
      x1: `${round(50 - x)}%`,
      y1: `${round(50 - y)}%`,
      x2: `${round(50 + x)}%`,
      y2: `${round(50 + y)}%`
    };
  }

  function gradientIdForSwatch(name) {
    return `wb-gradient-${String(name).replace(/[^a-z0-9_-]/gi, "-")}`;
  }

  function ensureGradient(name) {
    const swatch = swatches[name];
    if (!swatch || swatch.type !== "linear") return swatch?.value || swatches.ink.value;
    const id = gradientIdForSwatch(name);
    const defs = ensureDefs();
    let gradient = defs.querySelector(`[id="${id}"]`);
    if (!gradient) {
      gradient = createSvg("linearGradient", { id });
      defs.append(gradient);
    }
    const parsed = parseGradient(swatch.value);
    const vector = gradientVector(parsed.angle);
    for (const [attr, value] of Object.entries(vector)) setAttribute(gradient, attr, value);
    gradient.replaceChildren();
    for (const stop of parsed.stops) {
      gradient.append(createSvg("stop", { offset: stop.offset, "stop-color": stop.color }));
    }
    return `url(#${id})`;
  }

  function paintForSwatch(name) {
    const swatchName = swatches[name] ? name : "ink";
    const swatch = swatches[swatchName];
    return {
      name: swatchName,
      raw: swatch.value,
      value: swatch.type === "linear" ? ensureGradient(swatchName) : swatch.value
    };
  }

  function selectedPaint() {
    return paintForSwatch(state.activeSwatch);
  }

  function syncColorControls() {
    const paint = selectedPaint();
    for (const preview of nodes("[data-whiteboard-color-preview]")) {
      preview.style.background = paint.raw;
    }
    for (const label of nodes("[data-whiteboard-color-label]")) {
      label.textContent = swatchLabels[paint.name] || paint.name;
    }
    for (const button of nodes("[data-whiteboard-paint-mode]")) {
      const active = button.dataset.whiteboardPaintMode === state.paintMode;
      button.dataset.active = active ? "true" : "false";
      button.setAttribute("aria-pressed", active ? "true" : "false");
    }
    const solid = root.querySelector('[data-whiteboard-color-input="solid"]');
    const start = root.querySelector('[data-whiteboard-color-input="start"]');
    const end = root.querySelector('[data-whiteboard-color-input="end"]');
    const angle = root.querySelector("[data-whiteboard-gradient-angle]");
    if (solid) solid.value = state.customSolid;
    if (start) start.value = state.customGradientStart;
    if (end) end.value = state.customGradientEnd;
    if (angle) angle.value = String(state.customGradientAngle);
  }

  function setPaintMetadata(node, paint) {
    node.dataset.whiteboardSwatch = paint.name;
    node.dataset.whiteboardPaint = paint.raw;
  }

  function paintObject(id, swatchName) {
    const paint = paintForSwatch(swatchName || state.activeSwatch);
    for (const node of objectNodes(id)) {
      setPaintMetadata(node, paint);
      if (node.tagName === "text") {
        if (node.hasAttribute("data-whiteboard-object-id")) node.style.fill = paint.value;
        continue;
      }
      if (node.tagName === "path" || node.tagName === "line") {
        node.style.stroke = paint.value;
        if (node.classList.contains("wb-demo-path")) node.style.fill = "none";
        if (node.tagName === "line") node.style.fill = "none";
        continue;
      }
      if (node.tagName === "rect" || node.tagName === "ellipse" || node.tagName === "polygon") {
        node.style.fill = paint.value;
        node.style.stroke = paint.value;
      }
      if (node.tagName === "foreignObject") {
        node.style.color = paint.raw.startsWith("linear-gradient") ? swatches.ink.value : paint.raw;
        node.style.setProperty("--wb-object-paint", paint.raw);
      }
    }
  }

  function buildCustomPaint() {
    const angle = clamp(Number(state.customGradientAngle) || 135, 0, 360);
    state.customGradientAngle = angle;
    if (state.paintMode === "linear") {
      swatches.custom = {
        type: "linear",
        value: `linear-gradient(${angle}deg, ${state.customGradientStart} 0%, ${state.customGradientEnd} 100%)`
      };
      return;
    }
    swatches.custom = { type: "solid", value: state.customSolid };
  }

  function applyCustomPaint() {
    buildCustomPaint();
    applySwatch("custom");
  }

  function setPaintPopoverOpen(open) {
    for (const popover of nodes("[data-whiteboard-color-popover]")) {
      popover.hidden = !open;
    }
    for (const trigger of nodes("[data-whiteboard-color-trigger]")) {
      trigger.setAttribute("aria-expanded", open ? "true" : "false");
    }
  }

  function setShapeMenuOpen(open) {
    state.shapeMenuOpen = Boolean(open);
    for (const menu of nodes("[data-whiteboard-shape-menu]")) {
      menu.hidden = !state.shapeMenuOpen;
    }
    for (const trigger of nodes("[data-whiteboard-shape-menu-trigger]")) {
      trigger.setAttribute("aria-expanded", state.shapeMenuOpen ? "true" : "false");
    }
    syncToolbarPopoverState();
  }

  function setIconMenuOpen(open) {
    state.iconMenuOpen = Boolean(open);
    for (const menu of nodes("[data-whiteboard-icon-menu]")) {
      menu.hidden = !state.iconMenuOpen;
    }
    for (const trigger of nodes("[data-whiteboard-icon-menu-trigger]")) {
      trigger.setAttribute("aria-expanded", state.iconMenuOpen ? "true" : "false");
    }
    syncToolbarPopoverState();
  }

  function syncToolbarPopoverState() {
    root.dataset.whiteboardToolbarPopoverOpen = state.shapeMenuOpen || state.iconMenuOpen ? "true" : "false";
  }

  function hasOpenPaintPopover() {
    return nodes("[data-whiteboard-color-popover]").some((popover) => !popover.hidden);
  }

  function closeTransientUi(options = {}) {
    let changed = false;
    if (hasOpenPaintPopover()) changed = true;
    if (state.shapeMenuOpen) changed = true;
    if (state.iconMenuOpen) changed = true;
    if (state.shortcutPanelOpen) changed = true;
    if (options.closePanel === true && state.sidePanelOpen) changed = true;
    setPaintPopoverOpen(false);
    setShapeMenuOpen(false);
    setIconMenuOpen(false);
    setShortcutsPanelOpen(false);
    if (options.closePanel === true) setSidePanelOpen(false);
    hideTooltip();
    return changed;
  }

  function syncShapeMenu() {
    if (shapeTools.has(state.tool)) state.currentShapeTool = state.tool;
    const iconName = shapeToolIcons[state.currentShapeTool] || "whiteboard:rectangle";
    const label = toolLabels[state.currentShapeTool] || "Shapes";
    let currentIcons = nodes("[data-whiteboard-current-shape-icon]");
    if (!currentIcons.length) {
      currentIcons = nodes("[data-whiteboard-shape-menu-trigger] .wb-icon");
    }
    for (const icon of currentIcons) {
      renderDxIcon(icon, iconName);
    }
    for (const trigger of nodes("[data-whiteboard-shape-menu-trigger]")) {
      trigger.dataset.active = shapeTools.has(state.tool) ? "true" : "false";
      trigger.setAttribute("aria-label", `Choose shape tool. Current: ${label}`);
      setTooltip(trigger, `${label} (${triggerShortcutFor(state.currentShapeTool)}). Open grouped shape tools.`, "top");
    }
  }

  function triggerShortcutFor(tool) {
    return {
      rectangle: "R",
      diamond: "D",
      ellipse: "O",
      line: "L",
      arrow: "A",
      text: "T",
      table: "B",
      math: "M"
    }[tool] || "Shapes";
  }

  function setTooltip(node, label, placement = "top") {
    if (!node || !label) return;
    node.dataset.whiteboardTooltip = label;
    node.dataset.whiteboardTooltipPlacement = placement;
    node.removeAttribute("title");
  }

  function tooltipHost() {
    let host = root.querySelector("[data-whiteboard-floating-tooltip]");
    if (host) return host;
    host = document.createElement("div");
    host.className = "wb-floating-tooltip";
    host.dataset.whiteboardFloatingTooltip = "true";
    host.dataset.visible = "false";
    root.appendChild(host);
    return host;
  }

  function showTooltip(target) {
    const text = target?.dataset?.whiteboardTooltip;
    if (!text) return;
    const host = tooltipHost();
    const rect = target.getBoundingClientRect();
    const placement = target.dataset.whiteboardTooltipPlacement || "top";
    host.textContent = text;
    host.dataset.placement = placement;
    host.dataset.visible = "true";
    let x = rect.left + rect.width / 2;
    let y = rect.top - 10;
    let translate = "translate(-50%, -100%)";
    if (placement === "right") {
      x = rect.right + 10;
      y = rect.top + rect.height / 2;
      translate = "translate(0, -50%)";
    }
    if (placement === "bottom") {
      x = rect.left + rect.width / 2;
      y = rect.bottom + 10;
      translate = "translate(-50%, 0)";
    }
    if (placement === "left") {
      x = rect.left - 10;
      y = rect.top + rect.height / 2;
      translate = "translate(-100%, -50%)";
    }
    const margin = 10;
    x = clamp(x, margin, window.innerWidth - margin);
    y = clamp(y, margin, window.innerHeight - margin);
    host.style.left = `${round(x)}px`;
    host.style.top = `${round(y)}px`;
    host.style.setProperty("--wb-tooltip-transform", translate);
  }

  function hideTooltip() {
    const host = root.querySelector("[data-whiteboard-floating-tooltip]");
    if (host) host.dataset.visible = "false";
  }

  function bindTooltips() {
    if (root.dataset.whiteboardTooltipsBound === "true") return;
    root.dataset.whiteboardTooltipsBound = "true";
    root.addEventListener("pointerover", (event) => {
      const target = event.target.closest?.("[data-whiteboard-tooltip]");
      if (target) showTooltip(target);
    });
    root.addEventListener("pointerout", (event) => {
      const target = event.target.closest?.("[data-whiteboard-tooltip]");
      if (!target || target.contains(event.relatedTarget)) return;
      hideTooltip();
    });
    root.addEventListener("focusin", (event) => {
      const target = event.target.closest?.("[data-whiteboard-tooltip]");
      if (target) showTooltip(target);
    });
    root.addEventListener("focusout", hideTooltip);
    window.addEventListener("scroll", hideTooltip, true);
    window.addEventListener("resize", hideTooltip);
  }

  function hydrateTooltips() {
    for (const button of nodes("[data-whiteboard-tool-button]")) {
      const tool = button.dataset.whiteboardToolButton;
      setTooltip(button, toolHelp[tool] || button.getAttribute("aria-label"), "top");
    }
    for (const button of nodes("[data-whiteboard-command]")) {
      const command = button.dataset.whiteboardCommand;
      setTooltip(button, commandHelp[command] || button.getAttribute("aria-label"), "top");
    }
    for (const button of nodes("[data-whiteboard-zoom]")) {
      const zoom = button.dataset.whiteboardZoom;
      setTooltip(button, zoomHelp[zoom] || button.getAttribute("aria-label"), "top");
    }
    for (const button of nodes("[data-whiteboard-toggle]")) {
      const toggle = button.dataset.whiteboardToggle;
      setTooltip(button, toggleHelp[toggle] || button.getAttribute("aria-label"), "top");
    }
    for (const button of nodes("[data-whiteboard-shape-menu-trigger]")) {
      setTooltip(button, "Shapes. Open grouped tools for basic shapes, connectors, and content.", "top");
    }
    for (const button of nodes("[data-whiteboard-icon-menu-trigger]")) {
      setTooltip(button, "DX icons. Insert a source-owned icon object.", "top");
    }
    for (const button of nodes("[data-whiteboard-icon-insert]")) {
      setTooltip(button, button.getAttribute("aria-label"), "top");
    }
    for (const button of nodes("[data-whiteboard-media-pick]")) {
      setTooltip(button, button.getAttribute("aria-label"), "top");
    }
    for (const button of nodes("[data-whiteboard-swatch]")) {
      const label = button.getAttribute("aria-label") || swatchLabels[button.dataset.whiteboardSwatch];
      setTooltip(button, label ? `${label}. Apply this paint to the selected object.` : "Apply paint.", "top");
    }
    for (const button of nodes("[data-whiteboard-color-trigger]")) {
      setTooltip(button, "Paint picker. Choose solid or gradient colors.", "top");
    }
    for (const button of nodes("[data-whiteboard-apply-custom-paint]")) {
      setTooltip(button, "Apply the custom paint to the selected object.", "top");
    }
    for (const button of nodes("[data-whiteboard-field-step], [data-whiteboard-gradient-step]")) {
      setTooltip(button, button.getAttribute("aria-label"), "top");
    }
    for (const input of nodes("[data-whiteboard-measure-input], [data-whiteboard-gradient-angle], [data-whiteboard-color-input]")) {
      setTooltip(input, input.getAttribute("aria-label"), "top");
    }
    for (const input of nodes("[data-whiteboard-font-input]")) {
      setTooltip(input, "Type any Google Font family name, then apply it to selected text.", "top");
    }
    for (const button of nodes("[data-whiteboard-apply-font]")) {
      setTooltip(button, "Apply the selected Google Font to the selected text object.", "top");
    }
    for (const button of nodes("[data-whiteboard-add]")) {
      const label = button.textContent?.trim() || button.getAttribute("aria-label");
      setTooltip(button, label ? `Add ${label}. Inserts this library object on the board.` : "Add library object.", "top");
    }
  }

  function setSidePanelOpen(open) {
    state.sidePanelOpen = Boolean(open);
    if (state.sidePanelOpen) {
      setShortcutsPanelOpen(false);
      setShapeMenuOpen(false);
      setIconMenuOpen(false);
      setPaintPopoverOpen(false);
    }
    root.dataset.whiteboardPanelOpen = state.sidePanelOpen ? "true" : "false";
    for (const panel of nodes("[data-whiteboard-side-panel]")) {
      panel.dataset.collapsed = state.sidePanelOpen ? "false" : "true";
      panel.setAttribute("aria-hidden", state.sidePanelOpen ? "false" : "true");
    }
    for (const button of nodes('[data-whiteboard-toggle="side-panel"]')) {
      button.dataset.active = state.sidePanelOpen ? "true" : "false";
      button.setAttribute("aria-expanded", state.sidePanelOpen ? "true" : "false");
    }
    updateSelectionOutline();
  }

  function setShortcutsPanelOpen(open) {
    state.shortcutPanelOpen = Boolean(open);
    if (state.shortcutPanelOpen) {
      setSidePanelOpen(false);
      setShapeMenuOpen(false);
      setIconMenuOpen(false);
      setPaintPopoverOpen(false);
    }
    for (const panel of nodes("[data-whiteboard-shortcuts-panel]")) {
      panel.dataset.collapsed = state.shortcutPanelOpen ? "false" : "true";
      panel.setAttribute("aria-hidden", state.shortcutPanelOpen ? "false" : "true");
    }
    for (const button of nodes('[data-whiteboard-toggle="shortcuts"]')) {
      button.dataset.active = state.shortcutPanelOpen ? "true" : "false";
      button.setAttribute("aria-expanded", state.shortcutPanelOpen ? "true" : "false");
    }
  }

  function pushHistory() {
    state.history.push({
      svg: svg.innerHTML,
      selectedId: state.selectedId,
      controllerId: state.controllerId,
      activeSwatch: state.activeSwatch,
      zoom: state.zoom,
      nextId: state.nextId
    });
    if (state.history.length > 80) state.history.shift();
    state.future = [];
  }

  function restore(snapshot) {
    if (!snapshot) return;
    svg.innerHTML = snapshot.svg;
    state.selectedId = snapshot.selectedId;
    state.controllerId = snapshot.controllerId || "";
    state.activeSwatch = snapshot.activeSwatch || state.activeSwatch;
    state.zoom = snapshot.zoom;
    state.nextId = snapshot.nextId;
    syncOutlineRows();
    applyZoom();
    refresh();
    persist();
  }

  function undo() {
    const previous = state.history.pop();
    if (!previous) return;
    state.future.push({
      svg: svg.innerHTML,
      selectedId: state.selectedId,
      controllerId: state.controllerId,
      activeSwatch: state.activeSwatch,
      zoom: state.zoom,
      nextId: state.nextId
    });
    restore(previous);
  }

  function redo() {
    const next = state.future.pop();
    if (!next) return;
    state.history.push({
      svg: svg.innerHTML,
      selectedId: state.selectedId,
      controllerId: state.controllerId,
      activeSwatch: state.activeSwatch,
      zoom: state.zoom,
      nextId: state.nextId
    });
    restore(next);
  }

  function persist() {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify({
        svg: svg.innerHTML,
        selectedId: state.selectedId,
        controllerId: state.controllerId,
        activeSwatch: state.activeSwatch,
        zoom: state.zoom,
        nextId: state.nextId
      }));
    } catch {
      // Local persistence is optional in restricted browsers.
    }
  }

  function loadPersisted() {
    try {
      const saved = JSON.parse(localStorage.getItem(STORAGE_KEY) || "null");
      if (!saved?.svg || !saved.svg.includes("data-whiteboard-object-id")) return;
      svg.innerHTML = saved.svg;
      state.selectedId = saved.selectedId || state.selectedId;
      state.controllerId = saved.controllerId || "";
      state.activeSwatch = swatches[saved.activeSwatch] ? saved.activeSwatch : state.activeSwatch;
      state.zoom = Number(saved.zoom || 1);
      state.nextId = Number(saved.nextId || 1);
    } catch {
      localStorage.removeItem(STORAGE_KEY);
    }
  }

  function setTool(tool) {
    state.tool = tool;
    stage.dataset.whiteboardTool = tool;
    if (tool !== "select") state.controllerId = "";
    if (tool !== "select") setSidePanelOpen(false);
    for (const button of nodes("[data-whiteboard-tool-button]")) {
      const active = button.dataset.whiteboardToolButton === tool;
      button.dataset.active = active ? "true" : "false";
      button.setAttribute("aria-pressed", active ? "true" : "false");
    }
    for (const readout of nodes("[data-whiteboard-tool-readout]")) {
      readout.textContent = toolLabels[tool] || tool;
    }
    syncShapeMenu();
  }

  function selectObject(id, options = {}) {
    if (!id || !primaryObject(id)) return;
    state.selectedId = id;
    if (options.openPanel === true) setSidePanelOpen(true);
    refresh();
  }

  function showObjectController(id) {
    if (!id || !primaryObject(id)) return;
    state.controllerId = id;
    selectObject(id, { openPanel: true });
    updateSelectionOutline();
  }

  function refresh() {
    ensureDefs();
    for (const node of svgNodes("[data-whiteboard-object-id]")) {
      node.dataset.selected = node.dataset.whiteboardObjectId === state.selectedId ? "true" : "false";
    }
    for (const button of nodes("[data-whiteboard-select]")) {
      const active = button.dataset.whiteboardSelect === state.selectedId;
      button.dataset.active = active ? "true" : "false";
      const status = button.querySelector("small");
      if (status) status.textContent = active ? "Selected" : readableType(button.dataset.whiteboardSelect);
    }
    for (const button of nodes("[data-whiteboard-swatch]")) {
      button.dataset.active = button.dataset.whiteboardSwatch === state.activeSwatch ? "true" : "false";
    }
    const selected = selectedPrimary();
    for (const readout of nodes("[data-whiteboard-selection-readout]")) {
      readout.textContent = selected?.dataset.whiteboardName || selected?.dataset.whiteboardObjectId || "None";
    }
    updateSelectionOutline();
    updateInspector();
    updateCounts();
    applyZoom();
    syncColorControls();
    syncFontControls();
    setMinimapVisible(state.minimapVisible);
  }

  function readableType(id) {
    const node = primaryObject(id);
    if (!node) return "Layer";
    const kind = node.dataset.whiteboardKind;
    if (kind && toolLabels[kind]) return toolLabels[kind];
    if (node.tagName === "text") return "Text";
    if (node.tagName === "path") return "Connector";
    if (node.tagName === "ellipse") return "Ellipse";
    if (node.tagName === "polygon") return "Diamond";
    return "Shape";
  }

  function normalizeFontName(name) {
    return String(name || DEFAULT_TEXT_FONT).trim().replace(/\s+/g, " ").slice(0, 80) || DEFAULT_TEXT_FONT;
  }

  function fontFamilyForName(name) {
    const normalized = normalizeFontName(name).replace(/["\\]/g, "");
    return `"${normalized}", ${TEXT_FONT_FALLBACK}`;
  }

  function ensureGoogleFont(name) {
    const normalized = normalizeFontName(name);
    const key = normalized.toLowerCase().replace(/[^a-z0-9]+/g, "-");
    if (document.head.querySelector(`[data-whiteboard-google-font="${safeSelector(key)}"]`)) return;
    const link = document.createElement("link");
    link.rel = "stylesheet";
    link.href = `https://fonts.googleapis.com/css2?family=${encodeURIComponent(normalized).replace(/%20/g, "+")}:wght@400;500;600;700;800&display=swap`;
    link.dataset.whiteboardGoogleFont = key;
    document.head.append(link);
  }

  function setTextFont(node, fontName) {
    if (!node) return;
    const normalized = normalizeFontName(fontName);
    const family = fontFamilyForName(normalized);
    ensureGoogleFont(normalized);
    node.dataset.whiteboardFontName = normalized;
    node.dataset.whiteboardFontFamily = family;
    setAttribute(node, "font-family", family);
    node.style.fontFamily = family;
  }

  function syncFontControls() {
    const selected = editableTextObject(state.selectedId);
    const fontName = selected?.dataset.whiteboardFontName || DEFAULT_TEXT_FONT;
    for (const input of nodes("[data-whiteboard-font-input]")) {
      input.value = fontName;
      input.disabled = !selected;
    }
    for (const button of nodes("[data-whiteboard-apply-font]")) {
      button.disabled = !selected;
    }
  }

  function applyFontToSelectedText(fontName) {
    const selected = editableTextObject(state.selectedId);
    if (!selected) return false;
    pushHistory();
    setTextFont(selected, fontName);
    updateSelectionOutline();
    updateInspector();
    syncFontControls();
    persist();
    return true;
  }

  function updateSelectionOutline() {
    const outline = svg.querySelector("[data-whiteboard-selection-outline]");
    const bounds = objectBounds(state.controllerId);
    if (!outline) return;
    if (!bounds || !state.controllerId) {
      outline.setAttribute("hidden", "true");
      setAttribute(outline, "x", 0);
      setAttribute(outline, "y", 0);
      setAttribute(outline, "width", 0);
      setAttribute(outline, "height", 0);
      return;
    }
    outline.removeAttribute("hidden");
    setAttribute(outline, "x", round(bounds.x - 4));
    setAttribute(outline, "y", round(bounds.y - 4));
    setAttribute(outline, "width", round(bounds.width + 8));
    setAttribute(outline, "height", round(bounds.height + 8));
  }

  function updateInspector() {
    const bounds = inspectBounds(state.selectedId) || { x: 0, y: 0, width: 0, height: 0 };
    const values = {
      x: bounds.x,
      y: bounds.y,
      width: bounds.width,
      height: bounds.height
    };
    for (const input of nodes("[data-whiteboard-measure-input]")) {
      const field = input.dataset.whiteboardMeasureInput;
      input.value = String(Math.round(values[field] || 0));
      input.disabled = !state.selectedId;
    }
  }

  function syncOwnedLabelPosition(id) {
    const rect = primaryObject(id);
    if (!rect || rect.tagName !== "rect") return;
    const x = Number(rect.getAttribute("x") || 0);
    const y = Number(rect.getAttribute("y") || 0);
    const height = Number(rect.getAttribute("height") || 0);
    const label = objectNodes(id).find((node) => node.tagName === "text");
    if (!label) return;
    setAttribute(label, "x", round(x + 22));
    setAttribute(label, "y", round(y + Math.min(height / 2 + 12, 56)));
  }

  function editableTextObject(id) {
    const node = primaryObject(id);
    return node?.tagName === "text" ? node : null;
  }

  function positionTextEditor(node) {
    if (!textEditor) return;
    const stageRect = stage.getBoundingClientRect();
    const nodeRect = node.getBoundingClientRect();
    const left = Math.max(8, nodeRect.left - stageRect.left - 8);
    const top = Math.max(8, nodeRect.top - stageRect.top - 8);
    textEditor.style.left = `${Math.round(left)}px`;
    textEditor.style.top = `${Math.round(top)}px`;
    textEditor.style.width = `${Math.max(160, Math.round(nodeRect.width + 28))}px`;
  }

  function openTextEditor(id) {
    const node = editableTextObject(id);
    if (!node || !textEditor) {
      closeTextEditor();
      return false;
    }
    setPaintPopoverOpen(false);
    setShapeMenuOpen(false);
    setIconMenuOpen(false);
    setShortcutsPanelOpen(false);
    state.controllerId = "";
    selectObject(id, { openPanel: false });
    state.editingTextId = id;
    state.editingOriginalText = node.textContent || "";
    textEditor.value = state.editingOriginalText;
    textEditor.style.fontFamily = node.dataset.whiteboardFontFamily || node.getAttribute("font-family") || TEXT_FONT_FALLBACK;
    positionTextEditor(node);
    textEditor.hidden = false;
    window.setTimeout(() => {
      textEditor.focus();
      textEditor.select();
    }, 0);
    return true;
  }

  function closeTextEditor() {
    if (!textEditor) return;
    textEditor.hidden = true;
    state.editingTextId = "";
    state.editingOriginalText = "";
  }

  function commitTextEditor() {
    if (!textEditor || textEditor.hidden || !state.editingTextId) return;
    const node = editableTextObject(state.editingTextId);
    const nextText = textEditor.value.trim() || state.editingOriginalText || "Text";
    if (node && nextText !== state.editingOriginalText) {
      pushHistory();
      node.textContent = nextText;
      node.dataset.whiteboardName = node.dataset.whiteboardKind === "math" ? "Math" : "Text note";
      updateSelectionOutline();
      updateInspector();
      persist();
    }
    closeTextEditor();
  }

  function cancelTextEditor() {
    closeTextEditor();
  }

  function setSelectedGeometry(field, value) {
    if (!state.selectedId || !Number.isFinite(value)) return;
    const selected = selectedPrimary();
    const bounds = inspectBounds(state.selectedId);
    if (!selected || !bounds) return;
    const nextValue = Math.round(value);
    if (field === "x" || field === "y") {
      const offset = objectOffset(state.selectedId);
      const nextX = field === "x" ? offset.x + nextValue - bounds.x : offset.x;
      const nextY = field === "y" ? offset.y + nextValue - bounds.y : offset.y;
      setObjectOffset(state.selectedId, nextX, nextY);
    }
    if ((field === "width" || field === "height") && selected.tagName === "rect") {
      const size = Math.max(8, nextValue);
      setAttribute(selected, field, size);
      syncOwnedLabelPosition(state.selectedId);
      if (selected.dataset.whiteboardKind === "table") {
        updateTableGuides(state.selectedId, {
          x: Number(selected.getAttribute("x") || 0),
          y: Number(selected.getAttribute("y") || 0),
          width: Number(selected.getAttribute("width") || 0),
          height: Number(selected.getAttribute("height") || 0)
        });
      }
    }
    if ((field === "width" || field === "height") && selected.tagName === "ellipse") {
      const size = Math.max(8, nextValue);
      setAttribute(selected, field === "width" ? "rx" : "ry", round(size / 2));
    }
    if ((field === "width" || field === "height") && selected.tagName === "polygon") {
      const size = Math.max(8, nextValue);
      const box = {
        x: bounds.x,
        y: bounds.y,
        width: field === "width" ? size : bounds.width,
        height: field === "height" ? size : bounds.height
      };
      setAttribute(selected, "points", diamondPoints(box));
    }
    if ((field === "width" || field === "height") && selected.tagName === "foreignObject") {
      setAttribute(selected, field, Math.max(24, nextValue));
    }
    updateSelectionOutline();
    updateInspector();
    persist();
  }

  function changeSelectedGeometry(field, delta) {
    const input = root.querySelector(`[data-whiteboard-measure-input="${field}"]`);
    const current = Number(input?.value || 0);
    setSelectedGeometry(field, current + delta);
  }

  function applyGradientStep(delta) {
    if (!Number.isFinite(delta)) return false;
    state.customGradientAngle = clamp(state.customGradientAngle + delta, 0, 360);
    syncColorControls();
    return true;
  }

  function applyFieldStep(field, delta, recordHistory) {
    if (!field || !state.selectedId || !Number.isFinite(delta)) return false;
    if (recordHistory) pushHistory();
    changeSelectedGeometry(field, delta);
    return true;
  }

  function applyStepperButton(button, recordHistory) {
    const gradientStep = button.closest?.("[data-whiteboard-gradient-step]");
    if (gradientStep) return applyGradientStep(Number(gradientStep.dataset.whiteboardGradientStep || 0));
    const fieldStep = button.closest?.("[data-whiteboard-field-step]");
    if (fieldStep) {
      return applyFieldStep(
        fieldStep.dataset.whiteboardFieldStep,
        Number(fieldStep.dataset.whiteboardStep || 0),
        recordHistory
      );
    }
    return false;
  }

  function clearStepHold() {
    if (!state.stepHold) return;
    const button = state.stepHold.button;
    clearTimeout(state.stepHold.timeout);
    clearInterval(state.stepHold.interval);
    state.stepHold = null;
    window.setTimeout(() => {
      if (button?.dataset.whiteboardSuppressClick === "true") delete button.dataset.whiteboardSuppressClick;
    }, 160);
  }

  function startStepHold(event) {
    const button = event.target.closest?.("[data-whiteboard-gradient-step], [data-whiteboard-field-step]");
    if (!button) return;
    event.preventDefault();
    event.stopPropagation();
    button.dataset.whiteboardSuppressClick = "true";
    if (!applyStepperButton(button, true)) {
      delete button.dataset.whiteboardSuppressClick;
      return;
    }
    clearStepHold();
    state.stepHold = {
      button,
      timeout: window.setTimeout(() => {
        if (!state.stepHold?.button) return;
        state.stepHold.interval = window.setInterval(() => applyStepperButton(button, false), 56);
      }, 260),
      interval: 0
    };
    button.setPointerCapture?.(event.pointerId);
  }

  function updateCounts() {
    const count = new Set(svgNodes("[data-whiteboard-object-id]").map((node) => node.dataset.whiteboardObjectId)).size;
    const badge = root.querySelector(".wb-outline-panel .wb-panel-badge");
    if (badge) badge.textContent = `${count} layers`;
  }

  function applyZoom() {
    stage.style.width = "";
    stage.style.height = "";
    stage.style.setProperty("--wb-zoom-scale", String(state.zoom));
    stage.dataset.whiteboardZoomed = state.zoom === 1 ? "false" : "true";
  }

  function setZoom(next) {
    state.zoom = clamp(next, 0.35, 2.5);
    applyZoom();
    persist();
  }

  function setMinimapVisible(visible) {
    state.minimapVisible = Boolean(visible);
    for (const panel of nodes("[data-whiteboard-minimap-panel]")) {
      panel.dataset.whiteboardVisible = state.minimapVisible ? "true" : "false";
    }
    for (const button of nodes('[data-whiteboard-toggle="minimap"]')) {
      button.dataset.active = state.minimapVisible ? "true" : "false";
      button.setAttribute("aria-pressed", state.minimapVisible ? "true" : "false");
      button.setAttribute("aria-expanded", state.minimapVisible ? "true" : "false");
    }
    for (const readout of nodes("[data-whiteboard-minimap-readout]")) {
      readout.textContent = state.minimapVisible ? "On" : "Off";
    }
  }

  function applySwatch(swatch) {
    if (!swatches[swatch]) return;
    state.activeSwatch = swatch;
    state.paintMode = swatches[swatch].type === "linear" ? "linear" : "solid";
    const selected = selectedPrimary();
    if (selected) {
      pushHistory();
      paintObject(selected.dataset.whiteboardObjectId, swatch);
    }
    refresh();
    persist();
  }

  function nextObjectId(prefix) {
    const id = `wb-${prefix}-${state.nextId}`;
    state.nextId += 1;
    return id;
  }

  function boardCenterPoint() {
    const point = svg.createSVGPoint();
    point.x = window.innerWidth / 2;
    point.y = window.innerHeight / 2;
    const matrix = svg.getScreenCTM();
    if (!matrix) return { x: 520, y: 290 };
    const local = point.matrixTransform(matrix.inverse());
    return { x: local.x - 80, y: local.y - 60 };
  }

  function insertObjectNodes(items) {
    const outline = svg.querySelector("[data-whiteboard-selection-outline]");
    for (const item of items.filter(Boolean)) svg.insertBefore(item, outline || null);
  }

  function createForeignObject(id, kind, name, box) {
    return createSvg("foreignObject", {
      class: `wb-element wb-foreign-object wb-${kind}-object`,
      "data-whiteboard-element-id": id,
      "data-whiteboard-object-id": id,
      "data-whiteboard-kind": kind,
      "data-whiteboard-name": name,
      x: round(box.x),
      y: round(box.y),
      width: round(box.width),
      height: round(box.height)
    });
  }

  function addIconObject(iconToken, point) {
    const allowed = new Set(Object.values(shapeToolIcons).concat([
      "whiteboard:select",
      "whiteboard:move",
      "whiteboard:hand",
      "whiteboard:pen",
      "whiteboard:fit",
      "whiteboard:keyboard",
      "whiteboard:undo",
      "whiteboard:redo",
      "whiteboard:zoom-in",
      "whiteboard:zoom-out",
      "whiteboard:trash",
      "whiteboard:panel",
      "whiteboard:icons",
      "whiteboard:image",
      "whiteboard:audio",
      "whiteboard:video"
    ]));
    const token = allowed.has(iconToken) ? iconToken : "whiteboard:select";
    const label = token.replace("whiteboard:", "").replace(/-/g, " ");
    const start = point || boardCenterPoint();
    pushHistory();
    const id = nextObjectId("icon");
    const foreign = createForeignObject(id, "icon", `${label} icon`, { x: start.x, y: start.y, width: 82, height: 82 });
    foreign.dataset.dxIconObject = token;
    const shell = createHtml("div", { class: "wb-icon-object-shell", "data-whiteboard-owned-by": id, role: "img", "aria-label": `${label} icon` });
    shell.append(createDxIcon(token, "wb-icon wb-icon-object-glyph"));
    foreign.append(shell);
    insertObjectNodes([foreign]);
    paintObject(id, state.activeSwatch);
    markDxIcons();
    appendObject(id);
    setIconMenuOpen(false);
    return id;
  }

  function mediaBoxFor(kind, point) {
    const start = point || boardCenterPoint();
    if (kind === "audio") return { x: start.x, y: start.y, width: 340, height: 82 };
    if (kind === "video") return { x: start.x, y: start.y, width: 360, height: 216 };
    return { x: start.x, y: start.y, width: 280, height: 190 };
  }

  function addMediaObject(kind, source, name) {
    const cleanKind = ["image", "audio", "video"].includes(kind) ? kind : "image";
    const box = mediaBoxFor(cleanKind);
    pushHistory();
    const id = nextObjectId(cleanKind);
    const foreign = createForeignObject(id, cleanKind, name || cleanKind, box);
    const shell = createHtml("div", { class: `wb-media-object-shell wb-${cleanKind}-shell`, "data-whiteboard-owned-by": id });
    if (cleanKind === "image") {
      shell.append(createHtml("img", { src: source, alt: name || "Whiteboard image", draggable: "false" }));
    } else if (cleanKind === "audio") {
      shell.append(createHtml("audio", { src: source, controls: "controls", preload: "metadata" }));
    } else {
      shell.append(createHtml("video", { src: source, controls: "controls", preload: "metadata" }));
    }
    foreign.append(shell);
    insertObjectNodes([foreign]);
    appendObject(id);
    return id;
  }

  function readMediaFile(kind, file) {
    if (!file) return;
    const cleanKind = ["image", "audio", "video"].includes(kind) ? kind : "image";
    if (file.type && !file.type.startsWith(`${cleanKind}/`)) return;
    if (file.size > MAX_INLINE_MEDIA_BYTES) return;
    const reader = new FileReader();
    reader.addEventListener("load", () => {
      if (typeof reader.result === "string") addMediaObject(cleanKind, reader.result, file.name);
    });
    reader.readAsDataURL(file);
  }

  function appendObject(id) {
    addOutlineRow(id, primaryObject(id)?.dataset.whiteboardName || "Untitled", readableType(id));
    selectObject(id);
    persist();
  }

  function addOutlineRow(id, name, type) {
    const list = root.querySelector(".wb-outline-list");
    if (!list || root.querySelector(`[data-whiteboard-select="${safeSelector(id)}"]`)) return;
    const button = document.createElement("button");
    button.className = "wb-outline-item";
    button.type = "button";
    button.dataset.whiteboardSelect = id;
    button.innerHTML = `<span class="wb-outline-mark" data-element-type="${type.toLowerCase()}"></span><strong>${escapeText(name)}</strong><small>${escapeText(type)}</small>`;
    list.append(button);
    bindControlButtons();
  }

  function syncOutlineRows() {
    const list = root.querySelector(".wb-outline-list");
    if (!list) return;
    const objects = svgNodes("[data-whiteboard-object-id]");
    const ids = new Set(objects.map((node) => node.dataset.whiteboardObjectId).filter(Boolean));
    for (const row of nodes("[data-whiteboard-select]")) {
      if (!ids.has(row.dataset.whiteboardSelect)) row.remove();
    }
    for (const object of objects) {
      const id = object.dataset.whiteboardObjectId;
      if (!id) continue;
      addOutlineRow(id, object.dataset.whiteboardName || readableType(id), readableType(id));
    }
  }

  function escapeText(value) {
    return String(value).replace(/[&<>"']/g, (char) => ({
      "&": "&amp;",
      "<": "&lt;",
      ">": "&gt;",
      "\"": "&quot;",
      "'": "&#39;"
    })[char]);
  }

  function shapeDefaults(kind) {
    if (kind === "frame") {
      return { prefix: "frame", name: "Frame", className: "wb-element wb-demo-frame", width: 360, height: 180, minWidth: 140, minHeight: 92, rx: 12, label: "Frame", shape: "rect" };
    }
    if (kind === "flow-card") {
      return { prefix: "card", name: "Flow card", className: "wb-element wb-demo-lane", width: 220, height: 104, minWidth: 96, minHeight: 56, rx: 14, label: "Flow card", shape: "rect" };
    }
    if (kind === "diamond") {
      return { prefix: "diamond", name: "Diamond", className: "wb-element wb-demo-diamond", width: 160, height: 120, minWidth: 32, minHeight: 32, shape: "diamond" };
    }
    if (kind === "ellipse") {
      return { prefix: "ellipse", name: "Ellipse", className: "wb-element wb-demo-ellipse", width: 170, height: 120, minWidth: 28, minHeight: 28, shape: "ellipse" };
    }
    if (kind === "table") {
      return { prefix: "table", name: "Table", className: "wb-element wb-demo-table", width: 260, height: 160, minWidth: 120, minHeight: 72, rx: 8, shape: "table" };
    }
    return { prefix: "rectangle", name: "Rectangle", className: "wb-element wb-demo-lane", width: 180, height: 110, minWidth: 28, minHeight: 28, rx: 12, label: "", shape: "rect" };
  }

  function normalizedBox(start, end, minWidth, minHeight) {
    const deltaX = end.x - start.x;
    const deltaY = end.y - start.y;
    const width = Math.max(Math.abs(deltaX), minWidth);
    const height = Math.max(Math.abs(deltaY), minHeight);
    return {
      x: deltaX < 0 ? start.x - width : start.x,
      y: deltaY < 0 ? start.y - height : start.y,
      width,
      height
    };
  }

  function arrowHeadPoints(start, end) {
    const angle = Math.atan2(end.y - start.y, end.x - start.x);
    const size = 18;
    const left = {
      x: end.x - Math.cos(angle - Math.PI / 7) * size,
      y: end.y - Math.sin(angle - Math.PI / 7) * size
    };
    const right = {
      x: end.x - Math.cos(angle + Math.PI / 7) * size,
      y: end.y - Math.sin(angle + Math.PI / 7) * size
    };
    return `${round(end.x)},${round(end.y)} ${round(left.x)},${round(left.y)} ${round(right.x)},${round(right.y)}`;
  }

  function diamondPoints(box) {
    const centerX = box.x + box.width / 2;
    const centerY = box.y + box.height / 2;
    return [
      [centerX, box.y],
      [box.x + box.width, centerY],
      [centerX, box.y + box.height],
      [box.x, centerY]
    ].map(([x, y]) => `${round(x)},${round(y)}`).join(" ");
  }

  function updateTableGuides(id, box) {
    const guides = objectNodes(id).filter((node) => node.dataset.whiteboardTableGuide);
    const verticals = [1, 2].map((index) => box.x + (box.width * index) / 3);
    const horizontals = [1, 2].map((index) => box.y + (box.height * index) / 3);
    const positions = [
      { x1: verticals[0], y1: box.y, x2: verticals[0], y2: box.y + box.height },
      { x1: verticals[1], y1: box.y, x2: verticals[1], y2: box.y + box.height },
      { x1: box.x, y1: horizontals[0], x2: box.x + box.width, y2: horizontals[0] },
      { x1: box.x, y1: horizontals[1], x2: box.x + box.width, y2: horizontals[1] }
    ];
    positions.forEach((position, index) => {
      const line = guides[index];
      if (!line) return;
      setAttribute(line, "x1", round(position.x1));
      setAttribute(line, "y1", round(position.y1));
      setAttribute(line, "x2", round(position.x2));
      setAttribute(line, "y2", round(position.y2));
    });
  }

  function pathFromPoints(points) {
    return points.map((point, index) => `${index ? "L" : "M"} ${round(point.x)} ${round(point.y)}`).join(" ");
  }

  function createDraft(kind, start) {
    if (kind === "line") {
      const id = nextObjectId("line");
      const path = createSvg("path", {
        class: "wb-element wb-demo-line",
        "data-whiteboard-element-id": id,
        "data-whiteboard-object-id": id,
        "data-whiteboard-kind": "line",
        "data-whiteboard-name": "Line"
      });
      insertObjectNodes([path]);
      paintObject(id, state.activeSwatch);
      return { id, kind, start, points: [start] };
    }
    if (kind === "arrow") {
      const id = nextObjectId("arrow");
      const path = createSvg("path", {
        class: "wb-element wb-demo-arrow",
        "data-whiteboard-element-id": id,
        "data-whiteboard-object-id": id,
        "data-whiteboard-kind": "arrow",
        "data-whiteboard-name": "Arrow"
      });
      const head = createSvg("polygon", {
        class: "wb-demo-arrow-head",
        "data-whiteboard-owned-by": id
      });
      insertObjectNodes([path, head]);
      paintObject(id, state.activeSwatch);
      return { id, kind, start, points: [start] };
    }
    if (kind === "text") {
      const id = nextObjectId("text");
      const text = createSvg("text", {
        class: "wb-element wb-element-label wb-demo-note",
        "data-whiteboard-element-id": id,
        "data-whiteboard-object-id": id,
        "data-whiteboard-kind": "text",
        "data-whiteboard-name": "Text note",
        "data-whiteboard-font-name": DEFAULT_TEXT_FONT,
        "data-whiteboard-font-family": fontFamilyForName(DEFAULT_TEXT_FONT),
        "font-family": fontFamilyForName(DEFAULT_TEXT_FONT),
        x: round(start.x),
        y: round(start.y)
      });
      text.textContent = "Text";
      insertObjectNodes([text]);
      paintObject(id, state.activeSwatch);
      return { id, kind, start, points: [start] };
    }
    if (kind === "math") {
      const id = nextObjectId("math");
      const text = createSvg("text", {
        class: "wb-element wb-element-label wb-demo-math",
        "data-whiteboard-element-id": id,
        "data-whiteboard-object-id": id,
        "data-whiteboard-kind": "math",
        "data-whiteboard-name": "Math",
        "data-whiteboard-font-name": DEFAULT_TEXT_FONT,
        "data-whiteboard-font-family": fontFamilyForName(DEFAULT_TEXT_FONT),
        "font-family": fontFamilyForName(DEFAULT_TEXT_FONT),
        x: round(start.x),
        y: round(start.y)
      });
      text.textContent = "f(x)";
      insertObjectNodes([text]);
      paintObject(id, state.activeSwatch);
      return { id, kind, start, points: [start] };
    }
    if (kind === "freehand") {
      const id = nextObjectId("path");
      const path = createSvg("path", {
        class: "wb-element wb-demo-path",
        "data-whiteboard-element-id": id,
        "data-whiteboard-object-id": id,
        "data-whiteboard-kind": "freehand",
        "data-whiteboard-name": "Freehand path",
        d: `M ${round(start.x)} ${round(start.y)}`
      });
      insertObjectNodes([path]);
      paintObject(id, state.activeSwatch);
      return { id, kind, start, points: [start] };
    }

    const defaults = shapeDefaults(kind);
    const id = nextObjectId(defaults.prefix);
    const baseAttributes = {
      class: defaults.className,
      "data-whiteboard-element-id": id,
      "data-whiteboard-object-id": id,
      "data-whiteboard-kind": kind,
      "data-whiteboard-name": defaults.name
    };
    const primary = defaults.shape === "ellipse"
      ? createSvg("ellipse", { ...baseAttributes, cx: round(start.x), cy: round(start.y), rx: 1, ry: 1 })
      : defaults.shape === "diamond"
        ? createSvg("polygon", { ...baseAttributes, points: diamondPoints({ x: start.x, y: start.y, width: 1, height: 1 }) })
        : createSvg("rect", { ...baseAttributes, x: round(start.x), y: round(start.y), width: 1, height: 1, rx: defaults.rx || 0 });
    const items = [primary];
    if (defaults.shape === "table") {
      for (let index = 0; index < 4; index += 1) {
        items.push(createSvg("line", {
          class: "wb-demo-table-guide",
          "data-whiteboard-owned-by": id,
          "data-whiteboard-table-guide": String(index + 1)
        }));
      }
    }
    if (defaults.label) {
      const label = createSvg("text", {
        class: "wb-element-label wb-demo-lane-label",
        "data-whiteboard-owned-by": id,
        x: round(start.x + 22),
        y: round(start.y + 56)
      });
      label.textContent = defaults.label;
      items.push(label);
    }
    insertObjectNodes(items);
    paintObject(id, state.activeSwatch);
    return { id, kind, start, points: [start] };
  }

  function updateDraft(draft, end, forceDefault) {
    if (draft.kind === "line") {
      const start = draft.start;
      let target = end;
      if (forceDefault && distanceBetween(start, end) < CREATE_THRESHOLD) target = { x: start.x + 160, y: start.y };
      const path = primaryObject(draft.id);
      setAttribute(path, "d", `M ${round(start.x)} ${round(start.y)} L ${round(target.x)} ${round(target.y)}`);
      return;
    }

    if (draft.kind === "arrow") {
      const start = draft.start;
      let target = end;
      if (forceDefault && distanceBetween(start, end) < CREATE_THRESHOLD) target = { x: start.x + 160, y: start.y };
      const path = primaryObject(draft.id);
      const head = objectNodes(draft.id).find((node) => node.tagName === "polygon");
      setAttribute(path, "d", `M ${round(start.x)} ${round(start.y)} L ${round(target.x)} ${round(target.y)}`);
      setAttribute(head, "points", arrowHeadPoints(start, target));
      return;
    }

    if (draft.kind === "text" || draft.kind === "math") {
      const text = primaryObject(draft.id);
      const box = normalizedBox(draft.start, end, 80, 34);
      setAttribute(text, "x", round(box.x));
      setAttribute(text, "y", round(box.y + Math.min(46, Math.max(24, box.height))));
      setAttribute(text, "font-size", round(Math.min(52, Math.max(24, box.height))));
      return;
    }

    if (draft.kind === "freehand") {
      const path = primaryObject(draft.id);
      const last = draft.points[draft.points.length - 1];
      if (distanceBetween(last, end) > 3) draft.points.push(end);
      const points = draft.points.length > 1 ? draft.points : [draft.start, end];
      setAttribute(path, "d", pathFromPoints(points));
      return;
    }

    const defaults = shapeDefaults(draft.kind);
    const primary = primaryObject(draft.id);
    const useDefault = forceDefault && distanceBetween(draft.start, end) < CREATE_THRESHOLD;
    const box = useDefault
      ? { x: draft.start.x, y: draft.start.y, width: defaults.width, height: defaults.height }
      : normalizedBox(draft.start, end, defaults.minWidth, defaults.minHeight);
    if (defaults.shape === "ellipse") {
      setAttribute(primary, "cx", round(box.x + box.width / 2));
      setAttribute(primary, "cy", round(box.y + box.height / 2));
      setAttribute(primary, "rx", round(box.width / 2));
      setAttribute(primary, "ry", round(box.height / 2));
    } else if (defaults.shape === "diamond") {
      setAttribute(primary, "points", diamondPoints(box));
    } else {
      setAttribute(primary, "x", round(box.x));
      setAttribute(primary, "y", round(box.y));
      setAttribute(primary, "width", round(box.width));
      setAttribute(primary, "height", round(box.height));
      if (defaults.shape === "table") updateTableGuides(draft.id, box);
    }
    const label = objectNodes(draft.id).find((node) => node.tagName === "text");
    if (label) {
      setAttribute(label, "x", round(box.x + 22));
      setAttribute(label, "y", round(box.y + Math.min(box.height / 2 + 12, 56)));
    }
  }

  function removeDraft(draft) {
    for (const node of objectNodes(draft.id)) node.remove();
    if (state.history.length) state.history.pop();
    state.nextId = Math.max(1, state.nextId - 1);
    refresh();
  }

  function startCreate(event, kind) {
    pushHistory();
    const start = pointFromEvent(event);
    const draft = createDraft(kind, start);
    stage.dataset.whiteboardDrag = "create";
    state.drag = {
      mode: "create",
      id: draft.id,
      kind,
      draft,
      startX: start.x,
      startY: start.y,
      moved: false
    };
    updateDraft(draft, start, false);
    svg.setPointerCapture?.(event.pointerId);
  }

  function finishCreate(event) {
    const drag = state.drag;
    if (!drag || drag.mode !== "create") return;
    state.controllerId = "";
    const end = pointFromEvent(event);
    const start = { x: drag.startX, y: drag.startY };
    const moved = drag.moved || distanceBetween(start, end) >= CREATE_THRESHOLD;
    if (!moved && drag.kind !== "text") {
      removeDraft(drag.draft);
      return;
    }
    updateDraft(drag.draft, end, true);
    addOutlineRow(drag.id, primaryObject(drag.id)?.dataset.whiteboardName || readableType(drag.id), readableType(drag.id));
    selectObject(drag.id, { openPanel: false });
    persist();
  }

  function addObject(kind, point) {
    pushHistory();
    const start = { x: Math.max(40, point?.x || 520), y: Math.max(40, point?.y || 300) };
    const draft = createDraft(kind, start);
    const defaults = shapeDefaults(kind);
    const end = kind === "arrow"
      ? { x: start.x + 180, y: start.y }
      : { x: start.x + defaults.width, y: start.y + defaults.height };
    updateDraft(draft, end, true);
    appendObject(draft.id);
  }

  function deleteSelected() {
    if (!state.selectedId) return;
    pushHistory();
    for (const node of objectNodes(state.selectedId)) node.remove();
    const row = root.querySelector(`[data-whiteboard-select="${safeSelector(state.selectedId)}"]`);
    row?.remove();
    const next = svg.querySelector("[data-whiteboard-object-id]");
    state.selectedId = next?.dataset.whiteboardObjectId || "";
    state.controllerId = "";
    refresh();
    persist();
  }

  function clearBoard() {
    const objects = svgNodes("[data-whiteboard-object-id], [data-whiteboard-owned-by]");
    if (!objects.length) {
      state.selectedId = "";
      state.controllerId = "";
      setPaintPopoverOpen(false);
      setSidePanelOpen(false);
      refresh();
      persist();
      return;
    }
    pushHistory();
    for (const node of objects) node.remove();
    for (const row of nodes("[data-whiteboard-select]")) row.remove();
    state.selectedId = "";
    state.controllerId = "";
    state.nextId = 1;
    setPaintPopoverOpen(false);
    setSidePanelOpen(false);
    refresh();
    persist();
  }

  function startObjectDrag(event, id) {
    const point = pointFromEvent(event);
    const offset = objectOffset(id);
    stage.dataset.whiteboardDrag = "object";
    state.drag = {
      mode: "object",
      id,
      startX: point.x,
      startY: point.y,
      originX: offset.x,
      originY: offset.y
    };
    svg.setPointerCapture?.(event.pointerId);
  }

  function startPan(event) {
    stage.dataset.whiteboardDrag = "pan";
    state.drag = {
      mode: "pan",
      x: event.clientX,
      y: event.clientY,
      scrollLeft: scrollHost?.scrollLeft || 0,
      scrollTop: scrollHost?.scrollTop || 0
    };
  }

  function bindControl(selector, handler) {
    for (const button of nodes(selector)) {
      if (button.dataset.whiteboardBound === "true") continue;
      button.dataset.whiteboardBound = "true";
      button.addEventListener("click", (event) => {
        event.__dxWhiteboardHandled = true;
        handler(button, event);
      });
    }
  }

  function bindControlButtons() {
    bindControl("[data-whiteboard-tool-button]", (button) => setTool(button.dataset.whiteboardToolButton));
    bindControl("[data-whiteboard-command]", (button) => {
      if (button.dataset.whiteboardCommand === "undo") undo();
      if (button.dataset.whiteboardCommand === "redo") redo();
      if (button.dataset.whiteboardCommand === "clear") clearBoard();
    });
    bindControl("[data-whiteboard-zoom]", (button) => {
      if (button.dataset.whiteboardZoom === "in") setZoom(state.zoom * 1.12);
      if (button.dataset.whiteboardZoom === "out") setZoom(state.zoom / 1.12);
      if (button.dataset.whiteboardZoom === "fit") setZoom(1);
    });
    bindControl("[data-whiteboard-toggle]", (button) => {
      if (button.dataset.whiteboardToggle === "minimap") setMinimapVisible(!state.minimapVisible);
      if (button.dataset.whiteboardToggle === "side-panel") setSidePanelOpen(!state.sidePanelOpen);
      if (button.dataset.whiteboardToggle === "shortcuts") setShortcutsPanelOpen(!state.shortcutPanelOpen);
    });
    bindControl("[data-whiteboard-select]", (button) => selectObject(button.dataset.whiteboardSelect));
    bindControl("[data-whiteboard-swatch]", (button) => applySwatch(button.dataset.whiteboardSwatch));
    bindControl("[data-whiteboard-add]", (button) => addObject(button.dataset.whiteboardAdd, { x: 520, y: 290 }));
    bindControl("[data-whiteboard-color-trigger]", () => {
      const popover = root.querySelector("[data-whiteboard-color-popover]");
      setPaintPopoverOpen(Boolean(popover?.hidden));
    });
    bindControl("[data-whiteboard-paint-mode]", (button) => {
      state.paintMode = button.dataset.whiteboardPaintMode || "solid";
      syncColorControls();
    });
    bindControl("[data-whiteboard-apply-custom-paint]", () => applyCustomPaint());
    bindControl("[data-whiteboard-gradient-step]", (button) => {
      if (button.dataset.whiteboardSuppressClick === "true") {
        delete button.dataset.whiteboardSuppressClick;
        return;
      }
      applyStepperButton(button, false);
    });
    bindControl("[data-whiteboard-field-step]", (button) => {
      if (button.dataset.whiteboardSuppressClick === "true") {
        delete button.dataset.whiteboardSuppressClick;
        return;
      }
      applyStepperButton(button, true);
    });
  }

  function bindInputs() {
    for (const input of nodes("[data-whiteboard-measure-input]")) {
      if (input.dataset.whiteboardInputBound === "true") continue;
      input.dataset.whiteboardInputBound = "true";
      input.addEventListener("change", () => {
        pushHistory();
        setSelectedGeometry(input.dataset.whiteboardMeasureInput, Number(input.value));
      });
    }
    for (const input of nodes("[data-whiteboard-color-input], [data-whiteboard-gradient-angle]")) {
      if (input.dataset.whiteboardInputBound === "true") continue;
      input.dataset.whiteboardInputBound = "true";
      input.addEventListener("input", () => {
        if (input.dataset.whiteboardColorInput === "solid") state.customSolid = input.value;
        if (input.dataset.whiteboardColorInput === "start") state.customGradientStart = input.value;
        if (input.dataset.whiteboardColorInput === "end") state.customGradientEnd = input.value;
        if (input.hasAttribute("data-whiteboard-gradient-angle")) {
          state.customGradientAngle = clamp(Number(input.value || 0), 0, 360);
        }
        buildCustomPaint();
        state.activeSwatch = "custom";
        syncColorControls();
      });
    }
    for (const input of nodes("[data-whiteboard-font-input]")) {
      if (input.dataset.whiteboardInputBound === "true") continue;
      input.dataset.whiteboardInputBound = "true";
      input.addEventListener("keydown", (event) => {
        if (event.key === "Enter") {
          event.preventDefault();
          applyFontToSelectedText(input.value);
        }
        if (event.key === "Escape") {
          event.preventDefault();
          syncFontControls();
          input.blur();
          hideTooltip();
        }
      });
      input.addEventListener("change", () => applyFontToSelectedText(input.value));
    }
    for (const input of nodes("[data-whiteboard-media-input]")) {
      if (input.dataset.whiteboardInputBound === "true") continue;
      input.dataset.whiteboardInputBound = "true";
      input.addEventListener("cancel", () => {
        input.value = "";
        hideTooltip();
      });
      input.addEventListener("change", () => {
        const file = input.files?.[0];
        readMediaFile(input.dataset.whiteboardMediaInput, file);
        input.value = "";
        hideTooltip();
      });
    }
  }

  function bindTextEditor() {
    if (!textEditor || textEditor.dataset.whiteboardInputBound === "true") return;
    textEditor.dataset.whiteboardInputBound = "true";
    textEditor.addEventListener("keydown", (event) => {
      event.stopPropagation();
      if (event.key === "Enter") {
        event.preventDefault();
        hideTooltip();
        commitTextEditor();
      }
      if (event.key === "Escape") {
        event.preventDefault();
        hideTooltip();
        cancelTextEditor();
      }
    });
    textEditor.addEventListener("blur", () => commitTextEditor());
  }

  function isTypingTarget(target) {
    const tag = target?.tagName;
    return tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT" || target?.isContentEditable;
  }

  function handleControlClick(target) {
    const tooltipTarget = target.closest?.("[data-whiteboard-tooltip]");
    if (tooltipTarget) showTooltip(tooltipTarget);
    const toolButton = target.closest?.("[data-whiteboard-tool-button]");
    if (toolButton) {
      closeTransientUi();
      setTool(toolButton.dataset.whiteboardToolButton);
      if (shapeTools.has(toolButton.dataset.whiteboardToolButton)) setShapeMenuOpen(false);
      return true;
    }
    const shapeTrigger = target.closest?.("[data-whiteboard-shape-menu-trigger]");
    if (shapeTrigger) {
      setShapeMenuOpen(!state.shapeMenuOpen);
      setIconMenuOpen(false);
      return true;
    }
    const iconTrigger = target.closest?.("[data-whiteboard-icon-menu-trigger]");
    if (iconTrigger) {
      setIconMenuOpen(!state.iconMenuOpen);
      setShapeMenuOpen(false);
      setPaintPopoverOpen(false);
      return true;
    }
    const iconToken = target.closest?.("[data-whiteboard-icon-insert]")?.dataset.whiteboardIconInsert;
    if (iconToken) {
      closeTransientUi();
      addIconObject(iconToken);
      return true;
    }
    const mediaPick = target.closest?.("[data-whiteboard-media-pick]")?.dataset.whiteboardMediaPick;
    if (mediaPick) {
      closeTransientUi();
      root.querySelector(`[data-whiteboard-media-input="${safeSelector(mediaPick)}"]`)?.click();
      return true;
    }
    const command = target.closest?.("[data-whiteboard-command]")?.dataset.whiteboardCommand;
    if (command === "undo") {
      undo();
      return true;
    }
    if (command === "redo") {
      redo();
      return true;
    }
    if (command === "clear") {
      clearBoard();
      return true;
    }
    const zoom = target.closest?.("[data-whiteboard-zoom]")?.dataset.whiteboardZoom;
    if (zoom === "in") {
      setZoom(state.zoom * 1.12);
      return true;
    }
    if (zoom === "out") {
      setZoom(state.zoom / 1.12);
      return true;
    }
    if (zoom === "fit") {
      setZoom(1);
      return true;
    }
    const toggle = target.closest?.("[data-whiteboard-toggle]")?.dataset.whiteboardToggle;
    if (toggle === "minimap") {
      setMinimapVisible(!state.minimapVisible);
      return true;
    }
    if (toggle === "side-panel") {
      closeTransientUi();
      setSidePanelOpen(!state.sidePanelOpen);
      return true;
    }
    if (toggle === "shortcuts") {
      const wasOpen = state.shortcutPanelOpen;
      closeTransientUi();
      setShortcutsPanelOpen(!wasOpen);
      return true;
    }
    const colorTrigger = target.closest?.("[data-whiteboard-color-trigger]");
    if (colorTrigger) {
      const popover = root.querySelector("[data-whiteboard-color-popover]");
      setPaintPopoverOpen(Boolean(popover?.hidden));
      return true;
    }
    const paintMode = target.closest?.("[data-whiteboard-paint-mode]")?.dataset.whiteboardPaintMode;
    if (paintMode) {
      state.paintMode = paintMode;
      syncColorControls();
      return true;
    }
    if (target.closest?.("[data-whiteboard-apply-custom-paint]")) {
      applyCustomPaint();
      return true;
    }
    if (target.closest?.("[data-whiteboard-apply-font]")) {
      const input = root.querySelector("[data-whiteboard-font-input]");
      applyFontToSelectedText(input?.value || DEFAULT_TEXT_FONT);
      return true;
    }
    const gradientStep = target.closest?.("[data-whiteboard-gradient-step]")?.dataset.whiteboardGradientStep;
    if (gradientStep) {
      const button = target.closest("[data-whiteboard-gradient-step]");
      if (button.dataset.whiteboardSuppressClick === "true") {
        delete button.dataset.whiteboardSuppressClick;
        return true;
      }
      applyStepperButton(button, false);
      return true;
    }
    const fieldStep = target.closest?.("[data-whiteboard-field-step]");
    if (fieldStep && state.selectedId) {
      if (fieldStep.dataset.whiteboardSuppressClick === "true") {
        delete fieldStep.dataset.whiteboardSuppressClick;
        return true;
      }
      applyStepperButton(fieldStep, true);
      return true;
    }
    const selection = target.closest?.("[data-whiteboard-select]")?.dataset.whiteboardSelect;
    if (selection) {
      selectObject(selection);
      return true;
    }
    const swatch = target.closest?.("[data-whiteboard-swatch]")?.dataset.whiteboardSwatch;
    if (swatch) {
      applySwatch(swatch);
      return true;
    }
    const add = target.closest?.("[data-whiteboard-add]")?.dataset.whiteboardAdd;
    if (add) {
      addObject(add, { x: 520, y: 290 });
      return true;
    }
    return false;
  }

  root.addEventListener("click", (event) => {
    if (handleControlClick(event.target)) {
      event.preventDefault();
      event.stopPropagation();
      event.__dxWhiteboardHandled = true;
    }
  }, true);

  root.addEventListener("pointerdown", startStepHold, true);

  root.addEventListener("click", (event) => {
    if (event.__dxWhiteboardHandled) return;
    const target = event.target;
    if (!target.closest?.("[data-whiteboard-color-control]")) setPaintPopoverOpen(false);
    if (!target.closest?.("[data-whiteboard-tool-popover]")) {
      setShapeMenuOpen(false);
      setIconMenuOpen(false);
    }
  });

  svg.addEventListener("pointerdown", (event) => {
    const target = event.target;
    const mediaControl = target.closest?.("audio, video");
    if (mediaControl) {
      const mediaObject = target.closest?.("[data-whiteboard-object-id], [data-whiteboard-owned-by]");
      const mediaId = mediaObject?.dataset.whiteboardObjectId || mediaObject?.dataset.whiteboardOwnedBy;
      if (mediaId) {
        setTool("select");
        selectObject(mediaId);
      }
      return;
    }
    event.preventDefault();
    const object = target.closest?.("[data-whiteboard-object-id], [data-whiteboard-owned-by]");
    const id = object?.dataset.whiteboardObjectId || object?.dataset.whiteboardOwnedBy;
    if (id) {
      setTool("select");
      selectObject(id);
      pushHistory();
      startObjectDrag(event, id);
      return;
    }
    if (state.tool === "hand") {
      startPan(event);
      return;
    }
    if (state.tool !== "select") startCreate(event, state.tool);
  });

  svg.addEventListener("dblclick", (event) => {
    event.preventDefault();
    event.stopPropagation();
    const target = event.target;
    const object = target.closest?.("[data-whiteboard-object-id], [data-whiteboard-owned-by]");
    const id = object?.dataset.whiteboardObjectId || object?.dataset.whiteboardOwnedBy;
    if (!id) return;
    setTool("select");
    if (editableTextObject(id)) {
      openTextEditor(id);
      return;
    }
    showObjectController(id);
  });

  window.addEventListener("pointermove", (event) => {
    if (!state.drag) return;
    if (state.drag.mode === "pan" && scrollHost) {
      scrollHost.scrollLeft = state.drag.scrollLeft - (event.clientX - state.drag.x);
      scrollHost.scrollTop = state.drag.scrollTop - (event.clientY - state.drag.y);
      return;
    }
    if (state.drag.mode === "object") {
      state.controllerId = "";
      const point = pointFromEvent(event);
      setObjectOffset(
        state.drag.id,
        state.drag.originX + point.x - state.drag.startX,
        state.drag.originY + point.y - state.drag.startY
      );
      updateSelectionOutline();
      updateInspector();
      return;
    }
    if (state.drag.mode === "create") {
      const point = pointFromEvent(event);
      const start = { x: state.drag.startX, y: state.drag.startY };
      state.drag.moved = state.drag.moved || distanceBetween(start, point) >= CREATE_THRESHOLD;
      updateDraft(state.drag.draft, point, false);
      updateSelectionOutline();
      updateInspector();
    }
  }, { passive: false });

  window.addEventListener("pointerup", (event) => {
    if (!state.drag) return;
    const activeDrag = state.drag;
    if (activeDrag.mode === "create") finishCreate(event);
    state.drag = null;
    delete stage.dataset.whiteboardDrag;
    persist();
  });

  window.addEventListener("pointerup", clearStepHold);
  window.addEventListener("pointercancel", clearStepHold);
  window.addEventListener("blur", clearStepHold);

  window.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      if (event.target?.matches?.("[data-whiteboard-font-input]")) {
        event.preventDefault();
        syncFontControls();
        event.target.blur();
        hideTooltip();
        return;
      }
      if (!textEditor?.hidden) cancelTextEditor();
      const changed = closeTransientUi({ closePanel: true });
      if (changed) event.preventDefault();
      return;
    }
    if (event.key === "?" || (event.key === "/" && event.shiftKey)) {
      if (!isTypingTarget(event.target)) {
        event.preventDefault();
        const wasOpen = state.shortcutPanelOpen;
        closeTransientUi();
        setShortcutsPanelOpen(!wasOpen);
      }
      return;
    }
    if (isTypingTarget(event.target)) return;
    const key = event.key.toLowerCase();
    if ((event.ctrlKey || event.metaKey) && key === "z") {
      event.preventDefault();
      event.shiftKey ? redo() : undo();
      return;
    }
    if ((event.ctrlKey || event.metaKey) && key === "y") {
      event.preventDefault();
      redo();
      return;
    }
    if (event.key === "Delete" || event.key === "Backspace") {
      event.preventDefault();
      deleteSelected();
      return;
    }
    const shortcuts = { v: "select", h: "hand", p: "freehand", f: "freehand", l: "line", a: "arrow", r: "rectangle", d: "diamond", o: "ellipse", b: "table", m: "math", t: "text" };
    if (!event.ctrlKey && !event.metaKey && shortcuts[key]) {
      closeTransientUi();
      setTool(shortcuts[key]);
    }
  });

  ensureGoogleFont(DEFAULT_TEXT_FONT);
  loadPersisted();
  markDxIcons();
  hydrateTooltips();
  bindTooltips();
  syncOutlineRows();
  bindControlButtons();
  bindInputs();
  bindTextEditor();
  closeTextEditor();
  setTool(state.tool);
  setShapeMenuOpen(false);
  setIconMenuOpen(false);
  setSidePanelOpen(false);
  setShortcutsPanelOpen(false);
  setMinimapVisible(false);
  syncColorControls();
  refresh();
})();
