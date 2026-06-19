# WWW Same-Machine Raceboard Proof Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the WWW/Next/Svelte/Astro same-machine throughput proof reproducible from one source-owned TypeScript command without weakening release-readiness honesty.

**Architecture:** Keep `benchmarks/dx-runtime-throughput-benchmark.ts` as the measurement receipt writer. Add a small orchestration wrapper that can preflight prerequisites, optionally prepare heavy framework builds, start the four local targets, run the benchmark with exact URLs/PIDs/commands, and shut everything down. Readiness continues to reject missing, dry-run, non-OK, errored, or overclaiming receipts.

**Tech Stack:** Node.js TypeScript scripts, Rust release binaries for `dx-www` and `demo-server`, existing fair-counter Next/Svelte/Astro fixtures, and focused `node --test` contract checks.

---

### Task 1: Source-Owned Orchestrator

**Files:**
- Create: `benchmarks/dx-runtime-throughput-orchestrator.ts`
- Test: `benchmarks/dx-runtime-throughput-orchestrator.test.ts`

- [ ] **Step 1: Add a preflight mode**

`node benchmarks/dx-runtime-throughput-orchestrator.ts --mode preflight --json` must inspect release binaries, framework fixture installs, and build output paths without running package managers, Cargo, HTTP servers, or measurements.

- [ ] **Step 2: Add explicit prepare/run modes**

`--mode prepare` may run the heavy commands. `--mode run` may start local servers and invoke `dx-runtime-throughput-benchmark.ts`. `--mode all` may prepare and run. No heavy work happens unless the mode asks for it.

- [ ] **Step 3: Add a contract test**

The test must prove preflight is side-effect-light, the source includes the four exact target URLs, and the run path forwards `--dx-www-bin`, `--www-url`, `--next-url`, `--svelte-url`, `--astro-url`, `--*-pid`, and `--*-command` into the receipt writer.

### Task 2: Wire Readiness Replay

**Files:**
- Modify: `dx-www/src/cli/readiness.rs`
- Modify: `dx-www/src/cli/agent_context.rs`
- Test: `benchmarks/dx-runtime-throughput-receipt-contract.test.ts`
- Test: `benchmarks/dx-www-agent-context-command.test.ts`

- [ ] **Step 1: Point the active replay command at the orchestrator**

The active blocker should show one replay command:

```powershell
node benchmarks/dx-runtime-throughput-orchestrator.ts --mode all --jobs 6 --rounds 3 --requests 240 --concurrency 16 --out target/framework-comparison-20260529/throughput.json
```

- [ ] **Step 2: Keep the raw benchmark command visible**

The lower-level raw benchmark command stays in `readiness_replay_commands()` for workers who manually start servers.

- [ ] **Step 3: Keep claims false**

Do not change `release_ready`, `fastest_world_claim`, or the score. A current same-machine receipt still leaves paint/browser/provider proof as remaining gates.

### Task 3: Focused Verification

**Files:**
- Existing focused benchmark tests and readiness tests.

- [ ] **Step 1: Run Node contract tests**

```powershell
node --test --test-concurrency=1 benchmarks\dx-runtime-throughput-orchestrator.test.ts benchmarks\dx-runtime-throughput-receipt-contract.test.ts benchmarks\dx-www-agent-context-command.test.ts
```

- [ ] **Step 2: Run Rust format and compile**

```powershell
cargo fmt -p dx-www -- --check
cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www --message-format=short
```

- [ ] **Step 3: Regenerate local receipts**

```powershell
target\debug\dx-www.exe www readiness --write-receipts --json --full
target\debug\dx-www.exe check --json
```

Run `dx check` from `examples/template`.
