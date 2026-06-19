# WWW release-readiness Receipt Freshness Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make stale starter `dx check` receipts visibly unsafe when release-readiness release-gate metadata is missing.

**Architecture:** Keep the existing `dx_check_latest_receipt.rs` writer as the source of truth, then make docs-doctor and agent-context read the same release-readiness gate fields from `examples/template/.dx/receipts/check/check-latest.json`. Missing gate metadata is an error/blocker, even when the score is green.

**Tech Stack:** Rust CLI modules, serde_json receipts, Node test runner, focused Cargo tests.

---

### Task 1: Lock the Source Test

**Files:**
- Modify: `benchmarks/dx-www-docs-doctor.test.ts`
- Modify: `benchmarks/dx-www-agent-context-command.test.ts`

- [ ] Add assertions that the starter receipt includes `release_ready: false`, `fastest_world_claim: false`, `readiness_gate_status.release_ready: false`, `readiness_gate_status.fastest_world_claim: false`, and replay commands for `dx www readiness --json --full`, `dx www agent-context --json --full`, and `dx www docs-doctor --json`.
- [ ] Run `node --test --test-concurrency=1 .\benchmarks\dx-www-docs-doctor.test.ts .\benchmarks\dx-www-agent-context-command.test.ts` and verify the current fixture fails because the receipt is stale.

### Task 2: Surface Stale Receipt Gates

**Files:**
- Modify: `dx-www/src/cli/docs_doctor.rs`
- Modify: `dx-www/src/cli/agent_context.rs`

- [ ] Extend docs-doctor's starter receipt summary with release-readiness gate fields and replay commands.
- [ ] Add a docs-doctor error when the starter receipt lacks release-readiness gate metadata or tries to claim release readiness.
- [ ] Add an agent-context blocker for the same stale/misleading starter receipt state.

### Task 3: Refresh the Fixture

**Files:**
- Modify: `examples/template/.dx/receipts/check/check-latest.json`

- [ ] Bring the checked-in starter receipt up to the current `dx_check_latest_receipt.rs` schema by adding release-readiness release gates and replay commands.
- [ ] Preserve the existing score/section evidence; do not invent successful runtime proof.

### Task 4: Verify

**Files:**
- No new files expected.

- [ ] Run `node --test --test-concurrency=1 .\benchmarks\dx-www-docs-doctor.test.ts .\benchmarks\dx-www-agent-context-command.test.ts .\benchmarks\dx-www-readiness-foundation.test.ts`.
- [ ] Run focused Rust tests for `docs_doctor`, `agent_context`, and `dx_check_latest_receipt` if disk allows.
- [ ] Run `cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www` only if the focused Rust tests and disk state are healthy.
- [ ] Run `git diff --check` and a conflict-marker scan over touched files.
