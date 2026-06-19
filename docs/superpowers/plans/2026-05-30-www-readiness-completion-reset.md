# WWW release-readiness Completion Reset Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn the current release-readiness work from broad receipt scaffolding into honestly proven framework behavior with stable source, receipts, runtime checks, and browser/provider evidence.

**Architecture:** Stabilize one vertical proof lane at a time: source contract, generated receipt, `.sr` contract, `.machine` cache, focused test, real command replay, then runtime/browser proof. Do not expand claims until the lane has evidence. Keep the shared dirty worktree safe by touching only release-readiness-owned files and by avoiding broad builds until focused guards pass.

**Tech Stack:** Rust CLI (`dx-www`), DX serializer `.sr` and generated `.machine` receipts, Node `--test` TypeScript tests, targeted Cargo checks, Browser/Chrome only after source-level proof passes.

---

## Demanding Current Snapshot

Score estimate:

- Source/receipt scaffolding depth: **93/100**
- Runtime/browser/provider proof maturity: **64/100**
- Source-readiness advisory score: **97/100**
- Release proof maturity: **not release-ready**
- Public 99 readiness: **not achieved**

What has been attempted:

- release-readiness command/report scaffolding was added around score, proof graph, replay commands, delivery tiers, native event catalog, no-JS artifacts, bundle partition, production HTTP, route/action runtime, primitives, islands, reactivity, docs/onboarding, and devtools visual edit receipts.
- Focused release-readiness Node tests were added for foundation, proof graph, primitive receipts, production HTTP receipts, reactivity receipts, docs/onboarding receipts, native event browser binder import, browser receipt harness, tiny-static/public partition, Lighthouse/runtime guard, and docs-doctor.
- `dx check examples/template --json` was used to refresh `examples/template/.dx/receipts/check/check-latest.json` through the real executable after the stale replay-command failure was found.
- Current docs/onboarding patch exposed extra agent-context status fields and a separate blocker for generated/archive warning cleanup.

What is not proven enough yet:

- A lot of release-readiness is still source-owned receipt proof, not live runtime proof.
- `dx-www/src/cli/readiness.rs` and `dx-www/src/cli/docs_doctor.rs` are currently untracked in git, so they must be treated as lane-owned integration files, not stable baseline.
- The worktree is extremely dirty from many parallel lanes. Do not assume unrelated files are safe to rewrite.
- Browser/provider proof is still weak or missing for several gates.
- The score should not be raised to 99 until tiny-static, no-JS, native events, islands, state, primitives, production HTTP, route actions, devtools, docs-doctor, and check receipts all have replayable evidence.
- `READINESS_CURRENT_HONEST_SCORE = 97` is currently an advisory source score, not a release-readiness score. It must stay paired with `release_ready=false`, `fastest_world_claim=false`, and visible blockers until replay receipts prove the runtime.

## File Ownership For This Reset

release-readiness lane may edit:

- `G:/Dx/www/dx-www/src/cli/readiness.rs`
- `G:/Dx/www/dx-www/src/cli/docs_doctor.rs`
- `G:/Dx/www/dx-www/src/cli/agent_context.rs`
- `G:/Dx/www/dx-www/src/cli/dx_check_latest_receipt.rs`
- `G:/Dx/www/benchmarks/dx-www-readiness-*.test.ts`
- `G:/Dx/www/benchmarks/dx-www-docs-doctor.test.ts`
- `G:/Dx/www/benchmarks/dx-www-agent-context-command.test.ts`
- `G:/Dx/www/examples/template/.dx/receipts/check/check-latest.json`

release-readiness lane should not edit without a new checkpoint:

- unrelated package-lane benchmarks
- media-icon files
- broad core/runtime files already modified by other lanes
- global docs except release-readiness-specific plan/status docs

## Execution Plan

### Task 1: Stabilize The Current release-readiness Source Slice

- [ ] Run a targeted syntax/marker check:

```powershell
node --test --test-concurrency=1 .\benchmarks\dx-www-readiness-docs-onboarding-receipts.test.ts .\benchmarks\dx-www-readiness-proof-graph-receipt.test.ts .\benchmarks\dx-www-readiness-foundation.test.ts .\benchmarks\dx-www-agent-context-command.test.ts
```

Expected: either pass, or fail on concrete marker drift in `readiness.rs` / `agent_context.rs`.

- [ ] Fix only marker drift or compile-obvious issues in the release-readiness-owned files.

- [ ] Run:

```powershell
rustfmt --edition 2024 dx-www/src/cli/readiness.rs dx-www/src/cli/docs_doctor.rs dx-www/src/cli/agent_context.rs
```

Expected: no output.

### Task 2: Regenerate Receipts Through Real Commands

- [ ] Generate release-readiness local receipts from current source:

```powershell
.\target\debug\dx-www.exe www readiness --write-receipts --json --full
```

If `target/debug/dx-www.exe` is stale, build only the dx-www binary first:

```powershell
cargo build -j 1 -p dx-www --no-default-features --features cli --bin dx-www
```

- [ ] Refresh starter check receipt through the actual writer:

```powershell
.\target\debug\dx-www.exe check examples/template --json
```

- [ ] Confirm the starter receipt includes every current `readiness_replay_commands()` entry, including docs/onboarding and reactivity tests.

### Task 3: release-readiness Verification Ladder

- [ ] Run focused Node tests first:

```powershell
node --test --test-concurrency=1 .\benchmarks\dx-www-docs-doctor.test.ts .\benchmarks\dx-www-agent-context-command.test.ts .\benchmarks\dx-www-readiness-foundation.test.ts .\benchmarks\dx-www-readiness-proof-graph-receipt.test.ts .\benchmarks\dx-www-readiness-docs-onboarding-receipts.test.ts
```

- [ ] Run the targeted Rust receipt test:

```powershell
cargo test -j 1 -p dx-www --no-default-features --features cli readiness_write_receipts_writes_native_events_without_visual_fake --lib
```

- [ ] Run compile check only after the focused tests pass:

```powershell
cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www --message-format=short
```

### Task 4: Convert Receipt Scaffolding Into Runtime Proof

Do these in order. Each subtask needs source behavior, receipt, `.sr`, `.machine`, focused test, and replay command.

- [ ] Tiny-static/no-JS: prove meaningful HTML/CSS/link/form output with JS disabled and public bytes separated from evidence bytes.
- [ ] Native events: prove generated React-style `onClick`/`onInput`/pointer/etc. bind to real DOM listeners without React runtime.
- [ ] Reactivity: prove `state`, `derived`, `effect`, `action` update DOM directly, and unsupported React APIs diagnose instead of no-op.
- [ ] Islands: prove camelCase `clientLoad`, `clientVisible`, `clientIdle`, `clientOnly` ABI with no-JS fallback and explicit adapter boundary.
- [ ] Primitives: prove Image/Font/Script/Wasm behavior with owned receipts and no Next-clone overclaims.
- [ ] Production HTTP: prove ETag/304/Range/compression/precompressed/cache headers through actual preview/server paths.
- [ ] Route handlers/actions/streaming: prove POST, errors, CSRF/idempotency, streaming, cancellation, and fallback behavior.
- [ ] Devtools: prove inspect/cascade/preview/apply/undo receipts through real browser replay.
- [ ] Docs/check: prove docs-doctor and `dx check` can reject stale claims and print replay commands.

### Task 5: Browser And Provider Proof

- [ ] Use Browser/Chrome only after source, receipt, and focused Node/Rust tests pass.
- [ ] Capture JS-disabled and JS-enabled browser evidence for tiny-static/no-JS and rich interactive routes.
- [ ] Run Lighthouse/runtime/throughput benchmarks across tiny static, island, content, dashboard, and action routes.
- [ ] Keep `fastest_world_claim=false` until WWW beats or clearly ties the comparison matrix across multiple route shapes.

### Task 6: Final Hygiene

- [ ] Run:

```powershell
git diff --check
```

- [ ] Run conflict marker scan excluding generated/heavy folders:

```powershell
rg -n "<<<<<<<|=======|>>>>>>>" --glob "!target/**" --glob "!node_modules/**" --glob "!.dx/www/output/**"
```

- [ ] Produce a final report with three lists:
  - fully wired and verified
  - receipt/source-only foundation
  - still missing framework support

## Stop Rules

- Stop broad execution if release-readiness-owned files fail focused tests with many unrelated compile errors.
- Stop before browser/provider benchmarking if current source receipts cannot be generated.
- Stop and report if another lane edits the same release-readiness-owned file in a conflicting way.
