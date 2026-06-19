# DX Devtools MDN CSS Picker Polish Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the remaining hardcoded Devtools CSS editing surface with MDN-sourced CSS metadata, a real custom color/gradient picker, smaller inspect chrome, and cleaner production-grade UI.

**Architecture:** Keep Devtools framework-owned and dev-only. Clone/read canonical MDN `mdn/data`, generate a checked-in DX metadata artifact under `dx-www/src/cli/devtools`, serve it from `/_dx/devtools/css-data`, and make the injected runtime fetch it for property/value/selectors support while keeping small built-in fallbacks.

**Tech Stack:** Rust `dx-www` dev server, source-owned vanilla JS/CSS injected runtime, MDN `mdn/data` JSON, Node source-generation/test scripts only.

---

### Task 1: MDN CSS Metadata

**Files:**
- Create: `tools/devtools/generate-mdn-css-data.ts`
- Create: `dx-www/src/cli/devtools/css_data.rs`
- Create: `dx-www/src/cli/devtools/css_data.generated.json`
- Modify: `dx-www/src/cli/devtools/mod.rs`
- Modify: `dx-www/src/cli/devtools/assets.rs`

- [ ] Clone or refresh `https://github.com/mdn/data.git` into `target/mdn-data` with `--depth 1`.
- [ ] Generate metadata from `css/properties.json`, `css/selectors.json`, `css/at-rules.json`, `css/syntaxes.json`, `css/types.json`, `css/units.json`, and `package.json`.
- [ ] Include counts and source revision in the generated JSON.
- [ ] Serve metadata as `GET /_dx/devtools/css-data` through the existing devtools asset path.

### Task 2: Runtime CSS Catalog

**Files:**
- Modify: `dx-www/src/cli/devtools/assets.rs`
- Modify: `benchmarks/dx-devtools-framework-integration.test.ts`

- [ ] Add `ENDPOINTS.cssData`.
- [ ] Fetch the catalog during `refreshProtocol()`.
- [ ] Use catalog properties for the property picker and MDN syntax/value hints.
- [ ] Keep fallbacks so Devtools still opens if metadata cannot load.

### Task 3: Real Color And Gradient Picker

**Files:**
- Modify: `dx-www/src/cli/devtools/assets.rs`

- [ ] Add custom RGB/HSL/alpha channel controls using contenteditable numeric shells.
- [ ] Generate linear-gradient values from angle plus stops.
- [ ] Generate radial-gradient values from shape, position, and stops.
- [ ] Generate mesh-gradient style values as layered radial gradients.
- [ ] Keep quick preset circles as shortcuts only.

### Task 4: Devtools Chrome Polish

**Files:**
- Modify: `dx-www/src/cli/devtools/assets.rs`

- [ ] Make the inspect crosshair/claw tiny and non-obtrusive.
- [ ] Add smooth bounce animation to the draggable DX puck.
- [ ] Remove visible Inspector/Preview/Box edge badges from the app viewport.
- [ ] Remove decorative box-model cards that show static layer labels; keep real measured values only.

### Task 5: Focused Verification

**Files:**
- Modify: `benchmarks/dx-devtools-framework-integration.test.ts`

- [ ] Assert the MDN data endpoint exists and is dev-only.
- [ ] Assert generated metadata includes properties, selectors, syntaxes, units, and source revision.
- [ ] Assert the runtime has no native browser input/select/textarea controls.
- [ ] Assert quick presets are present but catalog-backed property picker is not hardcoded-only.

### Task 6: Install And Restart

**Files:**
- No source edits.

- [ ] Run `node --test benchmarks\dx-devtools-framework-integration.test.ts`.
- [ ] Run `cargo fmt --check`.
- [ ] Run `cargo build -j 6 -p dx-www --no-default-features --features cli --bin dx-www`.
- [ ] Copy `target\debug\dx-www.exe` to `G:\Dx\bin\dx.exe` and `G:\Dx\bin\dx-www.exe`.
- [ ] Restart `examples/template` on `127.0.0.1:3000` with `--devtools`.
