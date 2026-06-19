# DX Devtools Source-Owned Assets Split Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the injected DX Devtools runtime more production-maintainable by moving the large JS/CSS blobs out of `assets.rs` into first-class source-owned assets while keeping TypeScript-only guards and existing controls.

**Architecture:** Rust remains the server and injection owner. `assets.rs` serves `runtime.js` and `devtools.css` through `include_str!`, while `dx-www/src/cli/devtools/assets/runtime.js` and `dx-www/src/cli/devtools/assets/devtools.css` become directly readable and syntax-checkable source files. The focused TypeScript benchmark must read those assets directly and continue rejecting native controls and `.cjs` / `.mjs` drift.

**Tech Stack:** Rust `include_str!`, source-owned browser JS/CSS, Node `--test` TypeScript ESM guard, no npm, no bundler, no React DOM.

---

### Task 1: Extract Runtime JS

**Files:**
- Create: `dx-www/src/cli/devtools/assets/runtime.js`
- Modify: `dx-www/src/cli/devtools/assets.rs`

- [ ] **Step 1: Extract the exact `RUNTIME_JS` raw string**

Move the content between `const RUNTIME_JS: &str = r###"` and `"###;` into `dx-www/src/cli/devtools/assets/runtime.js`.

- [ ] **Step 2: Include the runtime from Rust**

Replace the raw string with:

```rust
const RUNTIME_JS: &str = include_str!("assets/runtime.js");
```

- [ ] **Step 3: Verify syntax directly**

Run:

```powershell
node --check dx-www\src\cli\devtools\assets\runtime.js
```

Expected: pass.

### Task 2: Extract Devtools CSS

**Files:**
- Create: `dx-www/src/cli/devtools/assets/devtools.css`
- Modify: `dx-www/src/cli/devtools/assets.rs`

- [ ] **Step 1: Extract the exact `DEVTOOLS_CSS` raw string**

Move the content between `const DEVTOOLS_CSS: &str = r###"` and `"###;` into `dx-www/src/cli/devtools/assets/devtools.css`.

- [ ] **Step 2: Include the CSS from Rust**

Replace the raw string with:

```rust
const DEVTOOLS_CSS: &str = include_str!("assets/devtools.css");
```

- [ ] **Step 3: Keep Vercel-dark token styling**

The CSS must keep `JetBrains Mono`, Vercel/Geist token fallbacks, no random red dot, 44px hit targets, and no native form selectors.

### Task 3: Update TypeScript Guards

**Files:**
- Modify: `benchmarks/dx-devtools-framework-integration.test.ts`

- [ ] **Step 1: Read split source assets**

Add helpers:

```ts
const devtoolsRuntime = read("dx-www/src/cli/devtools/assets/runtime.js");
const devtoolsCss = read("dx-www/src/cli/devtools/assets/devtools.css");
```

- [ ] **Step 2: Point runtime assertions at `runtime.js`**

Assertions for JS functions such as `sliderControl`, `onSelectKeydown`, `renderAll`, and `meshStageControl` should use `devtoolsRuntime` instead of `assets.rs`.

- [ ] **Step 3: Point CSS assertions at `devtools.css`**

Assertions for `.dx-devtools-*` CSS blocks should use `devtoolsCss` instead of extracting `DEVTOOLS_CSS` from `assets.rs`.

- [ ] **Step 4: Guard maintainability**

Add assertions:

```ts
assertSourceMatches(assetsRust, /include_str!\("assets\/runtime\.js"\)/);
assertSourceMatches(assetsRust, /include_str!\("assets\/devtools\.css"\)/);
assertSourceDoesNotMatch(assetsRust, /const\s+RUNTIME_JS:\s*&str\s*=\s*r###"/);
assertSourceDoesNotMatch(assetsRust, /const\s+DEVTOOLS_CSS:\s*&str\s*=\s*r###"/);
```

### Task 4: Bounded Verification and Install

**Files:**
- Verify: `dx-www/src/cli/devtools/assets.rs`
- Verify: `dx-www/src/cli/devtools/assets/runtime.js`
- Verify: `dx-www/src/cli/devtools/assets/devtools.css`
- Verify: `benchmarks/dx-devtools-framework-integration.test.ts`

- [ ] **Step 1: Run focused checks**

```powershell
node --check dx-www\src\cli\devtools\assets\runtime.js
node --test benchmarks\dx-devtools-framework-integration.test.ts
cargo fmt --check
cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www
```

- [ ] **Step 2: Rebuild and install only after checks pass**

```powershell
cargo build -j 6 -p dx-www --no-default-features --features cli --bin dx-www
```

Back up and replace `G:\Dx\bin\dx.exe` and `G:\Dx\bin\dx-www.exe`, restart `examples/template` on `127.0.0.1:3000`, and verify `/_dx/devtools/runtime.js`, `/_dx/devtools/devtools.css`, and the in-app browser UI.
