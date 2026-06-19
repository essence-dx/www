# DX Devtools Production Controls Pass Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the injected DX Devtools controls materially production-grade without adding native form controls, external browser packages, or CommonJS test drift.

**Architecture:** Keep the runtime source-owned in `dx-www/src/cli/devtools/assets.rs`, with small helpers for focus restoration, keyboard movement, and custom ARIA control semantics. Keep verification in the existing TypeScript guard file so `.cjs` / `.mjs` regressions and fake controls fail fast.

**Tech Stack:** Rust-served JS/CSS string assets, TypeScript Node test guards, no React DOM, no Vite/Webpack, no external UI runtime.

---

### Task 1: Custom Control Keyboard Contract

**Files:**
- Modify: `dx-www/src/cli/devtools/assets.rs`
- Modify: `benchmarks/dx-devtools-framework-integration.test.ts`

- [ ] **Step 1: Add a guard for complete keyboard controls**

Extend the existing Devtools custom controls test to require:

```ts
assertSourceMatches(assets, /PageUp[\s\S]*PageDown/, "sliders should support page-sized keyboard steps");
assertSourceMatches(positionPadControl, /onKeydown:\s*\(event\)\s*=>\s*onPositionKeydown/, "position pads should support keyboard movement");
assertSourceMatches(meshStageControl, /onKeydown:\s*\(event\)\s*=>\s*onPositionKeydown/, "mesh layer handles should support keyboard movement");
assertSourceMatches(assets, /aria-orientation/, "custom sliders should expose orientation");
assertSourceMatches(assets, /aria-valuetext/, "custom controls should expose readable values with units");
```

- [ ] **Step 2: Implement minimal runtime support**

In `assets.rs`, update `onSliderKeydown`, `sliderControl`, `angleDialControl`, `positionPadControl`, and `meshStageControl` so keyboard arrows, Home, End, PageUp, and PageDown adjust real values. Add readable `aria-valuetext`, `aria-orientation`, and coordinate text for position controls.

- [ ] **Step 3: Verify**

Run:

```powershell
node --test benchmarks\dx-devtools-framework-integration.test.ts
```

Expected: pass.

### Task 2: Custom Select, Tabs, Menu, and Focus Restoration

**Files:**
- Modify: `dx-www/src/cli/devtools/assets.rs`
- Modify: `benchmarks/dx-devtools-framework-integration.test.ts`

- [ ] **Step 1: Add a guard for focus and ARIA**

Extend the benchmark to require:

```ts
assertSourceMatches(assets, /captureFocusToken/, "renderAll should capture focus before replacing panels");
assertSourceMatches(assets, /restoreFocusToken/, "renderAll should restore focus after replacing panels");
assertSourceMatches(assets, /onSelectKeydown/, "custom select controls should handle keyboard navigation");
assertSourceMatches(assets, /onTabKeydown/, "custom tablists should handle arrow navigation");
assertSourceMatches(assets, /onMenuKeydown/, "puck menu should handle menu keyboard navigation");
```

- [ ] **Step 2: Implement minimal runtime support**

Add focus capture/restore around `renderAll()`. Add keyboard handlers for custom select popovers, picker tabs, inspector tabs, and puck menu. Escape should close popovers and menu before inspect mode.

- [ ] **Step 3: Verify**

Run the focused Node guard.

### Task 3: TypeScript-Only Tooling Guard

**Files:**
- Modify: `benchmarks/dx-devtools-framework-integration.test.ts`
- Verify: `tools/devtools/generate-mdn-css-data.ts`

- [ ] **Step 1: Preserve TypeScript-only contract**

Keep assertions that:

```ts
assert.ok(fs.existsSync(path.join(repoRoot, "tools/devtools/generate-mdn-css-data.ts")));
assert.ok(!fs.existsSync(path.join(repoRoot, "tools/devtools/generate-mdn-css-data.mjs")));
assert.deepEqual(devtoolsLegacyTests, [], "Devtools benchmark guards should stay TypeScript-only; no .cjs/.mjs drift");
```

- [ ] **Step 2: Verify syntax without rewriting generated data**

Run:

```powershell
node --check tools\devtools\generate-mdn-css-data.ts
```

Expected: pass.

### Task 4: Final Bounded Verification

**Files:**
- Verify: `dx-www/src/cli/devtools/assets.rs`
- Verify: `dx-www/src/cli/devtools/style_ops.rs`
- Verify: `benchmarks/dx-devtools-framework-integration.test.ts`

- [ ] **Step 1: Run focused checks**

```powershell
node --test benchmarks\dx-devtools-framework-integration.test.ts
node --check tools\devtools\generate-mdn-css-data.ts
cargo fmt --check
cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www
```

- [ ] **Step 2: Install only if checks pass**

```powershell
cargo build -j 6 -p dx-www --no-default-features --features cli --bin dx-www
```

Then copy `target\debug\dx-www.exe` to `G:\Dx\bin\dx.exe` and `G:\Dx\bin\dx-www.exe`, restart `examples/template` on port `3000`, and probe the live Devtools endpoints.
