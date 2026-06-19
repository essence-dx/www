var MODES = [
  { id: 0, key: "chrome",   name: "Chrome",   full: "Liquid Chrome",
    icon: '<svg viewBox="0 0 26 18"><path d="M1 12 C5 4 9 15 13 9 C17 3 21 13 25 7" fill="none" stroke="currentColor" stroke-width="1.5"/><path d="M1 15 C5 9 10 17 14 12 C18 8 22 15 25 11" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.45"/></svg>' },
  { id: 1, key: "silk",     name: "Silk",     full: "Silk Ribbons",
    icon: '<svg viewBox="0 0 26 18"><path d="M1 13 C8 11 12 3 25 4" fill="none" stroke="currentColor" stroke-width="1.5"/><path d="M1 15.5 C8 13.5 12 5.5 25 6.5" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.65"/><path d="M1 18 C8 16 12 8 25 9" fill="none" stroke="currentColor" stroke-width="1.5" opacity="0.35"/></svg>' },
  { id: 2, key: "bloom",    name: "Bloom",    full: "Soft Bloom",
    icon: '<svg viewBox="0 0 26 18"><circle cx="9" cy="8" r="5.5" fill="currentColor" opacity="0.35"/><circle cx="17" cy="11" r="4" fill="currentColor" opacity="0.6"/></svg>' },
  { id: 3, key: "aura",     name: "Aura",     full: "Aura Rings",
    icon: '<svg viewBox="0 0 26 18"><circle cx="13" cy="9" r="7" fill="none" stroke="currentColor" stroke-width="1.4" opacity="0.35"/><circle cx="13" cy="9" r="4.2" fill="none" stroke="currentColor" stroke-width="1.4" opacity="0.7"/><circle cx="13" cy="9" r="1.6" fill="currentColor"/></svg>' },
  { id: 4, key: "rays",     name: "Rays",     full: "Light Rays",
    icon: '<svg viewBox="0 0 26 18"><path d="M13 1 L7 17 M13 1 L13 17 M13 1 L19 17 M13 1 L2 13 M13 1 L24 13" fill="none" stroke="currentColor" stroke-width="1.3" opacity="0.8"/></svg>' },
  { id: 5, key: "halftone", name: "Halftone", full: "Halftone",
    icon: '<svg viewBox="0 0 26 18"><circle cx="4" cy="5" r="2.4" fill="currentColor"/><circle cx="11" cy="5" r="1.8" fill="currentColor"/><circle cx="18" cy="5" r="1.2" fill="currentColor"/><circle cx="24" cy="5" r="0.7" fill="currentColor"/><circle cx="4" cy="12" r="1.6" fill="currentColor"/><circle cx="11" cy="12" r="2.2" fill="currentColor"/><circle cx="18" cy="12" r="1.5" fill="currentColor"/><circle cx="24" cy="12" r="0.9" fill="currentColor"/></svg>' },
  { id: 6, key: "glyphs",   name: "Glyphs",   full: "Data Glyphs",
    icon: '<svg viewBox="0 0 26 18"><g fill="currentColor"><rect x="2" y="2" width="2" height="3"/><rect x="7" y="2" width="2" height="3" opacity="0.5"/><rect x="12" y="2" width="2" height="3"/><rect x="17" y="2" width="2" height="3" opacity="0.3"/><rect x="22" y="2" width="2" height="3" opacity="0.7"/><rect x="2" y="8" width="2" height="3" opacity="0.4"/><rect x="7" y="8" width="2" height="3"/><rect x="12" y="8" width="2" height="3" opacity="0.6"/><rect x="17" y="8" width="2" height="3"/><rect x="22" y="8" width="2" height="3" opacity="0.4"/><rect x="2" y="14" width="2" height="3" opacity="0.7"/><rect x="7" y="14" width="2" height="3" opacity="0.3"/><rect x="12" y="14" width="2" height="3" opacity="0.8"/><rect x="17" y="14" width="2" height="3" opacity="0.5"/><rect x="22" y="14" width="2" height="3"/></g></svg>' },
  { id: 7, key: "reeded",   name: "Reeded",   full: "Reeded Glass",
    icon: '<svg viewBox="0 0 26 18"><g stroke="currentColor" stroke-width="1.8" fill="none"><path d="M3 1 V17" opacity="0.9"/><path d="M8 1 V17" opacity="0.5"/><path d="M13 1 V17" opacity="0.9"/><path d="M18 1 V17" opacity="0.5"/><path d="M23 1 V17" opacity="0.9"/></g></svg>' },
  { id: 8, key: "mosaic",   name: "Mosaic",   full: "Pixel Bloom",
    icon: '<svg viewBox="0 0 26 18"><g fill="currentColor"><rect x="2" y="2" width="6" height="6" opacity="0.9"/><rect x="9" y="2" width="6" height="6" opacity="0.4"/><rect x="16" y="2" width="6" height="6" opacity="0.7"/><rect x="2" y="9" width="6" height="6" opacity="0.3"/><rect x="9" y="9" width="6" height="6" opacity="0.8"/><rect x="16" y="9" width="6" height="6" opacity="0.5"/></g></svg>' }
];

var ASPECTS = { "16:9": 16 / 9, "3:2": 1.5, "1:1": 1, "4:5": 0.8, "21:9": 21 / 9 };

var P = {
  mode: 7, seed: 9015,
  c1: "#ff6a00", c2: "#ffb347", c3: "#a81c00", c4: "#3d0c02", bg: "#070403",
  hue: 23, sat: 0.55, exposure: 1.016, contrast: 0.957,
  scale: 0.803, complex: 5.603, warp: 1.073, flow: 0.233, stretch: -0.089,
  light: 1.175, gloss: 44, lightAngle: 235, irid: 0.012, glow: 0.471,
  grain: 0.024, cell: 113, lines: 67, ca: 0.018, vig: 0.079, soft: 1.14,
  travel: 0.72, loop: 7.5,
  synthOn: false, modeB: 2, mixOp: 0, blend: 0.6,
  genomeOn: false, genes: [0,0,0.5,3, 0,0,0,0.5, 0.5,0.5,0.5,0],
  lockStyle: false,
  imgRes: "2160", vidRes: "1080", vidFps: "30", vidLen: "l2",
  gifW: "640", gifFps: 25, gifDither: true, gifLoop: true,
  aspect: "16:9"
};

var DEF_RANGE = {
  scale: [0.8, 1.8], complex: [3, 6], warp: [0.3, 1.5], flow: [0, 1], stretch: [-0.3, 0.5],
  light: [0.6, 1.6], gloss: [16, 80], lightAngle: [0, 360], irid: [0, 0.8], glow: [0, 0.5],
  contrast: [0.95, 1.2], grain: [0, 0.12], cell: [40, 140], lines: [20, 100], ca: [0, 0.4],
  vig: [0, 0.4], soft: [0.6, 1.4], sat: [0.95, 1.35], exposure: [0.95, 1.1], hue: [0, 0],
  travel: [0.3, 1.0], loop: [4, 8]
};

var RECIPES = {
  chrome: { tone: ["dark"], scale: [1.2, 2.4], complex: [2.8, 5], warp: [0.7, 1.9], flow: [0.2, 1.0],
    stretch: [0.25, 0.7], light: [1.2, 2.0], gloss: [36, 100], irid: [0.2, 0.85], glow: [0.15, 0.55],
    contrast: [1.0, 1.3], grain: [0.02, 0.1], ca: [0, 0.45], vig: [0.15, 0.5], sat: [1.0, 1.4],
    travel: [0.4, 1.0], loop: [4, 8] },
  silk: { tone: ["dark"], scale: [1.2, 2.2], warp: [0.3, 1.0], lines: [40, 90], gloss: [24, 80],
    light: [1.0, 1.9], stretch: [0, 0.4], irid: [0.2, 0.7], glow: [0.2, 0.6], grain: [0.02, 0.08],
    vig: [0.1, 0.4], sat: [1.0, 1.45], contrast: [1.0, 1.25], travel: [0.3, 0.8], loop: [4, 9] },
  bloom: { tone: ["light", "light", "dark"], scale: [0.8, 1.6], warp: [0.3, 1.4], soft: [0.8, 1.5],
    light: [0, 0.6], glow: [0, 0.3], grain: [0, 0.07], ca: [0, 0.25], vig: [0, 0.25],
    contrast: [0.9, 1.05], sat: [0.9, 1.3], travel: [0.5, 1.2], loop: [5, 10] },
  aura: { tone: ["light", "light", "dark"], scale: [0.9, 1.6], warp: [0.2, 1.0], soft: [0.9, 1.6],
    grain: [0, 0.06], contrast: [0.9, 1.05], sat: [0.85, 1.15], vig: [0, 0.2], glow: [0, 0.35],
    ca: [0, 0.08], travel: [0.3, 0.9], loop: [5, 10] },
  rays: { tone: ["light", "light", "dark"], warp: [0.3, 1.2], lines: [20, 90], grain: [0, 0.1],
    glow: [0.1, 0.5], contrast: [0.95, 1.2], sat: [1.0, 1.4], vig: [0, 0.3], travel: [0.2, 0.7],
    loop: [5, 10], scale: [0.9, 1.5] },
  halftone: { tone: ["light", "light", "dark"], cell: [60, 150], scale: [0.9, 1.8], warp: [0.4, 1.4],
    grain: [0, 0.05], contrast: [0.95, 1.15], sat: [0.95, 1.3], ca: [0, 0.2], vig: [0, 0.2],
    travel: [0.4, 1.0], loop: [4, 9] },
  glyphs: { tone: ["dark"], cell: [70, 150], scale: [0.8, 1.6], warp: [0.5, 1.5], grain: [0.02, 0.12],
    glow: [0.3, 0.8], contrast: [1.0, 1.3], sat: [1.0, 1.5], ca: [0, 0.5], vig: [0.2, 0.6],
    travel: [0.4, 1.0], loop: [3, 7] },
  reeded: { tone: ["light", "dark"], lines: [36, 90], warp: [0.6, 1.6], scale: [0.8, 1.5],
    grain: [0, 0.07], light: [0.4, 1.2], contrast: [0.95, 1.2], sat: [1.0, 1.35], vig: [0, 0.18],
    travel: [0.5, 1.1], loop: [5, 10] },
  mosaic: { tone: ["light", "dark"], cell: [40, 120], warp: [0.3, 1.2], scale: [0.8, 1.5],
    grain: [0, 0.04], contrast: [0.95, 1.15], sat: [0.95, 1.3], vig: [0, 0.15],
    travel: [0.5, 1.2], loop: [5, 10], soft: [0.8, 1.4] }
};

var FORM_KEYS = ["scale", "complex", "warp", "flow", "stretch"];
var LIGHT_KEYS = ["light", "gloss", "lightAngle", "irid", "glow", "contrast"];
var TEXTURE_KEYS = ["grain", "cell", "lines", "ca", "vig", "soft"];
var MOTION_KEYS = ["loop", "travel"];
var GRADE_KEYS = ["sat", "exposure"];

function rnd() { return Math.random(); }
function randIn(range) { return range[0] + rnd() * (range[1] - range[0]); }
function rangeFor(key) {
  var rec = RECIPES[MODES[P.mode].key] || {};
  return rec[key] || DEF_RANGE[key];
}

var activePreset = 3;
var setPresetActive = function () {};

function applyPalette(pal, presetIdx) {
  P.c1 = pal.colors[0]; P.c2 = pal.colors[1];
  P.c3 = pal.colors[2]; P.c4 = pal.colors[3];
  P.bg = pal.bg;
  activePreset = presetIdx;
  setPresetActive(presetIdx);
}

function randomizePalette() {
  var rec = RECIPES[MODES[P.mode].key] || { tone: ["dark", "light"] };
  var tone = rec.tone[Math.floor(rnd() * rec.tone.length)];
  if (rnd() < 0.25) {
    applyPalette(generateRandomPalette(rnd, tone), -1);
  } else {
    var pool = [];
    PALETTES.forEach(function (p, i) { if (p.tone === tone) pool.push(i); });
    var idx = pool[Math.floor(rnd() * pool.length)];
    applyPalette(PALETTES[idx], idx);
  }
  P.hue = 0;
  GRADE_KEYS.forEach(function (k) { P[k] = randIn(rangeFor(k)); });
}

function randomizeKeys(keys) {
  keys.forEach(function (k) {
    var v = randIn(rangeFor(k));
    if (k === "gloss" || k === "cell" || k === "lines" || k === "lightAngle") v = Math.round(v);
    if (k === "loop") v = Math.round(v * 2) / 2;
    P[k] = v;
  });
}

function newSeed() { P.seed = Math.floor(rnd() * 10000); }

function generateGenomeStyle() {
  P.genes = [
    Math.floor(rnd() * 6),
    Math.floor(rnd() * 5),
    rnd() * 1.6,
    2 + Math.floor(rnd() * 6),
    Math.floor(rnd() * 4),
    Math.floor(rnd() * 4),
    Math.floor(rnd() * 4),
    0.3 + rnd() * 1.2,
    rnd(),
    rnd(),
    0.2 + rnd() * 0.9,
    rnd()
  ];
  P.genomeOn = true;
  P.synthOn = false;
}

function genomeName() {
  var g = P.genes;
  var n = (g[0] * 7 + g[1] * 13 + g[4] * 29 + g[5] * 47 + g[6] * 71 +
    Math.round(g[11] * 99)) % 1000;
  var FIELD = ["FLUX", "RIDGE", "WAVE", "RING", "CELL", "FLOW"];
  return FIELD[g[0]] + " " + String(Math.round(n)).padStart(3, "0");
}

function styleName() {
  if (P.genomeOn) return genomeName();
  if (P.synthOn) return "SYN " + MODES[P.mode].name.slice(0, 3).toUpperCase() + "+" +
    MODES[P.modeB].name.slice(0, 3).toUpperCase();
  return MODES[P.mode].full;
}

function randomizeAll() {
  if (!P.lockStyle) {
    if (rnd() < 0.35) {
      generateGenomeStyle();
    } else {
      P.synthOn = false;
      P.genomeOn = false;
      P.mode = Math.floor(rnd() * MODES.length);
    }
  }
  randomizePalette();
  randomizeKeys(FORM_KEYS.concat(LIGHT_KEYS, TEXTURE_KEYS, MOTION_KEYS));
  newSeed();
  resetAdaptiveLiveBudget();
  refreshAll();
}

/* ---------------- saved styles (localStorage) ---------------- */

var STYLES_KEY = "dx-shader-styles-v1";
var LEGACY_STYLES_KEY = "lumen-styles-v1";
var MAX_SHARE_CODE_LENGTH = 8192;
var MAX_SAVED_STYLES_BYTES = 128 * 1024;
var MAX_SAVED_STYLES = 24;
var savedStylesHydrated = false;
var savedStylesCache = [];

function normalizeSavedStyles(styles) {
  if (!Array.isArray(styles)) return [];
  var normalized = [];
  for (var i = 0; i < styles.length && normalized.length < MAX_SAVED_STYLES; i++) {
    var style = styles[i];
    if (!style || typeof style !== "object") continue;
    if (typeof style.code !== "string" || style.code.length > MAX_SHARE_CODE_LENGTH) continue;
    normalized.push({
      name: typeof style.name === "string" ? style.name.slice(0, 96) : "Saved style",
      code: style.code,
      ts: Number(style.ts) || 0
    });
  }
  return normalized;
}

function trimSavedStylesForStorage(list) {
  var normalized = normalizeSavedStyles(list);
  while (normalized.length) {
    var serialized = JSON.stringify(normalized);
    if (serialized.length <= MAX_SAVED_STYLES_BYTES) {
      return { list: normalized, serialized: serialized };
    }
    normalized.pop();
  }
  return { list: [], serialized: "[]" };
}

function readSavedStylesRecord(key) {
  try {
    var raw = localStorage.getItem(key);
    if (raw === null) return { found: false, list: [] };
    if (!raw || raw.length > MAX_SAVED_STYLES_BYTES) return { found: true, list: [] };
    var parsed = JSON.parse(raw);
    return { found: true, list: normalizeSavedStyles(parsed) };
  }
  catch (e) { return { found: true, list: [] }; }
}
function persistSavedStyles(list) {
  var storage = trimSavedStylesForStorage(list);
  savedStylesCache = storage.list;
  savedStylesHydrated = true;
  try { localStorage.setItem(STYLES_KEY, storage.serialized); } catch (e) {}
}
function hydrateSavedStyles() {
  if (savedStylesHydrated) return;
  var current = readSavedStylesRecord(STYLES_KEY);
  savedStylesCache = current.found ? current.list : readSavedStylesRecord(LEGACY_STYLES_KEY).list;
  savedStylesHydrated = true;
  if (!current.found && savedStylesCache.length) persistSavedStyles(savedStylesCache);
}
function loadSavedStyles() {
  hydrateSavedStyles();
  return savedStylesCache.slice();
}
function saveCurrentStyle() {
  var list = loadSavedStyles();
  var name = styleName() + " " + String(Math.round(P.seed)).padStart(4, "0");
  list.unshift({ name: name, code: encodeDesign(), ts: Date.now() });
  if (list.length > MAX_SAVED_STYLES) list.length = MAX_SAVED_STYLES;
  persistSavedStyles(list);
  renderSavedStyles();
  UI.toast("Style saved: " + name);
}
var renderSavedStyles = function () {};

/* ---------------- design codes (share) ---------------- */

var SHARE_NUMS = [
  "hue", "sat", "exposure", "contrast",
  "scale", "complex", "warp", "flow", "stretch",
  "light", "gloss", "lightAngle", "irid", "glow",
  "grain", "cell", "lines", "ca", "vig", "soft",
  "travel", "loop"
];
var SHARE_NUM_LIMITS = {
  hue: [-180, 180], sat: [0, 2], exposure: [0.5, 1.6], contrast: [0.6, 1.6],
  scale: [0.5, 3], complex: [1, 8], warp: [0, 2.5], flow: [0, 2], stretch: [-1, 1],
  light: [0, 2.2], gloss: [4, 120], lightAngle: [0, 360], irid: [0, 1], glow: [0, 1],
  grain: [0, 0.4], cell: [14, 180], lines: [8, 160], ca: [0, 1], vig: [0, 1], soft: [0.3, 1.6],
  travel: [0, 1.5], loop: [2, 12]
};
var SHARE_INT_KEYS = { gloss: true, lightAngle: true, cell: true, lines: true };
var GENE_LIMITS = [
  [0, 5, true], [0, 4, true], [0, 1.6, false], [2, 7, true],
  [0, 3, true], [0, 3, true], [0, 3, true], [0.3, 1.5, false],
  [0, 1, false], [0, 1, false], [0.2, 1.1, false], [0, 1, false]
];
var SHARE_PREFIX = "DXS1.";
var LEGACY_SHARE_PREFIX = "LMN1.";
var SHARE_CODE_PATTERN = /^[A-Za-z0-9_-]+$/;

function safeDecodeHash(hash) {
  if (!hash || hash.length > MAX_SHARE_CODE_LENGTH) return "";
  try {
    return decodeURIComponent(hash);
  } catch (e) {
    return "";
  }
}

function clampNumber(value, min, max, fallback) {
  var next = Number(value);
  if (!isFinite(next)) return fallback;
  return Math.min(Math.max(next, min), max);
}

function clampShareNumber(key, value) {
  var limit = SHARE_NUM_LIMITS[key] || DEF_RANGE[key] || [0, 1];
  var fallback = isFinite(P[key]) ? P[key] : limit[0];
  var next = clampNumber(value, limit[0], limit[1], fallback);
  if (SHARE_INT_KEYS[key]) next = Math.round(next);
  if (key === "loop") next = Math.round(next * 2) / 2;
  return next;
}

function clampGene(index, value) {
  var limit = GENE_LIMITS[index] || [0, 1, false];
  var fallback = P.genes && isFinite(P.genes[index]) ? P.genes[index] : limit[0];
  var next = clampNumber(value, limit[0], limit[1], fallback);
  return limit[2] ? Math.round(next) : next;
}

function encodeDesign() {
  var arr = [3, P.mode, Math.round(P.seed),
    P.c1.slice(1), P.c2.slice(1), P.c3.slice(1), P.c4.slice(1), P.bg.slice(1),
    P.aspect];
  SHARE_NUMS.forEach(function (k) { arr.push(Math.round(P[k] * 1000) / 1000); });
  arr.push(P.synthOn ? 1 : 0, P.modeB | 0, P.mixOp | 0, Math.round(P.blend * 1000) / 1000);
  arr.push(P.genomeOn ? 1 : 0);
  P.genes.forEach(function (g) { arr.push(Math.round(g * 1000) / 1000); });
  var b64 = btoa(JSON.stringify(arr))
    .replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
  return SHARE_PREFIX + b64;
}

function decodeDesign(code) {
  try {
    if (typeof code !== "string" || code.length > MAX_SHARE_CODE_LENGTH) return false;
    code = code.trim().replace(/^#/, "");
    if (!code || code.length > MAX_SHARE_CODE_LENGTH) return false;
    var b64 = "";
    if (code.indexOf(SHARE_PREFIX) === 0) {
      b64 = code.slice(SHARE_PREFIX.length);
    } else if (code.indexOf(LEGACY_SHARE_PREFIX) === 0) {
      b64 = code.slice(LEGACY_SHARE_PREFIX.length);
    } else {
      return false;
    }
    if (!SHARE_CODE_PATTERN.test(b64)) return false;
    b64 = b64.replace(/-/g, "+").replace(/_/g, "/");
    while (b64.length % 4) b64 += "=";
    var arr = JSON.parse(atob(b64));
    var baseLen = 9 + SHARE_NUMS.length;
    var isV1 = Array.isArray(arr) && arr[0] === 1 && arr.length === baseLen;
    var isV2 = Array.isArray(arr) && arr[0] === 2 && arr.length === baseLen + 4;
    var isV3 = Array.isArray(arr) && arr[0] === 3 && arr.length === baseLen + 17;
    if (!isV1 && !isV2 && !isV3) return false;

    var hexOk = function (h) { return /^[0-9a-fA-F]{6}$/.test(h); };
    if (![arr[3], arr[4], arr[5], arr[6], arr[7]].every(hexOk)) return false;

    P.mode = Math.min(Math.max(Math.round(arr[1]) || 0, 0), MODES.length - 1);
    P.seed = Math.min(Math.max(Math.round(arr[2]) || 0, 0), 9999);
    P.c1 = "#" + arr[3]; P.c2 = "#" + arr[4]; P.c3 = "#" + arr[5];
    P.c4 = "#" + arr[6]; P.bg = "#" + arr[7];
    if (ASPECTS[arr[8]]) P.aspect = arr[8];
    SHARE_NUMS.forEach(function (k, i) {
      P[k] = clampShareNumber(k, arr[9 + i]);
    });
    if (isV2 || isV3) {
      P.synthOn = !!arr[baseLen];
      P.modeB = Math.min(Math.max(Math.round(arr[baseLen + 1]) || 0, 0), MODES.length - 1);
      P.mixOp = Math.min(Math.max(Math.round(arr[baseLen + 2]) || 0, 0), 4);
      var bl = Number(arr[baseLen + 3]);
      P.blend = isFinite(bl) ? Math.min(Math.max(bl, 0), 1) : 0.6;
    } else {
      P.synthOn = false;
    }
    if (isV3) {
      P.genomeOn = !!arr[baseLen + 4];
      P.genes = [];
      for (var gi = 0; gi < 12; gi++) {
        var gv = Number(arr[baseLen + 5 + gi]);
        P.genes.push(clampGene(gi, gv));
      }
    } else {
      P.genomeOn = false;
    }
    activePreset = -1;
    setPresetActive(-1);
    resetAdaptiveLiveBudget();
    refreshAll();
    if (typeof scheduleFitCanvas === "function") scheduleFitCanvas();
    else if (typeof fitCanvas === "function") fitCanvas();
    return true;
  } catch (e) {
    return false;
  }
}

function copyText(text, okMsg) {
  if (navigator.clipboard && navigator.clipboard.writeText) {
    navigator.clipboard.writeText(text).then(
      function () { UI.toast(okMsg); },
      function () { UI.toast("Could not access clipboard"); });
  } else {
    UI.toast("Clipboard not available in this browser");
  }
}

/* ---------------- UI wiring ---------------- */

var refreshers = [];
var META_KEYS = { mode: true, seed: true, loop: true };
function reg(fn) { refreshers.push(fn); return fn; }
function requestShaderRender() {
  cachedWeakRenderParams = null;
  if (typeof Engine !== "undefined" && Engine.markDirty) Engine.markDirty();
}
function refreshAll() {
  refreshers.forEach(function (f) { f(); });
  updateMeta();
  requestShaderRender();
}

function get(k) { return function () { return P[k]; }; }
function set(k) { return function (v) {
  if (P[k] === v) return;
  P[k] = v;
  if (META_KEYS[k]) updateMeta();
  requestShaderRender();
}; }

function fmt2(v) { return (+v).toFixed(2); }
function fmt1(v) { return (+v).toFixed(1); }
function fmtInt(v) { return String(Math.round(v)); }
function fmtDeg(v) { return Math.round(v) + "\u00b0"; }
function fmtSec(v) { return (+v).toFixed(1) + "s"; }

function buildRail() {
  var rail = document.getElementById("rail");

  /* STYLE */
  var sStyle = UI.section(rail, "Style", function () {
    P.synthOn = false;
    P.mode = Math.floor(rnd() * MODES.length);
    refreshAll();
  });
  reg(UI.modeGrid(sStyle, MODES, function () { return (P.synthOn || P.genomeOn) ? -1 : P.mode; },
    function (v) { P.mode = v; P.synthOn = false; P.genomeOn = false; refreshAll(); }));

  /* genome: brand-new generated styles */
  var synthRow = UI.el("div", "share-row synth-row", sStyle);
  var btnSynth = UI.el("button", "mini-btn", synthRow);
  btnSynth.innerHTML = '<svg viewBox="0 0 16 16"><path d="M8 1.5 L9.8 6.2 L14.5 8 L9.8 9.8 L8 14.5 L6.2 9.8 L1.5 8 L6.2 6.2 Z" fill="none" stroke="currentColor" stroke-width="1.3"/></svg>New style';
  btnSynth.addEventListener("click", function () {
    generateGenomeStyle();
    newSeed();
    refreshAll();
    UI.toast("New style discovered: " + genomeName());
  });
  var btnSave = UI.el("button", "mini-btn", synthRow);
  btnSave.innerHTML = '<svg viewBox="0 0 16 16"><path d="M3 2 H11 L14 5 V14 H3 Z" fill="none" stroke="currentColor" stroke-width="1.5"/><rect x="5.5" y="9" width="5" height="4" fill="currentColor"/></svg>Save style';
  btnSave.addEventListener("click", saveCurrentStyle);

  var synthCtl = UI.el("div", "synth-ctl", sStyle);
  reg(UI.slider(synthCtl, { label: "Synth blend", min: 0, max: 1, step: 0.01, fmt: fmt2,
    get: get("blend"), set: function (v) { P.blend = v; updateMeta(); requestShaderRender(); } }));
  reg(function () { synthCtl.style.display = P.synthOn ? "" : "none"; });

  var savedWrap = UI.el("div", "saved-styles", sStyle);
  renderSavedStyles = function () {
    var list = loadSavedStyles();
    var fragment = document.createDocumentFragment();
    list.forEach(function (st, i) {
      var chip = UI.el("button", "saved-chip", fragment);
      var label = UI.el("span", null, chip);
      label.textContent = st.name;
      var del = UI.el("span", "saved-del", chip);
      del.innerHTML = "&times;";
      del.addEventListener("click", function (ev) {
        ev.stopPropagation();
        var next = loadSavedStyles();
        var targetIndex = next.findIndex(function (item) {
          return item.ts === st.ts && item.name === st.name && item.code === st.code;
        });
        next.splice(targetIndex > -1 ? targetIndex : i, 1);
        persistSavedStyles(next);
        renderSavedStyles();
      });
      chip.addEventListener("click", function () {
        if (decodeDesign(st.code)) UI.toast("Loaded " + st.name);
      });
    });
    savedWrap.replaceChildren(fragment);
  };
  renderSavedStyles();

  reg(UI.lockRow(sStyle, { label: "Keep style when randomizing", get: get("lockStyle"), set: set("lockStyle") }));

  /* COLOR */
  var sColor = UI.section(rail, "Color", function () { randomizePalette(); refreshAll(); });
  setPresetActive = UI.presetChips(sColor, PALETTES, function (i) {
    applyPalette(PALETTES[i], i);
    refreshAll();
  });
  setPresetActive(activePreset);
  reg(UI.colorSwatches(sColor,
    [{ key: "c1", label: "A" }, { key: "c2", label: "B" }, { key: "c3", label: "C" }, { key: "c4", label: "D" },
     { gap: true }, { key: "bg", label: "BG" }],
    function (k) { return P[k]; },
    function (k, v) { P[k] = v; activePreset = -1; setPresetActive(-1); requestShaderRender(); }));
  reg(UI.slider(sColor, { label: "Hue shift", min: -180, max: 180, step: 1, fmt: fmtDeg, get: get("hue"), set: set("hue") }));
  reg(UI.slider(sColor, { label: "Saturation", min: 0, max: 2, step: 0.01, fmt: fmt2, get: get("sat"), set: set("sat") }));
  reg(UI.slider(sColor, { label: "Exposure", min: 0.5, max: 1.6, step: 0.01, fmt: fmt2, get: get("exposure"), set: set("exposure") }));

  /* FORM */
  var sForm = UI.section(rail, "Form", function () { randomizeKeys(FORM_KEYS); newSeed(); refreshAll(); });
  reg(UI.seedRow(sForm, { get: get("seed"), set: function (v) { P.seed = v; updateMeta(); refreshAll(); }, onDice: function () { newSeed(); refreshAll(); } }));
  reg(UI.slider(sForm, { label: "Zoom", min: 0.5, max: 3, step: 0.01, fmt: fmt2, get: get("scale"), set: set("scale") }));
  reg(UI.slider(sForm, { label: "Detail", min: 1, max: 8, step: 0.1, fmt: fmt1, get: get("complex"), set: set("complex") }));
  reg(UI.slider(sForm, { label: "Warp", min: 0, max: 2.5, step: 0.01, fmt: fmt2, get: get("warp"), set: set("warp") }));
  reg(UI.slider(sForm, { label: "Turbulence", min: 0, max: 2, step: 0.01, fmt: fmt2, get: get("flow"), set: set("flow") }));
  reg(UI.slider(sForm, { label: "Stretch", min: -1, max: 1, step: 0.01, fmt: fmt2, get: get("stretch"), set: set("stretch") }));

  /* LIGHTING */
  var sLight = UI.section(rail, "Lighting", function () { randomizeKeys(LIGHT_KEYS); refreshAll(); });
  reg(UI.slider(sLight, { label: "Intensity", min: 0, max: 2.2, step: 0.01, fmt: fmt2, get: get("light"), set: set("light") }));
  reg(UI.slider(sLight, { label: "Gloss", min: 4, max: 120, step: 1, fmt: fmtInt, get: get("gloss"), set: set("gloss") }));
  reg(UI.slider(sLight, { label: "Angle", min: 0, max: 360, step: 1, fmt: fmtDeg, get: get("lightAngle"), set: set("lightAngle") }));
  reg(UI.slider(sLight, { label: "Iridescence", min: 0, max: 1, step: 0.01, fmt: fmt2, get: get("irid"), set: set("irid") }));
  reg(UI.slider(sLight, { label: "Glow", min: 0, max: 1, step: 0.01, fmt: fmt2, get: get("glow"), set: set("glow") }));
  reg(UI.slider(sLight, { label: "Contrast", min: 0.6, max: 1.6, step: 0.01, fmt: fmt2, get: get("contrast"), set: set("contrast") }));

  /* TEXTURE */
  var sTex = UI.section(rail, "Texture", function () { randomizeKeys(TEXTURE_KEYS); refreshAll(); });
  reg(UI.slider(sTex, { label: "Grain", min: 0, max: 0.4, step: 0.005, fmt: fmt2, get: get("grain"), set: set("grain") }));
  reg(UI.slider(sTex, { label: "Density", min: 14, max: 180, step: 1, fmt: fmtInt, get: get("cell"), set: set("cell") }));
  reg(UI.slider(sTex, { label: "Ridges", min: 8, max: 160, step: 1, fmt: fmtInt, get: get("lines"), set: set("lines") }));
  reg(UI.slider(sTex, { label: "Aberration", min: 0, max: 1, step: 0.01, fmt: fmt2, get: get("ca"), set: set("ca") }));
  reg(UI.slider(sTex, { label: "Vignette", min: 0, max: 1, step: 0.01, fmt: fmt2, get: get("vig"), set: set("vig") }));
  reg(UI.slider(sTex, { label: "Softness", min: 0.3, max: 1.6, step: 0.01, fmt: fmt2, get: get("soft"), set: set("soft") }));

  /* MOTION */
  var sMotion = UI.section(rail, "Motion", function () { randomizeKeys(MOTION_KEYS); refreshAll(); });
  reg(UI.slider(sMotion, { label: "Loop length", min: 2, max: 12, step: 0.5, fmt: fmtSec, get: get("loop"), set: set("loop") }));
  reg(UI.slider(sMotion, { label: "Travel", min: 0, max: 1.5, step: 0.01, fmt: fmt2, get: get("travel"), set: set("travel") }));

  /* SHARE */
  var sShare = UI.section(rail, "Share", null);
  var shareRow = UI.el("div", "share-row", sShare);
  var btnCopyCode = UI.el("button", "mini-btn", shareRow);
  btnCopyCode.textContent = "Copy design code";
  btnCopyCode.addEventListener("click", function () {
    copyText(encodeDesign(), "Design code copied, paste it anywhere");
  });
  var btnCopyLink = UI.el("button", "mini-btn", shareRow);
  btnCopyLink.textContent = "Copy link";
  btnCopyLink.addEventListener("click", function () {
    var url = location.origin + location.pathname + "#" + encodeDesign();
    copyText(url, "Share link copied");
  });
  var pasteInput = UI.el("input", "num-input mono share-input", sShare);
  pasteInput.type = "text";
  pasteInput.placeholder = "Paste a design code\u2026";
  pasteInput.spellcheck = false;
  pasteInput.maxLength = MAX_SHARE_CODE_LENGTH;
  function tryApplyPaste() {
    var v = pasteInput.value.trim();
    if (!v) return;
    if (decodeDesign(v)) {
      UI.toast("Design loaded");
      pasteInput.value = "";
      pasteInput.blur();
    } else {
      UI.toast("Invalid design code");
    }
  }
  pasteInput.addEventListener("paste", function () { setTimeout(tryApplyPaste, 0); });
  pasteInput.addEventListener("keydown", function (e) {
    if (e.key === "Enter") tryApplyPaste();
    e.stopPropagation();
  });

  /* EXPORT: buttons open a dialog with preview and settings */
  var sExp = UI.section(rail, "Export", null);
  var grid = UI.el("div", "export-grid", sExp);
  UI.exportButton(grid, "Image", "PNG",
    '<svg viewBox="0 0 16 16"><rect x="1.5" y="1.5" width="13" height="13" rx="2" fill="none" stroke="currentColor" stroke-width="1.5"/><circle cx="5.5" cy="5.5" r="1.5" fill="currentColor"/><path d="M2 12 L6 8 L9 11 L11.5 8.5 L14 11" fill="none" stroke="currentColor" stroke-width="1.5"/></svg>',
    function () { openExportDialog("png"); });
  UI.exportButton(grid, "Video", "WEBM",
    '<svg viewBox="0 0 16 16"><rect x="1.5" y="3.5" width="9" height="9" rx="2" fill="none" stroke="currentColor" stroke-width="1.5"/><path d="M10.5 7 L14.5 4.5 V11.5 L10.5 9" fill="currentColor"/></svg>',
    function () { openExportDialog("video"); });
  UI.exportButton(grid, "Looping GIF", "GIF",
    '<svg viewBox="0 0 16 16"><path d="M13.5 8 a5.5 5.5 0 1 1 -1.6 -3.9" fill="none" stroke="currentColor" stroke-width="1.5"/><path d="M13.8 1.6 V4.4 H11" fill="none" stroke="currentColor" stroke-width="1.5"/></svg>',
    function () { openExportDialog("gif"); });
  UI.exportButton(grid, "Gradient set", "ZIP",
    '<svg viewBox="0 0 16 16"><rect x="1.5" y="1.5" width="5.5" height="5.5" rx="1.5" fill="none" stroke="currentColor" stroke-width="1.4"/><rect x="9" y="1.5" width="5.5" height="5.5" rx="1.5" fill="none" stroke="currentColor" stroke-width="1.4"/><rect x="1.5" y="9" width="5.5" height="5.5" rx="1.5" fill="none" stroke="currentColor" stroke-width="1.4"/><rect x="9" y="9" width="5.5" height="5.5" rx="1.5" fill="none" stroke="currentColor" stroke-width="1.4"/></svg>',
    function () { openSetGeneratorDialog(); });
}

/* ---------------- deferred export runtime ---------------- */

var deferredRuntimeScripts = {};
var exportRuntimeReady = null;

function cleanupDeferredRuntimeScript(script, onLoad, onError) {
  script.removeEventListener("load", onLoad);
  script.removeEventListener("error", onError);
}

function loadDeferredRuntimeScript(src, globalName) {
  if (window[globalName]) return Promise.resolve(window[globalName]);
  if (deferredRuntimeScripts[src]) return deferredRuntimeScripts[src];

  deferredRuntimeScripts[src] = new Promise(function (resolve, reject) {
    var script = document.createElement("script");
    script.src = src;
    script.async = true;
    script.dataset.dxShaderDeferredRuntime = globalName;
    function onLoad() {
      cleanupDeferredRuntimeScript(script, onLoad, onError);
      if (window[globalName]) {
        resolve(window[globalName]);
        return;
      }
      script.remove();
      reject(new Error(globalName + " did not register"));
    }
    function onError() {
      cleanupDeferredRuntimeScript(script, onLoad, onError);
      script.remove();
      reject(new Error("Could not load " + src));
    }
    script.addEventListener("load", onLoad, { once: true });
    script.addEventListener("error", onError, { once: true });
    document.head.appendChild(script);
  }).catch(function (error) {
    delete deferredRuntimeScripts[src];
    throw error;
  });

  return deferredRuntimeScripts[src];
}

function ensureExportRuntime() {
  if (window.FX && window.Exporter && window.Modals) return Promise.resolve(window.Modals);
  if (!exportRuntimeReady) {
    exportRuntimeReady = loadDeferredRuntimeScript("/dx-shader/js/fx.js", "FX")
      .then(function () {
        return loadDeferredRuntimeScript("/dx-shader/js/exporter.js", "Exporter");
      })
      .then(function () {
        return loadDeferredRuntimeScript("/dx-shader/js/modals.js", "Modals");
      })
      .catch(function (error) {
        exportRuntimeReady = null;
        throw error;
      });
  }
  return exportRuntimeReady;
}

function withExportRuntime(action) {
  if (!(window.Exporter && window.Modals)) UI.toast("Preparing export tools");
  return ensureExportRuntime()
    .then(action)
    .catch(function () {
      UI.toast("Export tools unavailable");
    });
}

function openExportDialog(kind) {
  withExportRuntime(function () { window.Modals.openExport(kind); });
}

function openSetGeneratorDialog() {
  withExportRuntime(function () { window.Modals.openSetGenerator(); });
}

function exportPngShortcut() {
  withExportRuntime(function () { window.Exporter.exportPNG(P, ASPECTS[P.aspect]); });
}

/* ---------------- stage / meta ---------------- */

var pendingFitCanvas = false;
var lastCanvasLayout = null;
var MAX_LIVE_PIXELS = 1400000;
var WEAK_LIVE_PIXELS = 850000;
var MIN_LIVE_PIXELS = 420000;
var STRONG_LIVE_FPS = 60;
var WEAK_LIVE_FPS = 30;
var ADAPTIVE_BUDGET_COOLDOWN_MS = 2500;
var ADAPTIVE_RESUME_HOLD_MS = 900;
var FPS_META_INTERVAL_MS = 250;
var adaptiveLivePixels = null;
var lastAdaptiveBudgetChangeAt = 0;
var lowFpsSamples = 0;
var stableFpsSamples = 0;
var cachedWeakRenderProfile = null;
var cachedWeakRenderParams = null;
var canvasResizeHandlersInstalled = false;
var canvasResizeObserver = null;
var lastMetaText = {};
var lastFpsMetaAt = 0;
var adaptiveBudgetHoldUntil = 0;

function weakDeviceProfile() {
  if (cachedWeakRenderProfile === null) cachedWeakRenderProfile = shouldUseReducedRenderProfile();
  return cachedWeakRenderProfile;
}

function baseLivePixelBudget() {
  return weakDeviceProfile() ? WEAK_LIVE_PIXELS : MAX_LIVE_PIXELS;
}

function livePixelBudget() {
  return adaptiveLivePixels || baseLivePixelBudget();
}

function liveTargetFps() {
  return weakDeviceProfile() ? WEAK_LIVE_FPS : STRONG_LIVE_FPS;
}

function resetAdaptiveLiveBudget() {
  adaptiveLivePixels = null;
  lastAdaptiveBudgetChangeAt = 0;
  lowFpsSamples = 0;
  stableFpsSamples = 0;
  lastCanvasLayout = null;
}

function canChangeAdaptiveBudget(now) {
  return lastAdaptiveBudgetChangeAt === 0 || now - lastAdaptiveBudgetChangeAt >= ADAPTIVE_BUDGET_COOLDOWN_MS;
}

function holdAdaptiveBudget(ms) {
  adaptiveBudgetHoldUntil = Math.max(adaptiveBudgetHoldUntil, Date.now() + ms);
}

function resetAdaptiveFpsSamples() {
  lowFpsSamples = 0;
  stableFpsSamples = 0;
}

function stableFpsThreshold() {
  return Math.round(liveTargetFps() * 0.9);
}

function lowFpsThreshold() {
  return Math.round(liveTargetFps() * 0.8);
}

function observeLiveFps(fps) {
  if (fps <= 0) {
    resetAdaptiveFpsSamples();
    holdAdaptiveBudget(ADAPTIVE_RESUME_HOLD_MS);
    return;
  }
  if (!Engine.isPlaying || !Engine.isPlaying()) return;
  var now = Date.now();
  if (now < adaptiveBudgetHoldUntil) {
    resetAdaptiveFpsSamples();
    return;
  }

  if (fps > 0 && fps < lowFpsThreshold()) {
    lowFpsSamples++;
    stableFpsSamples = 0;
  } else if (fps >= stableFpsThreshold()) {
    stableFpsSamples++;
    lowFpsSamples = 0;
  } else {
    lowFpsSamples = 0;
    stableFpsSamples = 0;
    return;
  }

  if (lowFpsSamples >= 3 && livePixelBudget() > MIN_LIVE_PIXELS) {
    if (!canChangeAdaptiveBudget(now)) return;
    adaptiveLivePixels = Math.max(MIN_LIVE_PIXELS, Math.round(livePixelBudget() * 0.75));
    lastAdaptiveBudgetChangeAt = now;
    lowFpsSamples = 0;
    stableFpsSamples = 0;
    lastCanvasLayout = null;
    scheduleFitCanvas();
    return;
  }

  if (stableFpsSamples >= 8 && adaptiveLivePixels && adaptiveLivePixels < baseLivePixelBudget()) {
    if (!canChangeAdaptiveBudget(now)) return;
    adaptiveLivePixels = Math.min(baseLivePixelBudget(), Math.round(adaptiveLivePixels * 1.15));
    lastAdaptiveBudgetChangeAt = now;
    stableFpsSamples = 0;
    lastCanvasLayout = null;
    scheduleFitCanvas();
  }
}

function applyWeakRenderProfile(params) {
  if (!weakDeviceProfile()) return params;
  if (cachedWeakRenderParams) return cachedWeakRenderParams;
  var next = copyRenderParams(params);
  next.complex = Math.min(next.complex, 4.5);
  next.ca = 0;
  next.glow = Math.min(next.glow, 0.35);
  cachedWeakRenderParams = next;
  return next;
}

function copyRenderParams(params) {
  var next = {};
  for (var key in params) {
    if (Object.prototype.hasOwnProperty.call(params, key)) next[key] = params[key];
  }
  return next;
}

function liveRenderParams() {
  return applyWeakRenderProfile(P);
}

function setTextIfChanged(id, text) {
  if (lastMetaText[id] === text) return;
  lastMetaText[id] = text;
  var element = document.getElementById(id);
  if (element) element.textContent = text;
}

function updateMeta() {
  var s = Engine.size();
  setTextIfChanged("meta-mode", styleName());
  setTextIfChanged("meta-seed", "seed " + String(Math.round(P.seed)).padStart(4, "0"));
  setTextIfChanged("meta-loop", P.loop.toFixed(1) + "s loop");
  setTextIfChanged("meta-res", s[0] + "\u00d7" + s[1]);
}

function cssPixel(value) {
  var number = parseFloat(value);
  return isFinite(number) ? number : 0;
}

function fitCanvas() {
  var frame = document.getElementById("canvas-frame");
  if (!frame) return;
  var style = window.getComputedStyle ? window.getComputedStyle(frame) : null;
  var paddingX = style ? cssPixel(style.paddingLeft) + cssPixel(style.paddingRight) : 0;
  var paddingY = style ? cssPixel(style.paddingTop) + cssPixel(style.paddingBottom) : 0;
  var availW = frame.clientWidth - paddingX;
  var availH = frame.clientHeight - paddingY;
  if (availW <= 0 || availH <= 0) return;
  var ar = ASPECTS[P.aspect];
  var w = availW, h = w / ar;
  if (h > availH) { h = availH; w = h * ar; }
  var cssW = Math.round(w);
  var cssH = Math.round(h);
  var pixelBudgetScale = Math.min(1, Math.sqrt(livePixelBudget() / (cssW * cssH)));
  var dpr = Math.min(window.devicePixelRatio || 1, guardedRenderScale(), pixelBudgetScale);
  var bufferW = Math.max(2, 2 * Math.round(w * dpr / 2));
  var bufferH = Math.max(2, 2 * Math.round(h * dpr / 2));
  var nextLayout = [cssW, cssH, bufferW, bufferH, P.aspect].join(":");
  if (lastCanvasLayout === nextLayout) return;
  lastCanvasLayout = nextLayout;
  var canvas = Engine.canvas();
  canvas.style.width = cssW + "px";
  canvas.style.height = cssH + "px";
  Engine.setSize(bufferW, bufferH);
  updateMeta();
}

function scheduleFitCanvas() {
  if (pendingFitCanvas) return;
  pendingFitCanvas = true;
  requestAnimationFrame(function () {
    pendingFitCanvas = false;
    fitCanvas();
  });
}

function installCanvasResizeHandlers() {
  if (canvasResizeHandlersInstalled) return;
  var frame = document.getElementById("canvas-frame");
  if (!frame) return;
  canvasResizeHandlersInstalled = true;
  if (typeof ResizeObserver === "function") {
    canvasResizeObserver = new ResizeObserver(scheduleFitCanvas);
    canvasResizeObserver.observe(frame);
  }
  window.addEventListener("resize", scheduleFitCanvas, { passive: true });
  if (window.visualViewport) {
    window.visualViewport.addEventListener("resize", scheduleFitCanvas, { passive: true });
  }
  window.addEventListener("orientationchange", scheduleFitCanvas, { passive: true });
}

function setPlayingUI(v) {
  Engine.setPlaying(v);
  document.getElementById("icon-pause").style.display = v ? "" : "none";
  document.getElementById("icon-play").style.display = v ? "none" : "";
}

/* ---------------- boot ---------------- */

var shaderBootStarted = false;

function getBootLoader() {
  return document.getElementById("shader-boot-loader");
}

function setBootLoaderState(state, detail) {
  var loader = getBootLoader();
  var status = document.getElementById("shader-boot-status");
  var action = document.getElementById("shader-boot-start");
  if (loader) loader.setAttribute("data-loader-state", state);
  if (status && detail) status.textContent = detail;
  if (action) action.hidden = state !== "manual";
}

function prepareRuntimeShell() {
  document.body.removeAttribute("data-shader-ready");
  ["overlay", "toast"].forEach(function (id) {
    var element = document.getElementById(id);
    if (!element) return;
    element.hidden = true;
    element.classList.add("is-hidden");
  });
  setBootLoaderState("preparing", "Checking GPU readiness");
}

function isEdgeBrowser() {
  return /\bEdg\//.test(window.navigator.userAgent || "");
}

function mediaMatches(query) {
  try {
    return Boolean(window.matchMedia && window.matchMedia(query).matches);
  } catch (e) {
    return false;
  }
}

function shaderSafetySignals() {
  var nav = window.navigator || {};
  var connection = nav.connection || nav.mozConnection || nav.webkitConnection;
  return {
    edgeBrowser: isEdgeBrowser(),
    lowMemory: typeof nav.deviceMemory === "number" && nav.deviceMemory <= 4,
    lowCores: typeof nav.hardwareConcurrency === "number" && nav.hardwareConcurrency <= 4,
    saveData: Boolean(connection && connection.saveData),
    slowConnection: Boolean(connection && /^(slow-2g|2g)$/.test(connection.effectiveType || "")),
    reducedData: mediaMatches("(prefers-reduced-data: reduce)"),
    reducedMotion: mediaMatches("(prefers-reduced-motion: reduce)")
  };
}

function shouldUseReducedRenderProfileFromSignals(signals) {
  return signals.lowMemory || signals.lowCores || signals.saveData ||
    signals.slowConnection || signals.reducedData;
}

function shouldUseReducedRenderProfile() {
  return shouldUseReducedRenderProfileFromSignals(shaderSafetySignals());
}

function shouldUseManualShaderStart() {
  var signals = shaderSafetySignals();
  return signals.edgeBrowser || signals.reducedMotion || shouldUseReducedRenderProfileFromSignals(signals);
}

function guardedRenderScale() {
  return weakDeviceProfile() ? 1 : 1.25;
}

function shouldSkipIntroAnimation() {
  return shouldUseManualShaderStart();
}

function startWhenVisible(start, delayMs) {
  var waitForVisible = function () {
    setBootLoaderState("loading", "Waiting for visible tab");
    document.addEventListener("visibilitychange", function onVisible() {
      if (document.hidden) {
        waitForVisible();
        return;
      }
      run();
    }, { once: true, passive: true });
  };
  var run = function () {
    setTimeout(function () {
      if (document.hidden) {
        waitForVisible();
        return;
      }
      start();
    }, delayMs);
  };
  if (!document.hidden) {
    run();
    return;
  }
  waitForVisible();
}

function scheduleShaderBoot(start) {
  var action = document.getElementById("shader-boot-start");
  if (shouldUseManualShaderStart()) {
    var loader = getBootLoader();
    if (loader) loader.setAttribute("data-loader-mode", "manual");
    setBootLoaderState("manual", "Safe start waiting");
    if (action) {
      action.addEventListener("click", function () {
        action.hidden = true;
        setBootLoaderState("loading", "Starting shader engine");
        startWhenVisible(start, 120);
      }, { once: true });
    }
    return;
  }

  setBootLoaderState("loading", "Warming shader engine");
  var boot = function () { startWhenVisible(start, 180); };
  if ("requestIdleCallback" in window) {
    window.requestIdleCallback(boot, { timeout: 1600 });
  } else {
    setTimeout(boot, 700);
  }
}

function applyInitialHash() {
  var rawHash = location.hash || "";
  if (rawHash.length <= 1 || rawHash.length > MAX_SHARE_CODE_LENGTH + 1) return;
  var hash = safeDecodeHash(rawHash.slice(1));
  if ((hash.indexOf(SHARE_PREFIX) === 0 || hash.indexOf(LEGACY_SHARE_PREFIX) === 0) && decodeDesign(hash)) {
    UI.toast("Shared design loaded");
  }
}

function bootShaderStudio() {
  if (shaderBootStarted) return;
  shaderBootStarted = true;

  var canvas = document.getElementById("view");
  ["overlay", "toast"].forEach(function (id) {
    var element = document.getElementById(id);
    if (!element) return;
    element.hidden = true;
    element.classList.add("is-hidden");
  });

  if (!canvas) {
    var frame = document.getElementById("canvas-frame");
    if (frame) {
      canvas = document.createElement("canvas");
      canvas.id = "view";
      frame.appendChild(canvas);
    }
  }

  try {
    Engine.init(canvas, function () { return liveRenderParams(); });
    Engine.setTargetFps(liveTargetFps());
    document.body.setAttribute("data-shader-ready", "true");
  } catch (e) {
    setBootLoaderState("error", "WebGL2 unavailable");
    var errorFrame = document.querySelector(".canvas-frame");
    if (errorFrame) errorFrame.innerHTML =
      '<div style="color:#9b9ba4;font-size:13px;max-width:380px;text-align:center;line-height:1.6">' +
      "WebGL2 is required. Please use a recent Chrome, Edge or Firefox.<br><span style=\"color:#5e5e68;font-size:11px\">" +
      String(e.message).split("\n")[0] + "</span></div>";
    return;
  }
  Engine.onContextChange(function (state, error) {
    if (state === "lost") {
      document.body.removeAttribute("data-shader-ready");
      setBootLoaderState("manual", "GPU context paused");
      UI.toast("GPU context paused");
    } else if (state === "restored") {
      document.body.setAttribute("data-shader-ready", "true");
      setBootLoaderState("ready", "Shader ready");
      lastCanvasLayout = null;
      scheduleFitCanvas();
      UI.toast("GPU context restored");
    } else {
      setBootLoaderState("error", "GPU recovery failed");
      UI.toast("GPU recovery failed" + (error && error.message ? ": " + error.message : ""));
    }
  });

  buildRail();

  /* aspect segmented control */
  var segRefresh = UI.segmented(
    document.getElementById("dx-shader-aspect-control"),
    Object.keys(ASPECTS).map(function (k) { return [k, k]; }),
    get("aspect"),
    function (v) { P.aspect = v; resetAdaptiveLiveBudget(); fitCanvas(); }
  );
  reg(segRefresh);

  applyInitialHash();
  fitCanvas();
  installCanvasResizeHandlers();

  /* intro: every element pops in big and settles into place,
     one after another. transform/opacity only, fully compositor-driven. */
  (function intro() {
    if (shouldSkipIntroAnimation()) return;
    holdAdaptiveBudget(3200);
    var items = [];
    document.querySelectorAll(".brand, .segmented button, .topbar-actions > *").forEach(function (n) { items.push(n); });
    items.push(document.querySelector(".canvas-frame"));
    items.push(document.querySelector(".stage-meta"));
    document.querySelectorAll(".rail-section").forEach(function (sec) {
      Array.prototype.forEach.call(sec.children, function (child) {
        if (child.classList.contains("mode-grid") ||
            child.classList.contains("preset-row") ||
            child.classList.contains("export-grid")) {
          Array.prototype.forEach.call(child.children, function (n) { items.push(n); });
        } else {
          items.push(child);
        }
      });
    });

    var d = 0;
    items.forEach(function (n, i) {
      if (!n) return;
      /* fast cadence inside the rail, slower for the hero pieces */
      d += i < 12 ? 50 : 16;
      n.classList.add("stagger-item");
      n.style.setProperty("--intro-d", Math.min(d, 2100) + "ms");
    });
    document.body.classList.add("intro");
    setTimeout(function () {
      document.body.classList.remove("intro");
      items.forEach(function (n) {
        if (!n) return;
        n.classList.remove("stagger-item");
        n.style.removeProperty("--intro-d");
        n.style.willChange = "auto";
      });
    }, 3000);
  })();

  Engine.onFps(function (fps) {
    observeLiveFps(fps);
    var now = Date.now();
    if (fps === 0 || now - lastFpsMetaAt >= FPS_META_INTERVAL_MS) {
      lastFpsMetaAt = now;
      setTextIfChanged("meta-fps", fps + " fps");
    }
  });

  document.getElementById("btn-random").addEventListener("click", randomizeAll);
  document.getElementById("btn-play").addEventListener("click", function () {
    setPlayingUI(!Engine.isPlaying());
  });
  document.getElementById("btn-export-png").addEventListener("click", function () { openExportDialog("png"); });
  document.getElementById("btn-export-video").addEventListener("click", function () { openExportDialog("video"); });
  document.getElementById("btn-export-gif").addEventListener("click", function () { openExportDialog("gif"); });
  document.getElementById("btn-set").addEventListener("click", openSetGeneratorDialog);

  document.addEventListener("keydown", function (e) {
    if (e.repeat) return;
    var tag = (e.target.tagName || "").toLowerCase();
    if (tag === "input" || tag === "select" || tag === "textarea") return;
    if (e.code === "Space") { e.preventDefault(); setPlayingUI(!Engine.isPlaying()); }
    else if (e.key === "r" || e.key === "R") { randomizeAll(); }
    else if (e.key === "s" || e.key === "S") { exportPngShortcut(); }
  });

  updateMeta();
  setBootLoaderState("ready", "Shader ready");
}

function bootWhenDomReady() {
  prepareRuntimeShell();
  scheduleShaderBoot(bootShaderStudio);
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", bootWhenDomReady, { once: true });
} else {
  bootWhenDomReady();
}
