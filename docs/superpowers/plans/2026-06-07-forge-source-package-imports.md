# Forge Source Package Imports Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Upgrade Forge and WWW so external ecosystems can be reviewed, sliced, and materialized into source-owned Forge packages without making `node_modules`, package-manager installs, or lifecycle scripts the project authority.

**Current completed slices:** `d6540e26` completes the focused npm review-gate parser split, help contract, reviewed `clsx` serializer receipt path, and focused verification. `84c5c0e8` adds the policy-first universal import firewall model for npm, pip, cargo, and Go without fetching packages or running installs. `5518d644` makes the reviewed `clsx` adapter receipt honest as an npm source slice. The current Task 5 slice is focused on physical Forge source aliases without resolver overreach. Broader ecosystem CLI surfaces and deeper authority-coherence checks remain open.

**Architecture:** Keep `dx add` for curated Forge packages and make `dx forge import <ecosystem>` the explicit external-source review lane. Imported packages pass through acquire, quarantine, analyze, slice, materialize, receipt, and optional rewrite phases. The app imports materialized source such as `@/forge/npm/clsx` or a reviewed alias like `@/three`; quarantine and upstream package trees are never importable app source.

**Tech Stack:** Rust CLI and ecosystem modules, Forge source manifests, serializer `.sr` plus `.machine` artifacts, BLAKE3 integrity, source-engine resolver aliases, Node `.ts` contract tests, focused Rust checks.

---

### Task 1: Preserve And Split The Existing npm Import Command

**Files:**
- Modify: `dx-www/src/cli/mod_parts/cli_forge_commands_a.rs`
- Create: `dx-www/src/cli/forge_import_npm_options.rs`
- Modify: `dx-www/src/cli/mod.rs`
- Modify: `dx-www/src/cli/help_text.rs`
- Test: `benchmarks/cli-forge-import-npm-options-split-safety.test.ts`
- Test: `benchmarks/cli-forge-import-npm-help-contract.test.ts`

- [x] Move inline `dx forge import npm` parsing into a focused options module.
- [x] Keep current behavior exactly: `--plan` works for package names, `--write` only materializes reviewed `clsx`.
- [x] Add `--json` as an alias for `--format json` if it follows local CLI style.
- [x] Update help text to show `dx forge import npm <package> --plan|--write`.
- [x] State clearly in help/docs that import is a review gate, not `npm install`.
- [x] Add source-contract tests that prove parsing is split and help text matches parser reality.

### Task 2: Add Source Import Plan Receipts

**Files:**
- Modify: `dx-www/src/cli/forge_npm_import_plan.rs`
- Modify: `dx-www/src/cli/serializer_artifacts.rs` only if a helper is needed
- Test: `dx-www/src/cli/tests/part_03.rs`
- Test: `benchmarks/forge-package-import-sr-machine-receipt.test.ts`

- [x] Add a schema-bearing import plan read model: `dx.forge.package_import_plan`.
- [x] Include ecosystem, package name, requested symbols, import alias, source kind, no-node-modules status, lifecycle-script status, materialization status, review findings, and next commands.
- [x] Write stable `.sr` plan artifacts under `.dx/forge/import-plans/<ecosystem>-<package>.sr`.
- [x] Generate matching `.machine` artifacts through the existing serializer pipeline for the reviewed `clsx --write` path.
- [x] Keep JSON output for terminals and compatibility while exposing `.sr` and `.machine` as the DX-native token/speed path.
- [x] Prove `clsx --plan` and `clsx --write` expose JSON, `.sr`, and machine paths.

### Task 3: Formalize The Universal Import Firewall Model

**Files:**
- Create: `core/src/ecosystem/forge_importer/mod.rs`
- Create: `core/src/ecosystem/forge_importer/types.rs`
- Create: `core/src/ecosystem/forge_importer/acquire.rs`
- Create: `core/src/ecosystem/forge_importer/quarantine.rs`
- Create: `core/src/ecosystem/forge_importer/analyze.rs`
- Create: `core/src/ecosystem/forge_importer/slice.rs`
- Create: `core/src/ecosystem/forge_importer/receipts.rs`
- Modify: `core/src/ecosystem/mod.rs`
- Test: `benchmarks/forge-import-security-gates.test.ts`

- [x] Define ecosystems: `npm`, `pip`, `cargo`, and `go`.
- [x] Define package source phases: `plan`, `acquire`, `quarantine`, `analyze`, `slice`, `materialize`, `rewrite`.
- [x] Define slice kinds: `adapter`, `source-copy`, `asset-slice`, `metadata-only`, and `blocked`.
- [x] Reject unsafe paths, absolute paths, backslashes, `..`, symlinks, and project escapes.
- [x] Mark lifecycle scripts, install hooks, native binaries, obfuscated blobs, dynamic execution, and huge unreviewed graphs as blocked or manual-review.
- [x] Keep this module policy-first; do not fetch live external package code in this task.

### Task 4: Add npm Source Slice Compatibility Around `clsx`

**Files:**
- Modify: `dx-www/src/cli/forge_npm_import_plan.rs`
- Modify: `core/src/ecosystem/forge_security.rs`
- Test: `dx-www/src/cli/tests/part_03.rs`
- Test: `benchmarks/forge-source-owned-package-review.test.ts`

- [x] Represent the reviewed `clsx` adapter as the first `npm` source slice.
- [x] Record origin as `npm-reviewed-adapter`, not curated launch package.
- [x] Add schema-bearing receipt fields: origin, BLAKE3 file hashes, declared license, selected exports, kept files, written files, rejected files, and policy gates.
- [x] Preserve the existing adapter path until a migration path is explicit.
- [x] Ensure local edits are not overwritten silently; identical files are kept, mismatches require review.

### Task 5: Make Forge Aliases Resolve Without Resolver Overreach

**Files:**
- Modify: `core/src/ecosystem/forge_three_scene.rs`
- Modify: `core/src/ecosystem/forge_registry_parts/registry_config.rs`
- Modify: `core/src/ecosystem/forge_registry_parts/package_lanes.rs` if needed
- Test: `benchmarks/forge-three-materialized-alias.test.ts`
- Test: `benchmarks/three-scene-package-doc.test.ts`

- [x] Add a reviewed materialized alias surface for Three-style usage only if the package already owns the source.
- [x] Prefer physical source files such as `three/index.ts` or `lib/forge/npm/three/index.ts` over resolver magic.
- [x] Let existing `@/` project-root resolution find the source-owned alias.
- [x] Do not make `node_modules/three` resolvable by default.
- [x] Document the difference between Forge package ID aliases and source import aliases.

### Task 6: Add Evidence Coherence And Security Docs

**Files:**
- Create: `docs/forge-import-security-model.md`
- Modify: `docs/SECURITY.md`
- Modify: `docs/forge-launch-limitations.md`
- Test: `benchmarks/forge-import-evidence-coherence.test.ts`
- Test: `benchmarks/forge-trust-policy-artifact-coherence.test.ts`

- [x] Document that `.dx/forge/package-status.json` is a read model, not an authority artifact.
- [x] Require authority agreement between source manifest, package lock, receipts, trust policy, and remote status before materialization claims pass.
- [x] Keep BLAKE3 package/file integrity separate from SHA-256 receipt freshness checks.
- [x] Explain that license and advisory data are declaration-only unless marked reviewed.
- [x] Add tests that fail when status claims lock/receipt/remote proof without referenced files.

### Task 7: Add Future Ecosystem Import Plan Surfaces

**Files:**
- Create: `core/src/ecosystem/forge_importer/npm.rs`
- Create: `core/src/ecosystem/forge_importer/pip.rs`
- Create: `core/src/ecosystem/forge_importer/cargo.rs`
- Create: `core/src/ecosystem/forge_importer/go.rs`
- Create: `docs/forge-universal-importers.md`

- [x] Define non-executing metadata acquisition rules for npm tarballs, PyPI wheels/sdists, crates.io crates, and Go modules.
- [x] Define manual-review triggers for each ecosystem.
- [x] Record that Forge import does not run `npm install`, `pip install`, `cargo add`, `go get`, setup hooks, or build scripts.
- [x] Leave live fetching behind explicit future commands and receipts.
- [x] Keep `dx add npm/foo`, `dx add pip/foo`, `dx add cargo/foo`, and `dx add go/foo` unsupported unless an accepted import receipt exists.

### Task 8: Verification And Commit

**Files:**
- All changed files from Tasks 1-7

- [x] Run focused Node contract tests for the touched files.
- [x] Run `cargo fmt --check`.
- [x] Run `cargo check -j6 -p dx-www --no-default-features --features cli --bin dx-www`.
- [x] Run `git diff --check`.
- [x] Commit only the related Forge/WWW import upgrade files.
- [x] Push/sync only after the focused checks pass or after reporting any blocker honestly.
