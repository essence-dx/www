# WWW Runtime Benchmark Polish Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Benchmark DX WWW against fresh Next.js, SvelteKit, and Astro starters for runtime, Lighthouse-style browser quality, and server throughput, then fix concrete WWW issues before scoring it honestly.

**Architecture:** Keep benchmark evidence source-owned under `docs/` and `target/` while keeping production framework edits small. Use the existing release binary and generated comparison starters; do not add runtime dependencies to WWW apps just to measure them.

**Tech Stack:** Rust/Cargo, PowerShell, Node test runner, Codex Browser plugin, optional Lighthouse CLI when available, and source-owned markdown reports.

---

### Task 1: Current State Documentation

**Files:**
- Create: `docs/DX_WWW_CURRENT_DETAILS_2026-05-29.md`
- Modify after benchmarks: `docs/DX_WWW_CURRENT_DETAILS_2026-05-29.md`

- [x] **Step 1: Record the current verified build/import/devtools state**

Write the current state summary with exact commands and measured comparison values from the previous verification pass.

- [x] **Step 2: Update the file after runtime benchmarks**

Add browser runtime, Lighthouse, and throughput sections with exact evidence.

### Task 2: Runtime Browser Benchmark

**Files:**
- Create if useful: `benchmarks/dx-runtime-browser-benchmark.test.ts`
- Modify: `docs/DX_WWW_CURRENT_DETAILS_2026-05-29.md`

- [x] **Step 1: Start each production preview server on a unique localhost port**

Use the built output or framework-native preview command for WWW, Next.js, SvelteKit, and Astro.

- [x] **Step 2: Measure browser runtime with Browser**

For each starter, capture:

```text
URL
document.readyState
performance.navigation/load timings
script count
stylesheet count
resource transfer size
console errors
visible first screen screenshot when practical
```

- [x] **Step 3: Record exact results**

Write the evidence into the current details markdown and keep screenshots only if they materially help review.

### Task 3: Lighthouse Or Browser Quality Benchmark

**Files:**
- Modify: `docs/DX_WWW_CURRENT_DETAILS_2026-05-29.md`

- [x] **Step 1: Detect Lighthouse availability**

Run:

```powershell
npx -y lighthouse --version
```

Expected: version output or a clear unavailable/error result.

- [x] **Step 2: Run Lighthouse only if practical**

For each local server:

```powershell
npx -y lighthouse http://127.0.0.1:<port>/ --quiet --chrome-flags="--headless=new" --output=json --output-path=target/framework-comparison-20260529/lighthouse/<name>.json
```

- [x] **Step 3: If Lighthouse cannot run, use Browser timing proof**

Document the fallback clearly and do not claim a Lighthouse score.

### Task 4: Server Throughput Benchmark

**Files:**
- Create if useful: `benchmarks/dx-server-throughput-benchmark.test.ts`
- Modify: `docs/DX_WWW_CURRENT_DETAILS_2026-05-29.md`

- [x] **Step 1: Use a lightweight Node benchmark harness**

Measure each local server with:

```text
warmup requests
fixed request count
bounded concurrency
p50/p95/p99 latency
requests/sec
error count
bytes read
```

- [x] **Step 2: Compare WWW against Next.js, SvelteKit, and Astro**

Use the same URL path and similar production serving mode where possible.

- [x] **Step 3: Investigate WWW regressions before patching**

If WWW is slower, use systematic debugging: identify whether the bottleneck is static file serving, route rendering, response headers, disk IO, or process startup.

### Task 5: Concrete WWW Fixes

**Files:**
- Modify only files proven by benchmark evidence.

- [x] **Step 1: Fix only measured issues**

Examples of allowed fixes:

```text
wrong cache headers
uncompressed oversized production asset
unnecessary dev-only payload in production
slow static file path
missing response headers hurting Lighthouse
```

- [x] **Step 2: Add a targeted regression guard**

Use TypeScript benchmark tests for production-cleanliness/runtime guards or focused Rust tests for server behavior.

### Task 6: Final Verification And Score

**Files:**
- Modify: `docs/DX_WWW_CURRENT_DETAILS_2026-05-29.md`
- Modify: `PLAN.md` only if a roadmap item status changes.

- [x] **Step 1: Run focused verification**

```powershell
node --test benchmarks\dx-imports-auto-import-scan.test.ts benchmarks\dx-imports-dts-generation.test.ts benchmarks\dx-imports-ide-types.test.ts benchmarks\dx-imports-used-only.test.ts benchmarks\dx-imports-explicit-alias.test.ts benchmarks\dx-imports-check-build-gate.test.ts benchmarks\dx-imports-receipts.test.ts benchmarks\public-framework-contract.test.ts benchmarks\public-framework-tools.test.ts benchmarks\dx-run-extension-list-orchestrator.test.ts benchmarks\dx-dev-extension-toolchain-hook.test.ts benchmarks\dx-devtools-framework-integration.test.ts
cargo fmt --check
cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www
cargo build --manifest-path G:\Dx\www\Cargo.toml -p dx-www --bin dx-www --release --locked -j6
git diff --check
rg -n "^(<<<<<<<|=======|>>>>>>>)" --glob '!target/**' --glob '!node_modules/**' --glob '!.dx/www/output/**' --glob '!vendor/**' --glob '!benchmarks/next-rust-conflict-markers.test.ts'
```

- [x] **Step 2: Score the benchmark gate**

Use:

```text
99 / 100 only if WWW wins or ties key runtime/server metrics and has no unresolved production-readiness gaps in the measured starter path.
95-98 / 100 if WWW wins build/footprint and is competitive in runtime/server proof but still lacks mature islands/image/font/wasm features.
90-94 / 100 if the current build/footprint win remains real but runtime/server proof is incomplete or mixed.
```
