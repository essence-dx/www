# WWW Public Framework Tools DX Style Split Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Close the `public-framework-tools-large` hygiene scorecard item by extracting the DX Style lane from `dx-www/src/cli/public_framework_tools.rs` into focused framework-owned modules without changing CLI behavior.

**Architecture:** Keep `public_framework_tools.rs` as the public CLI dispatcher and shared public-report helper layer. Move DX Style command execution, source scanning, token generation, compatibility checks, tests, and default CSS into `dx-www/src/cli/public_framework_tools/dx_style.rs`, with a narrow `style_helper_paths()` bridge for the parent import-map flow.

**Tech Stack:** Rust CLI module refactor, existing `node --test` benchmark guards, `cargo fmt --check`, `cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www`.

---

### Task 1: Preserve the Current Scorecard Baseline

**Files:**
- Read: `tools/hygiene/audit-repo-hygiene.ts`
- Read: `dx-www/src/cli/public_framework_tools.rs`

- [ ] **Step 1: Confirm the active flaw before edits**

Run:

```powershell
node tools\hygiene\audit-repo-hygiene.ts --json
```

Expected: JSON includes `"id":"public-framework-tools-large"` and `dx-www/src/cli/public_framework_tools.rs` above the configured line budget.

- [ ] **Step 2: Confirm git safety**

Run:

```powershell
git status --short --branch
git rev-parse --show-toplevel
git rev-parse --git-dir
git rev-parse --git-common-dir
```

Expected: working tree is on the intended `dev` branch, and unrelated untracked files are left untouched.

### Task 2: Extract DX Style Runtime Into a Child Module

**Files:**
- Modify: `dx-www/src/cli/public_framework_tools.rs`
- Create: `dx-www/src/cli/public_framework_tools/dx_style.rs`

- [ ] **Step 1: Add the child module declaration and imports**

In `dx-www/src/cli/public_framework_tools.rs`, add:

```rust
mod dx_style;

pub(super) use dx_style::run_dx_style;
use dx_style::{build_dx_style, check_dx_style, style_helper_paths};
```

Keep the existing external function names stable for `extension_orchestrator.rs`.

- [ ] **Step 2: Move the DX Style-owned code**

Move these coherent code clusters out of `public_framework_tools.rs` into `dx_style.rs`:

```text
DX Style constants and DxStylePaths
run_dx_style
style_pruning_report
build_dx_style
check_dx_style
style input file collection
CSS import and source directive parsing
scanned class token collection
DX Style token generation helpers
Tailwind/PostCSS compatibility scanners
hardcoded color findings
default_theme_css
default_generated_css
DX Style unit tests
```

Keep these shared helpers in the parent file:

```text
parse_subcommand_options
public_report
collect_files
collect_named_files
should_skip_path
write_json_receipt
resolve_project_path
dx_build_output_dir
normalize_path
normalize_relative_path
```

- [ ] **Step 3: Use a narrow parent bridge**

Implement this in `dx_style.rs`:

```rust
pub(super) fn style_helper_paths(project: &Path) -> Result<Vec<PathBuf>> {
    Ok(DxStylePaths::load(project)?.helper_inputs)
}
```

Then replace parent uses of `DxStylePaths::load(project)?.helper_inputs` with `style_helper_paths(project)?`.

- [ ] **Step 4: Keep visibility tight**

Only these functions should be `pub(super)` in `dx_style.rs`:

```rust
run_dx_style
build_dx_style
check_dx_style
style_helper_paths
```

Everything else remains private unless the compiler proves a narrow visibility bridge is required.

### Task 3: Update Source Guards Without Hiding the Split

**Files:**
- Modify only if needed: `benchmarks/public-framework-tools.test.ts`
- Modify only if needed: `benchmarks/dx-style-*.test.ts`
- Modify only if needed: `benchmarks/dx-run-extension-list-orchestrator.test.ts`
- Modify only if needed: `benchmarks/web-perf-receipt-mode-contract.test.ts`

- [ ] **Step 1: Find guards that read only the parent file**

Run:

```powershell
rg -n "public_framework_tools\.rs|publicFrameworkTools|public framework tools" benchmarks
```

Expected: a short list of benchmark guards that may need to read both the parent and `public_framework_tools/dx_style.rs`.

- [ ] **Step 2: Update guards to prove the new boundary**

For DX Style assertions, read both files explicitly:

```ts
const publicFrameworkTools = readFileSync(
  join(repoRoot, "dx-www/src/cli/public_framework_tools.rs"),
  "utf8",
);
const dxStyleTools = readFileSync(
  join(repoRoot, "dx-www/src/cli/public_framework_tools/dx_style.rs"),
  "utf8",
);
```

Assert dispatcher symbols in `publicFrameworkTools` and implementation symbols in `dxStyleTools` so tests do not pass because of accidental wrapper strings.

### Task 4: Verify the Focused Refactor

**Files:**
- Read: `tools/hygiene/audit-repo-hygiene.ts`
- Read: `dx-www/src/cli/public_framework_tools.rs`
- Read: `dx-www/src/cli/public_framework_tools/dx_style.rs`

- [ ] **Step 1: Format check**

Run:

```powershell
cargo fmt --check
```

Expected: pass. If it fails due formatting in touched Rust files, run `cargo fmt` and re-run `cargo fmt --check`.

- [ ] **Step 2: Compile the CLI**

Run:

```powershell
cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www
```

Expected: pass.

- [ ] **Step 3: Run hygiene and focused source guards**

Run:

```powershell
node --test benchmarks\repo-hygiene-audit.test.ts
node tools\hygiene\audit-repo-hygiene.ts --json
```

Expected: `public-framework-tools-large` is no longer active. `readyFor100` may remain false while other already-known scorecard items are still open.

- [ ] **Step 4: Run focused DX Style guards if their expectations were updated**

Run:

```powershell
node --test benchmarks\dx-style-launch-contract.test.ts
node --test benchmarks\dx-style-postcss-compatibility.test.ts
node --test benchmarks\dx-style-pruning-contract.test.ts
node --test benchmarks\dx-style-v43-source-scanner-contract.test.ts
node --test benchmarks\dx-style-v43-source-scanner-fixture-matrix.test.ts
node --test benchmarks\dx-style-lane8-no-runtime-integration.test.ts
```

Expected: pass, or report any pre-existing stale source guard separately from this refactor.

- [ ] **Step 5: Final diff hygiene**

Run:

```powershell
git diff --check
rg -n --glob "!**/target/**" --glob "!**/node_modules/**" --glob "!**/.dx/**" --glob "!**/www/output/**" --glob "!**/.git/**" "^(<<<<<<<|=======|>>>>>>>)"
```

Expected: no whitespace errors and no conflict markers.

### Task 5: Commit the Completed Slice

**Files:**
- Modify: `dx-www/src/cli/public_framework_tools.rs`
- Create: `dx-www/src/cli/public_framework_tools/dx_style.rs`
- Modify only if needed: benchmark guard files
- Create: `docs/superpowers/plans/2026-05-28-www-public-framework-tools-dx-style-split.md`

- [ ] **Step 1: Review the diff**

Run:

```powershell
git diff --stat
git diff -- dx-www/src/cli/public_framework_tools.rs dx-www/src/cli/public_framework_tools/dx_style.rs
```

Expected: the parent loses DX Style implementation code and the child owns it; unrelated user files are not included.

- [ ] **Step 2: Commit only related files**

Run:

```powershell
git add docs/superpowers/plans/2026-05-28-www-public-framework-tools-dx-style-split.md dx-www/src/cli/public_framework_tools.rs dx-www/src/cli/public_framework_tools/dx_style.rs benchmarks
git commit -m "refactor: split dx style public framework tools"
```

Expected: a focused commit after verification.

