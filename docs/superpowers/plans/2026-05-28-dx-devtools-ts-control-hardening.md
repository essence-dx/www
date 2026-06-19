# DX Devtools TS Control Hardening Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the framework-level DX Devtools lane more production-ready by removing Devtools-specific CJS/MJS tooling drift and hardening the injected custom controls.

**Architecture:** Keep the runtime source-owned inside `dx-www/src/cli/devtools/assets.rs`, with no npm, native input/select/textarea/range controls, React DOM, Next runtime, Vite, webpack, or external UI packages. Keep CSS metadata generation in a direct Node-run TypeScript file so the Devtools lane uses `.ts` for its guard/tooling.

**Tech Stack:** Rust-served dev-only assets, source-owned browser JavaScript/CSS, Node 22 direct `node --test` TypeScript guards, generated MDN CSS metadata.

---

### Task 1: TypeScript-Only Devtools Tooling

**Files:**
- Keep: `tools/devtools/generate-mdn-css-data.ts`
- Modify: `benchmarks/dx-devtools-framework-integration.test.ts`
- Modify: `docs/superpowers/plans/2026-05-28-dx-devtools-mdn-css-picker-polish.md`

- [ ] **Step 1: Verify the MDN generator is TypeScript**

The generator should remain source-owned and reviewable at:

```powershell
Test-Path G:\Dx\www\tools\devtools\generate-mdn-css-data.ts
Test-Path G:\Dx\www\tools\devtools\generate-mdn-css-data.mjs
```

Expected: `.ts` exists and `.mjs` does not.

- [ ] **Step 2: Update references to the generator**

Search:

```powershell
rg -n "generate-mdn-css-data\.(mjs|ts)" G:\Dx\www
```

Expected: Devtools references point at `.ts`; no Devtools plan or guard points at `.mjs`.

- [ ] **Step 3: Guard the TypeScript contract**

Add to `benchmarks/dx-devtools-framework-integration.test.ts`:

```ts
assert.ok(fs.existsSync(path.join(repoRoot, "tools/devtools/generate-mdn-css-data.ts")));
assert.ok(!fs.existsSync(path.join(repoRoot, "tools/devtools/generate-mdn-css-data.mjs")));
assertSourceDoesNotMatch(read("tools/devtools/generate-mdn-css-data.ts"), /require\(|module\.exports/);
```

- [ ] **Step 4: Verify direct Node execution still works**

Run:

```powershell
node --test benchmarks\dx-devtools-framework-integration.test.ts
node tools\devtools\generate-mdn-css-data.ts target\mdn-data
```

Expected: guard passes and generator refreshes `dx-www/src/cli/devtools/css_data.generated.json`.

### Task 2: Custom Control Behavior Hardening

**Files:**
- Modify: `dx-www/src/cli/devtools/assets.rs`
- Modify: `benchmarks/dx-devtools-framework-integration.test.ts`

- [ ] **Step 1: Add shared control state helpers**

Add helpers to close peer popovers and cap runtime issue handling:

```js
function closePopovers(except = "") {
  for (const key of ["propertyPopoverOpen", "valuePopoverOpen", "colorPopoverOpen", "radialShapePopoverOpen"]) {
    if (key !== except) STATE[key] = false;
  }
}
```

- [ ] **Step 2: Add keyboard support to custom sliders and dials**

Add `adjustColorPicker` and `onSliderKeydown` so custom controls support Arrow/Home/End without native range inputs:

```js
function adjustColorPicker(key, delta, min, max) {
  setColorPicker(key, clampNumber(STATE.colorPicker[key], min, max) + delta, min, max);
  renderAll();
}
```

- [ ] **Step 3: Add color-stop target controls**

Add `activeColorStop` to state and expose custom stop target buttons for `stopA`, `stopB`, `stopC`. Quick solid swatches inside gradient modes should update the active stop, while the existing final “Use linear/radial/mesh gradient” buttons still write `background-image`.

- [ ] **Step 4: Improve ARIA for tabs/menu/combobox-like controls**

Use `role="tab"`, `aria-selected`, `aria-controls`, `aria-haspopup`, `aria-expanded`, and a stable `id` on the puck menu.

- [ ] **Step 5: Guard the control behavior**

Extend `benchmarks/dx-devtools-framework-integration.test.ts` to assert:

```ts
assertSourceMatches(assets, /function\s+adjustColorPicker/);
assertSourceMatches(assets, /onKeydown:\s*\(event\)\s*=>\s*onSliderKeydown/);
assertSourceMatches(assets, /activeColorStop/);
assertSourceMatches(assets, /role:\s*"tab"[\s\S]*aria-selected/);
assertSourceMatches(assets, /aria-haspopup:\s*"menu"|aria-haspopup:\s*"listbox"/);
```

- [ ] **Step 6: Verify focused checks**

Run:

```powershell
node --test benchmarks\dx-devtools-framework-integration.test.ts
node -e "const fs=require('fs'); const s=fs.readFileSync('dx-www/src/cli/devtools/assets.rs','utf8'); const m=s.match(/const RUNTIME_JS: &str = r###\"([\s\S]*?)\"###;/); new Function(m[1]);"
cargo fmt --check
```

Expected: all pass.
