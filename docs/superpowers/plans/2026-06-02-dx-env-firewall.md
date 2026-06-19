# DX Env Firewall Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** Add a source-owned DX Env Firewall that lets developers edit a familiar `.env` viewport while durable values stay encrypted in `.dx/env/local.sr` with a generated `.dx/env/local.machine` contract.

**Architecture:** `dx env` becomes the CLI entrypoint for opening, locking, checking, and agent-safe reporting. A focused `cli/env_firewall/` module owns parsing, encryption, viewport state, receipts, and command rendering.

**Tech Stack:** Rust CLI, `dx-serializer` `.sr`/`.machine` generation, password-derived encryption, JSON read-model receipts, targeted Rust unit tests.

---

### Task 1: Env Firewall Core

**Files:**
- Create: `dx-www/src/cli/env_firewall/mod.rs`
- Create: `dx-www/src/cli/env_firewall/crypto.rs`
- Create: `dx-www/src/cli/env_firewall/files.rs`
- Create: `dx-www/src/cli/env_firewall/model.rs`
- Create: `dx-www/src/cli/env_firewall/tests.rs`
- Modify: `dx-www/Cargo.toml`

- [x] Add failing tests for locked viewport creation, encrypted store writes, unlock timeout behavior, and agent-safe redaction.
- [x] Implement parsing for `.env` lines without logging raw values.
- [x] Implement password-derived authenticated encryption for `.dx/env/local.sr`.
- [x] Generate `.dx/env/local.machine` from the `.sr` source.
- [x] Write receipts that prove key names, scopes, hashes, freshness, and redaction status without raw values.

### Task 2: CLI Integration

**Files:**
- Modify: `dx-www/src/cli/mod.rs`
- Modify: `dx-www/src/cli/mod_parts/cli_core_impl.rs`
- Modify: `dx-www/src/cli/help_text.rs`
- Modify: `dx-www/src/cli/env_firewall/mod.rs`

- [x] Add `dx env open --password <value>|--password-env <name> [--ttl-seconds 180]`.
- [x] Add `dx env lock --password <value>|--password-env <name>`.
- [x] Add `dx env reconcile --password <value>|--password-env <name>`.
- [x] Add `dx env check [--json]`.
- [x] Add `dx env agent-context [--json]`.
- [x] Ensure command output never prints secret values.

### Task 3: Documentation And Verification

**Files:**
- Modify: `WORLD.md`
- Modify: `AGENTS.md`
- Modify: `README.md`

- [x] Document DX Env Firewall as a native WWW capability.
- [x] Document the sealed `.env` viewport and encrypted `.dx/env` source of truth.
- [x] Run targeted tests first, then `cargo fmt --check`, `cargo check -j 6 -p dx-www --no-default-features --features cli --bin dx-www`, and `git diff --check`.
