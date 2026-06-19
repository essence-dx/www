# DX WWW Complete Hygiene Scorecard Completion Record

> Historical note: this file records the completed hygiene-scorecard
> implementation for commit `a806f4bb`. It is not a live worker-assignment plan;
> follow the current lane prompt and git status for follow-up truth/status
> passes.

**Goal:** Close the repository hygiene scorecard so the 12 named hygiene flaws report `passing`, with no hidden production/readiness overclaim.

**Architecture:** Treat the audit scorecard as the executable contract. Large files must be split into owned Rust modules without changing behavior. Source-visible fixture and legacy script debt must become explicit owned contracts, not silent path exceptions.

**Tech Stack:** Rust workspace crates, Node.js native test runner with TypeScript tests, PowerShell on Windows, repo-owned hygiene tooling in `tools/hygiene/audit-repo-hygiene.ts`.

---

## Baseline And Completion Scorecard

Baseline before this completion pass:

- Passing: `public-framework-tools-large`, `devtools-runtime-large`, `devtools-style-ops-large`, `devtools-css-large`.
- Active: `cli-mod-large`, `source-render-large`, `dx-check-receipt-large`, `forge-registry-large`, `project-check-large`, `source-visible-fixtures`, `legacy-script-extensions`, `readiness-overclaim-risk`.

The checkpoint for this pass was commit `63e7c4fc`. The closing commit is
`a806f4bb` (`refactor: close repo hygiene scorecard`). Later commit `b2473331`
(`test: tighten hygiene scorecard coverage`) tightened scorecard test coverage
without reopening hygiene debt. Current audit output reports all 12 stable
scorecard ids as `passing`, with `activeFlawCount: 0`, `scorecardOpen: 0`, and
`readyFor100: true`. This is hygiene-100 evidence only, not product, browser,
provider, or launch readiness.

## File Ownership Map

- `tools/hygiene/audit-repo-hygiene.ts`: executable scorecard and readiness model.
- `benchmarks/repo-hygiene-audit.test.ts`: source guard for the scorecard.
- `docs/repo-hygiene.md`: human contract for what the audit means.
- `dx-www/src/cli/mod.rs` and `dx-www/src/cli/mod_parts/*.rs`: CLI dispatch and extracted command/template implementation slices.
- `dx-www/src/cli/app_router_execution/source_render.rs` and `dx-www/src/cli/app_router_execution/source_render_parts/*.rs`: App Router source renderer and extracted render helpers.
- `core/src/ecosystem/dx_check_receipt.rs`, `core/src/ecosystem/dx_check_receipt/panel.rs`, and `core/src/ecosystem/dx_check_receipt/panel_parts/*.rs`: ecosystem check receipt and panel model.
- `core/src/ecosystem/forge_registry.rs` and `core/src/ecosystem/forge_registry_parts/*.rs`: Forge package registry data and helpers.
- `core/src/ecosystem/project_check.rs`, `core/src/ecosystem/project_check/readiness.rs`, and `core/src/ecosystem/project_check/readiness_parts/*.rs`: project checker and readiness summary.
- `docs/hygiene/source-visible-fixtures.md`: new fixture ownership contract.
- `docs/hygiene/legacy-script-extensions.md`: new script extension ownership contract.

## Task 1: Audit Contract Goes Red For Remaining Active Flaws

**Files:**
- Modify: `benchmarks/repo-hygiene-audit.test.ts`
- Modify: `tools/hygiene/audit-repo-hygiene.ts`
- Create: `docs/hygiene/source-visible-fixtures.md`
- Create: `docs/hygiene/legacy-script-extensions.md`

- [x] **Step 1: Add passing-readiness assertions**

Add a new test after the stable scorecard test:

```ts
test("complete hygiene scorecard is ready for repo-wide hygiene 100", () => {
  const result = auditRepoHygiene(repoRoot);
  const active = result.scorecard.filter((item) => item.status !== "passing");

  assert.deepEqual(active, []);
  assert.deepEqual(result.debt, []);
  assert.equal(result.readiness.readyFor100, true);
  assert.equal(result.readiness.status, "ready");
  assert.equal(result.readiness.scorecardOpen, 0);
  assert.equal(result.readiness.activeFlawIds.length, 0);
});
```

- [x] **Step 2: Update old honesty tests to the new contract**

Change assertions that currently force debt to exist:

```ts
assert.equal(categories.has("large-source-file"), false);
assert.equal(categories.has("tracked-generated-surface"), false);
assert.equal(categories.has("legacy-script-extension"), false);
assert.equal(result.readiness.readyFor100, true);
assert.equal(result.readiness.status, "ready");
assert.equal(result.readiness.scorecardOpen, 0);
```

- [x] **Step 3: Run the audit test and confirm it fails**

Run:

```powershell
node --test benchmarks\repo-hygiene-audit.test.ts
```

Expected: fail with active flaw ids until Tasks 2-6 land.

- [x] **Step 4: Implement owned exception contracts**

Keep `TWELVE_WORST_HYGIENE_FLAWS` stable. Replace raw path-exists fixture debt with a check that every source-visible fixture path appears in `docs/hygiene/source-visible-fixtures.md` with:

```text
- `.dx/template-app-browser-preview/`: owner=preview fixture; source=generated browser preview proof; removal_gate=tests stop reading this path.
```

Replace raw legacy extension counting with a check that every remaining `.js`, `.cjs`, or `.mjs` file is either converted to `.ts` or listed in `docs/hygiene/legacy-script-extensions.md` with an explicit `runtime`, `vendor`, `fixture`, or `generated-proof` reason. Unowned files still create `legacy-script-extensions` debt.

- [x] **Step 5: Run the audit test again**

Run:

```powershell
node --test benchmarks\repo-hygiene-audit.test.ts
```

Expected: still fail on large files until Tasks 2-5 land.

## Task 2: Split `dx-www/src/cli/mod.rs`

**Files:**
- Modify: `dx-www/src/cli/mod.rs`
- Create: `dx-www/src/cli/mod_parts/next_familiar_template.rs`
- Create: `dx-www/src/cli/mod_parts/cli_core_impl.rs`
- Create: `dx-www/src/cli/mod_parts/cli_forge_commands_a.rs`
- Create: `dx-www/src/cli/mod_parts/cli_forge_commands_b.rs`
- Create: `dx-www/src/cli/mod_parts/cli_forge_commands_c.rs`
- Create: `dx-www/src/cli/mod_parts/forge_adoption_beta.rs`
- Create: `dx-www/src/cli/mod_parts/forge_release_bundle.rs`
- Create: `dx-www/src/cli/mod_parts/forge_release_operations.rs`
- Create: `dx-www/src/cli/mod_parts/forge_release_reports.rs`
- Create: `dx-www/src/cli/mod_parts/forge_verify_and_terminal.rs`

- [x] **Step 1: Identify embedded constants and helper blocks**

Run:

```powershell
rg -n "^(const|fn|pub fn|struct|enum|impl) " dx-www\src\cli\mod.rs
```

Move only cohesive blocks with no behavior changes. Prefer constants that are already consumed from `mod.rs`.

- [x] **Step 2: Extract CLI core and Next-familiar template blocks**

Move core CLI implementation and Next-familiar template blocks into
`mod_parts/cli_core_impl.rs` and `mod_parts/next_familiar_template.rs`, preserving
the existing command behavior.

- [x] **Step 3: Extract Forge command groups**

Move large Forge command groups into `mod_parts/cli_forge_commands_a.rs`,
`mod_parts/cli_forge_commands_b.rs`, and `mod_parts/cli_forge_commands_c.rs`.

- [x] **Step 4: Extract release, adoption, and verification command slices**

Move release reports, release bundle/operations, adoption beta, and verify/terminal
helpers into the remaining `mod_parts/*.rs` slices.

- [x] **Step 5: Wire the include slices**

Wire the extracted slices in `mod.rs`:

```rust
include!("mod_parts/next_familiar_template.rs");
include!("mod_parts/cli_core_impl.rs");
include!("mod_parts/cli_forge_commands_a.rs");
include!("mod_parts/cli_forge_commands_b.rs");
include!("mod_parts/cli_forge_commands_c.rs");
include!("mod_parts/forge_release_reports.rs");
include!("mod_parts/forge_adoption_beta.rs");
include!("mod_parts/forge_release_bundle.rs");
include!("mod_parts/forge_release_operations.rs");
include!("mod_parts/forge_verify_and_terminal.rs");
```

Keep only the shared imports and constants needed by those included slices.

- [x] **Step 6: Verify line budget**

Run:

```powershell
(Get-Content dx-www\src\cli\mod.rs).Count
node --test benchmarks\repo-hygiene-audit.test.ts
```

Expected: `mod.rs` is below 5000 lines; audit no longer reports `cli-mod-large`.

## Task 3: Split App Router Source Renderer

**Files:**
- Modify: `dx-www/src/cli/app_router_execution/source_render.rs`
- Create: `dx-www/src/cli/app_router_execution/source_render_parts/client_component.rs`
- Create: `dx-www/src/cli/app_router_execution/source_render_parts/static_expression.rs`
- Create: `dx-www/src/cli/app_router_execution/source_render_parts/static_markup.rs`

- [x] **Step 1: Add include slices**

Create a `source_render_parts/` subfolder and include the extracted slices from
`source_render.rs`:

```rust
include!("source_render_parts/client_component.rs");
include!("source_render_parts/static_markup.rs");
include!("source_render_parts/static_expression.rs");
```

- [x] **Step 2: Extract client-component rendering helpers**

Move helpers that generate client component scripts, event bindings, state reflection, and hydration snippets into `client_component.rs`.

- [x] **Step 3: Extract static expression helpers**

Move static literal/expression helpers into `static_expression.rs`.

- [x] **Step 4: Extract static markup helpers**

Move pure HTML/string rendering helpers into `static_markup.rs`.

- [x] **Step 5: Verify line budget**

Run:

```powershell
(Get-Content dx-www\src\cli\app_router_execution\source_render.rs).Count
cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www
```

Expected: parent file below 4000 lines; compiler catches any visibility misses.

## Task 4: Split Ecosystem Receipt And Project Check Files

**Files:**
- Modify: `core/src/ecosystem/dx_check_receipt.rs`
- Create: `core/src/ecosystem/dx_check_receipt/panel.rs`
- Create: `core/src/ecosystem/dx_check_receipt/panel_parts/package_lanes.rs`
- Create: `core/src/ecosystem/dx_check_receipt/panel_parts/package_metrics.rs`
- Create: `core/src/ecosystem/dx_check_receipt/panel_parts/status_actions.rs`
- Create: `core/src/ecosystem/dx_check_receipt/panel_parts/style_evidence.rs`
- Create: `core/src/ecosystem/dx_check_receipt/panel_parts/tests_a.rs`
- Create: `core/src/ecosystem/dx_check_receipt/panel_parts/tests_b.rs`
- Modify: `core/src/ecosystem/project_check.rs`
- Create: `core/src/ecosystem/project_check/readiness.rs`
- Create: `core/src/ecosystem/project_check/readiness_parts/dx_style.rs`
- Create: `core/src/ecosystem/project_check/readiness_parts/forge.rs`
- Create: `core/src/ecosystem/project_check/readiness_parts/scoring.rs`
- Create: `core/src/ecosystem/project_check/readiness_parts/tests.rs`

- [x] **Step 1: Split receipt panel view model**

Move panel row structs, marker constants, and view-model builders into `dx_check_receipt/panel.rs`.

- [x] **Step 2: Split receipt package metrics and actions**

Move package lane, metric, status action, and style evidence helpers into
`dx_check_receipt/panel_parts/*.rs`.

- [x] **Step 3: Split receipt tests**

Move the receipt panel tests into `dx_check_receipt/panel_parts/tests_a.rs` and
`dx_check_receipt/panel_parts/tests_b.rs`.

- [x] **Step 4: Split project check style and Forge lanes**

Move dx-style receipt consumption helpers into
`project_check/readiness_parts/dx_style.rs` and Forge/package-readiness helpers
into `project_check/readiness_parts/forge.rs`.

- [x] **Step 5: Split project readiness summary**

Move final readiness score/status assembly into `project_check/readiness.rs` and
`project_check/readiness_parts/scoring.rs`.

- [x] **Step 6: Verify line budgets**

Run:

```powershell
(Get-Content core\src\ecosystem\dx_check_receipt.rs).Count
(Get-Content core\src\ecosystem\project_check.rs).Count
cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www
```

Expected: both parent files below 4000 lines.

## Task 5: Split Forge Registry

**Files:**
- Modify: `core/src/ecosystem/forge_registry.rs`
- Create: `core/src/ecosystem/forge_registry_parts/package_templates.rs`
- Create: `core/src/ecosystem/forge_registry_parts/package_lanes.rs`
- Create: `core/src/ecosystem/forge_registry_parts/registry_config.rs`
- Create: `core/src/ecosystem/forge_registry_parts/registry_operations.rs`
- Create: `core/src/ecosystem/forge_registry_parts/registry_receipts.rs`
- Create: `core/src/ecosystem/forge_registry_parts/remote_head_execution.rs`
- Create: `core/src/ecosystem/forge_registry_parts/tests.rs`
- Create: `core/src/ecosystem/forge_registry_parts/tests/a.rs`
- Create: `core/src/ecosystem/forge_registry_parts/tests/b.rs`

- [x] **Step 1: Extract package templates**

Move package template helpers into `forge_registry_parts/package_templates.rs`.

- [x] **Step 2: Extract lane grouping helpers**

Move category/lane grouping and lookup helpers into
`forge_registry_parts/package_lanes.rs`.

- [x] **Step 3: Extract registry operations, config, receipts, and remote-head helpers**

Move registry config, operation markdown, hash/status/receipt helpers, remote
HEAD execution helpers, and tests into `forge_registry_parts/*.rs`.

- [x] **Step 4: Verify line budget**

Run:

```powershell
(Get-Content core\src\ecosystem\forge_registry.rs).Count
cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www
```

Expected: parent file below 4000 lines.

## Task 6: Final Audit, Readiness, And Commit

**Files:**
- Modify: `docs/repo-hygiene.md`
- Modify: `README.md` only if it contains stale hygiene-readiness language.

- [x] **Step 1: Update docs to describe closed scorecard**

Move closed items into a `Closed Scorecard Items` section. Keep the distinction between hygiene readiness and product/browser/provider readiness.

- [x] **Step 2: Run focused verification**

Run:

```powershell
node --test benchmarks\repo-hygiene-audit.test.ts
node tools\hygiene\audit-repo-hygiene.ts --json
cargo fmt --check
cargo check -j 1 -p dx-www --no-default-features --features cli --bin dx-www
git diff --check
rg -n --glob "!**/target/**" --glob "!**/node_modules/**" --glob "!**/.dx/**" --glob "!**/www/output/**" --glob "!**/.git/**" "^(<<<<<<<|=======|>>>>>>>)"
```

Expected: audit reports `readyFor100: true`, `activeFlawCount: 0`, `scorecardOpen: 0`.

- [x] **Step 3: Commit only related files**

Actual closeout:

```powershell
git show --name-only --format="%h %s" a806f4bb
```

Commit `a806f4bb` contains the hygiene audit/tooling/docs and the actual split
module files listed above.

Do not stage `docs/DX_WWW_CURRENT_FEATURE_DOSSIER_2026-05-28.md` unless the user explicitly asks for it.

## Historical Six-Agent Implementation Split

These assignments describe the completed implementation split for commit
`a806f4bb`. They are not current lane ownership.

1. CLI split agent: `dx-www/src/cli/mod.rs` only.
2. App Router renderer agent: `dx-www/src/cli/app_router_execution/source_render.rs` only.
3. Receipt/project-check agent: `core/src/ecosystem/dx_check_receipt.rs` and `core/src/ecosystem/project_check.rs`.
4. Forge registry agent: `core/src/ecosystem/forge_registry.rs` only.
5. Fixture/legacy audit agent: `tools/hygiene/audit-repo-hygiene.ts`, `benchmarks/repo-hygiene-audit.test.ts`, and `docs/hygiene/*`.
6. Verification agent: focused checks, scorecard consistency, and final diff review.

## Self-Review

- Spec coverage: every active scorecard id maps to a task.
- Placeholder scan: this plan does not use `TBD`, `TODO`, or unspecified test commands.
- Type consistency: all audit status names match the current `auditRepoHygiene()` shape: `scorecard`, `readiness`, `readyFor100`, `scorecardOpen`, `activeFlawIds`.
